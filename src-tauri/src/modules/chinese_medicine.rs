use crate::modules::types::ApiResponse;
use serde::{Deserialize, Serialize};
use tauri::command;

// SQLite存储模式的导入
use crate::modules::database::get_database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChineseMedicineItem {
    pub id: Option<i64>,
    pub name: String,
    pub unit_price: i64, // 价格（以分为单位）
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

// 兼容旧接口的参数结构体
// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChineseMedicineCreateParams {
//     pub name: String,
//     pub description: Option<String>,
//     pub price: i64, // 价格（以分为单位）
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct ChineseMedicineUpdateParams {
//     pub id: i64, // 添加id字段
//     pub name: String,
//     pub description: Option<String>,
//     pub price: i64, // 价格（以分为单位）
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct ChineseMedicineSearchParams {
    pub current: i64,
    pub size: i64,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChineseMedicineListResponse {
    pub records: Vec<ChineseMedicineItem>,
    pub current: i64,
    pub size: i64,
    pub total: i64,
}

// 辅助函数：从数据库行转换为结构体

fn db_row_to_medicine_item(row: &rusqlite::Row) -> rusqlite::Result<ChineseMedicineItem> {
    Ok(ChineseMedicineItem {
        id: Some(row.get("id")?),
        name: row.get("name")?,
        unit_price: row.get("unit_price")?, // 数据库字段仍然是unit_price，但映射到price
        description: row.get("description")?,
        create_time: row.get("created_at")?,
        update_time: row.get("updated_at")?,
    })
}

#[command]
pub async fn get_chinese_medicine_list(
    params: ChineseMedicineSearchParams,
) -> Result<ApiResponse<ChineseMedicineListResponse>, String> {
    println!("接收到中药列表查询请求: {:?}", params);

    println!("🔧 使用 SQLite 存储模式获取中药列表");

    let db = get_database();
    let conn = db.get_connection();

    // 构建查询SQL
    let mut sql = "SELECT id, name,  unit_price, description, created_at, updated_at 
                             FROM chinese_medicines WHERE 1=1"
        .to_string();
    let mut sql_params: Vec<String> = Vec::new();

    if let Some(ref name) = params.name {
        if !name.is_empty() {
            sql.push_str(" AND name LIKE ?");
            sql_params.push(format!("%{}%", name));
        }
    }

    sql.push_str(" ORDER BY created_at DESC");

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("准备查询失败: {}", e))?;
    let medicine_iter = stmt
        .query_map(
            rusqlite::params_from_iter(sql_params.iter()),
            db_row_to_medicine_item,
        )
        .map_err(|e| format!("查询失败: {}", e))?;

    let mut all_records: Vec<ChineseMedicineItem> = medicine_iter
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("获取记录失败: {}", e))?;

    // 分页处理
    let total = all_records.len();
    let start = ((params.current - 1) * params.size) as usize;
    let end = std::cmp::min(start + params.size as usize, all_records.len());

    if start < all_records.len() {
        all_records = all_records[start..end].to_vec();
    } else {
        all_records = Vec::new();
    }

    println!(
        "🔍 查询结果: total={}, current_page_records={}",
        total,
        all_records.len()
    );
    for (i, record) in all_records.iter().enumerate() {
        println!(
            "  记录 {}: id={:?}, name={}, price={}",
            i + 1,
            record.id,
            record.name,
            record.unit_price
        );
    }

    Ok(ApiResponse {
        code: 200,
        msg: "查询成功".to_string(),
        data: ChineseMedicineListResponse {
            records: all_records.clone(),
            current: params.current,
            size: params.size,
            total: total as i64,
        },
    })
}

#[command]
pub async fn create_chinese_medicine_item(
    item: ChineseMedicineItem,
) -> Result<ApiResponse<ChineseMedicineItem>, String> {
    println!("接收到中药创建请求: {:?}", item);

    println!("🔧 使用 SQLite 存储模式创建中药");

    let db = get_database();
    let conn = db.get_connection();

    // 先检查名称是否已存在
    let mut check_stmt = conn
        .prepare("SELECT COUNT(*) FROM chinese_medicines WHERE name = ?1")
        .map_err(|e| format!("准备检查语句失败: {}", e))?;

    let count: i64 = check_stmt
        .query_row(rusqlite::params![item.name], |row| row.get(0))
        .map_err(|e| format!("检查名称是否存在失败: {}", e))?;

    if count > 0 {
        return Err(format!("中药名称 '{}' 已存在，请使用不同的名称", item.name));
    }

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut stmt = conn
        .prepare(
            "INSERT INTO chinese_medicines (name, unit_price, description, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )
        .map_err(|e| format!("准备插入语句失败: {}", e))?;

    stmt.execute(rusqlite::params![
        item.name,
        item.unit_price,
        item.description,
        now,
        now
    ])
    .map_err(|e| {
        // 如果仍然出现唯一约束错误，提供更友好的错误信息
        if e.to_string().contains("UNIQUE constraint failed") {
            format!("中药名称 '{}' 已存在，请使用不同的名称", item.name)
        } else {
            format!("插入记录失败: {}", e)
        }
    })?;

    let item_id = conn.last_insert_rowid();

    let created_item = ChineseMedicineItem {
        id: Some(item_id),
        name: item.name,
        unit_price: item.unit_price,
        description: item.description,
        create_time: Some(now.clone()),
        update_time: Some(now),
    };

    Ok(ApiResponse {
        code: 200,
        msg: "创建成功".to_string(),
        data: created_item,
    })
}

#[command]
pub async fn update_chinese_medicine_item(
    item: ChineseMedicineItem,
) -> Result<ApiResponse<ChineseMedicineItem>, String> {
    println!("接收到中药更新请求: id={:?}, item={:?}", item.id, item);

    println!("🔧 使用 SQLite 存储模式更新中药");

    let db = get_database();
    let conn = db.get_connection();

    // 检查名称是否已被其他记录使用
    let mut stmt = conn
        .prepare("SELECT COUNT(*) FROM chinese_medicines WHERE name = ?1 AND id != ?2")
        .map_err(|e| format!("准备查询失败: {}", e))?;

    let count: i64 = stmt
        .query_row(rusqlite::params![item.name, item.id], |row| row.get(0))
        .map_err(|e| format!("查询失败: {}", e))?;

    if count > 0 {
        return Err(format!("中药名称 '{}' 已存在，请使用不同的名称", item.name));
    }

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mut stmt = conn
        .prepare(
            "UPDATE chinese_medicines SET name = ?1, unit_price = ?2, description = ?3, 
             updated_at = ?4 WHERE id = ?5",
        )
        .map_err(|e| format!("准备更新语句失败: {}", e))?;

    let affected_rows = stmt
        .execute(rusqlite::params![
            item.name,
            item.unit_price,
            item.description,
            now,
            item.id
        ])
        .map_err(|e| format!("更新记录失败: {}", e))?;

    if affected_rows == 0 {
        return Err("记录不存在".to_string());
    }

    // 查询更新后的记录
    let mut stmt = conn
        .prepare(
            "SELECT id, name, unit_price, description, created_at, updated_at 
             FROM chinese_medicines WHERE id = ?1",
        )
        .map_err(|e| format!("准备查询语句失败: {}", e))?;

    let updated_item = stmt
        .query_row(rusqlite::params![item.id], db_row_to_medicine_item)
        .map_err(|e| format!("查询更新后记录失败: {}", e))?;

    Ok(ApiResponse {
        code: 200,
        msg: "更新成功".to_string(),
        data: updated_item,
    })
}

#[command]
pub async fn delete_chinese_medicine_item(id: i64) -> Result<ApiResponse<String>, String> {
    println!("接收到中药删除请求: id={}", id);

    println!("🔧 使用 SQLite 存储模式删除中药");

    let db = get_database();
    let conn = db.get_connection();

    let mut stmt = conn
        .prepare("DELETE FROM chinese_medicines WHERE id = ?1")
        .map_err(|e| format!("准备删除语句失败: {}", e))?;

    let affected_rows = stmt
        .execute(rusqlite::params![id])
        .map_err(|e| format!("删除记录失败: {}", e))?;

    if affected_rows == 0 {
        return Err("记录不存在".to_string());
    }

    Ok(ApiResponse {
        code: 200,
        msg: "删除成功".to_string(),
        data: "删除成功".to_string(),
    })
}

// 其他必要的函数，用简化的形式实现...
#[command]
pub async fn batch_delete_chinese_medicine_items(
    ids: Vec<i64>,
) -> Result<ApiResponse<String>, String> {
    println!("接收到批量删除中药请求: ids={:?}", ids);

    for id in ids {
        let _ = delete_chinese_medicine_item(id).await?;
    }

    Ok(ApiResponse {
        code: 200,
        msg: "批量删除成功".to_string(),
        data: "批量删除成功".to_string(),
    })
}

#[command]
pub async fn get_chinese_medicine_item(
    id: i64,
) -> Result<ApiResponse<ChineseMedicineItem>, String> {
    println!("接收到获取单个中药请求: id={}", id);

    let db = get_database();
    let conn = db.get_connection();

    let mut stmt = conn
        .prepare(
            "SELECT id, name, unit_price, description, created_at, updated_at FROM chinese_medicines WHERE id = ?1",
        )
        .map_err(|e| format!("准备查询语句失败: {}", e))?;

    let item = stmt
        .query_row(rusqlite::params![id], db_row_to_medicine_item)
        .map_err(|e| format!("查询记录失败: {}", e))?;

    Ok(ApiResponse {
        code: 200,
        msg: "查询成功".to_string(),
        data: item,
    })
}
