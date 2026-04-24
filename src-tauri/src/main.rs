#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod models;
mod services;

use commands::*;
use log::info;
use services::config::init_app_data;
use std::path::PathBuf;

fn get_log_dir() -> PathBuf {
    // 获取应用安装目录（exe 所在目录）
    let exe_path = std::env::current_exe().expect("Failed to get current exe path");
    let exe_dir = exe_path.parent().expect("Failed to get exe directory");
    
    // 在安装目录下创建 logs 文件夹
    let log_dir = exe_dir.join("logs");
    
    // 确保 logs 目录存在
    if !log_dir.exists() {
        let _ = std::fs::create_dir_all(&log_dir);
    }
    
    log_dir
}

fn main() {
    let log_dir = get_log_dir();
    
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::LogDir { file_name: None },
                ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder { path: log_dir, file_name: None },
                ))
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Stdout,
                ))
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            info!("AI Skills Manager starting...");

            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = init_app_data(&app_handle).await {
                    log::error!("Failed to initialize app data: {}", e);
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_config,
            get_app_dir_path,
            update_config,
            get_skills,
            create_skill,
            delete_skill,
            update_skill,
            get_skill_files,
            read_skill_file,
            write_skill_file,
            detect_tools,
            add_custom_tool,
            update_tool,
            open_path,
            get_market_data,
            get_market_data_with_cache,
            get_market_cache_age,
            install_skill,
            clear_market_cache,
            sync_skill,
            sync_all,
            browse_directory,
            get_tool_enabled_skills,
            enable_skill_for_tool,
            disable_skill_for_tool,
            get_installed_skills,
            batch_sync_skills_to_tools,
            batch_remove_skills_from_tools,
            check_admin_privileges,
            import_skill_from_github,
            discover_skills_from_github_repo,
            import_skills_from_github_repo,
            import_skills_from_github_repo_force,
            update_github_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
