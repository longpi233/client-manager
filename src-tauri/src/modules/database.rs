use rusqlite::{Connection, Result as SqliteResult};
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use tauri::{AppHandle, Manager};

// 数据库路径与全局单例管理器
static DATABASE_PATH: OnceLock<PathBuf> = OnceLock::new();
static DATABASE_MANAGER: OnceLock<DatabaseManager> = OnceLock::new();

#[derive(Clone)]
pub struct DatabaseManager {
    connection: Arc<Mutex<Connection>>,
}

struct Migration {
    version: i64,
    name: &'static str,
    sql: &'static str,
}

// 轻量级版本化迁移注册表，方便后续开发者扩展表和列，同时严格兼容现有库
const MIGRATIONS: &[Migration] = &[
    Migration {
        version: 1,
        name: "initial_clinic_schema",
        sql: "
            -- 机构信息表
            CREATE TABLE IF NOT EXISTS institutions (
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
            );

            -- 治疗项目表
            CREATE TABLE IF NOT EXISTS healing_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                unit_price INTEGER NOT NULL,  -- 价格以万分为单位存储 (1元 = 10000)
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            -- 中药表
            CREATE TABLE IF NOT EXISTS chinese_medicines (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                unit_price INTEGER NOT NULL,     -- 单价（以万分为单位存储）
                description TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            -- 统一医疗记录主表 (散客登记、针灸理疗、中医中药公共主表)
            CREATE TABLE IF NOT EXISTS medical_records (
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
                
                -- 针灸理疗特有字段
                chief_complaint TEXT,
                illness_history TEXT,
                physical_examination TEXT,
                past_history TEXT,
                laboratory_examination TEXT,
                treatment_method TEXT,
                
                -- 中医中药特有字段
                doses INTEGER, -- 付数
                single_dose_amount INTEGER -- 单付金额（以万分为单位）
            );

            -- 治疗项目记录表
            CREATE TABLE IF NOT EXISTS medical_treatment_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                medical_record_id INTEGER NOT NULL,
                treatment_item TEXT NOT NULL,
                count INTEGER NOT NULL DEFAULT 1,
                unit_price INTEGER NOT NULL,     -- 单价（以万分为单位）
                total_price INTEGER NOT NULL,    -- 总价（以万分为单位）
                created_at TEXT NOT NULL,
                FOREIGN KEY (medical_record_id) REFERENCES medical_records(id) ON DELETE CASCADE
            );

            -- 中医中药处方明细表
            CREATE TABLE IF NOT EXISTS medical_prescription_items (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                medical_record_id INTEGER NOT NULL,
                medicine_name TEXT NOT NULL,
                grams INTEGER NOT NULL,
                unit_price INTEGER NOT NULL,     -- 单价（以万分为单位）
                total_price INTEGER NOT NULL,    -- 总价（以万分为单位）
                created_at TEXT NOT NULL,
                FOREIGN KEY (medical_record_id) REFERENCES medical_records(id) ON DELETE CASCADE
            );

            -- 创建高频查询索引
            CREATE INDEX IF NOT EXISTS idx_medical_records_data_type ON medical_records(data_type);
            CREATE INDEX IF NOT EXISTS idx_medical_records_time ON medical_records(time_of_visit);
            CREATE INDEX IF NOT EXISTS idx_treatment_record_id ON medical_treatment_items(medical_record_id);
            CREATE INDEX IF NOT EXISTS idx_prescription_record_id ON medical_prescription_items(medical_record_id);
        ",
    },
];

impl DatabaseManager {
    pub fn new(db_path: &PathBuf) -> SqliteResult<Self> {
        let conn = Connection::open(db_path)?;

        // 基础参数调优：WAL模式、开启外键级联、性能提升
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA synchronous = NORMAL;
             PRAGMA busy_timeout = 5000;",
        )?;

        let manager = DatabaseManager {
            connection: Arc::new(Mutex::new(conn)),
        };

        // 执行结构迁移
        manager.apply_migrations()?;

        Ok(manager)
    }

    pub fn get_connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        self.connection.lock().unwrap()
    }

    // 版本化数据库迁移机制
    fn apply_migrations(&self) -> SqliteResult<()> {
        let conn = self.get_connection();

        conn.execute(
            "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                applied_at TEXT NOT NULL
            )",
            [],
        )?;

        for migration in MIGRATIONS {
            let already_applied: bool = conn
                .query_row(
                    "SELECT 1 FROM schema_migrations WHERE version = ?1",
                    [migration.version],
                    |_| Ok(true),
                )
                .unwrap_or(false);

            if !already_applied {
                println!(
                    "Applying migration {}: {}",
                    migration.version, migration.name
                );
                conn.execute_batch(migration.sql)?;

                let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                conn.execute(
                    "INSERT INTO schema_migrations (version, name, applied_at) VALUES (?1, ?2, ?3)",
                    rusqlite::params![migration.version, migration.name, now],
                )?;
            }
        }

        Ok(())
    }

    // 原生热备份实现，避免活跃 WAL 读写锁及文件损坏
    pub fn backup_database(&self, backup_path: &PathBuf) -> SqliteResult<()> {
        if let Some(parent) = backup_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let mut dst_conn = Connection::open(backup_path)?;
        let src_conn = self.get_connection();

        let backup = rusqlite::backup::Backup::new(&src_conn, &mut dst_conn)?;
        backup.run_to_completion(100, std::time::Duration::from_millis(5), None)?;

        Ok(())
    }

    // 原生热恢复实现
    pub fn restore_database(&self, backup_path: &PathBuf) -> SqliteResult<()> {
        let src_conn = Connection::open(backup_path)?;
        let mut dst_conn = self.get_connection();

        {
            let backup = rusqlite::backup::Backup::new(&src_conn, &mut *dst_conn)?;
            backup.run_to_completion(100, std::time::Duration::from_millis(5), None)?;
        }

        // 重新确保配置有效
        dst_conn.execute_batch(
            "PRAGMA foreign_keys = ON;
             PRAGMA synchronous = NORMAL;",
        )?;

        Ok(())
    }

    // 清空所有数据
    pub fn clear_all_data(&self) -> SqliteResult<()> {
        let conn = self.get_connection();

        conn.execute("DELETE FROM medical_treatment_items", [])?;
        conn.execute("DELETE FROM medical_prescription_items", [])?;
        conn.execute("DELETE FROM medical_records", [])?;
        conn.execute("DELETE FROM institutions", [])?;
        conn.execute("DELETE FROM healing_items", [])?;
        conn.execute("DELETE FROM chinese_medicines", [])?;

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
    println!("🚀 初始化数据库并配置连接...");

    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .expect("Failed to get app data directory");

    std::fs::create_dir_all(&app_data_dir)?;

    let db_path = app_data_dir.join("health_clinic.db");
    println!("数据库文件路径: {:?}", db_path);

    let _ = DATABASE_PATH
        .set(db_path.clone())
        .map_err(|_| "Failed to set database path")?;

    // 初始化全局单例管理器
    let manager = DatabaseManager::new(&db_path)?;
    let _ = DATABASE_MANAGER
        .set(manager)
        .map_err(|_| "Failed to set global database manager")?;

    println!("✅ 数据库初始化完成并准备就绪");
    Ok(())
}

pub fn get_database() -> DatabaseManager {
    // 优先从已初始化的全局单例获取
    if let Some(manager) = DATABASE_MANAGER.get() {
        return manager.clone();
    }

    // 容灾等待
    for _i in 0..50 {
        if let Some(manager) = DATABASE_MANAGER.get() {
            return manager.clone();
        }
        if let Some(db_path) = DATABASE_PATH.get() {
            if let Ok(new_mgr) = DatabaseManager::new(db_path) {
                let _ = DATABASE_MANAGER.set(new_mgr.clone());
                return new_mgr;
            }
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
