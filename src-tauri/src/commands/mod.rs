use crate::models::*;
use crate::services;
use tauri::AppHandle;

#[tauri::command]
pub async fn get_config(app: AppHandle) -> Result<serde_json::Value, String> {
    let config = services::config::load_config(&app).await?;
    
    // 返回设置和 GitHub Token
    let result = serde_json::json!({
        "settings": config.settings,
        "githubToken": config.github_token,
    });
    
    Ok(result)
}

#[tauri::command]
pub async fn update_github_token(app: AppHandle, token: Option<String>) -> Result<(), String> {
    let mut config = services::config::load_config(&app).await?;
    config.github_token = token;
    services::config::save_config(&app, &config).await?;
    Ok(())
}

#[tauri::command]
pub fn get_app_dir_path(app: AppHandle) -> Result<String, String> {
    services::config::get_app_dir(&app).map(|p| p.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn update_config(app: AppHandle, config: Config) -> Result<Config, String> {
    services::config::save_config(&app, &config).await
}

#[tauri::command]
pub async fn get_skills(app: AppHandle) -> Result<Vec<Skill>, String> {
    services::skills::get_skills(&app).await
}

#[tauri::command]
pub async fn create_skill(app: AppHandle, name: String, description: String) -> Result<Skill, String> {
    services::skills::create_skill(&app, &name, &description).await
}

#[tauri::command]
pub async fn delete_skill(app: AppHandle, name: String) -> Result<bool, String> {
    services::skills::delete_skill(&app, &name).await
}

#[tauri::command]
pub async fn update_skill(app: AppHandle, skill: Skill) -> Result<Skill, String> {
    services::skills::update_skill(&app, &skill).await
}

#[tauri::command]
pub async fn get_skill_files(app: AppHandle, skill_name: String) -> Result<Vec<SkillFile>, String> {
    services::skills::get_skill_files(&app, &skill_name).await
}

#[tauri::command]
pub async fn read_skill_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_skill_file(path: String, content: String) -> Result<bool, String> {
    std::fs::write(&path, content).map_err(|e| e.to_string())?;
    Ok(true)
}

#[tauri::command]
pub async fn detect_tools(app: AppHandle) -> Result<Vec<Tool>, String> {
    services::tools::detect_tools(&app).await
}

#[tauri::command]
pub async fn add_custom_tool(app: AppHandle, path: String) -> Result<Tool, String> {
    services::tools::add_custom_tool(&app, &path).await
}

#[tauri::command]
pub async fn update_tool(app: AppHandle, tool: Tool) -> Result<Tool, String> {
    services::tools::update_tool(&app, &tool).await
}

#[tauri::command]
pub async fn open_path(path: String) -> Result<(), String> {
    use std::path::Path;
    
    // 检查路径是否存在
    if !Path::new(&path).exists() {
        return Err(format!("PATH_NOT_FOUND:{}", path));
    }
    
    opener::open(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_market_data(app: AppHandle) -> Result<Vec<MarketItem>, String> {
    services::market::get_market_data(&app).await
}

#[tauri::command]
pub async fn get_market_data_with_cache(app: AppHandle) -> Result<MarketDataResult, String> {
    services::market::get_market_data_with_cache(&app).await
}

#[tauri::command]
pub async fn install_skill(app: AppHandle, market_id: String, github_token: Option<String>) -> Result<bool, String> {
    services::market::install_skill(&app, market_id, github_token).await
}

#[tauri::command]
pub async fn clear_market_cache(app: AppHandle) -> Result<bool, String> {
    services::market::clear_cache(&app).await
}

#[tauri::command]
pub fn get_market_cache_age(app: AppHandle) -> Result<Option<String>, String> {
    services::market::get_cache_age(&app)
}

#[tauri::command]
pub async fn sync_skill(app: AppHandle, skill_name: String) -> Result<SyncResult, String> {
    services::sync::sync_skill(&app, &skill_name).await
}

#[tauri::command]
pub async fn sync_all(app: AppHandle) -> Result<Vec<SyncResult>, String> {
    services::sync::sync_all(&app).await
}

#[tauri::command]
pub async fn get_tool_enabled_skills(app: AppHandle, tool_id: String) -> Result<Vec<String>, String> {
    services::tools::get_tool_enabled_skills(&app, &tool_id).await
}

#[tauri::command]
pub async fn enable_skill_for_tool(app: AppHandle, tool_id: String, skill_name: String) -> Result<(), String> {
    services::tools::enable_skill_for_tool(&app, &tool_id, &skill_name).await
}

#[tauri::command]
pub async fn disable_skill_for_tool(app: AppHandle, tool_id: String, skill_name: String) -> Result<(), String> {
    services::tools::disable_skill_for_tool(&app, &tool_id, &skill_name).await
}

#[tauri::command]
pub async fn get_installed_skills(app: AppHandle) -> Result<Vec<Skill>, String> {
    services::tools::get_installed_skills(&app).await
}

#[tauri::command]
pub async fn batch_sync_skills_to_tools(
    app: AppHandle,
    skill_names: Vec<String>,
    tool_ids: Vec<String>,
) -> Result<Vec<SyncResult>, String> {
    services::sync::batch_sync_skills_to_tools(&app, &skill_names, &tool_ids).await
}

#[tauri::command]
pub async fn batch_remove_skills_from_tools(
    app: AppHandle,
    skill_names: Vec<String>,
    tool_ids: Vec<String>,
) -> Result<Vec<SyncResult>, String> {
    services::sync::batch_remove_skills_from_tools(&app, &skill_names, &tool_ids).await
}

#[tauri::command]
pub async fn browse_directory(app: AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;
    let result = app.dialog()
        .file()
        .blocking_pick_folder();

    match result {
        Some(path) => Ok(path.to_string()),
        None => Err("No folder selected".to_string()),
    }
}

#[tauri::command]
pub async fn import_skill_from_github(
    app: AppHandle,
    owner: String,
    repo: String,
) -> Result<bool, String> {
    services::skills::import_skill_from_github(&app, &owner, &repo).await
}

#[tauri::command]
pub async fn discover_skills_from_github_repo(
    app: AppHandle,
    owner: String,
    repo: String,
) -> Result<Vec<services::skills::DiscoveredSkill>, String> {
    services::skills::discover_skills_from_github_repo(&app, &owner, &repo).await
}

#[tauri::command]
pub async fn import_skills_from_github_repo(
    app: AppHandle,
    owner: String,
    repo: String,
    selected_paths: Vec<String>,
) -> Result<Vec<String>, String> {
    services::skills::import_skills_from_github_repo(&app, &owner, &repo, selected_paths).await
}

#[tauri::command]
pub async fn import_skills_from_github_repo_force(
    app: AppHandle,
    owner: String,
    repo: String,
    selected_paths: Vec<String>,
) -> Result<Vec<String>, String> {
    services::skills::import_skills_from_github_repo_with_options(&app, &owner, &repo, selected_paths, true).await
}

#[tauri::command]
pub fn check_admin_privileges() -> Result<bool, String> {
    #[cfg(windows)]
    {
        use std::process::Command;
        
        // 使用 whoami /groups 命令检查是否在管理员组
        let output = Command::new("whoami")
            .args(&["/groups"])
            .output()
            .map_err(|e| e.to_string())?;
        
        let output_str = String::from_utf8_lossy(&output.stdout);
        // 检查输出中是否包含管理员相关的 SID
        // S-1-16-12288 是高完整性级别（管理员）
        // S-1-5-32-544 是 Administrators 组
        let is_admin = output_str.contains("S-1-16-12288") || 
                      output_str.contains("S-1-5-32-544") ||
                      output_str.contains("管理员") ||
                      output_str.contains("Administrators");
        
        Ok(is_admin)
    }
    
    #[cfg(not(windows))]
    {
        // macOS/Linux: 检查是否为 root 用户
        use std::process::Command;
        
        let output = Command::new("id")
            .arg("-u")
            .output()
            .map_err(|e| e.to_string())?;
        
        let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(uid == "0")
    }
}
