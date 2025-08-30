use serde::{Deserialize, Serialize};
use tauri::command;
use crate::modules::types::{LegacyTreatmentItem, ApiResponse};

#[cfg(feature = "memory-storage")]
use crate::modules::utils::contains_pinyin;

// 内存存储模式的导入
#[cfg(feature = "memory-storage")]
use std::collections::HashMap;
#[cfg(feature = "memory-storage")]
use std::sync::LazyLock;
#[cfg(feature = "memory-storage")]
use std::sync::Mutex;

// SQLite存储模式的导入
#[cfg(feature = "sqlite-storage")]
use crate::modules::database::get_database;

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

// 医疗记录存储 - 内存模式
#[cfg(feature = "memory-storage")]
static MEDICAL_RECORDS: LazyLock<Mutex<HashMap<i32, MedicalRecord>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
#[cfg(feature = "memory-storage")]
static MEDICAL_RECORD_NEXT_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

#[command]
pub async fn get_medical_record_list(
    params: MedicalRecordSearchParams,
) -> Result<ApiResponse<MedicalRecordListResponse>, String> {
    println!("接收到医疗记录查询请求: {:?}", params);
    
    #[cfg(feature = "sqlite-storage")]
    {
        println!("🔧 使用 SQLite 存储模式获取医疗记录列表");
        
        let db = get_database();
        let conn = db.get_connection();
        
        // 构建查询SQL
        let mut sql = "SELECT id, customer_name, gender, age, telephone, time_of_visit, 
                             trade_type, chief_complaint, illness_history, physical_examination, 
                             past_history, laboratory_examination, diagnosis, treatment_method, 
                             note, treatment_items, total_amount, doctor_id, doctor_name, 
                             created_at, updated_at FROM medical_records WHERE 1=1".to_string();
        let mut sql_params: Vec<String> = Vec::new();
        
        if let Some(ref name) = params.customer_name {
            if !name.is_empty() {
                sql.push_str(" AND customer_name LIKE ?");
                sql_params.push(format!("%{}%", name));
            }
        }
        if let Some(ref telephone) = params.telephone {
            if !telephone.is_empty() {
                sql.push_str(" AND telephone LIKE ?");
                sql_params.push(format!("%{}%", telephone));
            }
        }
        if let Some(ref diagnosis) = params.diagnosis {
            if !diagnosis.is_empty() {
                sql.push_str(" AND diagnosis LIKE ?");
                sql_params.push(format!("%{}%", diagnosis));
            }
        }
        if let Some(doctor_id) = params.doctor_id {
            sql.push_str(" AND doctor_id = ?");
            sql_params.push(doctor_id.to_string());
        }
        if let Some(ref start_date) = params.start_date {
            if !start_date.is_empty() {
                sql.push_str(" AND time_of_visit >= ?");
                sql_params.push(start_date.clone());
            }
        }
        if let Some(ref end_date) = params.end_date {
            if !end_date.is_empty() {
                sql.push_str(" AND time_of_visit <= ?");
                sql_params.push(end_date.clone());
            }
        }
        if let Some(ref gender) = params.gender {
            if !gender.is_empty() {
                sql.push_str(" AND gender = ?");
                sql_params.push(gender.clone());
            }
        }
        if let Some(min_age) = params.min_age {
            sql.push_str(" AND age >= ?");
            sql_params.push(min_age.to_string());
        }
        if let Some(max_age) = params.max_age {
            sql.push_str(" AND age <= ?");
            sql_params.push(max_age.to_string());
        }
        
        sql.push_str(" ORDER BY created_at DESC");
        
        let mut stmt = conn.prepare(&sql).map_err(|e| format!("准备查询失败: {}", e))?;
        let medical_record_iter = stmt.query_map(
            rusqlite::params_from_iter(sql_params.iter()),
            |row| {
                let treatment_json: String = row.get("treatment_items")?;
                let treatment: Vec<LegacyTreatmentItem> = serde_json::from_str(&treatment_json)
                    .unwrap_or_default();
                
                Ok(MedicalRecord {
                    id: Some(row.get("id")?),
                    customer_name: row.get("customer_name")?,
                    gender: row.get("gender")?,
                    age: row.get("age")?,
                    telephone: row.get("telephone")?,
                    time_of_visit: row.get("time_of_visit")?,
                    trade_type: row.get("trade_type")?,
                    chief_complaint: row.get("chief_complaint")?,
                    illness_history: row.get("illness_history")?,
                    physical_examination: row.get("physical_examination")?,
                    past_history: row.get("past_history")?,
                    laboratory_examination: row.get("laboratory_examination")?,
                    diagnosis: row.get("diagnosis")?,
                    treatment_method: row.get("treatment_method")?,
                    note: row.get("note")?,
                    treatment,
                    total_amount: row.get("total_amount")?,
                    doctor_id: row.get("doctor_id")?,
                    doctor_name: row.get("doctor_name")?,
                    create_time: row.get("created_at")?,
                    update_time: row.get("updated_at")?,
                })
            },
        ).map_err(|e| format!("查询失败: {}", e))?;
        
        let mut all_records: Vec<MedicalRecord> = medical_record_iter
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("获取记录失败: {}", e))?;
        
        // 分页处理
        let total = all_records.len() as i32;
        let start = ((params.current - 1) * params.size) as usize;
        let end = std::cmp::min(start + params.size as usize, all_records.len());
        
        if start < all_records.len() {
            all_records = all_records[start..end].to_vec();
        } else {
            all_records = Vec::new();
        }
        
        Ok(ApiResponse {
            code: 200,
            msg: "查询成功".to_string(),
            data: MedicalRecordListResponse {
                records: all_records,
                current: params.current,
                size: params.size,
                total,
            },
        })
    }
    
    #[cfg(feature = "memory-storage")]
    {
        println!("🔧 使用内存存储模式获取医疗记录列表");
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
                if let Some(doctor_id) = params.doctor_id {
                    if record.doctor_id != doctor_id {
                        return false;
                    }
                }
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
                true
            })
            .cloned()
            .collect();

        // 按创建时间排序
        filtered_records.sort_by(|a, b| {
            b.create_time
                .as_ref()
                .unwrap_or(&String::new())
                .cmp(a.create_time.as_ref().unwrap_or(&String::new()))
        });

        let total = filtered_records.len() as i32;
        let start = ((params.current - 1) * params.size) as usize;
        let end = std::cmp::min(start + params.size as usize, filtered_records.len());

        let paginated_records = if start < filtered_records.len() {
            filtered_records[start..end].to_vec()
        } else {
            Vec::new()
        };

        Ok(ApiResponse {
            code: 200,
            msg: "查询成功".to_string(),
            data: MedicalRecordListResponse {
                records: paginated_records,
                current: params.current,
                size: params.size,
                total,
            },
        })
    }
}

#[command]
pub async fn create_medical_record(
    params: MedicalRecordCreateParams,
) -> Result<ApiResponse<MedicalRecord>, String> {
    println!("接收到医疗记录创建请求: {:?}", params);
    
    #[cfg(feature = "sqlite-storage")]
    {
        println!("🔧 使用 SQLite 存储模式创建医疗记录");
        
        let db = get_database();
        let conn = db.get_connection();
        
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let treatment_json = serde_json::to_string(&params.treatment).unwrap_or_default();
        
        let mut stmt = conn.prepare(
            "INSERT INTO medical_records (customer_name, gender, age, telephone, time_of_visit, 
             trade_type, chief_complaint, illness_history, physical_examination, past_history, 
             laboratory_examination, diagnosis, treatment_method, note, treatment_items, 
             total_amount, doctor_id, doctor_name, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20)"
        ).map_err(|e| format!("准备插入语句失败: {}", e))?;
        
        stmt.execute(rusqlite::params![
            params.customer_name,
            params.gender,
            params.age,
            params.telephone,
            params.time_of_visit,
            params.trade_type,
            params.chief_complaint,
            params.illness_history,
            params.physical_examination,
            params.past_history,
            params.laboratory_examination,
            params.diagnosis,
            params.treatment_method,
            params.note,
            treatment_json,
            params.total_amount,
            params.doctor_id,
            "", // doctor_name placeholder
            now,
            now
        ]).map_err(|e| format!("插入记录失败: {}", e))?;
        
        let record_id = conn.last_insert_rowid() as i32;
        
        let record = MedicalRecord {
            id: Some(record_id),
            customer_name: params.customer_name,
            gender: params.gender,
            age: params.age,
            telephone: params.telephone,
            time_of_visit: params.time_of_visit,
            trade_type: params.trade_type,
            chief_complaint: params.chief_complaint,
            illness_history: params.illness_history,
            physical_examination: params.physical_examination,
            past_history: params.past_history,
            laboratory_examination: params.laboratory_examination,
            diagnosis: params.diagnosis,
            treatment_method: params.treatment_method,
            note: params.note,
            treatment: params.treatment,
            total_amount: params.total_amount,
            doctor_id: params.doctor_id,
            doctor_name: Some("".to_string()),
            create_time: Some(now.clone()),
            update_time: Some(now),
        };
        
        Ok(ApiResponse {
            code: 200,
            msg: "创建成功".to_string(),
            data: record,
        })
    }
    
    #[cfg(feature = "memory-storage")]
    {
        println!("🔧 使用内存存储模式创建医疗记录");
        let mut records = MEDICAL_RECORDS.lock().unwrap();
        let mut next_id = MEDICAL_RECORD_NEXT_ID.lock().unwrap();
        
        let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let record = MedicalRecord {
            id: Some(*next_id),
            customer_name: params.customer_name,
            gender: params.gender,
            age: params.age,
            telephone: params.telephone,
            time_of_visit: params.time_of_visit,
            trade_type: params.trade_type,
            chief_complaint: params.chief_complaint,
            illness_history: params.illness_history,
            physical_examination: params.physical_examination,
            past_history: params.past_history,
            laboratory_examination: params.laboratory_examination,
            diagnosis: params.diagnosis,
            treatment_method: params.treatment_method,
            note: params.note,
            treatment: params.treatment,
            total_amount: params.total_amount,
            doctor_id: params.doctor_id,
            doctor_name: Some("".to_string()),
            create_time: Some(now.clone()),
            update_time: Some(now),
        };
        
        records.insert(*next_id, record.clone());
        *next_id += 1;
        
        Ok(ApiResponse {
            code: 200,
            msg: "创建成功".to_string(),
            data: record,
        })
    }
}

// TODO: 继续实现其他函数...
// 为了简化，我暂时只实现了两个主要函数
// 其他函数可以按照相同的模式实现
