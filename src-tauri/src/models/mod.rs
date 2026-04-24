use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub editor: String,
    pub auto_sync: bool,
    pub cleanup_on_disable: bool,
    pub sync_notifications: bool,
    pub refresh_interval: u32,
    pub theme: String,
    pub font_family: String,
    pub font_size: u32,
    pub language: String,
    #[serde(default)]
    pub enabled_data_sources: Option<Vec<String>>,
    #[serde(default)]
    pub proxy_url: Option<String>,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            editor: "monaco".to_string(),
            auto_sync: true,
            cleanup_on_disable: false,
            sync_notifications: true,
            refresh_interval: 30,
            theme: "system".to_string(),
            font_family: "system-ui".to_string(),
            font_size: 14,
            language: "zh-CN".to_string(),
            enabled_data_sources: Some(vec![
                "awesome-claude-skills".to_string(),
                "skills-sh".to_string(),
            ]),
            proxy_url: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillBinding {
    pub global: bool,
    pub projects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub name: String,
    pub description: Option<String>,
    pub version: Option<String>,
    pub author: Option<String>,
    pub bindings: SkillBinding,
    pub enabled_tools: Vec<String>,
    pub enabled: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tool {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub detected: bool,
    pub config_path: String,
    pub skills_path: String,
    pub custom: bool,
    #[serde(default)]
    pub enabled_tools: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketItem {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub source: String,
    pub downloads: Option<u32>,
    pub installed: Option<bool>,
    pub repo: Option<String>,
    pub skill_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketDataResult {
    pub items: Vec<MarketItem>,
    pub cached: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub version: String,
    pub settings: Settings,
    pub github_token: Option<String>,
    pub enabled_sources: Vec<String>,
    pub tools: Vec<Tool>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            settings: Settings::default(),
            github_token: None,
            enabled_sources: vec![
                "awesome-claude-skills".to_string(),
                "skills.sh".to_string(),
            ],
            tools: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillFile {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub success: bool,
    pub skill_name: String,
    pub error: Option<String>,
}
