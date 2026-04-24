use crate::models::{Skill, SkillBinding, SkillFile};
use crate::services::config::{get_skills_dir, load_config};
use chrono::Utc;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use walkdir::WalkDir;

pub async fn get_skills(app: &AppHandle) -> Result<Vec<Skill>, String> {
    let skills_dir = get_skills_dir(app)?;
    let mut skills = Vec::new();

    if !skills_dir.exists() {
        return Ok(skills);
    }

    // 加载配置以获取工具信息
    let config = load_config(app).await.unwrap_or_default();

    for entry in fs::read_dir(&skills_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            let metadata_path = path.join(".ai-skills-manager.json");
            let mut skill = if metadata_path.exists() {
                let content = fs::read_to_string(&metadata_path).map_err(|e| e.to_string())?;
                log::info!("Loading skill metadata from {:?}: {}", metadata_path, content);
                let parsed: Result<Skill, _> = serde_json::from_str(&content);
                match parsed {
                    Ok(s) => {
                        log::info!("Parsed skill: name={}, description={:?}", s.name, s.description);
                        s
                    }
                    Err(e) => {
                        log::error!("Failed to parse skill metadata: {}, using default", e);
                        create_default_skill(&path)
                    }
                }
            } else {
                log::info!("No metadata file found at {:?}, using default", metadata_path);
                create_default_skill(&path)
            };
            
            // 检查技能在哪些工具中启用了软链接
            let skill_name_normalized = skill.name.replace(" ", "-").to_lowercase();
            let mut enabled_tool_names = Vec::new();
            
            for tool in &config.tools {
                if !tool.enabled {
                    continue;
                }
                let tool_skills_path = PathBuf::from(&tool.skills_path);
                let symlink_path = tool_skills_path.join(format!("{}@ai-skills-manager", skill_name_normalized));
                
                if symlink_path.exists() {
                    enabled_tool_names.push(tool.name.clone());
                }
            }
            
            skill.enabled_tools = enabled_tool_names;
            skills.push(skill);
        }
    }

    Ok(skills)
}

fn create_default_skill(path: &PathBuf) -> Skill {
    let name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();

    Skill {
        name,
        description: None,
        version: Some("1.0.0".to_string()),
        author: None,
        bindings: SkillBinding {
            global: true,
            projects: Vec::new(),
        },
        enabled_tools: Vec::new(),
        enabled: true,
        created_at: Utc::now().to_rfc3339(),
        updated_at: Utc::now().to_rfc3339(),
    }
}

pub async fn create_skill(app: &AppHandle, name: &str, description: &str) -> Result<Skill, String> {
    let skills_dir = get_skills_dir(app)?;
    let skill_dir = skills_dir.join(name);

    fs::create_dir_all(&skill_dir).map_err(|e| e.to_string())?;

    let now = Utc::now().to_rfc3339();
    let skill = Skill {
        name: name.to_string(),
        description: Some(description.to_string()),
        version: Some("1.0.0".to_string()),
        author: None,
        bindings: SkillBinding {
            global: true,
            projects: Vec::new(),
        },
        enabled_tools: Vec::new(),
        enabled: true,
        created_at: now.clone(),
        updated_at: now,
    };

    let metadata_path = skill_dir.join(".ai-skills-manager.json");
    let json = serde_json::to_string_pretty(&skill).map_err(|e| e.to_string())?;
    fs::write(&metadata_path, json).map_err(|e| e.to_string())?;

    Ok(skill)
}

pub async fn delete_skill(app: &AppHandle, name: &str) -> Result<bool, String> {
    let skills_dir = get_skills_dir(app)?;
    
    // 尝试使用原始名称
    let skill_dir = skills_dir.join(name);
    
    // 尝试使用转换后的名称（与安装时一致）
    let normalized_name = name.replace(" ", "-").to_lowercase();
    let normalized_skill_dir = skills_dir.join(&normalized_name);

    log::info!("Attempting to delete skill '{}' at path: {:?}", name, skill_dir);
    log::info!("Normalized name: '{}', path: {:?}", normalized_name, normalized_skill_dir);

    // 确定要删除的目录
    let dir_to_delete = if skill_dir.exists() {
        skill_dir
    } else if normalized_skill_dir.exists() {
        normalized_skill_dir
    } else {
        log::warn!("Skill directory does not exist for '{}' or '{}'", name, normalized_name);
        // 即使目录不存在，也返回成功，因为目标状态（skill 不存在）已经达成
        return Ok(true);
    };

    // 检查是否是目录
    if !dir_to_delete.is_dir() {
        return Err(format!("{:?} is not a directory", dir_to_delete));
    }

    // 尝试删除目录及其所有内容
    match fs::remove_dir_all(&dir_to_delete) {
        Ok(_) => {
            log::info!("Successfully deleted skill directory: {:?}", dir_to_delete);
        }
        Err(e) => {
            log::error!("Failed to delete skill directory {:?}: {}", dir_to_delete, e);
            return Err(format!("Failed to delete skill '{}': {}. Please check file permissions.", name, e));
        }
    }

    Ok(true)
}

pub async fn update_skill(app: &AppHandle, skill: &Skill) -> Result<Skill, String> {
    let skills_dir = get_skills_dir(app)?;
    let skill_dir = skills_dir.join(&skill.name);
    let metadata_path = skill_dir.join(".ai-skills-manager.json");

    let mut updated_skill = skill.clone();
    updated_skill.updated_at = Utc::now().to_rfc3339();

    let json = serde_json::to_string_pretty(&updated_skill).map_err(|e| e.to_string())?;
    fs::write(&metadata_path, json).map_err(|e| e.to_string())?;

    Ok(updated_skill)
}

pub async fn get_skill_files(app: &AppHandle, skill_name: &str) -> Result<Vec<SkillFile>, String> {
    let skills_dir = get_skills_dir(app)?;
    let skill_dir = skills_dir.join(skill_name);
    let mut files = Vec::new();

    if !skill_dir.exists() {
        return Ok(files);
    }

    for entry in WalkDir::new(&skill_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name != ".ai-skills-manager.json" {
                files.push(SkillFile {
                    name: name.to_string(),
                    path: path.to_string_lossy().to_string(),
                });
            }
        }
    }

    Ok(files)
}

/// 创建 HTTP 客户端，支持代理配置
fn create_http_client(proxy_url: Option<&str>) -> Result<reqwest::Client, String> {
    let mut builder = reqwest::Client::builder()
        .user_agent("AI-Skills-Manager/1.0")
        .timeout(std::time::Duration::from_secs(60));

    // 如果提供了代理地址，配置代理
    if let Some(proxy) = proxy_url {
        if !proxy.is_empty() {
            let proxy = reqwest::Proxy::all(proxy)
                .map_err(|e| format!("代理配置错误: {}", e))?;
            builder = builder.proxy(proxy);
        }
    }

    builder.build().map_err(|e| e.to_string())
}

/// 从 GitHub 导入单个 skill
pub async fn import_skill_from_github(
    app: &AppHandle,
    owner: &str,
    repo: &str,
) -> Result<bool, String> {
    // 加载配置获取代理设置
    let config = crate::services::config::load_config(app).await.unwrap_or_default();
    let proxy_url = config.settings.proxy_url.as_deref();

    let skills_dir = get_skills_dir(app)?;
    fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;

    let client = create_http_client(proxy_url)?;

    // 尝试常见的默认分支
    let branches = vec!["main", "master"];
    let mut default_branch = "main";
    let mut found_skill: Option<(String, String, String)> = None;

    // 定义可能包含 skills 的目录列表（按优先级排序）
    let possible_paths = vec![
        "",           // 根目录
        "skills",     // skills 目录
        "skill",      // skill 目录
        "src",        // src 目录
        "lib",        // lib 目录
        "packages",   // packages 目录
    ];

    // 尝试从各个分支和目录查找 SKILL.md
    for branch in &branches {
        for path in &possible_paths {
            let skill_md_url = if path.is_empty() {
                format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/SKILL.md",
                    owner, repo, branch
                )
            } else {
                format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}/SKILL.md",
                    owner, repo, branch, path
                )
            };

            let resp = client.get(&skill_md_url).send().await;
            if let Ok(resp) = resp {
                if resp.status().is_success() {
                    if let Ok(content) = resp.text().await {
                        let name = extract_name_from_skill_md(&content)
                            .unwrap_or_else(|| {
                                if path.is_empty() {
                                    repo.to_string()
                                } else {
                                    path.to_string()
                                }
                            });
                        found_skill = Some((name, content, path.to_string()));
                        default_branch = branch;
                        break;
                    }
                }
            }
        }
        if found_skill.is_some() {
            break;
        }
    }

    let (skill_name, skill_md_content, source_path) = found_skill.ok_or_else(|| {
        format!(
            "在仓库 {}/{} 中未找到 SKILL.md 文件。请确保仓库根目录或子目录中包含 SKILL.md 文件。",
            owner, repo
        )
    })?;

    // 规范化 skill 名称
    let skill_name_normalized = skill_name.replace(" ", "-").to_lowercase();
    let skill_dir = skills_dir.join(&skill_name_normalized);

    if skill_dir.exists() {
        return Err(format!("Skill '{}' 已经存在", skill_name));
    }

    // 创建 skill 目录
    fs::create_dir_all(&skill_dir).map_err(|e| e.to_string())?;

    // 下载 skill 文件
    // 由于 GitHub API 有速率限制，我们尝试下载一些常见的 skill 文件
    // 而不是通过 API 获取目录列表
    let files_to_download: Vec<String> = vec![
        "SKILL.md".to_string(),
        "README.md".to_string(),
        "rules.md".to_string(),
        "prompt.md".to_string(),
        "system.md".to_string(),
    ];

    let mut downloaded_count = 0;

    for file_name in files_to_download {
        // 跳过不需要的文件
        if file_name == "README.md"
            || file_name == "metadata.json"
            || file_name.starts_with('_')
            || file_name.starts_with('.')
        {
            continue;
        }

        let file_url = if source_path.is_empty() {
            format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}",
                owner, repo, default_branch, file_name
            )
        } else {
            format!(
                "https://raw.githubusercontent.com/{}/{}/{}/{}/{}",
                owner, repo, default_branch, source_path, file_name
            )
        };

        let file_resp = client.get(&file_url).send().await;

        if let Ok(resp) = file_resp {
            if resp.status().is_success() {
                if let Ok(bytes) = resp.bytes().await {
                    let file_path = skill_dir.join(&file_name);
                    if fs::write(&file_path, &bytes).is_ok() {
                        downloaded_count += 1;
                    }
                }
            }
        }
    }

    if downloaded_count == 0 {
        // 如果没有下载任何文件，删除创建的目录
        let _ = fs::remove_dir_all(&skill_dir);
        return Err("未能下载任何文件".to_string());
    }

    // 解析 SKILL.md 获取描述和版本
    let (description, version) = parse_skill_md_frontmatter(&skill_md_content);

    // 创建 metadata
    let metadata = crate::models::Skill {
        name: skill_name.clone(),
        description,
        version: version.or_else(|| Some("1.0.0".to_string())),
        author: Some(owner.to_string()),
        bindings: crate::models::SkillBinding {
            global: true,
            projects: Vec::new(),
        },
        enabled_tools: Vec::new(),
        enabled: true,
        created_at: chrono::Utc::now().to_rfc3339(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    };

    let metadata_path = skill_dir.join(".ai-skills-manager.json");
    let json = serde_json::to_string_pretty(&metadata).map_err(|e| e.to_string())?;
    fs::write(&metadata_path, json).map_err(|e| e.to_string())?;

    log::info!(
        "Successfully imported skill '{}' from GitHub {}/{}",
        skill_name, owner, repo
    );

    Ok(true)
}

/// 发现的 Skill 信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DiscoveredSkill {
    pub name: String,
    pub path: String,
    pub description: Option<String>,
}

/// 从 GitHub 仓库发现可用的 skills（不实际导入）
pub async fn discover_skills_from_github_repo(
    app: &AppHandle,
    owner: &str,
    repo: &str,
) -> Result<Vec<DiscoveredSkill>, String> {
    // 加载配置获取代理设置
    let config = crate::services::config::load_config(app).await.unwrap_or_default();
    let proxy_url = config.settings.proxy_url.as_deref();

    let client = create_http_client(proxy_url)?;

    // 尝试常见的默认分支
    let branches = vec!["main", "master"];
    let mut discovered_skills: Vec<DiscoveredSkill> = Vec::new();

    // 尝试从 skills 目录发现多个 skills
    for branch in &branches {
        // 首先尝试通过 GitHub API 获取 skills 目录的内容列表
        let api_url = format!(
            "https://api.github.com/repos/{}/{}/contents/skills?ref={}",
            owner, repo, branch
        );
        
        let mut request = client
            .get(&api_url)
            .header("User-Agent", "AI-Skills-Manager");
        
        // 如果配置了 GitHub Token，使用它进行认证以避免速率限制
        if let Some(ref token) = config.github_token {
            request = request.header("Authorization", format!("Bearer {}", token));
            log::info!("Using GitHub token for API authentication");
        }
        
        let resp = request.send().await;
        
        if let Ok(resp) = resp {
            let status = resp.status();
            log::info!("GitHub API response status: {}", status);
            
            if status.is_success() {
                match resp.json::<Vec<serde_json::Value>>().await {
                    Ok(contents) => {
                        log::info!("Found {} items in skills directory", contents.len());
                        // 遍历目录中的所有子目录
                        for item in contents {
                            if let Some(item_type) = item.get("type").and_then(|t| t.as_str()) {
                                if item_type == "dir" {
                                    if let Some(skill_name) = item.get("name").and_then(|n| n.as_str()) {
                                        log::info!("Found skill directory: {}", skill_name);
                                        // 检查该目录下是否有 SKILL.md
                                        let skill_md_url = format!(
                                            "https://raw.githubusercontent.com/{}/{}/{}/skills/{}/SKILL.md",
                                            owner, repo, branch, skill_name
                                        );
                                        
                                        let resp = client.get(&skill_md_url).send().await;
                                        if let Ok(resp) = resp {
                                            if resp.status().is_success() {
                                                if let Ok(content) = resp.text().await {
                                                    let name = extract_name_from_skill_md(&content)
                                                    .unwrap_or_else(|| skill_name.to_string());
                                                let (description, _) = parse_skill_md_frontmatter(&content);
                                                log::info!("Successfully discovered skill: {}", name);
                                                discovered_skills.push(DiscoveredSkill {
                                                    name,
                                                    path: format!("skills/{}", skill_name),
                                                    description,
                                                });
                                                }
                                            } else {
                                                log::warn!("SKILL.md not found for skill: {}", skill_name);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to parse GitHub API response: {}", e);
                    }
                }
            } else {
                let status_code = status.as_u16();
                log::warn!("GitHub API returned status: {}", status_code);
                
                if status_code == 403 {
                    // API 被限制，尝试其他方法
                    log::warn!("GitHub API rate limited, falling back to alternative discovery methods");
                } else if status_code == 404 {
                    log::warn!("Skills directory not found in repository");
                }
            }
        } else if let Err(e) = resp {
            log::error!("Failed to call GitHub API: {}", e);
        }

        // 如果 API 调用失败或被限制，尝试从 README 解析
        if discovered_skills.is_empty() {
            // 尝试获取 skills/README.md 来解析目录结构
            let readme_url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/skills/README.md",
                owner, repo, branch
            );
            
            let resp = client.get(&readme_url).send().await;
            if let Ok(resp) = resp {
                if resp.status().is_success() {
                    if let Ok(content) = resp.text().await {
                        // 从 README 中解析 skill 列表
                        let skills_from_readme = parse_skills_from_readme(&content);
                        for skill_name in skills_from_readme {
                            let skill_md_url = format!(
                                "https://raw.githubusercontent.com/{}/{}/{}/skills/{}/SKILL.md",
                                owner, repo, branch, skill_name
                            );
                            
                            let resp = client.get(&skill_md_url).send().await;
                            if let Ok(resp) = resp {
                                if resp.status().is_success() {
                                    if let Ok(content) = resp.text().await {
                                        let name = extract_name_from_skill_md(&content)
                                            .unwrap_or_else(|| skill_name.clone());
                                        let (description, _) = parse_skill_md_frontmatter(&content);
                                        discovered_skills.push(DiscoveredSkill {
                                            name,
                                            path: format!("skills/{}", skill_name),
                                            description,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // 如果还是没有发现，尝试遍历更多可能的命名模式
        if discovered_skills.is_empty() {
            discovered_skills = discover_skills_by_patterns(&client, owner, repo, branch).await;
        }

        if !discovered_skills.is_empty() {
            break;
        }
    }

    // 如果没有找到 skills 目录结构，尝试根目录下的单个 SKILL.md
    if discovered_skills.is_empty() {
        for branch in &branches {
            let skill_md_url = format!(
                "https://raw.githubusercontent.com/{}/{}/{}/SKILL.md",
                owner, repo, branch
            );

            let resp = client.get(&skill_md_url).send().await;
            if let Ok(resp) = resp {
                if resp.status().is_success() {
                    if let Ok(content) = resp.text().await {
                        let name = extract_name_from_skill_md(&content)
                            .unwrap_or_else(|| repo.to_string());
                        let (description, _) = parse_skill_md_frontmatter(&content);
                        discovered_skills.push(DiscoveredSkill {
                            name,
                            path: "".to_string(),
                            description,
                        });
                        break;
                    }
                }
            }
        }
    }

    Ok(discovered_skills)
}

/// 通过命名模式发现 skills（备用方法）
async fn discover_skills_by_patterns(
    client: &reqwest::Client,
    owner: &str,
    repo: &str,
    branch: &str,
) -> Vec<DiscoveredSkill> {
    let mut discovered_skills: Vec<DiscoveredSkill> = Vec::new();
    
    // 扩展的命名模式列表 - 基于常见的 skill 命名约定
    let patterns = vec![
        // baoyu 系列
        "baoyu-xhs-images", "baoyu-infographic", "baoyu-markdown-to-html", 
        "baoyu-imagine", "baoyu-ppt", "baoyu-mermaid", "baoyu-sql",
        "baoyu-code-review", "baoyu-prd", "baoyu-arch", "baoyu-prompt",
        "baoyu-translate", "baoyu-summary", "baoyu-write", "baoyu-read",
        // skill 系列
        "skill-xhs-images", "skill-infographic", "skill-markdown-to-html",
        "skill-imagine", "skill-ppt", "skill-mermaid", "skill-sql",
        "skill-code-review", "skill-prd", "skill-arch", "skill-prompt",
        // ai 系列
        "ai-xhs-images", "ai-infographic", "ai-markdown-to-html",
        "ai-imagine", "ai-ppt", "ai-mermaid", "ai-sql",
        "ai-code-review", "ai-prd", "ai-arch", "ai-prompt",
        // claude 系列
        "claude-xhs-images", "claude-infographic", "claude-markdown-to-html",
        "claude-imagine", "claude-ppt", "claude-mermaid", "claude-sql",
        "claude-code-review", "claude-prd", "claude-arch", "claude-prompt",
        // 其他常见模式
        "xhs-images", "infographic", "markdown-to-html", "imagine", "ppt",
        "mermaid", "sql", "code-review", "prd", "arch", "prompt",
        "translate", "summary", "write", "read", "search", "web", "api",
        "tool", "helper", "generator", "creator", "builder", "analyzer",
        "optimizer", "formatter", "converter", "extractor", "validator",
        "checker", "tester", "debugger", "docs", "doc", "document",
        "image", "img", "chart", "diagram", "flow", "workflow",
        "git", "github", "deploy", "build", "test", "lint", "format",
        "refactor", "review", "debug", "fix", "improve", "enhance",
        "frontend", "backend", "fullstack", "mobile", "desktop",
        "react", "vue", "angular", "svelte", "nextjs", "nuxt",
        "python", "rust", "go", "java", "typescript", "javascript",
        "css", "html", "sql", "nosql", "database", "db",
    ];
    
    // 尝试所有预定义的模式
    for skill_name in &patterns {
        let skill_md_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/skills/{}/SKILL.md",
            owner, repo, branch, skill_name
        );

        let resp = client.get(&skill_md_url).send().await;
        if let Ok(resp) = resp {
            if resp.status().is_success() {
                if let Ok(content) = resp.text().await {
                    let name = extract_name_from_skill_md(&content)
                        .unwrap_or_else(|| skill_name.to_string());
                    
                    // 检查是否已经添加过
                    if !discovered_skills.iter().any(|s: &DiscoveredSkill| s.path == format!("skills/{}", skill_name)) {
                        let (description, _) = parse_skill_md_frontmatter(&content);
                        discovered_skills.push(DiscoveredSkill {
                            name,
                            path: format!("skills/{}", skill_name),
                            description,
                        });
                    }
                }
            }
        }
    }

    // 还尝试一些纯数字命名的 skill
    for i in 1..=50 {
        let skill_name = format!("skill-{}", i);
        let skill_md_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/skills/{}/SKILL.md",
            owner, repo, branch, skill_name
        );

        let resp = client.get(&skill_md_url).send().await;
        if let Ok(resp) = resp {
            if resp.status().is_success() {
                if let Ok(content) = resp.text().await {
                    let name = extract_name_from_skill_md(&content)
                        .unwrap_or_else(|| skill_name.clone());
                    
                    if !discovered_skills.iter().any(|s: &DiscoveredSkill| s.path == format!("skills/{}", skill_name)) {
                        let (description, _) = parse_skill_md_frontmatter(&content);
                        discovered_skills.push(DiscoveredSkill {
                            name,
                            path: format!("skills/{}", skill_name),
                            description,
                        });
                    }
                }
            }
        }
    }
    
    // 尝试字母顺序的 skill (a-z)
    for c in 'a'..='z' {
        let skill_name = format!("skill-{}", c);
        let skill_md_url = format!(
            "https://raw.githubusercontent.com/{}/{}/{}/skills/{}/SKILL.md",
            owner, repo, branch, skill_name
        );

        let resp = client.get(&skill_md_url).send().await;
        if let Ok(resp) = resp {
            if resp.status().is_success() {
                if let Ok(content) = resp.text().await {
                    let name = extract_name_from_skill_md(&content)
                        .unwrap_or_else(|| skill_name.clone());
                    
                    if !discovered_skills.iter().any(|s: &DiscoveredSkill| s.path == format!("skills/{}", skill_name)) {
                        let (description, _) = parse_skill_md_frontmatter(&content);
                        discovered_skills.push(DiscoveredSkill {
                            name,
                            path: format!("skills/{}", skill_name),
                            description,
                        });
                    }
                }
            }
        }
    }
    
    discovered_skills
}

/// 从 README 内容中解析 skill 列表
fn parse_skills_from_readme(content: &str) -> Vec<String> {
    let mut skills = Vec::new();
    
    // 查找 markdown 中的链接，格式如：[skill-name](./skills/skill-name/) 或 [skill-name](skills/skill-name)
    for line in content.lines() {
        // 匹配相对路径链接
        if line.contains("skills/") || line.contains("./skills/") {
            // 尝试提取 skill 名称
            if let Some(start) = line.find("skills/") {
                let start = start + 7; // "skills/".len()
                let rest = &line[start..];
                let end = rest.find(|c: char| c == '/' || c == ')' || c == ']' || c.is_whitespace())
                    .unwrap_or(rest.len());
                let skill_name = &rest[..end];
                if !skill_name.is_empty() && !skills.contains(&skill_name.to_string()) {
                    skills.push(skill_name.to_string());
                }
            }
        }
    }
    
    skills
}

/// 从 GitHub 导入指定的 skills
pub async fn import_skills_from_github_repo(
    app: &AppHandle,
    owner: &str,
    repo: &str,
    selected_paths: Vec<String>,
) -> Result<Vec<String>, String> {
    // 加载配置获取代理设置
    let config = crate::services::config::load_config(app).await.unwrap_or_default();
    let proxy_url = config.settings.proxy_url.as_deref();

    let skills_dir = get_skills_dir(app)?;
    fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;

    let client = create_http_client(proxy_url)?;

    // 尝试常见的默认分支
    let branches = vec!["main", "master"];
    let mut default_branch = "main";
    let mut found_skills: Vec<(String, String, String)> = Vec::new(); // (skill_name, content, path)

    // 查找选中的 skills
    for branch in &branches {
        for path in &selected_paths {
            let skill_md_url = if path.is_empty() {
                format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/SKILL.md",
                    owner, repo, branch
                )
            } else {
                format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}/SKILL.md",
                    owner, repo, branch, path
                )
            };

            let resp = client.get(&skill_md_url).send().await;
            if let Ok(resp) = resp {
                if resp.status().is_success() {
                    if let Ok(content) = resp.text().await {
                        let name = extract_name_from_skill_md(&content)
                            .unwrap_or_else(|| {
                                if path.is_empty() {
                                    repo.to_string()
                                } else {
                                    path.split('/').last().unwrap_or(repo).to_string()
                                }
                            });
                        found_skills.push((name, content, path.clone()));
                        default_branch = branch;
                    }
                }
            }
        }
        
        if !found_skills.is_empty() {
            break;
        }
    }

    if found_skills.is_empty() {
        return Err(format!(
            "在仓库 {}/{} 中未找到选中的 SKILL.md 文件。",
            owner, repo
        ));
    }

    // 导入所有找到的 skills
    let mut imported_skills: Vec<String> = Vec::new();

    for (skill_name, skill_md_content, source_path) in found_skills {
        // 规范化 skill 名称
        let skill_name_normalized = skill_name.replace(" ", "-").to_lowercase();
        let skill_dir = skills_dir.join(&skill_name_normalized);

        if skill_dir.exists() {
            log::warn!("Skill '{}' 已经存在，跳过", skill_name);
            continue;
        }

        // 创建 skill 目录
        if let Err(e) = fs::create_dir_all(&skill_dir) {
            log::error!("创建 skill 目录失败: {}", e);
            continue;
        }

        // 下载 skill 文件
        let files_to_download: Vec<String> = vec![
            "SKILL.md".to_string(),
            "README.md".to_string(),
            "rules.md".to_string(),
            "prompt.md".to_string(),
            "system.md".to_string(),
        ];

        let mut downloaded_count = 0;

        for file_name in &files_to_download {
            if file_name == "README.md" || file_name.starts_with('_') || file_name.starts_with('.')
            {
                continue;
            }

            let file_url = if source_path.is_empty() {
                format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}",
                    owner, repo, default_branch, file_name
                )
            } else {
                format!(
                    "https://raw.githubusercontent.com/{}/{}/{}/{}/{}",
                    owner, repo, default_branch, source_path, file_name
                )
            };

            let file_resp = client.get(&file_url).send().await;

            if let Ok(resp) = file_resp {
                if resp.status().is_success() {
                    if let Ok(bytes) = resp.bytes().await {
                        let file_path = skill_dir.join(&file_name);
                        if fs::write(&file_path, &bytes).is_ok() {
                            downloaded_count += 1;
                        }
                    }
                }
            }
        }

        if downloaded_count == 0 {
            let _ = fs::remove_dir_all(&skill_dir);
            continue;
        }

        // 解析 SKILL.md 获取描述和版本
        let (description, version) = parse_skill_md_frontmatter(&skill_md_content);

        // 创建 metadata
        let metadata = crate::models::Skill {
            name: skill_name.clone(),
            description,
            version: version.or_else(|| Some("1.0.0".to_string())),
            author: Some(owner.to_string()),
            bindings: crate::models::SkillBinding {
                global: true,
                projects: Vec::new(),
            },
            enabled_tools: Vec::new(),
            enabled: true,
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        let metadata_path = skill_dir.join(".ai-skills-manager.json");
        if let Ok(json) = serde_json::to_string_pretty(&metadata) {
            let _ = fs::write(&metadata_path, json);
        }

        log::info!(
            "Successfully imported skill '{}' from GitHub {}/{}",
            skill_name, owner, repo
        );

        imported_skills.push(skill_name);
    }

    if imported_skills.is_empty() {
        return Err("未能成功导入任何 skill".to_string());
    }

    Ok(imported_skills)
}

fn extract_name_from_skill_md(content: &str) -> Option<String> {
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
