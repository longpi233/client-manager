use serde::{Deserialize, Serialize};
use tauri::command;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use crate::modules::types::ApiResponse;
use crate::modules::utils::contains_pinyin;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChineseMedicineItem {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub status: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChineseMedicineSearchParams {
    pub current: i32,
    pub size: i32,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChineseMedicineListResponse {
    pub records: Vec<ChineseMedicineItem>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChineseMedicineStatistics {
    pub total: i32,
    pub enabled: i32,
    pub disabled: i32,
}

// 模拟数据存储（实际项目中应该使用数据库）
static CHINESE_MEDICINE_ITEMS: LazyLock<Mutex<HashMap<i32, ChineseMedicineItem>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static CHINESE_MEDICINE_NEXT_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

#[command]
pub async fn get_chinese_medicine_list(
    params: ChineseMedicineSearchParams,
) -> Result<ApiResponse<ChineseMedicineListResponse>, String> {
    let items = CHINESE_MEDICINE_ITEMS.lock().unwrap();

    // 模拟搜索过滤，支持拼音查询
    let mut filtered_items: Vec<ChineseMedicineItem> = items
        .values()
        .filter(|item| {
            if let Some(ref name) = params.name {
                if !name.is_empty() && !contains_pinyin(&item.name, name) {
                    return false;
                }
            }
            if let Some(ref status) = params.status {
                if !status.is_empty() && item.status != *status {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // 排序
    filtered_items.sort_by(|a, b| b.id.cmp(&a.id));

    let total = filtered_items.len() as i32;
    let start = ((params.current - 1) * params.size) as usize;
    let end = std::cmp::min(start + params.size as usize, filtered_items.len());

    let records = if start < filtered_items.len() {
        filtered_items[start..end].to_vec()
    } else {
        Vec::new()
    };

    let response = ChineseMedicineListResponse {
        records,
        current: params.current,
        size: params.size,
        total,
    };

    Ok(ApiResponse {
        code: 200,
        msg: "success".to_string(),
        data: response,
    })
}

#[command]
pub async fn create_chinese_medicine_item(item: ChineseMedicineItem) -> Result<ApiResponse<ChineseMedicineItem>, String> {
    let mut items = CHINESE_MEDICINE_ITEMS.lock().unwrap();
    let mut next_id = CHINESE_MEDICINE_NEXT_ID.lock().unwrap();

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

#[command]
pub async fn update_chinese_medicine_item(item: ChineseMedicineItem) -> Result<ApiResponse<ChineseMedicineItem>, String> {
    let mut items = CHINESE_MEDICINE_ITEMS.lock().unwrap();

    if let Some(id) = item.id {
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
        Err("无效的ID".to_string())
    }
}

#[command]
pub async fn delete_chinese_medicine_item(id: i32) -> Result<ApiResponse<bool>, String> {
    let mut items = CHINESE_MEDICINE_ITEMS.lock().unwrap();

    if items.remove(&id).is_some() {
        Ok(ApiResponse {
            code: 200,
            msg: "删除成功".to_string(),
            data: true,
        })
    } else {
        Err("记录不存在".to_string())
    }
}

#[command]
pub async fn batch_delete_chinese_medicine_items(ids: Vec<i32>) -> Result<ApiResponse<i32>, String> {
    let mut items = CHINESE_MEDICINE_ITEMS.lock().unwrap();
    let mut deleted_count = 0;

    for id in ids {
        if items.remove(&id).is_some() {
            deleted_count += 1;
        }
    }

    Ok(ApiResponse {
        code: 200,
        msg: format!("批量删除成功，共删除{}条记录", deleted_count),
        data: deleted_count,
    })
}

#[command]
pub async fn get_chinese_medicine_item(id: i32) -> Result<ApiResponse<ChineseMedicineItem>, String> {
    let items = CHINESE_MEDICINE_ITEMS.lock().unwrap();

    if let Some(item) = items.get(&id) {
        Ok(ApiResponse {
            code: 200,
            msg: "获取成功".to_string(),
            data: item.clone(),
        })
    } else {
        Err("记录不存在".to_string())
    }
}

#[command]
pub async fn get_chinese_medicine_statistics() -> Result<ApiResponse<ChineseMedicineStatistics>, String> {
    let items = CHINESE_MEDICINE_ITEMS.lock().unwrap();
    
    let total = items.len() as i32;
    let enabled = items.values().filter(|item| item.status == "1").count() as i32;
    let disabled = items.values().filter(|item| item.status == "2").count() as i32;

    let statistics = ChineseMedicineStatistics {
        total,
        enabled,
        disabled,
    };

    Ok(ApiResponse {
        code: 200,
        msg: "获取统计信息成功".to_string(),
        data: statistics,
    })
}

#[command]
pub async fn batch_update_chinese_medicine_status(ids: Vec<i32>, status: String) -> Result<ApiResponse<i32>, String> {
    let mut items = CHINESE_MEDICINE_ITEMS.lock().unwrap();
    let mut updated_count = 0;

    for id in ids {
        if let Some(item) = items.get_mut(&id) {
            item.status = status.clone();
            item.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
            updated_count += 1;
        }
    }

    Ok(ApiResponse {
        code: 200,
        msg: format!("批量更新状态成功，共更新{}条记录", updated_count),
        data: updated_count,
    })
}

// 初始化示例数据
pub fn init_sample_data() {
    let mut items = CHINESE_MEDICINE_ITEMS.lock().unwrap();
    let mut next_id = CHINESE_MEDICINE_NEXT_ID.lock().unwrap();

    if items.is_empty() {
        let sample_items = vec![
            ChineseMedicineItem {
                id: Some(1),
                name: "人参".to_string(),
                description: Some("大补元气，复脉固脱".to_string()),
                price: 15.0,
                status: "1".to_string(),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            ChineseMedicineItem {
                id: Some(2),
                name: "黄芪".to_string(),
                description: Some("补气固表，利尿托毒".to_string()),
                price: 8.0,
                status: "1".to_string(),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            ChineseMedicineItem {
                id: Some(3),
                name: "当归".to_string(),
                description: Some("补血活血，调经止痛".to_string()),
                price: 12.0,
                status: "1".to_string(),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            ChineseMedicineItem {
                id: Some(4),
                name: "川芎".to_string(),
                description: Some("活血行气，祛风止痛".to_string()),
                price: 10.0,
                status: "1".to_string(),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            ChineseMedicineItem {
                id: Some(5),
                name: "白芍".to_string(),
                description: Some("养血敛阴，柔肝止痛".to_string()),
                price: 6.0,
                status: "2".to_string(), // 禁用状态
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            ChineseMedicineItem {
                id: Some(6),
                name: "茯苓".to_string(),
                description: Some("利水渗湿，健脾宁心".to_string()),
                price: 5.0,
                status: "1".to_string(),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            ChineseMedicineItem {
                id: Some(7),
                name: "甘草".to_string(),
                description: Some("补脾益气，清热解毒".to_string()),
                price: 3.0,
                status: "1".to_string(),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            ChineseMedicineItem {
                id: Some(8),
                name: "枸杞".to_string(),
                description: Some("滋补肝肾，益精明目".to_string()),
                price: 18.0,
                status: "1".to_string(),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
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

        println!("已初始化 {} 条示例中药数据", items.len());
    }
}
