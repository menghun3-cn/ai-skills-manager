use crate::models::{MarketDataResult, MarketItem, Skill, SkillBinding};
use crate::services::config::{get_app_dir, get_skills_dir};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use tauri::AppHandle;

const CACHE_EXPIRY_HOURS: u64 = 24;

#[derive(Debug, Deserialize)]
struct GithubContentItem {
    name: String,
    #[serde(rename = "type")]
    item_type: String,
    #[serde(default)]
    download_url: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct MarketCache {
    items: Vec<MarketItem>,
    timestamp: String,
}

#[derive(Deserialize)]
struct GitTreeEntry {
    path: String,
    #[serde(rename = "type")]
    entry_type: String,
}

#[derive(Deserialize)]
struct GitTree {
    tree: Vec<GitTreeEntry>,
}

#[derive(Deserialize)]
struct CommitInfo {
    sha: String,
}

fn to_title_case(s: &str) -> String {
    s.split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_frontmatter_name(content: &str) -> Option<String> {
    let trimmed = content.trim();
    if !trimmed.starts_with("---") {
        return None;
    }
    let rest = &trimmed[3..];
    let end_idx = rest.find("---")?;
    let frontmatter = &rest[..end_idx];
    for line in frontmatter.lines() {
        let line = line.trim();
        if let Some(val) = line.strip_prefix("name:") {
            return Some(
                val.trim()
                    .trim_matches('"')
                    .trim_matches('\'')
                    .to_string(),
            );
        }
    }
    None
}

fn parse_skill_md_frontmatter(content: &str) -> (Option<String>, Option<String>) {
    let trimmed = content.trim();
    if !trimmed.starts_with("---") {
        return (None, None);
    }
    let rest = &trimmed[3..];
    if let Some(end_idx) = rest.find("---") {
        let frontmatter = &rest[..end_idx];
        let mut description = None;
        let mut version = None;
        for line in frontmatter.lines() {
            let line = line.trim();
            if let Some(val) = line.strip_prefix("description:") {
                description = Some(val.trim().trim_matches('"').trim_matches('\'').to_string());
            } else if let Some(val) = line.strip_prefix("version:") {
                version = Some(val.trim().trim_matches('"').trim_matches('\'').to_string());
            }
        }
        (description, version)
    } else {
        (None, None)
    }
}

async fn discover_skill_path_in_repo(
    client: &reqwest::Client,
    owner_repo: &str,
    skill_id: &str,
    github_token: Option<&str>,
) -> Result<(String, String), String> {
    log::info!("[discover_skill_path] owner_repo={}, skill_id={}", owner_repo, skill_id);

    let commits_url = format!("https://api.github.com/repos/{}/commits/HEAD", owner_repo);
    let mut commits_req = client
        .get(&commits_url)
        .header("User-Agent", "AI-Skills-Manager/1.0")
        .header("Accept", "application/vnd.github.v3+json");
    
    if let Some(token) = github_token {
        commits_req = commits_req.header("Authorization", format!("Bearer {}", token));
    }
    
    let commit_sha = match commits_req.send().await
    {
        Ok(r) if r.status().is_success() => r
            .json::<CommitInfo>()
            .await
            .map(|c| c.sha)
            .unwrap_or_else(|_| "HEAD".to_string()),
        _ => "HEAD".to_string(),
    };
    log::info!("[discover_skill_path] commit_sha={}", commit_sha);

    let tree_url = format!(
        "https://api.github.com/repos/{}/git/trees/{}?recursive=1",
        owner_repo, commit_sha
    );
    let mut tree_req = client
        .get(&tree_url)
        .header("User-Agent", "AI-Skills-Manager/1.0")
        .header("Accept", "application/vnd.github.v3+json");
    
    if let Some(token) = github_token {
        tree_req = tree_req.header("Authorization", format!("Bearer {}", token));
    }
    
    let tree_resp = tree_req.send()
        .await
        .map_err(|e| format!("获取仓库文件树失败: {}", e))?;
    
    let tree_status = tree_resp.status();
    let tree_text = tree_resp
        .text()
        .await
        .map_err(|e| format!("读取文件树响应失败: {}", e))?;
    
    log::info!("[discover_skill_path] Tree API status: {}", tree_status);
    
    if !tree_status.is_success() {
        return Err(format!("GitHub Tree API 返回错误: HTTP {} - {}", tree_status, tree_text));
    }
    
    let tree: GitTree = serde_json::from_str(&tree_text)
        .map_err(|e| {
            log::error!("[discover_skill_path] 解析文件树失败: {}", e);
            log::error!("[discover_skill_path] 响应内容: {}", &tree_text[..tree_text.len().min(500)]);
            format!("解析文件树失败: {}", e)
        })?;

    let skill_md_paths: Vec<String> = tree
        .tree
        .into_iter()
        .filter(|e| {
            e.entry_type == "blob"
                && (e.path.ends_with("/SKILL.md") || e.path == "SKILL.md")
        })
        .map(|e| e.path)
        .collect();

    if skill_md_paths.is_empty() {
        return Err(format!("仓库 {} 中未找到任何 SKILL.md 文件", owner_repo));
    }

    log::info!("[discover_skill_path] 找到 {} 个 SKILL.md", skill_md_paths.len());

    let skill_id_lower = skill_id.to_lowercase();

    for path in &skill_md_paths {
        let dir_name = std::path::Path::new(path)
            .parent()
            .and_then(|p| p.file_name())
            .map(|n| n.to_string_lossy().to_lowercase())
            .unwrap_or_default();

        if dir_name == skill_id_lower {
            let parent = std::path::Path::new(path)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default();
            log::info!("[discover_skill_path] 按目录名匹配: {} → {}", skill_id, parent);
            return Ok((parent, commit_sha));
        }
    }

    for path in &skill_md_paths {
        let raw_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}",
            owner_repo, commit_sha, path
        );

        if let Ok(resp) = client
            .get(&raw_url)
            .header("User-Agent", "AI-Skills-Manager/1.0")
            .send()
            .await
        {
            if resp.status().is_success() {
                if let Ok(content) = resp.text().await {
                    if let Some(name) = extract_frontmatter_name(&content) {
                        if name.to_lowercase() == skill_id_lower {
                            let parent = std::path::Path::new(path)
                                .parent()
                                .map(|p| p.to_string_lossy().to_string())
                                .unwrap_or_default();
                            log::info!(
                                "[discover_skill_path] 按 frontmatter name 匹配: {} → {}",
                                skill_id, parent
                            );
                            return Ok((parent, commit_sha));
                        }
                    }
                }
            }
        }
    }

    Err(format!(
        "在仓库 {} 中未找到名为 '{}' 的 Skill",
        owner_repo, skill_id
    ))
}

fn is_cache_valid(timestamp: &str) -> bool {
    if let Ok(cache_time) = chrono::DateTime::parse_from_rfc3339(timestamp) {
        let now = chrono::Utc::now();
        let duration = now.signed_duration_since(cache_time);
        return duration.num_hours() < CACHE_EXPIRY_HOURS as i64;
    }
    false
}

pub async fn get_market_data(app: &AppHandle) -> Result<Vec<MarketItem>, String> {
    let result = get_market_data_with_cache(app).await?;
    Ok(result.items)
}

pub async fn get_market_data_with_cache(app: &AppHandle) -> Result<MarketDataResult, String> {
    let skills_dir = get_skills_dir(app).ok();
    let installed_skill_names = if let Some(ref dir) = skills_dir {
        get_installed_skill_names(dir)
    } else {
        std::collections::HashSet::new()
    };

    if let Some(cached_items) = load_cache(app)? {
        if is_cache_valid(&cached_items.timestamp) {
            log::info!("Using cached market data");
            let items = cached_items.items.into_iter().map(|mut item| {
                let skill_name = item.name.replace(" ", "-").to_lowercase();
                item.installed = Some(installed_skill_names.contains(&skill_name));
                item
            }).collect();
            return Ok(MarketDataResult {
                items,
                cached: true,
            });
        }
    }

    log::info!("Fetching fresh market data...");
    let mut all_items = Vec::new();

    match fetch_awesome_claude_skills().await {
        Ok(items) => all_items.extend(items),
        Err(e) => log::warn!("Failed to fetch awesome-claude-skills: {}", e),
    }

    match fetch_skills_sh().await {
        Ok(items) => all_items.extend(items),
        Err(e) => log::warn!("Failed to fetch skills.sh: {}", e),
    }

    if all_items.is_empty() {
        if let Some(cached_items) = load_cache(app)? {
            log::warn!("API failed, using stale cache");
            let items = cached_items.items.into_iter().map(|mut item| {
                let skill_name = item.name.replace(" ", "-").to_lowercase();
                item.installed = Some(installed_skill_names.contains(&skill_name));
                item
            }).collect();
            return Ok(MarketDataResult {
                items,
                cached: true,
            });
        }
        return Err("No market data available".to_string());
    }

    let items_with_installed_status: Vec<MarketItem> = all_items.into_iter().map(|mut item| {
        let skill_name = item.name.replace(" ", "-").to_lowercase();
        item.installed = Some(installed_skill_names.contains(&skill_name));
        item
    }).collect();

    save_cache(app, &items_with_installed_status)?;

    Ok(MarketDataResult {
        items: items_with_installed_status,
        cached: false,
    })
}

fn get_installed_skill_names(skills_dir: &std::path::Path) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    if let Ok(entries) = fs::read_dir(skills_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            if entry.path().is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    names.insert(name.to_lowercase());
                }
            }
        }
    }
    names
}

fn get_cache_path(app: &AppHandle) -> Result<std::path::PathBuf, String> {
    let app_dir = get_app_dir(app)?;
    Ok(app_dir.join("market_cache.json"))
}

fn load_cache(app: &AppHandle) -> Result<Option<MarketCache>, String> {
    let cache_path = get_cache_path(app)?;

    if !cache_path.exists() {
        return Ok(None);
    }

    let content = fs::read_to_string(&cache_path).map_err(|e| e.to_string())?;
    let cache: MarketCache = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    Ok(Some(cache))
}

fn save_cache(app: &AppHandle, items: &[MarketItem]) -> Result<(), String> {
    let app_dir = get_app_dir(app)?;
    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;

    let cache = MarketCache {
        items: items.to_vec(),
        timestamp: chrono::Utc::now().to_rfc3339(),
    };

    let content = serde_json::to_string_pretty(&cache).map_err(|e| e.to_string())?;
    let cache_path = get_cache_path(app)?;
    fs::write(cache_path, content).map_err(|e| e.to_string())?;

    log::info!("Market cache saved");
    Ok(())
}

async fn fetch_awesome_claude_skills() -> Result<Vec<MarketItem>, String> {
    let client = reqwest::Client::builder()
        .user_agent("AI-Skills-Manager/1.0")
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| e.to_string())?;

    let response = client
        .get("https://api.github.com/repos/ComposioHQ/awesome-claude-skills/contents")
        .header("Accept", "application/vnd.github.v3+json")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()));
    }

    let contents: Vec<GithubContentItem> = response.json().await.map_err(|e| e.to_string())?;

    let items: Vec<MarketItem> = contents
        .into_iter()
        .filter(|item| item.item_type == "dir" && !item.name.starts_with('.') && item.name != "composio-skills" && item.name != "connect-apps-plugin")
        .map(|item| {
            let skill_id = item.name.to_lowercase().replace(" ", "-");
            // 尝试获取 SKILL.md 文件的内容来提取描述
            let description = None; // 暂时保持 None，后续可以通过异步获取
            MarketItem {
                id: format!("awesome-{}", skill_id),
                name: to_title_case(&item.name),
                description,
                author: None,
                source: "awesome-claude-skills".to_string(),
                downloads: None,
                installed: Some(false),
                repo: Some(format!("https://github.com/ComposioHQ/awesome-claude-skills/tree/main/{}", item.name)),
                skill_id: Some(skill_id),
            }
        })
        .collect();

    Ok(items)
}

#[derive(Deserialize)]
struct SkillsShSearchResponse {
    skills: Vec<SkillsShSkill>,
}

#[derive(Deserialize)]
struct SkillsShSkill {
    #[serde(rename = "skillId")]
    skill_id: String,
    name: String,
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    installs: Option<u64>,
    source: String,
}

fn skill_to_market_item(s: SkillsShSkill) -> Option<MarketItem> {
    log::debug!("[skill_to_market_item] Input: name='{}', skill_id='{}', source='{}', installs={:?}",
        s.name, s.skill_id, s.source, s.installs);

    // source 格式: "owner/repo"
    let repo = s.source;
    let skill_id = s.skill_id;

    if repo.is_empty() || skill_id.is_empty() {
        log::warn!("[skill_to_market_item] Skipped: empty repo or skill_id");
        return None;
    }

    let item = MarketItem {
        id: format!("skills-sh-{}", skill_id.to_lowercase().replace(" ", "-")),
        name: s.name,
        description: s.description,
        author: None,
        source: "skills.sh".to_string(),
        downloads: s.installs.map(|d| d as u32),
        installed: Some(false),
        repo: Some(repo.clone()),
        skill_id: Some(skill_id.clone()),
    };
    log::debug!("[skill_to_market_item] Created item: id='{}', source='{}'", item.id, item.source);
    Some(item)
}

async fn fetch_skills_sh_search(client: &reqwest::Client, query: &str) -> Result<Vec<MarketItem>, String> {
    let url = format!("https://skills.sh/api/search?q={}&limit=50", query);
    log::info!("[fetch_skills_sh_search] Requesting: {}", url);

    let response = match client.get(&url).send().await {
        Ok(r) if r.status().is_success() => {
            log::info!("[fetch_skills_sh_search] HTTP {} for query '{}'", r.status(), query);
            r
        }
        Ok(r) => {
            log::warn!("[fetch_skills_sh_search] HTTP {} for query '{}'", r.status(), query);
            return Ok(Vec::new());
        }
        Err(e) => {
            log::warn!("[fetch_skills_sh_search] Request failed: {}", e);
            return Ok(Vec::new());
        }
    };

    let body = response.text().await.map_err(|e| e.to_string())?;
    log::info!("[fetch_skills_sh_search] Query '{}' response: {}", query, &body[..body.len().min(500)]);

    let data: SkillsShSearchResponse = serde_json::from_str(&body).map_err(|e| {
        log::error!("[fetch_skills_sh_search] JSON parse error: {}", e);
        e.to_string()
    })?;

    let items: Vec<MarketItem> = data.skills.into_iter().filter_map(skill_to_market_item).collect();
    log::info!("[fetch_skills_sh_search] Query '{}' converted {} items", query, items.len());
    Ok(items)
}

async fn fetch_skills_sh() -> Result<Vec<MarketItem>, String> {
    let client = reqwest::Client::builder()
        .user_agent("AI-Skills-Manager/1.0")
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;

    let mut all_skills: std::collections::HashMap<String, MarketItem> = std::collections::HashMap::new();

    // 搜索 API: q 参数最少需要 2 个字符
    // 使用常见的 2 字符组合来获取数据
    let search_queries = [
        "re", "sk", "de", "co", "an", "pr", "te", "be", "ma", "in",
        "ge", "ch", "li", "st", "fi", "wo", "sh", "lo", "ca", "ex",
        "ve", "go", "ru", "se", "cl", "ju", "qu", "gr", "ba", "me",
        "ho", "da", "mi", "no", "so", "la", "we", "do", "ti", "ri",
        "pi", "vi", "ki", "di", "bi", "wi", "zi", "xi", "ci", "vi"
    ];

    for query in search_queries.iter() {
        match fetch_skills_sh_search(&client, query).await {
            Ok(items) => {
                log::info!("[fetch_skills_sh] Query '{}' returned {} items", query, items.len());
                for item in items {
                    all_skills.insert(item.id.clone(), item);
                }
            }
            Err(e) => {
                log::warn!("[fetch_skills_sh] Query '{}' failed: {}", query, e);
                continue;
            }
        }
    }

    log::info!("[fetch_skills_sh] Fetched {} unique skills from skills.sh", all_skills.len());
    Ok(all_skills.into_values().collect())
}

pub async fn install_skill(app: &AppHandle, market_id: String, github_token: Option<String>) -> Result<bool, String> {
    let cache = load_cache(app)?
        .ok_or_else(|| "Market cache not found. Please refresh the market.".to_string())?;

    let item = cache.items.iter()
        .find(|i| i.id == market_id)
        .ok_or_else(|| "Skill not found in market".to_string())?;

    let skills_dir = get_skills_dir(app)?;
    fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;

    let skill_name = item.name.replace(" ", "-").to_lowercase();
    let skill_dir = skills_dir.join(&skill_name);

    if skill_dir.exists() {
        return Err(format!("Skill '{}' is already installed", item.name));
    }

    let github_repo = item.repo.as_ref()
        .ok_or_else(|| "Repository information not found for this skill".to_string())?;
    let skill_id = item.skill_id.as_ref()
        .ok_or_else(|| "Skill ID not found for this skill".to_string())?;

    let client = reqwest::Client::builder()
        .user_agent("AI-Skills-Manager/1.0")
        .timeout(Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;

    log::info!("[install_skill] 开始发现路径: repo={}, skill_id={}", github_repo, skill_id);

    let (source_path, _commit_sha) = discover_skill_path_in_repo(&client, github_repo, skill_id, github_token.as_deref()).await?;

    log::info!("[install_skill] 路径发现成功: path={}", source_path);

    let contents_url = if source_path.is_empty() {
        format!(
            "https://api.github.com/repos/{}/contents?ref=HEAD",
            github_repo
        )
    } else {
        format!(
            "https://api.github.com/repos/{}/contents/{}?ref=HEAD",
            github_repo, source_path
        )
    };

    log::info!("[install_skill] Contents API: {}", contents_url);
    let mut contents_req = client
        .get(&contents_url)
        .header("Accept", "application/vnd.github.v3+json");
    
    if let Some(token) = github_token {
        contents_req = contents_req.header("Authorization", format!("Bearer {}", token));
    }
    
    let resp = contents_req.send()
        .await
        .map_err(|e| format!("Contents API 请求失败: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Contents API 返回 HTTP {}", resp.status()));
    }

    let entries: Vec<GithubContentItem> = resp
        .json()
        .await
        .map_err(|e| format!("解析 Contents API 响应失败: {}", e))?;

    fs::create_dir_all(&skill_dir).map_err(|e| e.to_string())?;

    let mut skill_md_content: Option<String> = None;
    let mut files_downloaded = 0usize;

    for entry in &entries {
        if entry.item_type != "file" {
            continue;
        }

        let fname = &entry.name;
        if fname == "README.md"
            || fname == "metadata.json"
            || fname.starts_with('_')
            || fname.starts_with('.')
        {
            continue;
        }

        let download_url = match &entry.download_url {
            Some(u) if !u.is_empty() => u.clone(),
            _ => {
                log::info!("[install_skill] 跳过 {} (无 download_url)", fname);
                continue;
            }
        };

        let file_resp = client
            .get(&download_url)
            .header("User-Agent", "AI-Skills-Manager/1.0")
            .send()
            .await
            .map_err(|e| format!("下载文件 {} 失败: {}", fname, e))?;

        if !file_resp.status().is_success() {
            log::info!("[install_skill] 跳过 {} (HTTP {})", fname, file_resp.status());
            continue;
        }

        let bytes = file_resp.bytes().await
            .map_err(|e| format!("读取文件 {} 失败: {}", fname, e))?;

        let file_path = skill_dir.join(fname);
        fs::write(&file_path, &bytes).map_err(|e| e.to_string())?;
        files_downloaded += 1;

        if fname == "SKILL.md" {
            skill_md_content = String::from_utf8(bytes.to_vec()).ok();
        }
    }

    log::info!("[install_skill] 下载完成: {} 个文件", files_downloaded);

    let (description, version) = if let Some(ref content) = skill_md_content {
        parse_skill_md_frontmatter(content)
    } else {
        (None, None)
    };

    let final_description = description.or_else(|| item.description.clone());
    let final_version = version.or_else(|| Some("1.0.0".to_string()));

    let metadata = Skill {
        name: item.name.clone(),
        description: final_description,
        version: final_version,
        author: item.author.clone(),
        bindings: SkillBinding {
            global: true,
            projects: Vec::new(),
        },
        enabled_tools: Vec::new(),
        enabled: true,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    };

    let metadata_path = skill_dir.join(".ai-skills-manager.json");
    let json = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    fs::write(&metadata_path, json).map_err(|e| e.to_string())?;

    // 注意：安装技能时不再自动创建软链接，用户需要在 Skills 页面或工具管理中手动启用
    log::info!("[install_skill] 技能 '{}' 安装完成，软链接需手动配置", skill_name);

    if let Ok(Some(mut cache)) = load_cache(app) {
        if let Some(installed_item) = cache.items.iter_mut().find(|i| i.id == market_id) {
            installed_item.installed = Some(true);
        }
        let _ = save_cache(app, &cache.items);
    }

    log::info!("Successfully installed skill: {}", item.name);
    Ok(true)
}

pub async fn clear_cache(app: &AppHandle) -> Result<bool, String> {
    let cache_path = get_cache_path(app)?;

    if cache_path.exists() {
        fs::remove_file(&cache_path).map_err(|e| e.to_string())?;
        log::info!("Market cache cleared");
    }

    Ok(true)
}

pub fn get_cache_age(app: &AppHandle) -> Result<Option<String>, String> {
    if let Some(cache) = load_cache(app)? {
        if let Ok(cache_time) = chrono::DateTime::parse_from_rfc3339(&cache.timestamp) {
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(cache_time);
            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            if hours > 0 {
                return Ok(Some(format!("{}h {}m ago", hours, minutes)));
            } else {
                return Ok(Some(format!("{}m ago", minutes)));
            }
        }
    }
    Ok(None)
}
