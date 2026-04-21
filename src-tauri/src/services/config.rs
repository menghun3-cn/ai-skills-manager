use crate::models::Config;
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;
use log::info;

pub async fn init_app_data(app: &AppHandle) -> Result<(), String> {
    let app_dir = get_app_dir(app)?;
    let skills_dir = get_skills_dir(app)?;

    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    fs::create_dir_all(&skills_dir).map_err(|e| e.to_string())?;

    let config_path = app_dir.join("config.json");
    if !config_path.exists() {
        let default_config = Config::default();
        let json = serde_json::to_string_pretty(&default_config).map_err(|e| e.to_string())?;
        fs::write(&config_path, json).map_err(|e| e.to_string())?;
        info!("Created default config at {:?}", config_path);
    }

    Ok(())
}

pub fn get_app_dir(_app: &AppHandle) -> Result<PathBuf, String> {
    dirs::home_dir()
        .map(|p| p.join(".ai-skills-manager"))
        .ok_or_else(|| "Could not find home directory".to_string())
}

pub fn get_skills_dir(app: &AppHandle) -> Result<PathBuf, String> {
    get_app_dir(app).map(|p| p.join("skills"))
}

pub async fn load_config(app: &AppHandle) -> Result<Config, String> {
    let app_dir = get_app_dir(app)?;
    let config_path = app_dir.join("config.json");

    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = fs::read_to_string(&config_path).map_err(|e| e.to_string())?;
    let config: Config = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(config)
}

pub async fn save_config(app: &AppHandle, config: &Config) -> Result<Config, String> {
    let app_dir = get_app_dir(app)?;
    let config_path = app_dir.join("config.json");

    let json = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&config_path, json).map_err(|e| e.to_string())?;

    Ok(config.clone())
}

pub async fn reset_config(app: &AppHandle) -> Result<Config, String> {
    let app_dir = get_app_dir(app)?;
    let config_path = app_dir.join("config.json");

    // 删除旧配置文件
    if config_path.exists() {
        fs::remove_file(&config_path).map_err(|e| e.to_string())?;
        info!("Removed old config at {:?}", config_path);
    }

    // 创建默认配置
    let default_config = Config::default();
    save_config(app, &default_config).await?;
    
    Ok(default_config)
}
