use crate::modules::types::ApiResponse;
use serde::{Deserialize, Serialize};
use tauri::command;

#[cfg(feature = "sqlite-storage")]
use crate::modules::database::get_database;
#[cfg(feature = "sqlite-storage")]
use chrono::Utc;
#[cfg(feature = "sqlite-storage")]
use rusqlite::{params, Row};

#[cfg(feature = "memory-storage")]
use crate::modules::utils::contains_pinyin;
#[cfg(feature = "memory-storage")]
use std::collections::HashMap;
#[cfg(feature = "memory-storage")]
use std::sync::LazyLock;
#[cfg(feature = "memory-storage")]
use std::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HealingItem {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,  // 价格（以分为单位）
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealingSearchParams {
    pub current: i32,
    pub size: i32,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealingListResponse {
    pub records: Vec<HealingItem>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

// 内存存储（用于测试模式）
#[cfg(feature = "memory-storage")]
static HEALING_ITEMS: LazyLock<Mutex<HashMap<i32, HealingItem>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[cfg(feature = "memory-storage")]
static NEXT_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

// SQLite 辅助函数
#[cfg(feature = "sqlite-storage")]
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
    #[cfg(feature = "sqlite-storage")]
    {
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
        let total: i32 = conn
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

    #[cfg(feature = "memory-storage")]
    {
        let items = HEALING_ITEMS.lock().unwrap();
        let mut filtered_items: Vec<HealingItem> = items
            .values()
            .filter(|item| {
                // 名称过滤
                if let Some(name) = &params.name {
                    if !name.is_empty() && !contains_pinyin(&item.name, name) {
                        return false;
                    }
                }

                // 状态过滤
                if let Some(status) = &params.status {
                    if !status.is_empty() && item.status != *status {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        // 按创建时间倒序排序
        filtered_items.sort_by(|a, b| b.create_time.cmp(&a.create_time));

        let total = filtered_items.len() as i32;
        let start = ((params.current - 1) * params.size) as usize;
        let end = (start + params.size as usize).min(filtered_items.len());

        let records = if start < filtered_items.len() {
            filtered_items[start..end].to_vec()
        } else {
            Vec::new()
        };

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
}

#[command]
pub async fn create_healing_item(item: HealingItem) -> Result<ApiResponse<HealingItem>, String> {
    #[cfg(feature = "sqlite-storage")]
    {
        let db = get_database();
        let conn = db.get_connection();
        
        // 检查是否已存在相同名称的康复项目
        let check_result: Result<i32, rusqlite::Error> = conn.query_row(
            "SELECT COUNT(*) FROM healing_items WHERE name = ?1",
            params![item.name],
            |row| row.get(0),
        );
        
        match check_result {
            Ok(count) => {
                if count > 0 {
                    return Err(format!("康复项目名称 '{}' 已存在，请使用其他名称", item.name));
                }
            }
            Err(e) => {
                return Err(format!("检查重复名称失败: {}", e));
            }
        }
        
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let result = conn.execute(
            "INSERT INTO healing_items (name, unit_price, description, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![item.name, item.price, item.description, now, now],
        );

        match result {
            Ok(_) => {
                let id = conn.last_insert_rowid() as i32;
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

    #[cfg(feature = "memory-storage")]
    {
        let mut items = HEALING_ITEMS.lock().unwrap();
        let mut next_id = NEXT_ID.lock().unwrap();

        // 检查是否已存在相同名称的康复项目
        for existing_item in items.values() {
            if existing_item.name == item.name {
                return Err(format!("康复项目名称 '{}' 已存在，请使用其他名称", item.name));
            }
        }

        let mut new_item = item;
        new_item.id = Some(*next_id);
        new_item.create_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
        new_item.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

        items.insert(*next_id, new_item.clone());
        *next_id += 1;

        Ok(ApiResponse {
            code: 200,
            msg: "创建成功".to_string(),
            data: new_item,
        })
    }
}

#[command]
pub async fn update_healing_item(item: HealingItem) -> Result<ApiResponse<HealingItem>, String> {
    #[cfg(feature = "sqlite-storage")]
    {
        let db = get_database();
        let conn = db.get_connection();
        
        if let Some(id) = item.id {
            // 检查是否已存在相同名称的其他康复项目
            let check_result: Result<i32, rusqlite::Error> = conn.query_row(
                "SELECT COUNT(*) FROM healing_items WHERE name = ?1 AND id != ?2",
                params![item.name, id],
                |row| row.get(0),
            );
            
            match check_result {
                Ok(count) => {
                    if count > 0 {
                        return Err(format!("康复项目名称 '{}' 已存在，请使用其他名称", item.name));
                    }
                }
                Err(e) => {
                    return Err(format!("检查重复名称失败: {}", e));
                }
            }
            
            let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

            let result = conn.execute(
                "UPDATE healing_items SET name = ?1, unit_price = ?2, 
                 description = ?3, updated_at = ?4 WHERE id = ?5",
                params![
                    item.name,
                    item.price,
                    item.description,
                    now,
                    id
                ],
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

    #[cfg(feature = "memory-storage")]
    {
        let mut items = HEALING_ITEMS.lock().unwrap();

        if let Some(id) = item.id {
            // 检查是否已存在相同名称的其他康复项目
            for (existing_id, existing_item) in items.iter() {
                if existing_item.name == item.name && *existing_id != id {
                    return Err(format!("康复项目名称 '{}' 已存在，请使用其他名称", item.name));
                }
            }
            
            if let Some(existing_item) = items.get_mut(&id) {
                existing_item.name = item.name;
                existing_item.description = item.description;
                existing_item.price = item.price;
                existing_item.status = item.status;
                existing_item.update_time =
                    Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

                Ok(ApiResponse {
                    code: 200,
                    msg: "更新成功".to_string(),
                    data: existing_item.clone(),
                })
            } else {
                Err("记录不存在".to_string())
            }
        } else {
            Err("ID不能为空".to_string())
        }
    }
}

#[command]
pub async fn delete_healing_item(id: i32) -> Result<ApiResponse<String>, String> {
    #[cfg(feature = "sqlite-storage")]
    {
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

    #[cfg(feature = "memory-storage")]
    {
        let mut items = HEALING_ITEMS.lock().unwrap();

        if items.remove(&id).is_some() {
            Ok(ApiResponse {
                code: 200,
                msg: "删除成功".to_string(),
                data: "ok".to_string(),
            })
        } else {
            Err("记录不存在".to_string())
        }
    }
}

#[command]
pub async fn get_healing_item(id: i32) -> Result<ApiResponse<HealingItem>, String> {
    #[cfg(feature = "sqlite-storage")]
    {
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

    #[cfg(feature = "memory-storage")]
    {
        let items = HEALING_ITEMS.lock().unwrap();

        if let Some(item) = items.get(&id) {
            Ok(ApiResponse {
                code: 200,
                msg: "查询成功".to_string(),
                data: item.clone(),
            })
        } else {
            Err("记录不存在".to_string())
        }
    }
}

#[command]
pub async fn batch_delete_healing_items(ids: Vec<i32>) -> Result<ApiResponse<String>, String> {
    #[cfg(feature = "sqlite-storage")]
    {
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

    #[cfg(feature = "memory-storage")]
    {
        let mut items = HEALING_ITEMS.lock().unwrap();
        let mut deleted_count = 0;

        for id in ids {
            if items.remove(&id).is_some() {
                deleted_count += 1;
            }
        }

        Ok(ApiResponse {
            code: 200,
            msg: format!("批量删除成功，删除了{}条记录", deleted_count),
            data: "ok".to_string(),
        })
    }
}

// 初始化示例数据（仅内存模式）
#[cfg(feature = "memory-storage")]
pub fn init_sample_data() {
    let mut items = HEALING_ITEMS.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();

    if items.is_empty() {
        let sample_items = vec![
            HealingItem {
                id: Some(1),
                name: "针灸治疗".to_string(),
                description: Some("传统中医针灸，疏通经络，调节气血".to_string()),
                price: 80.0,
                create_time: Some("2024-01-01 10:00:00".to_string()),
                update_time: Some("2024-01-01 10:00:00".to_string()),
            },
            HealingItem {
                id: Some(2),
                name: "推拿按摩".to_string(),
                description: Some("专业推拿按摩，缓解肌肉疲劳".to_string()),
                price: 120.0,
                create_time: Some("2024-01-01 10:00:00".to_string()),
                update_time: Some("2024-01-01 10:00:00".to_string()),
            },
        ];

        for item in sample_items {
            if let Some(id) = item.id {
                items.insert(id, item);
                if id >= *next_id {
                    *next_id = id + 1;
                }
            }
        }
    }
}

// SQLite 模式下的初始化（不需要示例数据）
#[cfg(feature = "sqlite-storage")]
pub fn init_sample_data() {
    // SQLite 模式下不需要初始化示例数据
    // 数据将从数据库中加载
}
