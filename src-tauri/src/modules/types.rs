use serde::{Deserialize, Serialize};

// 记录类型枚举
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum RecordType {
    Customer,      // 散客登记
    Medical,       // 门诊病历  
    Prescription,  // 中药处方
}

impl RecordType {
    pub fn to_string(&self) -> String {
        match self {
            RecordType::Customer => "customer".to_string(),
            RecordType::Medical => "medical".to_string(),
            RecordType::Prescription => "prescription".to_string(),
        }
    }

    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "customer" => Some(RecordType::Customer),
            "medical" => Some(RecordType::Medical),
            "prescription" => Some(RecordType::Prescription),
            _ => None,
        }
    }
}

// 统一的医疗记录结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalRecord {
    pub id: Option<i32>,
    pub record_type: String,
    pub customer_name: String,
    pub gender: String,
    pub age: i32,
    pub telephone: Option<String>,
    pub time_of_visit: String,
    pub trade_type: String,
    pub diagnosis: String,
    pub note: Option<String>,
    
    // 门诊病历特有字段
    pub chief_complaint: Option<String>,
    pub illness_history: Option<String>,
    pub physical_examination: Option<String>,
    pub past_history: Option<String>,
    pub laboratory_examination: Option<String>,
    pub treatment_method: Option<String>,
    
    // 中药处方特有字段
    pub doses: Option<i32>,
    pub single_dose_amount: Option<i64>,  // 单付金额（以分为单位）
    
    // 通用字段
    pub total_amount: i64,  // 总金额（以分为单位）
    pub doctor_id: Option<i32>,
    pub doctor_name: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    
    // 关联数据
    pub treatment_items: Option<Vec<TreatmentItemRecord>>,
    pub prescription_items: Option<Vec<PrescriptionItemRecord>>,
}

// 治疗项目记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreatmentItemRecord {
    pub id: Option<i32>,
    pub medical_record_id: i32,
    pub treatment_item_name: String,
    pub treatment_count: i32,
    pub unit_price: i64,  // 单价（以分为单位）
    pub total_price: i64, // 总价（以分为单位）
    pub created_at: String,
}

// 中药处方项目记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrescriptionItemRecord {
    pub id: Option<i32>,
    pub medical_record_id: i32,
    pub medicine_name: String,
    pub grams: f64,
    pub unit_price: i64,  // 单价（以分为单位）
    pub total_price: i64, // 总价（以分为单位）
    pub created_at: String,
}

// 统一的康养项目结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreatmentItem {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: Option<i64>,  // 价格（以分为单位）
    pub duration: Option<i32>, // 治疗时长（分钟）
    pub doctor_id: Option<i32>,
}

// 兼容旧的结构体格式（表单提交时使用）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegacyTreatmentItem {
    pub treatment_item: String,
    pub count: i32,
    pub unit_price: Option<i64>,  // 单价（以分为单位）
    pub total_price: Option<i64>, // 总价（以分为单位）
}

// 中药处方表单项目
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrescriptionFormItem {
    pub medicine_name: String,
    pub grams: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}
