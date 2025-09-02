use crate::modules::database::get_database;
use crate::modules::types::ApiResponse;
use chrono::Local;
use rusqlite::{params, Row};
use serde::{Deserialize, Serialize};
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealingItem {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub price: i64, // 价格（以分为单位）
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealingSearchParams {
    pub current: i64,
    pub size: i64,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealingListResponse {
    pub records: Vec<HealingItem>,
    pub current: i64,
    pub size: i64,
    pub total: i64,
}

// SQLite 辅助函数
fn row_to_healing_item(row: &Row) -> rusqlite::Result<HealingItem> {
    Ok(HealingItem {
        id: Some(row.get("id")?),
        name: row.get("name")?,
        description: row.get("description")?,
        price: row.get("unit_price")?,
        create_time: Some(row.get("created_at")?),
        update_time: Some(row.get("updated_at")?),
    })
}

#[command]
pub async fn get_healing_list(
    params: HealingSearchParams,
) -> Result<ApiResponse<HealingListResponse>, String> {
    println!("🔧 使用 SQLite 存储模式获取康养列表");
    let db = get_database();
    let conn = db.get_connection();

    let mut where_clauses = Vec::new();
    let mut query_params = Vec::new();

    if let Some(name) = &params.name {
        if !name.is_empty() {
            where_clauses.push("name LIKE ?");
            query_params.push(format!("%{}%", name));
        }
    }

    let where_clause = if where_clauses.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", where_clauses.join(" AND "))
    };

    // 获取总数
    let count_query = format!("SELECT COUNT(*) FROM healing_items{}", where_clause);
    let total: i64 = conn
        .query_row(
            &count_query,
            rusqlite::params_from_iter(&query_params),
            |row| row.get(0),
        )
        .map_err(|e| format!("查询总数失败: {}", e))?;

    // 获取分页数据
    let offset = (params.current - 1) * params.size;
    let query = format!(
        "SELECT * FROM healing_items{} ORDER BY created_at DESC LIMIT ? OFFSET ?",
        where_clause
    );

    let mut stmt = conn
        .prepare(&query)
        .map_err(|e| format!("准备查询失败: {}", e))?;
    let mut param_values = query_params;
    param_values.push(params.size.to_string());
    param_values.push(offset.to_string());

    let healing_iter = stmt
        .query_map(
            rusqlite::params_from_iter(&param_values),
            row_to_healing_item,
        )
        .map_err(|e| format!("执行查询失败: {}", e))?;

    let mut records = Vec::new();
    for item in healing_iter {
        records.push(item.map_err(|e| format!("解析记录失败: {}", e))?);
    }

    Ok(ApiResponse {
        code: 200,
        msg: "查询成功".to_string(),
        data: HealingListResponse {
            records,
            current: params.current,
            size: params.size,
            total,
        },
    })
}

#[command]
pub async fn create_healing_item(item: HealingItem) -> Result<ApiResponse<HealingItem>, String> {
    let db = get_database();
    let conn = db.get_connection();

    // 检查是否已存在相同名称的康复项目
    let check_result: Result<i64, rusqlite::Error> = conn.query_row(
        "SELECT COUNT(*) FROM healing_items WHERE name = ?1",
        params![item.name],
        |row| row.get(0),
    );

    match check_result {
        Ok(count) => {
            if count > 0 {
                return Err(format!(
                    "康复项目名称 '{}' 已存在，请使用其他名称",
                    item.name
                ));
            }
        }
        Err(e) => {
            return Err(format!("检查重复名称失败: {}", e));
        }
    }

    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let result = conn.execute(
        "INSERT INTO healing_items (name, unit_price, description, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
        params![item.name, item.price, item.description, now, now],
    );

    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid();
            let mut new_item = item;
            new_item.id = Some(id);
            new_item.create_time = Some(now.clone());
            new_item.update_time = Some(now);

            Ok(ApiResponse {
                code: 200,
                msg: "创建成功".to_string(),
                data: new_item,
            })
        }
        Err(e) => Err(format!("创建失败: {}", e)),
    }
}

#[command]
pub async fn update_healing_item(item: HealingItem) -> Result<ApiResponse<HealingItem>, String> {
    let db = get_database();
    let conn = db.get_connection();

    if let Some(id) = item.id {
        // 检查是否已存在相同名称的其他康复项目
        let check_result: Result<i64, rusqlite::Error> = conn.query_row(
            "SELECT COUNT(*) FROM healing_items WHERE name = ?1 AND id != ?2",
            params![item.name, id],
            |row| row.get(0),
        );

        match check_result {
            Ok(count) => {
                if count > 0 {
                    return Err(format!(
                        "康复项目名称 '{}' 已存在，请使用其他名称",
                        item.name
                    ));
                }
            }
            Err(e) => {
                return Err(format!("检查重复名称失败: {}", e));
            }
        }

        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            "UPDATE healing_items SET name = ?1, unit_price = ?2, 
                 description = ?3, updated_at = ?4 WHERE id = ?5",
            params![item.name, item.price, item.description, now, id],
        );

        match result {
            Ok(updated) => {
                if updated > 0 {
                    let mut updated_item = item;
                    updated_item.update_time = Some(now);
                    Ok(ApiResponse {
                        code: 200,
                        msg: "更新成功".to_string(),
                        data: updated_item,
                    })
                } else {
                    Err("记录不存在".to_string())
                }
            }
            Err(e) => Err(format!("更新失败: {}", e)),
        }
    } else {
        Err("ID不能为空".to_string())
    }
}

#[command]
pub async fn delete_healing_item(id: i64) -> Result<ApiResponse<String>, String> {
    let db = get_database();
    let conn = db.get_connection();

    let result = conn.execute("DELETE FROM healing_items WHERE id = ?1", params![id]);

    match result {
        Ok(deleted) => {
            if deleted > 0 {
                Ok(ApiResponse {
                    code: 200,
                    msg: "删除成功".to_string(),
                    data: "ok".to_string(),
                })
            } else {
                Err("记录不存在".to_string())
            }
        }
        Err(e) => Err(format!("删除失败: {}", e)),
    }
}

#[command]
pub async fn get_healing_item(id: i64) -> Result<ApiResponse<HealingItem>, String> {
    let db = get_database();
    let conn = db.get_connection();

    let result = conn.query_row(
        "SELECT * FROM healing_items WHERE id = ?1",
        params![id],
        row_to_healing_item,
    );

    match result {
        Ok(item) => Ok(ApiResponse {
            code: 200,
            msg: "查询成功".to_string(),
            data: item,
        }),
        Err(rusqlite::Error::QueryReturnedNoRows) => Err("记录不存在".to_string()),
        Err(e) => Err(format!("查询失败: {}", e)),
    }
}

#[command]
pub async fn batch_delete_healing_items(ids: Vec<i64>) -> Result<ApiResponse<String>, String> {
    let db = get_database();
    let conn = db.get_connection();

    let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let query = format!("DELETE FROM healing_items WHERE id IN ({})", placeholders);

    let params: Vec<&dyn rusqlite::ToSql> =
        ids.iter().map(|id| id as &dyn rusqlite::ToSql).collect();

    let result = conn.execute(&query, &params[..]);

    match result {
        Ok(deleted) => Ok(ApiResponse {
            code: 200,
            msg: format!("批量删除成功，删除了{}条记录", deleted),
            data: "ok".to_string(),
        }),
        Err(e) => Err(format!("批量删除失败: {}", e)),
    }
}
