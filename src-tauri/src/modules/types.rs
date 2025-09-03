use serde::{Deserialize, Serialize};

// 统一的医疗记录结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MedicalRecord {
    pub id: Option<i64>,
    pub data_type: String,
    pub customer_name: String,
    pub gender: String,
    pub age: i64,
    pub telephone: Option<String>,
    pub time_of_visit: String,
    pub trade_type: String,
    pub diagnosis: String,
    pub note: Option<String>,
    pub total_amount: i64, // 总金额（以分为单位）
    pub doctor_id: Option<i64>,
    pub doctor_name: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,

    // 门诊病历特有字段
    pub chief_complaint: Option<String>,
    pub illness_history: Option<String>,
    pub physical_examination: Option<String>,
    pub past_history: Option<String>,
    pub laboratory_examination: Option<String>,
    pub treatment_method: Option<String>,

    // 中药处方特有字段
    pub doses: Option<i64>,
    pub single_dose_amount: Option<i64>, // 单付金额（以分为单位）

    // 关联数据
    // 散客登记/门诊病历
    pub treatment_items: Option<Vec<TreatmentItemRecord>>,
    // 中药处方
    pub prescription_items: Option<Vec<PrescriptionItemRecord>>,
}

// 治疗项目记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreatmentItemRecord {
    pub id: Option<i64>,
    pub medical_record_id: Option<i64>,
    pub treatment_item: String,
    pub count: i64,
    pub unit_price: i64,  // 单价（以分为单位）
    pub total_price: i64, // 总价（以分为单位）
    pub created_at: Option<String>,
}

// 中药处方项目记录
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrescriptionItemRecord {
    pub id: Option<i64>,
    pub medical_record_id: Option<i64>,
    pub medicine_name: String,
    pub grams: i64,
    pub unit_price: i64,  // 单价（以分为单位）
    pub total_price: i64, // 总价（以分为单位）
    pub created_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i64,
    pub msg: String,
    pub data: T,
}
