use crate::models::{Tool, Skill};
use crate::services::config::{load_config, save_config, get_skills_dir};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use serde::{Deserialize, Serialize};
use log::info;

#[cfg(windows)]
use std::os::windows::fs as windows_fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolVariant {
    id: String,
    name: String,
    global_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolConfig {
    id: String,
    name: String,
    project_path: String,
    global_path: String,
    #[serde(default)]
    variants: Vec<ToolVariant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ToolsConfig {
    tools: Vec<ToolConfig>,
}

fn load_tools_config() -> Result<ToolsConfig, String> {
    let config_path = PathBuf::from("tools-config.json");
    
    if !config_path.exists() {
        // 如果配置文件不存在，使用内置的默认配置
        return Ok(get_default_tools_config());
    }
    
    let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let config: ToolsConfig = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(config)
}

fn get_default_tools_config() -> ToolsConfig {
    let config_json = include_str!("../../tools-config.json");
    serde_json::from_str(config_json).expect("Failed to parse default tools config")
}

fn expand_path(path: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        let expanded = path.replace("%APPDATA%", &std::env::var("APPDATA").unwrap_or_default());
        let expanded = expanded.replace("%LOCALAPPDATA%", &std::env::var("LOCALAPPDATA").unwrap_or_default());
        let expanded = expanded.replace("%USERPROFILE%", &std::env::var("USERPROFILE").unwrap_or_default());
        expanded.replace("~", &std::env::var("USERPROFILE").unwrap_or_default())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let expanded = path.replace("~", &std::env::var("HOME").unwrap_or_default());
        expanded.replace("$HOME", &std::env::var("HOME").unwrap_or_default())
    }
}

fn normalize_path(unix_path: &str) -> String {
    #[cfg(target_os = "windows")]
    {
        // Windows: 将Unix路径转换为Windows格式
        unix_path.replace('/', "\\")
    }
    #[cfg(not(target_os = "windows"))]
    {
        // macOS/Linux: 保持Unix格式
        unix_path.to_string()
    }
}

pub async fn detect_tools(app: &AppHandle) -> Result<Vec<Tool>, String> {
    let mut tools = Vec::new();
    let mut config = load_config(app).await.unwrap_or_default();
    let tools_config = load_tools_config()?;

    for tool_def in tools_config.tools {
        // 如果有variants，检测每个variant
        if !tool_def.variants.is_empty() {
            for variant in &tool_def.variants {
                let skills_path = expand_path(&normalize_path(&variant.global_path));
                // 配置路径是skills路径的父目录
                let config_path = std::path::Path::new(&skills_path)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| skills_path.clone());
                let exists = fs::metadata(&skills_path).map(|m| m.is_dir()).unwrap_or(false);

                // 调试日志
                info!("{} 路径调试:", variant.id);
                info!("  原始路径: {}", variant.global_path);
                info!("  展开后: {}", skills_path);
                info!("  配置路径: {}", config_path);
                info!("  是否存在: {}", exists);

                let tool = Tool {
                    id: variant.id.clone(),
                    name: variant.name.clone(),
                    enabled: false,
                    detected: exists,
                    config_path: config_path.clone(),
                    skills_path: skills_path.clone(),
                    custom: false,
                    enabled_tools: Some(Vec::new()),
                };

                // 检查是否已存在此工具 - 强制更新路径
                if let Some(existing) = config.tools.iter_mut().find(|t| t.id == variant.id) {
                    existing.detected = exists;
                    existing.name = tool.name.clone(); // 更新名称
                    // 强制更新路径，不管是否存在
                    existing.config_path = tool.config_path.clone();
                    existing.skills_path = tool.skills_path.clone();
                    tools.push(existing.clone());
                } else {
                    tools.push(tool);
                }
            }
        } else {
            // 普通工具检测
            let normalized = normalize_path(&tool_def.global_path);
                let skills_path = expand_path(&normalized);
                // 配置路径是skills路径的父目录
                let config_path = std::path::Path::new(&skills_path)
                    .parent()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| skills_path.clone());
                let exists = fs::metadata(&skills_path).map(|m| m.is_dir()).unwrap_or(false);

                // 调试日志
                if tool_def.id == "cursor" {
                    info!("Cursor路径调试:");
                    info!("  原始路径: {}", tool_def.global_path);
                    info!("  规范化后: {}", normalized);
                    info!("  展开后: {}", skills_path);
                    info!("  配置路径: {}", config_path);
                    info!("  是否存在: {}", exists);
                }

                let tool = Tool {
                    id: tool_def.id.clone(),
                    name: tool_def.name.clone(),
                    enabled: false,
                    detected: exists,
                    config_path: config_path.clone(),
                    skills_path: skills_path.clone(),
                    custom: false,
                    enabled_tools: Some(Vec::new()),
                };

            // 检查是否已存在此工具 - 强制更新路径
            if let Some(existing) = config.tools.iter_mut().find(|t| t.id == tool_def.id) {
                existing.detected = exists;
                existing.name = tool.name.clone(); // 更新名称
                // 强制更新路径，不管是否存在
                existing.config_path = tool.config_path.clone();
                existing.skills_path = tool.skills_path.clone();
                tools.push(existing.clone());
            } else {
                tools.push(tool);
            }
        }
    }

    config.tools = tools.clone();
    let _ = save_config(app, &config).await;

    Ok(tools)
}

pub async fn add_custom_tool(app: &AppHandle, path: &str) -> Result<Tool, String> {
    let mut config = load_config(app).await.unwrap_or_default();

    let id = format!("custom-{}", config.tools.len());
    let name = path.split(['\\', '/']).last().unwrap_or("Custom Tool").to_string();

    let tool = Tool {
        id,
        name,
        enabled: true,
        detected: true,
        config_path: path.to_string(),
        skills_path: format!("{}\\skills", path),
        custom: true,
        enabled_tools: Some(Vec::new()),
    };

    config.tools.push(tool.clone());
    let _ = save_config(app, &config).await;

    Ok(tool)
}

pub async fn update_tool(app: &AppHandle, tool: &Tool) -> Result<Tool, String> {
    let mut config = load_config(app).await.unwrap_or_default();

    if let Some(existing) = config.tools.iter_mut().find(|t| t.id == tool.id) {
        *existing = tool.clone();
    }

    let _ = save_config(app, &config).await;
    Ok(tool.clone())
}

/// 获取工具的已启用技能列表
/// 从软链接实际存在性读取，而非配置文件（确保数据一致性）
pub async fn get_tool_enabled_skills(app: &AppHandle, tool_id: &str) -> Result<Vec<String>, String> {
    let config = load_config(app).await.unwrap_or_default();

    let tool = config.tools.iter().find(|t| t.id == tool_id)
        .ok_or_else(|| format!("Tool '{}' not found", tool_id))?;

    let skills_dir = get_skills_dir(app)?;
    let tool_skills_path = PathBuf::from(&tool.skills_path);
    let mut enabled_skills = Vec::new();

    // 遍历所有已安装的技能，检查软链接是否存在
    if skills_dir.exists() {
        for entry in fs::read_dir(&skills_dir).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();

            if path.is_dir() {
                let skill_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();

                // 检查软链接是否存在（使用规范化名称）
                let normalized_name = skill_name.replace(" ", "-").to_lowercase();
                let symlink_path = tool_skills_path.join(format!("{}@ai-skills-manager", normalized_name));

                if symlink_path.exists() {
                    enabled_skills.push(skill_name);
                }
            }
        }
    }

    Ok(enabled_skills)
}

/// 为工具启用技能（创建软链接）
pub async fn enable_skill_for_tool(app: &AppHandle, tool_id: &str, skill_name: &str) -> Result<(), String> {
    let config = load_config(app).await.unwrap_or_default();
    
    let tool = config.tools.iter().find(|t| t.id == tool_id)
        .ok_or_else(|| format!("Tool '{}' not found", tool_id))?;
    
    if !tool.enabled {
        return Err(format!("Tool '{}' is not enabled", tool.name));
    }
    
    let skills_dir = get_skills_dir(app)?;
    
    // 尝试原始名称
    let skill_dir = skills_dir.join(skill_name);
    // 尝试转换后的名称（与安装时一致：空格替换为连字符并转为小写）
    let normalized_name = skill_name.replace(" ", "-").to_lowercase();
    let normalized_skill_dir = skills_dir.join(&normalized_name);
    
    // 确定实际使用的技能目录
    let actual_skill_dir = if skill_dir.exists() {
        skill_dir
    } else if normalized_skill_dir.exists() {
        normalized_skill_dir
    } else {
        return Err(format!("Skill '{}' not found", skill_name));
    };
    
    let target_dir = PathBuf::from(&tool.skills_path);
    
    // 确保目标目录存在
    if let Err(e) = fs::create_dir_all(&target_dir) {
        return Err(format!("Failed to create skills directory: {}", e));
    }
    
    // 始终使用规范化名称创建软链接，确保与检测逻辑一致
    let link_path = target_dir.join(format!("{}@ai-skills-manager", normalized_name));
    
    // 如果已存在，先删除
    if link_path.exists() {
        // 首先尝试作为目录删除（Windows 目录软链接）
        #[cfg(windows)]
        {
            if fs::remove_dir(&link_path).is_err() {
                // 如果目录删除失败，尝试作为文件删除
                if let Err(e) = fs::remove_file(&link_path) {
                    return Err(format!("Failed to remove existing symlink: {}", e));
                }
            }
        }
        #[cfg(not(windows))]
        {
            fs::remove_file(&link_path)
                .map_err(|e| format!("Failed to remove existing symlink: {}", e))?;
        }
    }
    
    // 创建软链接
    #[cfg(windows)]
    {
        match windows_fs::symlink_dir(&actual_skill_dir, &link_path) {
            Ok(_) => {
                log::info!("Created symlink for skill '{}' in tool '{}'", normalized_name, tool.name);
            }
            Err(e) => {
                let error_msg = format!("{}", e);
                // 检查是否是权限错误
                if error_msg.contains("1314") || error_msg.contains("权限") || error_msg.contains("privilege") {
                    return Err(format!("PERMISSION_DENIED: 创建软链接失败，权限不足。请使用管理员身份运行软件。"));
                }
                return Err(format!("Failed to create symlink: {}", e));
            }
        }
    }
    #[cfg(not(windows))]
    {
        match std::os::unix::fs::symlink(&actual_skill_dir, &link_path) {
            Ok(_) => {
                log::info!("Created symlink for skill '{}' in tool '{}'", normalized_name, tool.name);
            }
            Err(e) => {
                return Err(format!("Failed to create symlink: {}", e));
            }
        }
    }
    
    // 更新配置 - 使用原始 skill_name 保持一致性
    let mut config = load_config(app).await.unwrap_or_default();
    if let Some(tool) = config.tools.iter_mut().find(|t| t.id == tool_id) {
        let mut enabled_tools = tool.enabled_tools.clone().unwrap_or_default();
        if !enabled_tools.contains(&skill_name.to_string()) {
            enabled_tools.push(skill_name.to_string());
            tool.enabled_tools = Some(enabled_tools);
        }
    }
    let _ = save_config(app, &config).await;
    
    Ok(())
}

/// 为工具停用技能（删除软链接）
pub async fn disable_skill_for_tool(app: &AppHandle, tool_id: &str, skill_name: &str) -> Result<(), String> {
    let config = load_config(app).await.unwrap_or_default();
    
    let tool = config.tools.iter().find(|t| t.id == tool_id)
        .ok_or_else(|| format!("Tool '{}' not found", tool_id))?;
    
    let target_dir = PathBuf::from(&tool.skills_path);
    
    // 尝试原始名称的软链接
    let link_path = target_dir.join(format!("{}@ai-skills-manager", skill_name));
    // 尝试转换后的名称（与安装时一致：空格替换为连字符并转为小写）
    let normalized_name = skill_name.replace(" ", "-").to_lowercase();
    let normalized_link_path = target_dir.join(format!("{}@ai-skills-manager", normalized_name));
    
    // 删除软链接（尝试两种名称）
    // Windows 上使用 remove_dir 删除目录软链接，使用 remove_file 删除文件软链接
    for path in [&link_path, &normalized_link_path] {
        if path.exists() {
            // 首先尝试作为目录删除
            #[cfg(windows)]
            {
                match fs::remove_dir(path) {
                    Ok(_) => {
                        log::info!("Removed symlink (dir) for skill from tool '{}'", tool.name);
                        continue;
                    }
                    Err(e) => {
                        log::debug!("Failed to remove symlink as dir: {}, trying as file", e);
                    }
                }
            }
            // 然后尝试作为文件删除
            match fs::remove_file(path) {
                Ok(_) => {
                    log::info!("Removed symlink (file) for skill from tool '{}'", tool.name);
                }
                Err(e) => {
                    // 如果删除失败，检查是否是权限错误
                    let error_msg = format!("{}", e);
                    if error_msg.contains("5") || error_msg.contains("拒绝访问") || error_msg.contains("Access") {
                        log::warn!("Permission denied when removing symlink: {}", path.display());
                        // 尝试使用 remove_dir_all 作为最后的手段
                        match fs::remove_dir_all(path) {
                            Ok(_) => {
                                log::info!("Removed symlink using remove_dir_all for '{}'", tool.name);
                            }
                            Err(e2) => {
                                return Err(format!("无法删除软链接，权限不足。请使用管理员身份运行软件。原始错误: {}; 尝试删除目录错误: {}", e, e2));
                            }
                        }
                    } else {
                        return Err(format!("Failed to remove symlink: {}", e));
                    }
                }
            }
        }
    }
    
    // 更新配置
    let mut config = load_config(app).await.unwrap_or_default();
    if let Some(tool) = config.tools.iter_mut().find(|t| t.id == tool_id) {
        let mut enabled_tools = tool.enabled_tools.clone().unwrap_or_default();
        enabled_tools.retain(|s| s != skill_name);
        tool.enabled_tools = Some(enabled_tools);
    }
    let _ = save_config(app, &config).await;
    
    Ok(())
}

/// 获取所有已安装的技能列表
pub async fn get_installed_skills(app: &AppHandle) -> Result<Vec<Skill>, String> {
    crate::services::skills::get_skills(app).await
}
