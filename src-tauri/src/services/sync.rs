use crate::models::{SyncResult, Tool};
use crate::services::config::{get_skills_dir, load_config};
use std::fs;
use std::path::PathBuf;
use tauri::AppHandle;

#[cfg(windows)]
use std::os::windows::fs as windows_fs;

pub async fn sync_skill(app: &AppHandle, skill_name: &str) -> Result<SyncResult, String> {
    let skills_dir = get_skills_dir(app).map_err(|e| e.to_string())?;
    let skill_dir = skills_dir.join(skill_name);

    if !skill_dir.exists() {
        return Ok(SyncResult {
            success: false,
            skill_name: skill_name.to_string(),
            error: Some("Skill directory not found".to_string()),
        });
    }

    let config = load_config(app).await.unwrap_or_default();
    let enabled_tools: Vec<&Tool> = config.tools.iter().filter(|t| t.enabled).collect();

    let mut success_count = 0;
    let mut errors = Vec::new();
    
    // 使用标准化的技能名称创建软链接（空格替换为连字符并转为小写）
    let skill_name_normalized = skill_name.replace(" ", "-").to_lowercase();

    for tool in enabled_tools {
        let target_dir = PathBuf::from(&tool.skills_path);

        if !target_dir.exists() {
            if let Err(e) = fs::create_dir_all(&target_dir) {
                errors.push(format!("{}: {}", tool.name, e));
                continue;
            }
        }

        let link_path = target_dir.join(format!("{}@ai-skills-manager", skill_name_normalized));

        if link_path.exists() {
            // 首先尝试作为目录删除（Windows 目录软链接）
            #[cfg(windows)]
            {
                if fs::remove_dir(&link_path).is_err() {
                    // 如果目录删除失败，尝试作为文件删除
                    let _ = fs::remove_file(&link_path);
                }
            }
            #[cfg(not(windows))]
            {
                let _ = fs::remove_file(&link_path);
            }
        }

        #[cfg(windows)]
        {
            match windows_fs::symlink_dir(&skill_dir, &link_path) {
                Ok(_) => (),
                Err(e) => errors.push(format!("{}: {}", tool.name, e)),
            }
        }
    }

    Ok(SyncResult {
        success: errors.is_empty(),
        skill_name: skill_name.to_string(),
        error: if errors.is_empty() {
            None
        } else {
            Some(errors.join("; "))
        },
    })
}

pub async fn sync_all(app: &AppHandle) -> Result<Vec<SyncResult>, String> {
    let skills_dir = get_skills_dir(app).map_err(|e| e.to_string())?;
    let mut results = Vec::new();

    if !skills_dir.exists() {
        return Ok(results);
    }

    for entry in fs::read_dir(&skills_dir).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            let skill_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string();

            match sync_skill(app, &skill_name).await {
                Ok(result) => results.push(result),
                Err(e) => results.push(SyncResult {
                    success: false,
                    skill_name,
                    error: Some(e),
                }),
            }
        }
    }

    Ok(results)
}

/// 批量同步多个技能到指定工具
pub async fn batch_sync_skills_to_tools(
    app: &AppHandle,
    skill_names: &[String],
    tool_ids: &[String],
) -> Result<Vec<SyncResult>, String> {
    let config = load_config(app).await.unwrap_or_default();
    let mut results = Vec::new();

    // 获取指定的工具
    let target_tools: Vec<&Tool> = config
        .tools
        .iter()
        .filter(|t| tool_ids.contains(&t.id))
        .collect();

    if target_tools.is_empty() {
        return Err("未找到指定的工具".to_string());
    }

    for skill_name in skill_names {
        match sync_skill_to_tools(app, skill_name, &target_tools).await {
            Ok(result) => results.push(result),
            Err(e) => results.push(SyncResult {
                success: false,
                skill_name: skill_name.clone(),
                error: Some(e),
            }),
        }
    }

    Ok(results)
}

/// 批量从指定工具移除多个技能
pub async fn batch_remove_skills_from_tools(
    app: &AppHandle,
    skill_names: &[String],
    tool_ids: &[String],
) -> Result<Vec<SyncResult>, String> {
    let config = load_config(app).await.unwrap_or_default();
    let mut results = Vec::new();

    // 获取指定的工具
    let target_tools: Vec<&Tool> = config
        .tools
        .iter()
        .filter(|t| tool_ids.contains(&t.id))
        .collect();

    if target_tools.is_empty() {
        return Err("未找到指定的工具".to_string());
    }

    for skill_name in skill_names {
        let mut success = true;
        let mut errors = Vec::new();
        
        // 使用标准化的技能名称（空格替换为连字符并转为小写）
        let skill_name_normalized = skill_name.replace(" ", "-").to_lowercase();

        for tool in &target_tools {
            let target_dir = PathBuf::from(&tool.skills_path);
            // 尝试删除标准化名称的软链接
            let link_path = target_dir.join(format!("{}@ai-skills-manager", skill_name_normalized));
            
            if link_path.exists() {
                // 首先尝试作为目录删除（Windows 目录软链接）
                #[cfg(windows)]
                {
                    if fs::remove_dir(&link_path).is_ok() {
                        continue;
                    }
                }
                // 然后尝试作为文件删除
                if let Err(e) = fs::remove_file(&link_path) {
                    // 检查是否是权限错误
                    let error_msg = format!("{}", e);
                    if error_msg.contains("5") || error_msg.contains("拒绝访问") || error_msg.contains("Access") {
                        // 尝试使用 remove_dir_all 作为最后的手段
                        if let Err(e2) = fs::remove_dir_all(&link_path) {
                            success = false;
                            errors.push(format!("{}: 权限不足，无法删除软链接。请使用管理员身份运行软件。错误: {}", tool.name, e2));
                        }
                    } else {
                        success = false;
                        errors.push(format!("{}: {}", tool.name, e));
                    }
                }
            }
        }

        results.push(SyncResult {
            success,
            skill_name: skill_name.clone(),
            error: if errors.is_empty() {
                None
            } else {
                Some(errors.join("; "))
            },
        });
    }

    Ok(results)
}

/// 同步单个技能到指定工具列表
async fn sync_skill_to_tools(
    app: &AppHandle,
    skill_name: &str,
    tools: &[&Tool],
) -> Result<SyncResult, String> {
    let skills_dir = get_skills_dir(app).map_err(|e| e.to_string())?;
    let skill_dir = skills_dir.join(skill_name);

    if !skill_dir.exists() {
        return Ok(SyncResult {
            success: false,
            skill_name: skill_name.to_string(),
            error: Some("Skill directory not found".to_string()),
        });
    }

    let mut success = true;
    let mut errors = Vec::new();
    
    // 使用标准化的技能名称创建软链接（空格替换为连字符并转为小写）
    let skill_name_normalized = skill_name.replace(" ", "-").to_lowercase();

    for tool in tools {
        let target_dir = PathBuf::from(&tool.skills_path);

        if !target_dir.exists() {
            if let Err(e) = fs::create_dir_all(&target_dir) {
                errors.push(format!("{}: {}", tool.name, e));
                success = false;
                continue;
            }
        }

        let link_path = target_dir.join(format!("{}@ai-skills-manager", skill_name_normalized));

        if link_path.exists() {
            let _ = fs::remove_file(&link_path);
        }

        #[cfg(windows)]
        {
            match windows_fs::symlink_dir(&skill_dir, &link_path) {
                Ok(_) => (),
                Err(e) => {
                    errors.push(format!("{}: {}", tool.name, e));
                    success = false;
                }
            }
        }
        #[cfg(not(windows))]
        {
            match std::os::unix::fs::symlink(&skill_dir, &link_path) {
                Ok(_) => (),
                Err(e) => {
                    errors.push(format!("{}: {}", tool.name, e));
                    success = false;
                }
            }
        }
    }

    Ok(SyncResult {
        success,
        skill_name: skill_name.to_string(),
        error: if errors.is_empty() {
            None
        } else {
            Some(errors.join("; "))
        },
    })
}
