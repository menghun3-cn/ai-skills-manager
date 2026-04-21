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
