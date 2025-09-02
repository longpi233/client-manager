pub mod types;
pub mod utils;
pub mod database;
pub mod medical_record; // 新的统一医疗记录模块
pub mod healing_items;
pub mod institution;
pub mod chinese_medicine;
pub mod data_management; // 数据管理模块

// 重新导出模块中的公共函数和类型
pub use types::*;
pub use utils::*;

// 数据库相关导出
pub use database::{
    init_database, get_database, backup_database_to_file, 
    restore_database_from_file, clear_all_database_data, 
    get_database_statistics
};

// 新的统一医疗记录模块导出
pub use medical_record::{
    create_unified_medical_record,
    get_unified_medical_record_list,
    get_unified_medical_record_detail,
    delete_unified_medical_record,
    delete_unified_medical_records_batch
};

pub use healing_items::{
    get_healing_list, create_healing_item, update_healing_item,
    delete_healing_item, batch_delete_healing_items, get_healing_item
};

pub use institution::{
    get_institution_info, update_institution_info
};

pub use chinese_medicine::{
    get_chinese_medicine_list, create_chinese_medicine_item, update_chinese_medicine_item,
    delete_chinese_medicine_item, batch_delete_chinese_medicine_items, get_chinese_medicine_item
};

// 数据管理模块导出
pub use data_management::{
    DataManagement, DatabaseBackup, DatabaseStatus, CompactResult, ImportResult,
    get_database_backups, create_database_backup, restore_database_backup,
    delete_database_backup, get_database_status, compact_database,
    export_data_as_json, import_data_from_json,
    open_database_file, open_backup_file, open_backup_directory,
    get_database_file_path, get_backup_file_path
};
