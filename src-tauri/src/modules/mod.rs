pub mod chinese_medicine;
pub mod data_management;
pub mod database;
pub mod healing_items;
pub mod institution;
pub mod medical_record; // 新的统一医疗记录模块
pub mod types;
pub mod utils; // 数据管理模块

// 重新导出模块中的公共函数和类型
pub use types::*;
pub use utils::*;

// 数据库相关导出
pub use database::{
    backup_database_to_file, clear_all_database_data, get_database, get_database_statistics,
    init_database, restore_database_from_file,
};

// 新的统一医疗记录模块导出
pub use medical_record::{
    create_unified_medical_record, delete_unified_medical_record,
    delete_unified_medical_records_batch, get_unified_medical_record_detail,
    get_unified_medical_record_list,
};

pub use healing_items::{
    batch_delete_healing_items, create_healing_item, delete_healing_item, get_healing_item,
    get_healing_list, update_healing_item,
};

pub use institution::{get_institution_info, update_institution_info};

pub use chinese_medicine::{
    batch_delete_chinese_medicine_items, create_chinese_medicine_item,
    delete_chinese_medicine_item, get_chinese_medicine_item, get_chinese_medicine_list,
    update_chinese_medicine_item,
};

// 数据管理模块导出
pub use data_management::{
    compact_database, create_database_backup, delete_database_backup, export_data_as_json,
    get_backup_file_path, get_database_backups, get_database_file_path, get_database_status,
    import_data_from_json, open_backup_directory, open_backup_file, open_database_file,
    restore_database_backup, CompactResult, DataManagement, DatabaseBackup, DatabaseStatus,
    ImportResult,
};
