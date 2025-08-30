use crate::modules::types::ApiResponse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChineseMedicinePrescriptionItem {
    pub medicine_name: String,
    pub unit_price: f64,
    pub grams: f64,
    pub total_price: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChineseMedicinePrescriptionRecord {
    pub id: Option<i32>,
    pub customer_id: i32,
    pub customer_name: String,
    pub gender: String,
    pub age: i32,
    pub phone: String,
    pub diagnosis: String,
    pub prescription: Vec<ChineseMedicinePrescriptionItem>,
    pub doses: i32,
    pub single_dose_amount: f64,
    pub total_amount: f64,
    pub doctor_id: String,
    pub doctor_name: String,
    pub time_of_visit: String,
    pub note: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChineseMedicinePrescriptionSearchParams {
    pub current: i32,
    pub size: i32,
    pub customer_name: Option<String>,
    pub telephone: Option<String>,
    pub diagnosis: Option<String>,
    pub medicine_name: Option<String>,
    pub doctor_id: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub gender: Option<String>,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChineseMedicinePrescriptionListResponse {
    pub records: Vec<ChineseMedicinePrescriptionRecord>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

// 模拟数据存储
static PRESCRIPTION_RECORDS: LazyLock<Mutex<HashMap<i32, ChineseMedicinePrescriptionRecord>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static PRESCRIPTION_NEXT_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

#[command]
pub async fn get_chinese_medicine_prescription_list(
    params: ChineseMedicinePrescriptionSearchParams,
) -> ApiResponse<ChineseMedicinePrescriptionListResponse> {
    println!("接收到中药处方查询请求: {:?}", params);
    let records = PRESCRIPTION_RECORDS.lock().unwrap();
    let mut filtered_records: Vec<ChineseMedicinePrescriptionRecord> =
        records.values().cloned().collect();

    // 过滤条件
    if let Some(customer_name) = &params.customer_name {
        if !customer_name.is_empty() {
            filtered_records.retain(|record| record.customer_name.contains(customer_name));
        }
    }

    if let Some(telephone) = &params.telephone {
        if !telephone.is_empty() {
            filtered_records.retain(|record| record.phone.contains(telephone));
        }
    }

    if let Some(diagnosis) = &params.diagnosis {
        if !diagnosis.is_empty() {
            filtered_records.retain(|record| record.diagnosis.contains(diagnosis));
        }
    }

    if let Some(ref gender) = params.gender {
        if !gender.is_empty() {
            filtered_records.retain(|record| record.gender == *gender);
        }
    }

    if let Some(min_age) = params.min_age {
        filtered_records.retain(|record| record.age >= min_age);
    }

    if let Some(max_age) = params.max_age {
        filtered_records.retain(|record| record.age <= max_age);
    }

    if let Some(medicine_name) = &params.medicine_name {
        if !medicine_name.is_empty() {
            filtered_records.retain(|record| {
                record
                    .prescription
                    .iter()
                    .any(|item| item.medicine_name.contains(medicine_name))
            });
        }
    }

    if let Some(doctor_id) = &params.doctor_id {
        if !doctor_id.is_empty() {
            filtered_records.retain(|record| record.doctor_id == *doctor_id);
        }
    }

    // 日期过滤
    if let Some(start_date) = &params.start_date {
        if !start_date.is_empty() {
            filtered_records.retain(|record| {
                let record_date = record.time_of_visit.split('T').next().unwrap_or(&record.time_of_visit);
                record_date >= start_date.as_str()
            });
        }
    }
    
    if let Some(end_date) = &params.end_date {
        if !end_date.is_empty() {
            filtered_records.retain(|record| {
                let record_date = record.time_of_visit.split('T').next().unwrap_or(&record.time_of_visit);
                record_date <= end_date.as_str()
            });
        }
    }

    // 排序（按就诊时间倒序）
    filtered_records.sort_by(|a, b| b.time_of_visit.cmp(&a.time_of_visit));

    let total = filtered_records.len() as i32;
    let start_index = ((params.current - 1) * params.size) as usize;
    let end_index = (start_index + params.size as usize).min(filtered_records.len());

    let page_records = if start_index < filtered_records.len() {
        filtered_records[start_index..end_index].to_vec()
    } else {
        vec![]
    };

    ApiResponse {
        code: 200,
        msg: "查询成功".to_string(),
        data: ChineseMedicinePrescriptionListResponse {
            records: page_records,
            current: params.current,
            size: params.size,
            total,
        },
    }
}

#[command]
pub async fn create_chinese_medicine_prescription(
    record: ChineseMedicinePrescriptionRecord,
) -> ApiResponse<ChineseMedicinePrescriptionRecord> {
    let mut records = PRESCRIPTION_RECORDS.lock().unwrap();
    let mut next_id = PRESCRIPTION_NEXT_ID.lock().unwrap();

    let mut new_record = record;
    new_record.id = Some(*next_id);

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    new_record.create_time = Some(now.clone());
    new_record.update_time = Some(now);

    records.insert(*next_id, new_record.clone());
    *next_id += 1;

    ApiResponse {
        code: 200,
        msg: "创建成功".to_string(),
        data: new_record,
    }
}

#[command]
pub async fn get_chinese_medicine_prescription(
    id: i32,
) -> ApiResponse<ChineseMedicinePrescriptionRecord> {
    let records = PRESCRIPTION_RECORDS.lock().unwrap();

    match records.get(&id) {
        Some(record) => ApiResponse {
            code: 200,
            msg: "查询成功".to_string(),
            data: record.clone(),
        },
        None => ApiResponse {
            code: 404,
            msg: "处方记录不存在".to_string(),
            data: ChineseMedicinePrescriptionRecord {
                id: None,
                customer_id: 0,
                customer_name: String::new(),
                gender: String::new(),
                age: 0,
                phone: String::new(),
                diagnosis: String::new(),
                prescription: vec![],
                doses: 0,
                single_dose_amount: 0.0,
                total_amount: 0.0,
                doctor_id: String::new(),
                doctor_name: String::new(),
                time_of_visit: String::new(),
                note: None,
                create_time: None,
                update_time: None,
            },
        },
    }
}

#[command]
pub async fn update_chinese_medicine_prescription(
    record: ChineseMedicinePrescriptionRecord,
) -> ApiResponse<ChineseMedicinePrescriptionRecord> {
    let mut records = PRESCRIPTION_RECORDS.lock().unwrap();

    if let Some(id) = record.id {
        if records.contains_key(&id) {
            let mut updated_record = record;
            updated_record.update_time =
                Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

            records.insert(id, updated_record.clone());

            ApiResponse {
                code: 200,
                msg: "更新成功".to_string(),
                data: updated_record,
            }
        } else {
            ApiResponse {
                code: 404,
                msg: "处方记录不存在".to_string(),
                data: record,
            }
        }
    } else {
        ApiResponse {
            code: 400,
            msg: "无效的记录ID".to_string(),
            data: record,
        }
    }
}

#[command]
pub async fn delete_chinese_medicine_prescription(id: i32) -> ApiResponse<String> {
    let mut records = PRESCRIPTION_RECORDS.lock().unwrap();

    match records.remove(&id) {
        Some(_) => ApiResponse {
            code: 200,
            msg: "删除成功".to_string(),
            data: "success".to_string(),
        },
        None => ApiResponse {
            code: 404,
            msg: "处方记录不存在".to_string(),
            data: "not_found".to_string(),
        },
    }
}

// 初始化示例数据
pub fn init_sample_data() {
    let mut records = PRESCRIPTION_RECORDS.lock().unwrap();
    let mut next_id = PRESCRIPTION_NEXT_ID.lock().unwrap();

    if records.is_empty() {
        let sample_records = vec![
            // 2025年8月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(1),
                customer_id: 1,
                customer_name: "张三".to_string(),
                gender: "Male".to_string(),
                age: 35,
                phone: "13800138001".to_string(),
                diagnosis: "颈椎病".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "葛根".to_string(),
                        unit_price: 0.3,
                        grams: 15.0,
                        total_price: 4.5,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "白芍".to_string(),
                        unit_price: 0.4,
                        grams: 12.0,
                        total_price: 4.8,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "甘草".to_string(),
                        unit_price: 0.2,
                        grams: 6.0,
                        total_price: 1.2,
                    },
                ],
                doses: 7,
                single_dose_amount: 10.5,
                total_amount: 73.5,
                doctor_id: "1".to_string(),
                doctor_name: "李明".to_string(),
                time_of_visit: "2025-08-15 11:00:00".to_string(),
                note: Some("饭后服用，避免辛辣".to_string()),
                create_time: Some("2025-08-15 11:00:00".to_string()),
                update_time: Some("2025-08-15 11:00:00".to_string()),
            },
            ChineseMedicinePrescriptionRecord {
                id: Some(2),
                customer_id: 2,
                customer_name: "王丽".to_string(),
                gender: "Female".to_string(),
                age: 28,
                phone: "13800138003".to_string(),
                diagnosis: "失眠多梦".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "酸枣仁".to_string(),
                        unit_price: 0.8,
                        grams: 15.0,
                        total_price: 12.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "茯苓".to_string(),
                        unit_price: 0.15,
                        grams: 20.0,
                        total_price: 3.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "远志".to_string(),
                        unit_price: 0.6,
                        grams: 10.0,
                        total_price: 6.0,
                    },
                ],
                doses: 14,
                single_dose_amount: 21.0,
                total_amount: 294.0,
                doctor_id: "2".to_string(),
                doctor_name: "王医生".to_string(),
                time_of_visit: "2025-08-22 15:30:00".to_string(),
                note: Some("睡前服用，避免茶水".to_string()),
                create_time: Some("2025-08-22 15:30:00".to_string()),
                update_time: Some("2025-08-22 15:30:00".to_string()),
            },
            // 2025年6月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(3),
                customer_id: 3,
                customer_name: "刘建国".to_string(),
                gender: "Male".to_string(),
                age: 45,
                phone: "13800138005".to_string(),
                diagnosis: "腰椎间盘突出".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "独活".to_string(),
                        unit_price: 0.5,
                        grams: 12.0,
                        total_price: 6.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "杜仲".to_string(),
                        unit_price: 0.7,
                        grams: 15.0,
                        total_price: 10.5,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "续断".to_string(),
                        unit_price: 0.4,
                        grams: 10.0,
                        total_price: 4.0,
                    },
                ],
                doses: 10,
                single_dose_amount: 20.5,
                total_amount: 205.0,
                doctor_id: "1".to_string(),
                doctor_name: "李明".to_string(),
                time_of_visit: "2025-06-10 10:30:00".to_string(),
                note: Some("温水冲服，配合理疗".to_string()),
                create_time: Some("2025-06-10 10:30:00".to_string()),
                update_time: Some("2025-06-10 10:30:00".to_string()),
            },
            // 2025年7月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(4),
                customer_id: 4,
                customer_name: "陈小明".to_string(),
                gender: "Male".to_string(),
                age: 32,
                phone: "13800138007".to_string(),
                diagnosis: "慢性浅表性胃炎".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "白术".to_string(),
                        unit_price: 0.35,
                        grams: 15.0,
                        total_price: 5.25,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "陈皮".to_string(),
                        unit_price: 0.25,
                        grams: 10.0,
                        total_price: 2.5,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "半夏".to_string(),
                        unit_price: 0.6,
                        grams: 8.0,
                        total_price: 4.8,
                    },
                ],
                doses: 14,
                single_dose_amount: 12.55,
                total_amount: 175.7,
                doctor_id: "3".to_string(),
                doctor_name: "张医生".to_string(),
                time_of_visit: "2025-07-18 17:00:00".to_string(),
                note: Some("空腹服用，忌生冷".to_string()),
                create_time: Some("2025-07-18 17:00:00".to_string()),
                update_time: Some("2025-07-18 17:00:00".to_string()),
            },
            // 2025年9月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(5),
                customer_id: 5,
                customer_name: "李雅雯".to_string(),
                gender: "Female".to_string(),
                age: 26,
                phone: "13800138009".to_string(),
                diagnosis: "月经不调".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "当归".to_string(),
                        unit_price: 0.45,
                        grams: 12.0,
                        total_price: 5.4,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "川芎".to_string(),
                        unit_price: 0.5,
                        grams: 8.0,
                        total_price: 4.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "益母草".to_string(),
                        unit_price: 0.2,
                        grams: 20.0,
                        total_price: 4.0,
                    },
                ],
                doses: 21,
                single_dose_amount: 13.4,
                total_amount: 281.4,
                doctor_id: "2".to_string(),
                doctor_name: "王医生".to_string(),
                time_of_visit: "2025-09-05 12:00:00".to_string(),
                note: Some("经期停服，温水冲服".to_string()),
                create_time: Some("2025-09-05 12:00:00".to_string()),
                update_time: Some("2025-09-05 12:00:00".to_string()),
            },
            // 2024年12月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(6),
                customer_id: 6,
                customer_name: "赵大伯".to_string(),
                gender: "Male".to_string(),
                age: 68,
                phone: "13800138011".to_string(),
                diagnosis: "骨关节炎".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "威灵仙".to_string(),
                        unit_price: 0.3,
                        grams: 15.0,
                        total_price: 4.5,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "桑寄生".to_string(),
                        unit_price: 0.4,
                        grams: 20.0,
                        total_price: 8.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "牛膝".to_string(),
                        unit_price: 0.25,
                        grams: 12.0,
                        total_price: 3.0,
                    },
                ],
                doses: 14,
                single_dose_amount: 15.5,
                total_amount: 217.0,
                doctor_id: "1".to_string(),
                doctor_name: "李明".to_string(),
                time_of_visit: "2024-12-25 15:00:00".to_string(),
                note: Some("饭后服用，注意保暖".to_string()),
                create_time: Some("2024-12-25 15:00:00".to_string()),
                update_time: Some("2024-12-25 15:00:00".to_string()),
            },
            // 2025年1月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(7),
                customer_id: 7,
                customer_name: "孙美华".to_string(),
                gender: "Female".to_string(),
                age: 42,
                phone: "13800138013".to_string(),
                diagnosis: "缺铁性贫血".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "熟地黄".to_string(),
                        unit_price: 0.3,
                        grams: 20.0,
                        total_price: 6.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "阿胶".to_string(),
                        unit_price: 2.0,
                        grams: 8.0,
                        total_price: 16.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "红枣".to_string(),
                        unit_price: 0.15,
                        grams: 15.0,
                        total_price: 2.25,
                    },
                ],
                doses: 28,
                single_dose_amount: 24.25,
                total_amount: 679.0,
                doctor_id: "3".to_string(),
                doctor_name: "张医生".to_string(),
                time_of_visit: "2025-01-08 10:30:00".to_string(),
                note: Some("早晚空腹服用".to_string()),
                create_time: Some("2025-01-08 10:30:00".to_string()),
                update_time: Some("2025-01-08 10:30:00".to_string()),
            },
            // 2025年3月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(8),
                customer_id: 8,
                customer_name: "周志强".to_string(),
                gender: "Male".to_string(),
                age: 55,
                phone: "13800138015".to_string(),
                diagnosis: "慢性支气管炎".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "川贝母".to_string(),
                        unit_price: 1.2,
                        grams: 6.0,
                        total_price: 7.2,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "桔梗".to_string(),
                        unit_price: 0.35,
                        grams: 10.0,
                        total_price: 3.5,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "甘草".to_string(),
                        unit_price: 0.2,
                        grams: 6.0,
                        total_price: 1.2,
                    },
                ],
                doses: 10,
                single_dose_amount: 11.9,
                total_amount: 119.0,
                doctor_id: "1".to_string(),
                doctor_name: "李明".to_string(),
                time_of_visit: "2025-03-12 16:00:00".to_string(),
                note: Some("温水冲服，戒烟".to_string()),
                create_time: Some("2025-03-12 16:00:00".to_string()),
                update_time: Some("2025-03-12 16:00:00".to_string()),
            },
            // 2025年5月数据
            ChineseMedicinePrescriptionRecord {
                id: Some(9),
                customer_id: 9,
                customer_name: "钱小花".to_string(),
                gender: "Female".to_string(),
                age: 34,
                phone: "13800138017".to_string(),
                diagnosis: "过敏性皮炎".to_string(),
                prescription: vec![
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "连翘".to_string(),
                        unit_price: 0.25,
                        grams: 12.0,
                        total_price: 3.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "金银花".to_string(),
                        unit_price: 0.4,
                        grams: 15.0,
                        total_price: 6.0,
                    },
                    ChineseMedicinePrescriptionItem {
                        medicine_name: "蒲公英".to_string(),
                        unit_price: 0.18,
                        grams: 20.0,
                        total_price: 3.6,
                    },
                ],
                doses: 7,
                single_dose_amount: 12.6,
                total_amount: 88.2,
                doctor_id: "2".to_string(),
                doctor_name: "王医生".to_string(),
                time_of_visit: "2025-05-20 11:00:00".to_string(),
                note: Some("清淡饮食，避免辛辣".to_string()),
                create_time: Some("2025-05-20 11:00:00".to_string()),
                update_time: Some("2025-05-20 11:00:00".to_string()),
            },
        ];

        for record in sample_records {
            if let Some(id) = record.id {
                records.insert(id, record);
                *next_id = (*next_id).max(id + 1);
            }
        }
        println!("已初始化 {} 条示例中药处方数据", records.len());
    }
}
