use serde::{Deserialize, Serialize};
use tauri::command;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use crate::modules::types::{ApiResponse, LegacyTreatmentItem};
use crate::modules::utils::contains_pinyin;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Customer {
    pub id: Option<i32>,
    pub name: String,
    pub phone: String,
    pub age: i32,
    pub gender: String,
    pub time_of_visit: String,
    pub trade_type: String,
    pub diagnosis: String,
    pub note: Option<String>,
    pub status: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerSearchParams {
    pub current: i32,
    pub size: i32,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub diagnosis: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub status: Option<String>,
    pub gender: Option<String>,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerListResponse {
    pub records: Vec<Customer>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerRegistration {
    pub id: Option<i32>,
    pub customer_id: i32,
    pub treatment_items: Vec<LegacyTreatmentItem>,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub payment_status: String,
    pub registration_date: String,
    pub notes: Option<String>,
    pub status: String,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationSearchParams {
    pub current: i32,
    pub size: i32,
    pub customer_id: Option<i32>,
    pub registration_date: Option<String>,
    pub payment_status: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationListResponse {
    pub records: Vec<CustomerRegistration>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomerStatistics {
    pub total_customers: i32,
    pub active_customers: i32,
    pub total_registrations: i32,
    pub total_revenue: f64,
}

// 模拟数据存储（实际项目中应该使用数据库）
static CUSTOMERS: LazyLock<Mutex<HashMap<i32, Customer>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static REGISTRATIONS: LazyLock<Mutex<HashMap<i32, CustomerRegistration>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static NEXT_CUSTOMER_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));
static NEXT_REGISTRATION_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

#[command]
pub async fn get_customer_list(
    params: CustomerSearchParams,
) -> Result<ApiResponse<CustomerListResponse>, String> {
    println!("接收到散客列表查询请求: {:?}", params);
    let customers = CUSTOMERS.lock().unwrap();
    println!("当前客户总数: {}", customers.len());

    // 模拟搜索过滤，支持拼音查询
    let mut filtered_customers: Vec<Customer> = customers
        .values()
        .filter(|customer| {
            if let Some(ref name) = params.name {
                if !name.is_empty() && !contains_pinyin(&customer.name, name) {
                    return false;
                }
            }
            if let Some(ref phone) = params.phone {
                if !phone.is_empty() && !customer.phone.contains(phone) {
                    return false;
                }
            }
            if let Some(ref diagnosis) = params.diagnosis {
                if !diagnosis.is_empty() && !customer.diagnosis.contains(diagnosis) {
                    return false;
                }
            }
            if let Some(ref gender) = params.gender {
                if !gender.is_empty() && customer.gender != *gender {
                    return false;
                }
            }
            if let Some(min_age) = params.min_age {
                if customer.age < min_age {
                    return false;
                }
            }
            if let Some(max_age) = params.max_age {
                if customer.age > max_age {
                    return false;
                }
            }
            if let Some(ref status) = params.status {
                if !status.is_empty() && customer.status != *status {
                    return false;
                }
            }
            
            // 日期过滤
            if let Some(ref start_date) = params.start_date {
                if !start_date.is_empty() {
                    let customer_date = customer.time_of_visit.split('T').next().unwrap_or(&customer.time_of_visit);
                    if customer_date < start_date.as_str() {
                        return false;
                    }
                }
            }
            if let Some(ref end_date) = params.end_date {
                if !end_date.is_empty() {
                    let customer_date = customer.time_of_visit.split('T').next().unwrap_or(&customer.time_of_visit);
                    if customer_date > end_date.as_str() {
                        return false;
                    }
                }
            }
            
            true
        })
        .cloned()
        .collect();

    // 排序
    filtered_customers.sort_by(|a, b| b.id.cmp(&a.id));

    let total = filtered_customers.len() as i32;
    let start = ((params.current - 1) * params.size) as usize;
    let end = std::cmp::min(start + params.size as usize, filtered_customers.len());

    let records = if start < filtered_customers.len() {
        filtered_customers[start..end].to_vec()
    } else {
        Vec::new()
    };
    
    println!("过滤后的客户数量: {}, 返回记录数: {}", total, records.len());
    for customer in &records {
        println!("返回客户: ID={:?}, 姓名={}, 电话={}", customer.id, customer.name, customer.phone);
    }

    let response = CustomerListResponse {
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
pub async fn create_customer(customer: Customer) -> Result<ApiResponse<Customer>, String> {
    let mut customers = CUSTOMERS.lock().unwrap();
    let mut next_id = NEXT_CUSTOMER_ID.lock().unwrap();

    let mut new_customer = customer;
    new_customer.id = Some(*next_id);
    new_customer.create_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    new_customer.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

    customers.insert(*next_id, new_customer.clone());
    *next_id += 1;

    Ok(ApiResponse {
        code: 200,
        msg: "创建成功".to_string(),
        data: new_customer,
    })
}

#[command]
pub async fn update_customer(customer: Customer) -> Result<ApiResponse<Customer>, String> {
    let mut customers = CUSTOMERS.lock().unwrap();

    if let Some(id) = customer.id {
        if let Some(existing_customer) = customers.get_mut(&id) {
            existing_customer.name = customer.name;
            existing_customer.phone = customer.phone;
            existing_customer.age = customer.age;
            existing_customer.gender = customer.gender;
            existing_customer.time_of_visit = customer.time_of_visit;
            existing_customer.trade_type = customer.trade_type;
            existing_customer.diagnosis = customer.diagnosis;
            existing_customer.note = customer.note;
            existing_customer.status = customer.status;
            existing_customer.update_time =
                Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

            Ok(ApiResponse {
                code: 200,
                msg: "更新成功".to_string(),
                data: existing_customer.clone(),
            })
        } else {
            Err("客户不存在".to_string())
        }
    } else {
        Err("无效的ID".to_string())
    }
}

#[command]
pub async fn delete_customer(id: i32) -> Result<ApiResponse<bool>, String> {
    let mut customers = CUSTOMERS.lock().unwrap();

    if customers.remove(&id).is_some() {
        Ok(ApiResponse {
            code: 200,
            msg: "删除成功".to_string(),
            data: true,
        })
    } else {
        Err("客户不存在".to_string())
    }
}

#[command]
pub async fn create_customer_registration(
    registration: CustomerRegistration,
) -> Result<ApiResponse<CustomerRegistration>, String> {
    let mut registrations = REGISTRATIONS.lock().unwrap();
    let mut next_id = NEXT_REGISTRATION_ID.lock().unwrap();

    let mut new_registration = registration;
    new_registration.id = Some(*next_id);
    new_registration.create_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    new_registration.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

    registrations.insert(*next_id, new_registration.clone());
    *next_id += 1;

    Ok(ApiResponse {
        code: 200,
        msg: "登记成功".to_string(),
        data: new_registration,
    })
}

#[command]
pub async fn get_customer_registrations(
    params: RegistrationSearchParams,
) -> Result<ApiResponse<RegistrationListResponse>, String> {
    let registrations = REGISTRATIONS.lock().unwrap();

    // 模拟搜索过滤
    let mut filtered_registrations: Vec<CustomerRegistration> = registrations
        .values()
        .filter(|registration| {
            if let Some(customer_id) = params.customer_id {
                if registration.customer_id != customer_id {
                    return false;
                }
            }
            if let Some(ref date) = params.registration_date {
                if !date.is_empty() && !registration.registration_date.contains(date) {
                    return false;
                }
            }
            if let Some(ref payment_status) = params.payment_status {
                if !payment_status.is_empty() && registration.payment_status != *payment_status {
                    return false;
                }
            }
            if let Some(ref status) = params.status {
                if !status.is_empty() && registration.status != *status {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // 排序
    filtered_registrations.sort_by(|a, b| b.id.cmp(&a.id));

    let total = filtered_registrations.len() as i32;
    let start = ((params.current - 1) * params.size) as usize;
    let end = std::cmp::min(start + params.size as usize, filtered_registrations.len());

    let records = if start < filtered_registrations.len() {
        filtered_registrations[start..end].to_vec()
    } else {
        Vec::new()
    };

    let response = RegistrationListResponse {
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
pub async fn get_customer_statistics() -> Result<ApiResponse<CustomerStatistics>, String> {
    let customers = CUSTOMERS.lock().unwrap();
    let registrations = REGISTRATIONS.lock().unwrap();

    let total_customers = customers.len() as i32;
    let active_customers = customers
        .values()
        .filter(|customer| customer.status == "1")
        .count() as i32;
    let total_registrations = registrations.len() as i32;
    let total_revenue: f64 = registrations
        .values()
        .map(|registration| registration.paid_amount)
        .sum();

    let statistics = CustomerStatistics {
        total_customers,
        active_customers,
        total_registrations,
        total_revenue,
    };

    Ok(ApiResponse {
        code: 200,
        msg: "获取统计信息成功".to_string(),
        data: statistics,
    })
}

// 初始化示例数据
pub fn init_sample_data() {
    let mut customers = CUSTOMERS.lock().unwrap();
    let mut registrations = REGISTRATIONS.lock().unwrap();
    let mut next_customer_id = NEXT_CUSTOMER_ID.lock().unwrap();
    let mut next_registration_id = NEXT_REGISTRATION_ID.lock().unwrap();

    if customers.is_empty() {
        let sample_customers = vec![
            // 2025年8月数据
            Customer {
                id: Some(1),
                name: "张三".to_string(),
                phone: "13800138001".to_string(),
                age: 35,
                gender: "1".to_string(),
                time_of_visit: "2025-08-15 10:30:00".to_string(),
                trade_type: "1".to_string(),
                diagnosis: "颈椎病".to_string(),
                note: Some("散客登记备注".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-08-15 10:30:00".to_string()),
                update_time: Some("2025-08-15 10:30:00".to_string()),
            },
            Customer {
                id: Some(2),
                name: "王丽".to_string(),
                phone: "13800138003".to_string(),
                age: 28,
                gender: "2".to_string(),
                time_of_visit: "2025-08-22 14:20:00".to_string(),
                trade_type: "2".to_string(),
                diagnosis: "失眠多梦".to_string(),
                note: None,
                status: "1".to_string(),
                create_time: Some("2025-08-22 14:20:00".to_string()),
                update_time: Some("2025-08-22 14:20:00".to_string()),
            },
            // 2025年6月数据
            Customer {
                id: Some(3),
                name: "刘建国".to_string(),
                phone: "13800138005".to_string(),
                age: 45,
                gender: "1".to_string(),
                time_of_visit: "2025-06-10 09:15:00".to_string(),
                trade_type: "3".to_string(),
                diagnosis: "腰椎间盘突出".to_string(),
                note: Some("需要定期复查".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-06-10 09:15:00".to_string()),
                update_time: Some("2025-06-10 09:15:00".to_string()),
            },
            // 2025年7月数据
            Customer {
                id: Some(4),
                name: "陈小明".to_string(),
                phone: "13800138007".to_string(),
                age: 32,
                gender: "1".to_string(),
                time_of_visit: "2025-07-18 16:45:00".to_string(),
                trade_type: "2".to_string(),
                diagnosis: "慢性胃炎".to_string(),
                note: Some("注意饮食".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-07-18 16:45:00".to_string()),
                update_time: Some("2025-07-18 16:45:00".to_string()),
            },
            // 2025年9月数据
            Customer {
                id: Some(5),
                name: "李雅雯".to_string(),
                phone: "13800138009".to_string(),
                age: 26,
                gender: "2".to_string(),
                time_of_visit: "2025-09-05 11:20:00".to_string(),
                trade_type: "1".to_string(),
                diagnosis: "月经不调".to_string(),
                note: Some("需要调理".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-09-05 11:20:00".to_string()),
                update_time: Some("2025-09-05 11:20:00".to_string()),
            },
            // 2024年12月数据
            Customer {
                id: Some(6),
                name: "赵大伯".to_string(),
                phone: "13800138011".to_string(),
                age: 68,
                gender: "1".to_string(),
                time_of_visit: "2024-12-25 14:30:00".to_string(),
                trade_type: "2".to_string(),
                diagnosis: "骨关节炎".to_string(),
                note: Some("老年患者，注意保暖".to_string()),
                status: "1".to_string(),
                create_time: Some("2024-12-25 14:30:00".to_string()),
                update_time: Some("2024-12-25 14:30:00".to_string()),
            },
            // 2025年1月数据
            Customer {
                id: Some(7),
                name: "孙美华".to_string(),
                phone: "13800138013".to_string(),
                age: 42,
                gender: "2".to_string(),
                time_of_visit: "2025-01-08 09:45:00".to_string(),
                trade_type: "1".to_string(),
                diagnosis: "缺铁性贫血".to_string(),
                note: Some("加强营养".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-01-08 09:45:00".to_string()),
                update_time: Some("2025-01-08 09:45:00".to_string()),
            },
            // 2025年3月数据
            Customer {
                id: Some(8),
                name: "周志强".to_string(),
                phone: "13800138015".to_string(),
                age: 55,
                gender: "1".to_string(),
                time_of_visit: "2025-03-12 15:30:00".to_string(),
                trade_type: "2".to_string(),
                diagnosis: "慢性支气管炎".to_string(),
                note: Some("戒烟，定期复查".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-03-12 15:30:00".to_string()),
                update_time: Some("2025-03-12 15:30:00".to_string()),
            },
            // 2025年5月数据
            Customer {
                id: Some(9),
                name: "钱小花".to_string(),
                phone: "13800138017".to_string(),
                age: 34,
                gender: "2".to_string(),
                time_of_visit: "2025-05-20 10:15:00".to_string(),
                trade_type: "1".to_string(),
                diagnosis: "过敏性皮炎".to_string(),
                note: Some("避免过敏原".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-05-20 10:15:00".to_string()),
                update_time: Some("2025-05-20 10:15:00".to_string()),
            },
        ];

        for customer in sample_customers {
            if let Some(id) = customer.id {
                customers.insert(id, customer);
                if id >= *next_customer_id {
                    *next_customer_id = id + 1;
                }
            }
        }

        println!("已初始化 {} 条示例客户数据", customers.len());
    }

    if registrations.is_empty() {
        let sample_registrations = vec![
            // 客户1 - 张三 (2025-08-15)
            CustomerRegistration {
                id: Some(1),
                customer_id: 1,
                treatment_items: vec![
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
                    LegacyTreatmentItem {
                        treatment_item: "拔罐疗法".to_string(),
                        count: 1,
                        unit_price: Some(50.0),
                        total_price: Some(50.0),
                    },
                ],
                total_amount: 250.0,
                paid_amount: 250.0,
                payment_status: "已支付".to_string(),
                registration_date: "2025-08-15".to_string(),
                notes: Some("颈椎病综合理疗".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-08-15 10:30:00".to_string()),
                update_time: Some("2025-08-15 10:30:00".to_string()),
            },
            // 客户2 - 王丽 (2025-08-22)
            CustomerRegistration {
                id: Some(2),
                customer_id: 2,
                treatment_items: vec![
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
                    LegacyTreatmentItem {
                        treatment_item: "足疗按摩".to_string(),
                        count: 2,
                        unit_price: Some(45.0),
                        total_price: Some(90.0),
                    },
                ],
                total_amount: 865.0,
                paid_amount: 865.0,
                payment_status: "已支付".to_string(),
                registration_date: "2025-08-22".to_string(),
                notes: Some("失眠多梦调理疗程".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-08-22 14:20:00".to_string()),
                update_time: Some("2025-08-22 14:20:00".to_string()),
            },
            // 客户3 - 刘建国 (2025-06-10)
            CustomerRegistration {
                id: Some(3),
                customer_id: 3,
                treatment_items: vec![
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
                    LegacyTreatmentItem {
                        treatment_item: "中医推拿".to_string(),
                        count: 3,
                        unit_price: Some(75.0),
                        total_price: Some(225.0),
                    },
                    LegacyTreatmentItem {
                        treatment_item: "热敷疗法".to_string(),
                        count: 1,
                        unit_price: Some(40.0),
                        total_price: Some(40.0),
                    },
                ],
                total_amount: 995.0,
                paid_amount: 800.0,
                payment_status: "部分支付".to_string(),
                registration_date: "2025-06-10".to_string(),
                notes: Some("腰椎间盘突出综合治疗".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-06-10 09:15:00".to_string()),
                update_time: Some("2025-06-10 09:15:00".to_string()),
            },
            // 客户4 - 陈小明 (2025-07-18)
            CustomerRegistration {
                id: Some(4),
                customer_id: 4,
                treatment_items: vec![
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
                paid_amount: 750.0,
                payment_status: "已支付".to_string(),
                registration_date: "2025-07-18".to_string(),
                notes: Some("慢性胃炎治疗".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-07-18 16:45:00".to_string()),
                update_time: Some("2025-07-18 16:45:00".to_string()),
            },
            // 客户5 - 李雅雯 (2025-09-05)
            CustomerRegistration {
                id: Some(5),
                customer_id: 5,
                treatment_items: vec![
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
                paid_amount: 1475.0,
                payment_status: "已支付".to_string(),
                registration_date: "2025-09-05".to_string(),
                notes: Some("月经不调调理".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-09-05 11:20:00".to_string()),
                update_time: Some("2025-09-05 11:20:00".to_string()),
            },
            // 客户6 - 赵大伯 (2024-12-25)
            CustomerRegistration {
                id: Some(6),
                customer_id: 6,
                treatment_items: vec![
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
                paid_amount: 1000.0,
                payment_status: "部分支付".to_string(),
                registration_date: "2024-12-25".to_string(),
                notes: Some("骨关节炎治疗".to_string()),
                status: "1".to_string(),
                create_time: Some("2024-12-25 14:30:00".to_string()),
                update_time: Some("2024-12-25 14:30:00".to_string()),
            },
            // 客户7 - 孙美华 (2025-01-08)
            CustomerRegistration {
                id: Some(7),
                customer_id: 7,
                treatment_items: vec![
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
                paid_amount: 1114.0,
                payment_status: "已支付".to_string(),
                registration_date: "2025-01-08".to_string(),
                notes: Some("缺铁性贫血治疗".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-01-08 09:45:00".to_string()),
                update_time: Some("2025-01-08 09:45:00".to_string()),
            },
            // 客户8 - 周志强 (2025-03-12)
            CustomerRegistration {
                id: Some(8),
                customer_id: 8,
                treatment_items: vec![
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
                paid_amount: 595.0,
                payment_status: "已支付".to_string(),
                registration_date: "2025-03-12".to_string(),
                notes: Some("慢性支气管炎治疗".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-03-12 15:30:00".to_string()),
                update_time: Some("2025-03-12 15:30:00".to_string()),
            },
            // 客户9 - 钱小花 (2025-05-20)
            CustomerRegistration {
                id: Some(9),
                customer_id: 9,
                treatment_items: vec![
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
                paid_amount: 274.0,
                payment_status: "已支付".to_string(),
                registration_date: "2025-05-20".to_string(),
                notes: Some("过敏性皮炎治疗".to_string()),
                status: "1".to_string(),
                create_time: Some("2025-05-20 10:15:00".to_string()),
                update_time: Some("2025-05-20 10:15:00".to_string()),
            },
        ];

        for registration in sample_registrations {
            if let Some(id) = registration.id {
                registrations.insert(id, registration);
                if id >= *next_registration_id {
                    *next_registration_id = id + 1;
                }
            }
        }

        println!("已初始化 {} 条示例登记数据", registrations.len());
    }
}

#[command]
pub async fn get_customer_treatments(#[allow(non_snake_case)] customerId: i32) -> Result<ApiResponse<Vec<LegacyTreatmentItem>>, String> {
    println!("正在获取客户 {} 的治疗项目", customerId);
    let registrations = REGISTRATIONS.lock().unwrap();
    
    // 查找该客户的所有登记记录
    let customer_registrations: Vec<&CustomerRegistration> = registrations
        .values()
        .filter(|reg| reg.customer_id == customerId && reg.status == "1")
        .collect();
    
    println!("找到 {} 条该客户的登记记录", customer_registrations.len());
    
    // 合并所有治疗项目
    let mut all_treatments = Vec::new();
    for registration in customer_registrations {
        println!("登记记录 ID: {:?}, 治疗项目数量: {}", registration.id, registration.treatment_items.len());
        all_treatments.extend(registration.treatment_items.clone());
    }
    
    println!("总共找到 {} 个治疗项目", all_treatments.len());
    for (index, item) in all_treatments.iter().enumerate() {
        println!("项目 {}: {} x{} = ¥{:?}", index + 1, item.treatment_item, item.count, item.total_price);
    }
    
    Ok(ApiResponse {
        code: 200,
        msg: "获取治疗项目成功".to_string(),
        data: all_treatments,
    })
}
