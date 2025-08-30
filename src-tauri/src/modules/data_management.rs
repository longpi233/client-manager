use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use crate::modules::database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DatabaseBackup {
    pub id: String,
    pub filename: String,
    pub timestamp: String,
    pub size: u64,
    pub description: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStatus {
    pub size: u64,
    pub last_backup: Option<String>,
    pub record_count: i64,
    pub tables: Vec<TableInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableInfo {
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompactResult {
    pub original_size: u64,
    pub new_size: u64,
    pub saved_space: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ImportResult {
    pub imported_records: i64,
    pub errors: Vec<String>,
}

pub struct DataManagement;

impl DataManagement {
    /// 获取数据库文件路径
    fn get_database_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        database::get_database_path()
            .ok_or_else(|| "数据库尚未初始化".into())
    }

    /// 获取备份目录路径
    fn get_backup_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let backup_dir = dirs::data_local_dir()
            .ok_or("无法获取应用数据目录")?
            .join("健康诊所管理系统")
            .join("backups");
        
        if !backup_dir.exists() {
            fs::create_dir_all(&backup_dir)?;
        }
        
        Ok(backup_dir)
    }

    /// 获取备份列表
    pub fn get_backup_list() -> Result<Vec<DatabaseBackup>, Box<dyn std::error::Error>> {
        let backup_dir = Self::get_backup_directory()?;
        let mut backups = Vec::new();

        if backup_dir.exists() {
            for entry in fs::read_dir(backup_dir)? {
                let entry = entry?;
                let path = entry.path();
                
                if path.extension().and_then(|s| s.to_str()) == Some("db") {
                    if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {
                        let metadata = fs::metadata(&path)?;
                        let created_at = metadata.created()
                            .or_else(|_| metadata.modified())?;
                        
                        let created_datetime: DateTime<Utc> = created_at.into();
                        
                        // 从文件名中提取信息
                        let parts: Vec<&str> = filename.splitn(3, '_').collect();
                        let (id, timestamp, description) = if parts.len() >= 2 {
                            (
                                parts[0].to_string(),
                                parts[1].replace(".db", ""),
                                if parts.len() > 2 {
                                    Some(parts[2].replace(".db", "").replace("-", " "))
                                } else {
                                    None
                                }
                            )
                        } else {
                            (
                                Uuid::new_v4().to_string(),
                                created_datetime.format("%Y%m%d_%H%M%S").to_string(),
                                None
                            )
                        };

                        backups.push(DatabaseBackup {
                            id,
                            filename: filename.to_string(),
                            timestamp,
                            size: metadata.len(),
                            description,
                            created_at: created_datetime.to_rfc3339(),
                        });
                    }
                }
            }
        }

        // 按创建时间倒序排列
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        Ok(backups)
    }

    /// 创建数据库备份
    pub fn create_backup(description: Option<String>) -> Result<DatabaseBackup, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        let backup_dir = Self::get_backup_directory()?;
        
        if !db_path.exists() {
            return Err("数据库文件不存在".into());
        }

        let now = Utc::now();
        let timestamp = now.format("%Y%m%d_%H%M%S").to_string();
        let backup_id = Uuid::new_v4().to_string()[..8].to_string();
        
        let backup_filename = if let Some(desc) = &description {
            let clean_desc = desc.replace(" ", "-").replace("/", "-").replace("\\", "-");
            format!("{}_{}_backup_{}.db", backup_id, timestamp, clean_desc)
        } else {
            format!("{}_{}_backup.db", backup_id, timestamp)
        };
        
        let backup_path = backup_dir.join(&backup_filename);
        
        // 复制数据库文件
        fs::copy(&db_path, &backup_path)?;
        
        let metadata = fs::metadata(&backup_path)?;
        
        Ok(DatabaseBackup {
            id: backup_id,
            filename: backup_filename,
            timestamp,
            size: metadata.len(),
            description,
            created_at: now.to_rfc3339(),
        })
    }

    /// 恢复数据库备份
    pub fn restore_backup(backup_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        let backup_dir = Self::get_backup_directory()?;
        
        // 查找对应的备份文件
        let backups = Self::get_backup_list()?;
        let backup = backups.iter()
            .find(|b| b.id == backup_id)
            .ok_or("备份文件不存在")?;
        
        let backup_path = backup_dir.join(&backup.filename);
        
        if !backup_path.exists() {
            return Err("备份文件不存在".into());
        }
        
        // 创建当前数据库的临时备份
        let temp_backup = format!("temp_before_restore_{}.db", Utc::now().format("%Y%m%d_%H%M%S"));
        let temp_backup_path = backup_dir.join(temp_backup);
        
        if db_path.exists() {
            fs::copy(&db_path, &temp_backup_path)?;
        }
        
        // 恢复备份
        match fs::copy(&backup_path, &db_path) {
            Ok(_) => {
                // 删除临时备份
                if temp_backup_path.exists() {
                    let _ = fs::remove_file(temp_backup_path);
                }
                Ok(true)
            }
            Err(e) => {
                // 恢复失败，回滚
                if temp_backup_path.exists() {
                    let _ = fs::copy(&temp_backup_path, &db_path);
                    let _ = fs::remove_file(temp_backup_path);
                }
                Err(e.into())
            }
        }
    }

    /// 删除数据库备份
    pub fn delete_backup(backup_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let backup_dir = Self::get_backup_directory()?;
        
        // 查找对应的备份文件
        let backups = Self::get_backup_list()?;
        let backup = backups.iter()
            .find(|b| b.id == backup_id)
            .ok_or("备份文件不存在")?;
        
        let backup_path = backup_dir.join(&backup.filename);
        
        if backup_path.exists() {
            fs::remove_file(backup_path)?;
            Ok(true)
        } else {
            Err("备份文件不存在".into())
        }
    }

    /// 获取数据库状态信息
    pub fn get_database_status() -> Result<DatabaseStatus, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        
        let size = if db_path.exists() {
            fs::metadata(&db_path)?.len()
        } else {
            0
        };

        // 获取最后备份时间
        let last_backup = Self::get_backup_list()?
            .first()
            .map(|b| b.created_at.clone());

        let mut record_count = 0;
        let mut tables = Vec::new();

        if db_path.exists() {
            let conn = Connection::open(&db_path)?;
            
            // 获取所有表的信息
            let table_names = vec![
                "medical_records",
                "treatment_items", 
                "prescription_items",
                "customers",
                "chinese_medicines",
                "chinese_medicine_prescriptions",
                "healing_items",
                "institutions"
            ];

            for table_name in table_names {
                match conn.prepare(&format!("SELECT COUNT(*) FROM {}", table_name)) {
                    Ok(mut stmt) => {
                        if let Ok(count) = stmt.query_row([], |row| row.get::<_, i64>(0)) {
                            tables.push(TableInfo {
                                name: table_name.to_string(),
                                count,
                            });
                            record_count += count;
                        }
                    }
                    Err(_) => {
                        // 表不存在，跳过
                        continue;
                    }
                }
            }
        }

        Ok(DatabaseStatus {
            size,
            last_backup,
            record_count,
            tables,
        })
    }

    /// 压缩数据库
    pub fn compact_database() -> Result<CompactResult, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        
        if !db_path.exists() {
            return Err("数据库文件不存在".into());
        }

        let original_size = fs::metadata(&db_path)?.len();
        
        // 连接数据库并执行VACUUM
        let conn = Connection::open(&db_path)?;
        conn.execute("VACUUM", [])?;
        conn.close().map_err(|(_, e)| e)?;
        
        let new_size = fs::metadata(&db_path)?.len();
        let saved_space = original_size.saturating_sub(new_size);
        
        Ok(CompactResult {
            original_size,
            new_size,
            saved_space,
        })
    }

    /// 导出数据为JSON格式
    pub fn export_data_as_json(tables: Vec<String>) -> Result<String, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        
        if !db_path.exists() {
            return Err("数据库文件不存在".into());
        }

        let conn = Connection::open(&db_path)?;
        let mut export_data = HashMap::new();

        for table_name in tables {
            let mut stmt = conn.prepare(&format!("SELECT * FROM {}", table_name))?;
            let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();
            
            let rows = stmt.query_map([], |row| {
                let mut record = HashMap::new();
                for (i, column_name) in column_names.iter().enumerate() {
                    let value: Result<String, _> = row.get(i);
                    record.insert(column_name.to_string(), value.unwrap_or_default());
                }
                Ok(record)
            })?;

            let mut table_data = Vec::new();
            for row in rows {
                table_data.push(row?);
            }
            
            export_data.insert(table_name, table_data);
        }

        let json_data = serde_json::to_string_pretty(&export_data)?;
        Ok(json_data)
    }

    /// 从JSON导入数据
    pub fn import_data_from_json(json_data: &str) -> Result<ImportResult, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        
        if !db_path.exists() {
            return Err("数据库文件不存在".into());
        }

        let import_data: HashMap<String, Vec<HashMap<String, String>>> = 
            serde_json::from_str(json_data)?;

        let conn = Connection::open(&db_path)?;
        let mut imported_records = 0;
        let mut errors = Vec::new();

        let tx = conn.unchecked_transaction()?;

        for (table_name, records) in import_data {
            for record in records {
                if record.is_empty() {
                    continue;
                }

                let columns: Vec<String> = record.keys().cloned().collect();
                let placeholders: Vec<String> = columns.iter().map(|_| "?".to_string()).collect();
                
                let sql = format!(
                    "INSERT OR REPLACE INTO {} ({}) VALUES ({})",
                    table_name,
                    columns.join(", "),
                    placeholders.join(", ")
                );

                let values: Vec<String> = columns.iter()
                    .map(|col| record.get(col).cloned().unwrap_or_default())
                    .collect();

                match tx.execute(&sql, rusqlite::params_from_iter(values)) {
                    Ok(_) => imported_records += 1,
                    Err(e) => errors.push(format!("导入表 {} 时出错: {}", table_name, e)),
                }
            }
        }

        tx.commit()?;

        Ok(ImportResult {
            imported_records,
            errors,
        })
    }

    /// 在系统文件管理器中打开数据库文件
    pub fn open_database_file() -> Result<bool, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        
        if !db_path.exists() {
            return Err("数据库文件不存在".into());
        }

        Self::open_file_in_explorer(&db_path)
    }

    /// 在系统文件管理器中打开备份文件
    pub fn open_backup_file(backup_id: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let backup_dir = Self::get_backup_directory()?;
        
        // 查找对应的备份文件
        let backups = Self::get_backup_list()?;
        let backup = backups.iter()
            .find(|b| b.id == backup_id)
            .ok_or("备份文件不存在")?;
        
        let backup_path = backup_dir.join(&backup.filename);
        
        if !backup_path.exists() {
            return Err("备份文件不存在".into());
        }

        Self::open_file_in_explorer(&backup_path)
    }

    /// 在系统文件管理器中打开备份目录
    pub fn open_backup_directory() -> Result<bool, Box<dyn std::error::Error>> {
        let backup_dir = Self::get_backup_directory()?;
        Self::open_file_in_explorer(&backup_dir)
    }

    /// 获取数据库文件完整路径
    pub fn get_database_file_path() -> Result<String, Box<dyn std::error::Error>> {
        let db_path = Self::get_database_path()?;
        Ok(db_path.to_string_lossy().to_string())
    }

    /// 获取备份文件完整路径
    pub fn get_backup_file_path(backup_id: &str) -> Result<String, Box<dyn std::error::Error>> {
        let backup_dir = Self::get_backup_directory()?;
        
        // 查找对应的备份文件
        let backups = Self::get_backup_list()?;
        let backup = backups.iter()
            .find(|b| b.id == backup_id)
            .ok_or("备份文件不存在")?;
        
        let backup_path = backup_dir.join(&backup.filename);
        Ok(backup_path.to_string_lossy().to_string())
    }

    /// 在系统文件管理器中打开文件或目录的通用方法
    fn open_file_in_explorer(path: &PathBuf) -> Result<bool, Box<dyn std::error::Error>> {
        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("explorer")
                .args(["/select,", &path.to_string_lossy()])
                .spawn()?;
        }

        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open")
                .args(["-R", &path.to_string_lossy()])
                .spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            if let Some(parent) = path.parent() {
                std::process::Command::new("xdg-open")
                    .arg(parent)
                    .spawn()?;
            } else {
                std::process::Command::new("xdg-open")
                    .arg(path)
                    .spawn()?;
            }
        }

        Ok(true)
    }
}

// Tauri 命令函数
#[tauri::command]
pub fn get_database_backups() -> Result<Vec<DatabaseBackup>, String> {
    DataManagement::get_backup_list()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn create_database_backup(description: Option<String>) -> Result<DatabaseBackup, String> {
    DataManagement::create_backup(description)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn restore_database_backup(backup_id: String) -> Result<bool, String> {
    DataManagement::restore_backup(&backup_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_database_backup(backup_id: String) -> Result<bool, String> {
    DataManagement::delete_backup(&backup_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_database_status() -> Result<DatabaseStatus, String> {
    DataManagement::get_database_status()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn compact_database() -> Result<CompactResult, String> {
    DataManagement::compact_database()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn export_data_as_json(tables: Vec<String>) -> Result<String, String> {
    DataManagement::export_data_as_json(tables)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn import_data_from_json(json_data: String) -> Result<ImportResult, String> {
    DataManagement::import_data_from_json(&json_data)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_database_file() -> Result<bool, String> {
    DataManagement::open_database_file()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_backup_file(backup_id: String) -> Result<bool, String> {
    DataManagement::open_backup_file(&backup_id)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn open_backup_directory() -> Result<bool, String> {
    DataManagement::open_backup_directory()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_database_file_path() -> Result<String, String> {
    DataManagement::get_database_file_path()
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_backup_file_path(backup_id: String) -> Result<String, String> {
    DataManagement::get_backup_file_path(&backup_id)
        .map_err(|e| e.to_string())
}
