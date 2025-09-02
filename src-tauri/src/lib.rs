// 模块声明
pub mod modules;

// 重新导出所有模块的公共接口
pub use modules::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("正在启动 Tauri 应用...");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // 治疗项目管理
            get_healing_list,
            create_healing_item,
            update_healing_item,
            delete_healing_item,
            batch_delete_healing_items,
            get_healing_item,
            // 中药管理
            get_chinese_medicine_list,
            create_chinese_medicine_item,
            update_chinese_medicine_item,
            delete_chinese_medicine_item,
            batch_delete_chinese_medicine_items,
            get_chinese_medicine_item,
            // 机构管理
            get_institution_info,
            update_institution_info,
            // 统一医疗记录管理 (新版本)
            create_unified_medical_record,
            get_unified_medical_record_list,
            get_unified_medical_record_detail,
            delete_unified_medical_record,
            delete_unified_medical_records_batch,
            // 数据库管理
            backup_database_to_file,
            restore_database_from_file,
            clear_all_database_data,
            get_database_statistics,
            // 数据管理新功能
            get_database_backups,
            create_database_backup,
            restore_database_backup,
            delete_database_backup,
            get_database_status,
            compact_database,
            export_data_as_json,
            import_data_from_json,
            // 文件管理功能
            open_database_file,
            open_backup_file,
            open_backup_directory,
            get_database_file_path,
            get_backup_file_path
        ])
        .setup(|app| {
            println!("🚀 进入 Tauri setup 函数！");

            // 初始化数据库
            println!("🚀 正在初始化 SQLite 数据库...");
            match modules::database::init_database(app.handle()) {
                Ok(_) => println!("✅ 数据库初始化成功"),
                Err(e) => {
                    eprintln!("❌ 数据库初始化失败: {}", e);
                    std::process::exit(1);
                }
            }

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            println!("✅ setup 函数完成");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
