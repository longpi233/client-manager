use serde::{Deserialize, Serialize};

// 统一的康养项目结构体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TreatmentItem {
    pub id: Option<i32>,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub price: Option<f64>,
    pub duration: Option<i32>, // 治疗时长（分钟）
    pub doctor_id: Option<i32>,
}

// 兼容旧的结构体格式（散客登记页面使用）
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LegacyTreatmentItem {
    pub treatment_item: String,
    pub count: i32,
    pub unit_price: Option<f64>,
    pub total_price: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub msg: String,
    pub data: T,
}
