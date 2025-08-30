use crate::modules::types::{ApiResponse, LegacyTreatmentItem};
use serde::{Deserialize, Serialize};
use tauri::command;

// 内存存储模式的导入
#[cfg(feature = "memory-storage")]
use crate::modules::utils::contains_pinyin;
#[cfg(feature = "memory-storage")]
use std::collections::HashMap;
#[cfg(feature = "memory-storage")]
use std::sync::LazyLock;
#[cfg(feature = "memory-storage")]
use std::sync::Mutex;

// SQLite存储模式的导入
#[cfg(feature = "sqlite-storage")]
use crate::modules::database::get_database;
#[cfg(feature = "sqlite-storage")]
use chrono::Utc;
#[cfg(feature = "sqlite-storage")]
use rusqlite::params;

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

// 模拟数据存储（实际项目中应该使用数据库） - 内存模式
#[cfg(feature = "memory-storage")]
static CUSTOMERS: LazyLock<Mutex<HashMap<i32, Customer>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
#[cfg(feature = "memory-storage")]
static REGISTRATIONS: LazyLock<Mutex<HashMap<i32, CustomerRegistration>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
#[cfg(feature = "memory-storage")]
static NEXT_CUSTOMER_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));
#[cfg(feature = "memory-storage")]
static NEXT_REGISTRATION_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

// 暂时使用简化版本，只实现SQLite存储模式下的关键函数
#[cfg(feature = "sqlite-storage")]
#[command]
pub async fn get_customer_list(
    params: CustomerSearchParams,
) -> Result<ApiResponse<CustomerListResponse>, String> {
    println!("🔧 使用 SQLite 存储模式获取客户列表");
    println!("接收到客户列表查询请求: {:?}", params);

    let db = get_database();
    let conn = db.get_connection();

    // 构建WHERE子句
    let mut where_clauses = Vec::new();
    let mut query_params = Vec::new();

    if let Some(name) = &params.name {
        if !name.is_empty() {
            where_clauses.push("name LIKE ?");
            query_params.push(format!("%{}%", name));
        }
    }

    if let Some(phone) = &params.phone {
        if !phone.is_empty() {
            where_clauses.push("phone LIKE ?");
            query_params.push(format!("%{}%", phone));
        }
    }

    if let Some(diagnosis) = &params.diagnosis {
        if !diagnosis.is_empty() {
            where_clauses.push("diagnosis LIKE ?");
            query_params.push(format!("%{}%", diagnosis));
        }
    }

    if let Some(gender) = &params.gender {
        if !gender.is_empty() {
            where_clauses.push("gender = ?");
            query_params.push(gender.clone());
        }
    }

    if let Some(status) = &params.status {
        if !status.is_empty() {
            where_clauses.push("status = ?");
            query_params.push(status.clone());
        }
    }

    if let Some(min_age) = params.min_age {
        where_clauses.push("age >= ?");
        query_params.push(min_age.to_string());
    }

    if let Some(max_age) = params.max_age {
        where_clauses.push("age <= ?");
        query_params.push(max_age.to_string());
    }

    if let Some(start_date) = &params.start_date {
        if !start_date.is_empty() {
            where_clauses.push("time_of_visit >= ?");
            query_params.push(start_date.clone());
        }
    }

    if let Some(end_date) = &params.end_date {
        if !end_date.is_empty() {
            where_clauses.push("time_of_visit <= ?");
            query_params.push(end_date.clone());
        }
    }

    let where_clause = if where_clauses.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", where_clauses.join(" AND "))
    };

    // 获取总数
    let count_query = format!("SELECT COUNT(*) FROM customers{}", where_clause);
    let total: i32 = conn
        .query_row(
            &count_query,
            rusqlite::params_from_iter(&query_params),
            |row| row.get(0),
        )
        .map_err(|e| format!("查询总数失败: {}", e))?;

    // 获取分页数据
    let offset = (params.current - 1) * params.size;
    let data_query = format!(
        "SELECT id, name, phone, age, gender, time_of_visit, trade_type, diagnosis, note, status, created_at, updated_at 
         FROM customers{} ORDER BY created_at DESC LIMIT {} OFFSET {}",
        where_clause, params.size, offset
    );

    let mut stmt = conn
        .prepare(&data_query)
        .map_err(|e| format!("准备查询语句失败: {}", e))?;

    let customer_iter = stmt
        .query_map(rusqlite::params_from_iter(&query_params), |row| {
            Ok(Customer {
                id: Some(row.get(0)?),
                name: row.get(1)?,
                phone: row.get(2)?,
                age: row.get(3)?,
                gender: row.get(4)?,
                time_of_visit: row.get(5)?,
                trade_type: row.get(6)?,
                diagnosis: row.get(7)?,
                note: row.get(8)?,
                status: row.get(9)?,
                create_time: Some(row.get(10)?),
                update_time: Some(row.get(11)?),
            })
        })
        .map_err(|e| format!("执行查询失败: {}", e))?;

    let mut customers = Vec::new();
    for customer in customer_iter {
        customers.push(customer.map_err(|e| format!("解析客户数据失败: {}", e))?);
    }

    println!(
        "🔍 查询结果: total={}, current_page_records={}",
        total,
        customers.len()
    );
    for (i, customer) in customers.iter().enumerate() {
        println!(
            "  记录 {}: id={:?}, name={}, phone={}",
            i + 1,
            customer.id,
            customer.name,
            customer.phone
        );
    }

    Ok(ApiResponse {
        code: 200,
        msg: "查询成功".to_string(),
        data: CustomerListResponse {
            records: customers,
            current: params.current,
            size: params.size,
            total,
        },
    })
}

// 内存存储模式下的实现保持原样但加上条件编译
#[cfg(feature = "memory-storage")]
#[command]
pub async fn get_customer_list(
    params: CustomerSearchParams,
) -> Result<ApiResponse<CustomerListResponse>, String> {
    println!("🔧 使用内存存储模式获取客户列表");

    let customers = CUSTOMERS.lock().unwrap();
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
            true
        })
        .cloned()
        .collect();

    // 按创建时间排序
    filtered_customers.sort_by(|a, b| {
        b.create_time
            .as_ref()
            .unwrap_or(&String::new())
            .cmp(a.create_time.as_ref().unwrap_or(&String::new()))
    });

    let total = filtered_customers.len() as i32;
    let start = ((params.current - 1) * params.size) as usize;
    let end = std::cmp::min(start + params.size as usize, filtered_customers.len());

    let paginated_customers = if start < filtered_customers.len() {
        filtered_customers[start..end].to_vec()
    } else {
        Vec::new()
    };

    Ok(ApiResponse {
        code: 200,
        msg: "查询成功".to_string(),
        data: CustomerListResponse {
            records: paginated_customers,
            current: params.current,
            size: params.size,
            total,
        },
    })
}

// 创建客户
#[cfg(feature = "sqlite-storage")]
#[command]
pub async fn create_customer(customer: Customer) -> Result<ApiResponse<Customer>, String> {
    println!("接收到客户创建请求: {:?}", customer);

    let db = get_database();
    let conn = db.get_connection();

    // 检查是否已存在相同电话号码的客户
    let check_result: Result<i32, rusqlite::Error> = conn.query_row(
        "SELECT COUNT(*) FROM customers WHERE phone = ?1",
        params![customer.phone],
        |row| row.get(0),
    );

    match check_result {
        Ok(count) => {
            if count > 0 {
                return Err(format!(
                    "电话号码 '{}' 已存在，请使用其他号码",
                    customer.phone
                ));
            }
        }
        Err(e) => {
            return Err(format!("检查重复电话号码失败: {}", e));
        }
    }

    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let result = conn.execute(
        "INSERT INTO customers (name, phone, age, gender, time_of_visit, trade_type, diagnosis, note, status, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)",
        params![
            customer.name,
            customer.phone,
            customer.age,
            customer.gender,
            customer.time_of_visit,
            customer.trade_type,
            customer.diagnosis,
            customer.note,
            customer.status,
            now,
            now
        ],
    );

    match result {
        Ok(_) => {
            let id = conn.last_insert_rowid() as i32;
            let mut new_customer = customer;
            new_customer.id = Some(id);
            new_customer.create_time = Some(now.clone());
            new_customer.update_time = Some(now);

            println!("✅ 客户创建成功，ID: {}", id);
            Ok(ApiResponse {
                code: 200,
                msg: "创建成功".to_string(),
                data: new_customer,
            })
        }
        Err(e) => {
            println!("❌ 客户创建失败: {}", e);
            Err(format!("创建失败: {}", e))
        }
    }
}

#[cfg(feature = "memory-storage")]
#[command]
pub async fn create_customer(customer: Customer) -> Result<ApiResponse<Customer>, String> {
    let mut customers = CUSTOMERS.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();

    // 检查是否已存在相同电话号码的客户
    for existing_customer in customers.values() {
        if existing_customer.phone == customer.phone {
            return Err(format!(
                "电话号码 '{}' 已存在，请使用其他号码",
                customer.phone
            ));
        }
    }

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

// 创建客户登记
#[cfg(feature = "sqlite-storage")]
#[command]
pub async fn create_customer_registration(
    registration: CustomerRegistration,
) -> Result<ApiResponse<CustomerRegistration>, String> {
    println!("接收到客户登记创建请求: {:?}", registration);

    let db = get_database();
    let conn = db.get_connection();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 先插入主记录
    let result = conn.execute(
        "INSERT INTO customer_registrations (customer_id, total_amount, paid_amount, payment_status, registration_date, notes, status, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            registration.customer_id,
            registration.total_amount,
            registration.paid_amount,
            registration.payment_status,
            registration.registration_date,
            registration.notes,
            registration.status,
            now,
            now
        ],
    );

    match result {
        Ok(_) => {
            let registration_id = conn.last_insert_rowid() as i32;

            // 插入治疗项目详情
            for item in &registration.treatment_items {
                let item_result = conn.execute(
                    "INSERT INTO registration_treatment_items (registration_id, treatment_item, count, unit_price, total_price) 
                     VALUES (?1, ?2, ?3, ?4, ?5)",
                    params![
                        registration_id,
                        item.treatment_item,
                        item.count,
                        item.unit_price.unwrap_or(0),  // 使用0而不是0.0
                        item.total_price.unwrap_or(0)  // 使用0而不是0.0
                    ],
                );

                if let Err(e) = item_result {
                    println!("❌ 插入治疗项目失败: {}", e);
                    return Err(format!("插入治疗项目失败: {}", e));
                }
            }

            let mut new_registration = registration;
            new_registration.id = Some(registration_id);
            new_registration.create_time = Some(now.clone());
            new_registration.update_time = Some(now);

            println!("✅ 客户登记创建成功，ID: {}", registration_id);
            Ok(ApiResponse {
                code: 200,
                msg: "登记成功".to_string(),
                data: new_registration,
            })
        }
        Err(e) => {
            println!("❌ 客户登记创建失败: {}", e);
            Err(format!("登记失败: {}", e))
        }
    }
}

#[cfg(feature = "memory-storage")]
#[command]
pub async fn create_customer_registration(
    registration: CustomerRegistration,
) -> Result<ApiResponse<CustomerRegistration>, String> {
    // 内存存储模式的简单实现
    let mut new_registration = registration;
    new_registration.id = Some(1); // 简单的ID分配
    new_registration.create_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    new_registration.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

    Ok(ApiResponse {
        code: 200,
        msg: "登记成功".to_string(),
        data: new_registration,
    })
}

// TODO: 其他函数暂时保持简化状态或使用条件编译
