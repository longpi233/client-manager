use serde::{Deserialize, Serialize};
use tauri::command;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use crate::modules::types::{LegacyTreatmentItem, ApiResponse};
use crate::modules::utils::contains_pinyin;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalRecord {
    pub id: Option<i32>,
    pub customer_name: String,
    pub gender: String,
    pub age: i32,
    pub telephone: String,
    pub time_of_visit: String,
    pub trade_type: String,
    pub chief_complaint: String,
    pub illness_history: String,
    pub physical_examination: String,
    pub past_history: String,
    pub laboratory_examination: String,
    pub diagnosis: String,
    pub treatment_method: String,
    pub note: String,
    pub treatment: Vec<LegacyTreatmentItem>,
    pub total_amount: f64,
    pub doctor_id: i32,
    pub doctor_name: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicalRecordCreateParams {
    pub customer_name: String,
    pub gender: String,
    pub age: i32,
    pub telephone: String,
    pub time_of_visit: String,
    pub trade_type: String,
    pub chief_complaint: String,
    pub illness_history: String,
    pub physical_examination: String,
    pub past_history: String,
    pub laboratory_examination: String,
    pub diagnosis: String,
    pub treatment_method: String,
    pub note: String,
    pub treatment: Vec<LegacyTreatmentItem>,
    pub total_amount: f64,
    pub doctor_id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicalRecordSearchParams {
    pub current: i32,
    pub size: i32,
    pub customer_name: Option<String>,
    pub telephone: Option<String>,
    pub diagnosis: Option<String>,
    pub doctor_id: Option<i32>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub gender: Option<String>,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MedicalRecordListResponse {
    pub records: Vec<MedicalRecord>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

// 医疗记录存储
static MEDICAL_RECORDS: LazyLock<Mutex<HashMap<i32, MedicalRecord>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static MEDICAL_RECORD_NEXT_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

#[command]
pub async fn get_medical_record_list(
    params: MedicalRecordSearchParams,
) -> Result<ApiResponse<MedicalRecordListResponse>, String> {
    println!("接收到医疗记录查询请求: {:?}", params);
    let records = MEDICAL_RECORDS.lock().unwrap();

    // 模拟搜索过滤
    let mut filtered_records: Vec<MedicalRecord> = records
        .values()
        .filter(|record| {
            if let Some(ref name) = params.customer_name {
                if !name.is_empty() && !contains_pinyin(&record.customer_name, name) {
                    return false;
                }
            }
            if let Some(ref telephone) = params.telephone {
                if !telephone.is_empty() && !record.telephone.contains(telephone) {
                    return false;
                }
            }
            if let Some(ref diagnosis) = params.diagnosis {
                if !diagnosis.is_empty() && !record.diagnosis.contains(diagnosis) {
                    return false;
                }
            }
            if let Some(ref gender) = params.gender {
                if !gender.is_empty() && record.gender != *gender {
                    return false;
                }
            }
            if let Some(min_age) = params.min_age {
                if record.age < min_age {
                    return false;
                }
            }
            if let Some(max_age) = params.max_age {
                if record.age > max_age {
                    return false;
                }
            }
            if let Some(doctor_id) = params.doctor_id {
                if record.doctor_id != doctor_id {
                    return false;
                }
            }
            
            // 日期过滤
            if let Some(ref start_date) = params.start_date {
                if !start_date.is_empty() {
                    let record_date = record.time_of_visit.split('T').next().unwrap_or(&record.time_of_visit);
                    if record_date < start_date.as_str() {
                        return false;
                    }
                }
            }
            if let Some(ref end_date) = params.end_date {
                if !end_date.is_empty() {
                    let record_date = record.time_of_visit.split('T').next().unwrap_or(&record.time_of_visit);
                    if record_date > end_date.as_str() {
                        return false;
                    }
                }
            }
            
            true
        })
        .cloned()
        .collect();

    // 按创建时间排序（最新的在前）
    filtered_records.sort_by(|a, b| {
        b.create_time
            .as_ref()
            .unwrap_or(&String::new())
            .cmp(a.create_time.as_ref().unwrap_or(&String::new()))
    });

    // 分页
    let total = filtered_records.len() as i32;
    let start = ((params.current - 1) * params.size) as usize;
    let end = (start + params.size as usize).min(filtered_records.len());
    let page_records = filtered_records[start..end].to_vec();

    Ok(ApiResponse {
        code: 200,
        msg: "查询成功".to_string(),
        data: MedicalRecordListResponse {
            records: page_records,
            current: params.current,
            size: params.size,
            total,
        },
    })
}

#[command]
pub async fn create_medical_record(
    data: MedicalRecordCreateParams,
) -> Result<ApiResponse<MedicalRecord>, String> {
    let mut records = MEDICAL_RECORDS.lock().unwrap();
    let mut next_id = MEDICAL_RECORD_NEXT_ID.lock().unwrap();

    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let new_record = MedicalRecord {
        id: Some(*next_id),
        customer_name: data.customer_name,
        gender: data.gender,
        age: data.age,
        telephone: data.telephone,
        time_of_visit: data.time_of_visit,
        trade_type: data.trade_type,
        chief_complaint: data.chief_complaint,
        illness_history: data.illness_history,
        physical_examination: data.physical_examination,
        past_history: data.past_history,
        laboratory_examination: data.laboratory_examination,
        diagnosis: data.diagnosis,
        treatment_method: data.treatment_method,
        note: data.note,
        treatment: data.treatment,
        total_amount: data.total_amount,
        doctor_id: data.doctor_id,
        doctor_name: None, // 可以后续通过doctor_id查询医师姓名
        create_time: Some(now.clone()),
        update_time: Some(now),
    };

    records.insert(*next_id, new_record.clone());
    *next_id += 1;

    Ok(ApiResponse {
        code: 200,
        msg: "门诊病历创建成功".to_string(),
        data: new_record,
    })
}

#[command]
pub async fn get_medical_record(id: i32) -> Result<ApiResponse<MedicalRecord>, String> {
    let records = MEDICAL_RECORDS.lock().unwrap();

    match records.get(&id) {
        Some(record) => Ok(ApiResponse {
            code: 200,
            msg: "查询成功".to_string(),
            data: record.clone(),
        }),
        None => Err("门诊病历不存在".to_string()),
    }
}

#[command]
pub async fn update_medical_record(
    id: i32,
    data: MedicalRecordCreateParams,
) -> Result<ApiResponse<MedicalRecord>, String> {
    let mut records = MEDICAL_RECORDS.lock().unwrap();

    match records.get_mut(&id) {
        Some(record) => {
            record.customer_name = data.customer_name;
            record.gender = data.gender;
            record.age = data.age;
            record.telephone = data.telephone;
            record.time_of_visit = data.time_of_visit;
            record.trade_type = data.trade_type;
            record.chief_complaint = data.chief_complaint;
            record.illness_history = data.illness_history;
            record.physical_examination = data.physical_examination;
            record.past_history = data.past_history;
            record.laboratory_examination = data.laboratory_examination;
            record.diagnosis = data.diagnosis;
            record.treatment_method = data.treatment_method;
            record.note = data.note;
            record.treatment = data.treatment;
            record.total_amount = data.total_amount;
            record.doctor_id = data.doctor_id;
            record.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

            Ok(ApiResponse {
                code: 200,
                msg: "门诊病历更新成功".to_string(),
                data: record.clone(),
            })
        }
        None => Err("门诊病历不存在".to_string()),
    }
}

#[command]
pub async fn delete_medical_record(id: i32) -> Result<ApiResponse<bool>, String> {
    let mut records = MEDICAL_RECORDS.lock().unwrap();

    match records.remove(&id) {
        Some(_) => Ok(ApiResponse {
            code: 200,
            msg: "门诊病历删除成功".to_string(),
            data: true,
        }),
        None => Err("门诊病历不存在".to_string()),
    }
}

#[command]
pub async fn generate_medical_record_pdf(id: i32) -> Result<ApiResponse<String>, String> {
    // 模拟PDF生成功能
    let records = MEDICAL_RECORDS.lock().unwrap();
    
    match records.get(&id) {
        Some(_record) => {
            // 这里应该实现真正的PDF生成逻辑
            Ok(ApiResponse {
                code: 200,
                msg: "门诊病历PDF生成成功".to_string(),
                data: format!("medical_record_{}.pdf", id),
            })
        }
        None => Err("门诊病历不存在".to_string()),
    }
}

#[command]
pub async fn generate_fee_list_pdf(id: i32) -> Result<ApiResponse<String>, String> {
    // 模拟PDF生成功能
    let records = MEDICAL_RECORDS.lock().unwrap();
    
    match records.get(&id) {
        Some(_record) => {
            // 这里应该实现真正的费用清单PDF生成逻辑
            Ok(ApiResponse {
                code: 200,
                msg: "费用清单PDF生成成功".to_string(),
                data: format!("fee_list_{}.pdf", id),
            })
        }
        None => Err("门诊病历不存在".to_string()),
    }
}

// 初始化示例数据
pub fn init_sample_data() {
    let mut records = MEDICAL_RECORDS.lock().unwrap();
    let mut next_id = MEDICAL_RECORD_NEXT_ID.lock().unwrap();

    if records.is_empty() {
        let sample_records = vec![
            // 2025年8月数据
            MedicalRecord {
                id: Some(1),
                customer_name: "张三".to_string(),
                gender: "1".to_string(), // 1-男
                age: 35,
                telephone: "13800138001".to_string(),
                time_of_visit: "2025-08-15 10:30:00".to_string(),
                trade_type: "1".to_string(), // 1-现金结算
                chief_complaint: "颈椎疼痛3天".to_string(),
                illness_history: "患者3天前开始出现颈椎疼痛症状".to_string(),
                physical_examination: "神志清楚，血压正常，颈椎活动受限".to_string(),
                past_history: "既往无特殊病史".to_string(),
                laboratory_examination: "血常规正常".to_string(),
                diagnosis: "颈椎病".to_string(),
                treatment_method: "针灸治疗".to_string(),
                note: "建议注意休息，避免长时间低头".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "针灸治疗".to_string(),
                        count: 1,
                        unit_price: Some(120.0),
                        total_price: Some(120.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "推拿按摩".to_string(),
                        count: 1,
                        unit_price: Some(80.0),
                        total_price: Some(80.0),
                    },
                ],
                total_amount: 200.0,
                doctor_id: 1,
                doctor_name: Some("李医生".to_string()),
                create_time: Some("2025-08-15 10:30:00".to_string()),
                update_time: Some("2025-08-15 10:30:00".to_string()),
            },
            MedicalRecord {
                id: Some(2),
                customer_name: "王丽".to_string(),
                gender: "2".to_string(), // 2-女
                age: 28,
                telephone: "13800138003".to_string(),
                time_of_visit: "2025-08-22 14:20:00".to_string(),
                trade_type: "2".to_string(), // 2-医保结算
                chief_complaint: "失眠多梦1周".to_string(),
                illness_history: "患者1周前开始出现睡眠困难，多梦".to_string(),
                physical_examination: "神志清楚，精神状态一般".to_string(),
                past_history: "既往有焦虑症病史".to_string(),
                laboratory_examination: "血常规正常，甲状腺功能正常".to_string(),
                diagnosis: "失眠症".to_string(),
                treatment_method: "中药调理".to_string(),
                note: "建议规律作息，避免咖啡因".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "中药调理".to_string(),
                        count: 7,
                        unit_price: Some(85.0),
                        total_price: Some(595.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "艾灸疗法".to_string(),
                        count: 3,
                        unit_price: Some(60.0),
                        total_price: Some(180.0),
                    },
                ],
                total_amount: 775.0,
                doctor_id: 2,
                doctor_name: Some("王医生".to_string()),
                create_time: Some("2025-08-22 14:20:00".to_string()),
                update_time: Some("2025-08-22 14:20:00".to_string()),
            },
            // 2025年6月数据
            MedicalRecord {
                id: Some(3),
                customer_name: "刘建国".to_string(),
                gender: "1".to_string(), // 1-男
                age: 45,
                telephone: "13800138005".to_string(),
                time_of_visit: "2025-06-10 09:15:00".to_string(),
                trade_type: "1".to_string(), // 1-现金结算
                chief_complaint: "腰痛伴下肢放射痛2周".to_string(),
                illness_history: "患者2周前无明显诱因出现腰痛".to_string(),
                physical_examination: "腰椎生理弯曲存在，压痛阳性".to_string(),
                past_history: "既往有腰椎间盘突出病史".to_string(),
                laboratory_examination: "腰椎CT示L4-5椎间盘突出".to_string(),
                diagnosis: "腰椎间盘突出症".to_string(),
                treatment_method: "综合理疗".to_string(),
                note: "建议卧床休息，定期复查".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "理疗康复".to_string(),
                        count: 5,
                        unit_price: Some(110.0),
                        total_price: Some(550.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "电针疗法".to_string(),
                        count: 2,
                        unit_price: Some(90.0),
                        total_price: Some(180.0),
                    },
                ],
                total_amount: 730.0,
                doctor_id: 1,
                doctor_name: Some("李医生".to_string()),
                create_time: Some("2025-06-10 09:15:00".to_string()),
                update_time: Some("2025-06-10 09:15:00".to_string()),
            },
            // 2025年7月数据
            MedicalRecord {
                id: Some(4),
                customer_name: "陈小明".to_string(),
                gender: "1".to_string(),
                age: 32,
                telephone: "13800138007".to_string(),
                time_of_visit: "2025-07-18 16:45:00".to_string(),
                trade_type: "2".to_string(), // 医保结算
                chief_complaint: "胃痛反酸2天".to_string(),
                illness_history: "患者2天前进食辛辣食物后出现上腹痛".to_string(),
                physical_examination: "腹部软，上腹部压痛".to_string(),
                past_history: "既往有慢性胃炎病史".to_string(),
                laboratory_examination: "胃镜检查示浅表性胃炎".to_string(),
                diagnosis: "慢性浅表性胃炎".to_string(),
                treatment_method: "中药治疗".to_string(),
                note: "饮食清淡，忌辛辣刺激".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "胃痛中药方".to_string(),
                        count: 14,
                        unit_price: Some(45.0),
                        total_price: Some(630.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "腹部按摩".to_string(),
                        count: 2,
                        unit_price: Some(60.0),
                        total_price: Some(120.0),
                    },
                ],
                total_amount: 750.0,
                doctor_id: 3,
                doctor_name: Some("张医生".to_string()),
                create_time: Some("2025-07-18 16:45:00".to_string()),
                update_time: Some("2025-07-18 16:45:00".to_string()),
            },
            // 2025年9月数据
            MedicalRecord {
                id: Some(5),
                customer_name: "李雅雯".to_string(),
                gender: "2".to_string(),
                age: 26,
                telephone: "13800138009".to_string(),
                time_of_visit: "2025-09-05 11:20:00".to_string(),
                trade_type: "1".to_string(), // 现金结算
                chief_complaint: "月经不调3个月".to_string(),
                illness_history: "患者3个月来月经周期不规律".to_string(),
                physical_examination: "妇科检查无异常".to_string(),
                past_history: "既往月经规律".to_string(),
                laboratory_examination: "激素六项检查基本正常".to_string(),
                diagnosis: "月经不调".to_string(),
                treatment_method: "中药调理".to_string(),
                note: "保持心情舒畅，规律作息".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "调经中药".to_string(),
                        count: 21,
                        unit_price: Some(55.0),
                        total_price: Some(1155.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "经络推拿".to_string(),
                        count: 4,
                        unit_price: Some(80.0),
                        total_price: Some(320.0),
                    },
                ],
                total_amount: 1475.0,
                doctor_id: 2,
                doctor_name: Some("王医生".to_string()),
                create_time: Some("2025-09-05 11:20:00".to_string()),
                update_time: Some("2025-09-05 11:20:00".to_string()),
            },
            // 2024年12月数据
            MedicalRecord {
                id: Some(6),
                customer_name: "赵大伯".to_string(),
                gender: "1".to_string(),
                age: 68,
                telephone: "13800138011".to_string(),
                time_of_visit: "2024-12-25 14:30:00".to_string(),
                trade_type: "2".to_string(), // 医保结算
                chief_complaint: "关节疼痛1个月".to_string(),
                illness_history: "患者1个月来多关节疼痛，以膝关节为重".to_string(),
                physical_examination: "双膝关节轻度肿胀，活动受限".to_string(),
                past_history: "既往有高血压、糖尿病病史".to_string(),
                laboratory_examination: "X线示骨关节炎改变".to_string(),
                diagnosis: "骨关节炎".to_string(),
                treatment_method: "中药配合物理治疗".to_string(),
                note: "避免过度活动，注意保暖".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "活血化瘀中药".to_string(),
                        count: 14,
                        unit_price: Some(65.0),
                        total_price: Some(910.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "热敷理疗".to_string(),
                        count: 8,
                        unit_price: Some(40.0),
                        total_price: Some(320.0),
                    },
                ],
                total_amount: 1230.0,
                doctor_id: 1,
                doctor_name: Some("李医生".to_string()),
                create_time: Some("2024-12-25 14:30:00".to_string()),
                update_time: Some("2024-12-25 14:30:00".to_string()),
            },
            // 2025年1月数据
            MedicalRecord {
                id: Some(7),
                customer_name: "孙美华".to_string(),
                gender: "2".to_string(),
                age: 42,
                telephone: "13800138013".to_string(),
                time_of_visit: "2025-01-08 09:45:00".to_string(),
                trade_type: "1".to_string(), // 现金结算
                chief_complaint: "头晕乏力1周".to_string(),
                illness_history: "患者1周来出现头晕、乏力症状".to_string(),
                physical_examination: "面色苍白，心率稍快".to_string(),
                past_history: "既往有贫血病史".to_string(),
                laboratory_examination: "血常规示轻度贫血".to_string(),
                diagnosis: "缺铁性贫血".to_string(),
                treatment_method: "补血中药治疗".to_string(),
                note: "加强营养，多食含铁食物".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "补血中药".to_string(),
                        count: 28,
                        unit_price: Some(38.0),
                        total_price: Some(1064.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "营养指导".to_string(),
                        count: 1,
                        unit_price: Some(50.0),
                        total_price: Some(50.0),
                    },
                ],
                total_amount: 1114.0,
                doctor_id: 3,
                doctor_name: Some("张医生".to_string()),
                create_time: Some("2025-01-08 09:45:00".to_string()),
                update_time: Some("2025-01-08 09:45:00".to_string()),
            },
            // 2025年3月数据
            MedicalRecord {
                id: Some(8),
                customer_name: "周志强".to_string(),
                gender: "1".to_string(),
                age: 55,
                telephone: "13800138015".to_string(),
                time_of_visit: "2025-03-12 15:30:00".to_string(),
                trade_type: "2".to_string(), // 医保结算
                chief_complaint: "咳嗽咳痰2周".to_string(),
                illness_history: "患者2周来干咳，少量白痰".to_string(),
                physical_examination: "双肺呼吸音粗糙，可闻及干性啰音".to_string(),
                past_history: "既往有慢性支气管炎病史".to_string(),
                laboratory_examination: "胸片示肺纹理增粗".to_string(),
                diagnosis: "慢性支气管炎急性发作".to_string(),
                treatment_method: "止咳化痰中药".to_string(),
                note: "戒烟，避免粉尘刺激".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "止咳化痰中药".to_string(),
                        count: 10,
                        unit_price: Some(42.0),
                        total_price: Some(420.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "雾化吸入".to_string(),
                        count: 5,
                        unit_price: Some(35.0),
                        total_price: Some(175.0),
                    },
                ],
                total_amount: 595.0,
                doctor_id: 1,
                doctor_name: Some("李医生".to_string()),
                create_time: Some("2025-03-12 15:30:00".to_string()),
                update_time: Some("2025-03-12 15:30:00".to_string()),
            },
            // 2025年5月数据
            MedicalRecord {
                id: Some(9),
                customer_name: "钱小花".to_string(),
                gender: "2".to_string(),
                age: 34,
                telephone: "13800138017".to_string(),
                time_of_visit: "2025-05-20 10:15:00".to_string(),
                trade_type: "1".to_string(), // 现金结算
                chief_complaint: "皮肤过敏瘙痒3天".to_string(),
                illness_history: "患者3天前接触花粉后出现皮肤瘙痒".to_string(),
                physical_examination: "面颊部可见红色皮疹，有抓痕".to_string(),
                past_history: "既往有过敏性皮炎病史".to_string(),
                laboratory_examination: "过敏原检测示花粉过敏".to_string(),
                diagnosis: "过敏性皮炎".to_string(),
                treatment_method: "清热解毒中药外用".to_string(),
                note: "避免接触过敏原，保持皮肤清洁".to_string(),
                treatment: vec![
                    LegacyTreatmentItem {
                        treatment_item: "清热解毒中药".to_string(),
                        count: 7,
                        unit_price: Some(32.0),
                        total_price: Some(224.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "外用药膏".to_string(),
                        count: 2,
                        unit_price: Some(25.0),
                        total_price: Some(50.0),
                    },
                ],
                total_amount: 274.0,
                doctor_id: 2,
                doctor_name: Some("王医生".to_string()),
                create_time: Some("2025-05-20 10:15:00".to_string()),
                update_time: Some("2025-05-20 10:15:00".to_string()),
            },
        ];

        for record in sample_records {
            if let Some(id) = record.id {
                records.insert(id, record);
                if id >= *next_id {
                    *next_id = id + 1;
                }
            }
        }

        println!("已初始化 {} 条示例医疗记录数据", records.len());
    }
}
