use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use tauri::{AppHandle, Manager};

// 使用 OnceLock 替代 static mut，更安全
static DATABASE_PATH: OnceLock<PathBuf> = OnceLock::new();

pub struct DatabaseManager {
    connection: Mutex<Connection>,
}

impl DatabaseManager {
    pub fn new(db_path: &PathBuf) -> SqliteResult<Self> {
        let conn = Connection::open(db_path)?;

        // 创建所有表
        Self::create_tables(&conn)?;

        Ok(DatabaseManager {
            connection: Mutex::new(conn),
        })
    }

    pub fn get_connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.connection.lock().unwrap()
    }

    fn create_tables(conn: &Connection) -> SqliteResult<()> {
        // 机构信息表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS institutions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                address TEXT,
                contact_phone TEXT,
                license_number TEXT,
                established_date TEXT,
                description TEXT,
                business_hours TEXT,
                services TEXT, -- JSON string
                doctor_management TEXT, -- JSON string
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        // 治疗项目表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS healing_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                unit_price INTEGER NOT NULL,  -- 价格以万分为单位存储
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // 中药表
        conn.execute(
            "CREATE TABLE IF NOT EXISTS chinese_medicines (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                unit_price INTEGER NOT NULL,     -- 单价（以万分为单位）
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;

        // 统一医疗记录主表 (包含散客登记、门诊病历、中药处方的通用信息)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS medical_records (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                data_type TEXT NOT NULL, -- 'customer', 'medical', 'prescription'
                customer_name TEXT NOT NULL,
                gender TEXT NOT NULL,
                age INTEGER NOT NULL,
                telephone TEXT,
                time_of_visit TEXT NOT NULL,
                trade_type TEXT NOT NULL,
                diagnosis TEXT NOT NULL,
                note TEXT,
                total_amount INTEGER NOT NULL DEFAULT 0, -- 总金额（以万分为单位）
                doctor_id INTEGER,
                doctor_name TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                
                -- 门诊病历特有字段
                chief_complaint TEXT,
                illness_history TEXT,
                physical_examination TEXT,
                past_history TEXT,
                laboratory_examination TEXT,
                treatment_method TEXT,
                
                -- 中药处方特有字段
                doses INTEGER, -- 付数
                single_dose_amount INTEGER -- 单付金额（以万分为单位）
            )",
            [],
        )?;

        // 治疗项目记录表 (记录治疗项目)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS medical_treatment_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                medical_record_id INTEGER NOT NULL,
                treatment_item TEXT NOT NULL,
                count INTEGER NOT NULL DEFAULT 1,
                unit_price INTEGER NOT NULL,     -- 单价（以万分为单位）
                total_price INTEGER NOT NULL,    -- 总价（以万分为单位）
                created_at TEXT NOT NULL,
                FOREIGN KEY (medical_record_id) REFERENCES medical_records(id) ON DELETE CASCADE
            )",
            [],
        )?;

        // 中药处方记录表 (记录中药处方详情)
        conn.execute(
            "CREATE TABLE IF NOT EXISTS medical_prescription_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                medical_record_id INTEGER NOT NULL,
                medicine_name TEXT NOT NULL,
                grams INTEGER NOT NULL,
                unit_price INTEGER NOT NULL,     -- 单价（以万分为单位）
                total_price INTEGER NOT NULL,    -- 总价（以万分为单位）
                created_at TEXT NOT NULL,
                FOREIGN KEY (medical_record_id) REFERENCES medical_records(id) ON DELETE CASCADE
            )",
            [],
        )?;

        Ok(())
    }

    // 备份数据库
    pub fn backup_database(&self, backup_path: &PathBuf) -> SqliteResult<()> {
        let conn = self.get_connection();

        // 获取数据库路径
        let db_path = conn.path().ok_or_else(|| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some("Cannot determine database path".to_string()),
            )
        })?;

        // 简化的备份实现 - 复制数据库文件
        std::fs::copy(db_path, backup_path).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(format!("Failed to backup database: {}", e)),
            )
        })?;

        Ok(())
    }

    // 从备份恢复数据库
    pub fn restore_database(&self, backup_path: &PathBuf) -> SqliteResult<()> {
        let conn = self.get_connection();

        // 获取数据库路径
        let db_path = conn.path().ok_or_else(|| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some("Cannot determine database path".to_string()),
            )
        })?;

        // 简化的恢复实现 - 复制备份文件
        std::fs::copy(backup_path, db_path).map_err(|e| {
            rusqlite::Error::SqliteFailure(
                rusqlite::ffi::Error::new(rusqlite::ffi::SQLITE_IOERR),
                Some(format!("Failed to restore database: {}", e)),
            )
        })?;

        Ok(())
    }

    // 清空所有数据
    pub fn clear_all_data(&self) -> SqliteResult<()> {
        let conn = self.get_connection();

        conn.execute("DELETE FROM institutions", [])?;
        conn.execute("DELETE FROM healing_items", [])?;
        conn.execute("DELETE FROM chinese_medicines", [])?;
        conn.execute("DELETE FROM medical_records", [])?;
        conn.execute("DELETE FROM medical_treatment_items", [])?;
        conn.execute("DELETE FROM medical_prescription_items", [])?;

        Ok(())
    }

    // 获取数据库统计信息
    pub fn get_database_stats(&self) -> SqliteResult<serde_json::Value> {
        let conn = self.get_connection();

        let healing_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM healing_items", [], |row| row.get(0))?;

        let institution_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM institutions", [], |row| row.get(0))?;

        let medical_record_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM medical_records", [], |row| row.get(0))?;

        let treatment_item_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM medical_treatment_items", [], |row| {
                row.get(0)
            })?;

        let prescription_item_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM medical_prescription_items",
            [],
            |row| row.get(0),
        )?;

        let medicine_count: i64 =
            conn.query_row("SELECT COUNT(*) FROM chinese_medicines", [], |row| {
                row.get(0)
            })?;

        // 按记录类型统计
        let customer_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM medical_records WHERE data_type = 'customer'",
            [],
            |row| row.get(0),
        )?;

        let medical_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM medical_records WHERE data_type = 'medical'",
            [],
            |row| row.get(0),
        )?;

        let prescription_count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM medical_records WHERE data_type = 'prescription'",
            [],
            |row| row.get(0),
        )?;

        Ok(serde_json::json!({
            "healing_items": healing_count,
            "institutions": institution_count,
            "medical_records": medical_record_count,
            "treatment_items": treatment_item_count,
            "prescription_items": prescription_item_count,
            "chinese_medicines": medicine_count,
            "customers": customer_count,
            "medical_records_type": medical_count,
            "prescriptions": prescription_count
        }))
    }
}

pub fn init_database(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    println!("保存数据库路径用于后续连接...");

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("health_clinic.db");
    println!("数据库文件路径: {:?}", db_path);

    // 保存数据库路径
    let _ = DATABASE_PATH
        .set(db_path.clone())
        .map_err(|_| "Failed to set database path")?;

    // 创建并初始化数据库表
    let manager = DatabaseManager::new(&db_path)?;
    drop(manager); // 立即释放连接

    println!("数据库初始化完成");
    Ok(())
}

pub fn get_database() -> DatabaseManager {
    // 等待数据库初始化，最多等待 5 秒
    for _i in 0..50 {
        if let Some(db_path) = DATABASE_PATH.get() {
            return DatabaseManager::new(db_path).expect("Failed to create database manager");
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    panic!("数据库初始化超时，请检查应用配置");
}

// 数据库操作接口
#[tauri::command]
pub async fn backup_database_to_file(backup_path: String) -> Result<String, String> {
    let db = get_database();
    let backup_path = PathBuf::from(backup_path);

    db.backup_database(&backup_path)
        .map_err(|e| format!("备份失败: {}", e))?;

    Ok("数据库备份成功".to_string())
}

#[tauri::command]
pub async fn restore_database_from_file(backup_path: String) -> Result<String, String> {
    let db = get_database();
    let backup_path = PathBuf::from(backup_path);

    db.restore_database(&backup_path)
        .map_err(|e| format!("恢复失败: {}", e))?;

    Ok("数据库恢复成功".to_string())
}

#[tauri::command]
pub async fn clear_all_database_data() -> Result<String, String> {
    let db = get_database();

    db.clear_all_data()
        .map_err(|e| format!("清空数据失败: {}", e))?;

    Ok("所有数据已清空".to_string())
}

#[tauri::command]
pub async fn get_database_statistics() -> Result<serde_json::Value, String> {
    let db = get_database();

    db.get_database_stats()
        .map_err(|e| format!("获取统计信息失败: {}", e))
}

/// 获取数据库文件路径
pub fn get_database_path() -> Option<PathBuf> {
    DATABASE_PATH.get().cloned()
}
