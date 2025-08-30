use crate::modules::database::get_database;
use crate::modules::types::{
    ApiResponse, MedicalRecord, PrescriptionFormItem, PrescriptionItemRecord,
    TreatmentItemRecord, LegacyTreatmentItem,
};
use chrono::Utc;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::command;

// 创建医疗记录的统一参数结构
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateMedicalRecordParams {
    pub record_type: String, // "customer", "medical", "prescription"
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
    pub single_dose_amount: Option<i64>, // 单付金额（以分为单位）
    pub total_amount: Option<i64>,       // 总金额（以分为单位）
    
    // 治疗项目 (散客登记和门诊病历使用)
    pub treatment_items: Option<Vec<LegacyTreatmentItem>>,
    
    // 中药处方项目 (中药处方使用)
    pub prescription_items: Option<Vec<PrescriptionFormItem>>,
    
    pub doctor_id: Option<i32>,
    pub doctor_name: Option<String>,
}

// 查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMedicalRecordsParams {
    pub current: i32,
    pub size: i32,
    pub record_type: Option<String>,
    pub customer_name: Option<String>,
    pub telephone: Option<String>,
    pub diagnosis: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub gender: Option<String>,
    pub min_age: Option<i32>,
    pub max_age: Option<i32>,
    pub trade_type: Option<String>,
}

// 分页查询结果
#[derive(Debug, Serialize, Deserialize)]
pub struct PagedMedicalRecords {
    pub records: Vec<MedicalRecord>,
    pub total: i32,
    pub current: i32,
    pub size: i32,
}

// 创建医疗记录 (统一接口)
#[command]
pub async fn create_unified_medical_record(params: CreateMedicalRecordParams) -> Result<ApiResponse<i32>, String> {
    let db = get_database();
    let conn = db.get_connection();
    let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 计算总金额（以分为单位）
    let total_amount: i64 = if let Some(total) = params.total_amount {
        // 优先使用前端传递的总金额
        total
    } else {
        // 回退到计算逻辑
        let mut calculated_total: i64 = 0;
        
        if let Some(treatment_items) = &params.treatment_items {
            for item in treatment_items {
                if let Some(total_price) = item.total_price {
                    calculated_total += total_price;
                }
            }
        }
        
        if let Some(prescription_items) = &params.prescription_items {
            for item in prescription_items {
                if let Some(total_price) = item.total_price {
                    calculated_total += total_price;
                } else {
                    // 如果没有传递总价，从数据库查询单价计算
                    let unit_price: i64 = conn
                        .query_row(
                            "SELECT unit_price FROM chinese_medicines WHERE name = ? AND status = '1'",
                            params![item.medicine_name],
                            |row| row.get(0)
                        )
                        .unwrap_or(0);
                    calculated_total += (unit_price as f64 * item.grams) as i64;
                }
            }
            
            // 如果是中药处方，需要乘以付数
            if let Some(doses) = params.doses {
                calculated_total *= doses as i64;
            }
        }
        
        calculated_total
    };

    // 插入主记录
    let result = conn.execute(
        "INSERT INTO medical_records (
            record_type, customer_name, gender, age, telephone, time_of_visit, 
            trade_type, diagnosis, note, chief_complaint, illness_history, 
            physical_examination, past_history, laboratory_examination, 
            treatment_method, doses, single_dose_amount, total_amount, 
            doctor_id, doctor_name, status, created_at, updated_at
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            params.record_type,
            params.customer_name,
            params.gender,
            params.age,
            params.telephone,
            params.time_of_visit,
            params.trade_type,
            params.diagnosis,
            params.note,
            params.chief_complaint,
            params.illness_history,
            params.physical_examination,
            params.past_history,
            params.laboratory_examination,
            params.treatment_method,
            params.doses,
            params.single_dose_amount.or_else(|| {
                if params.record_type == "prescription" && params.doses.is_some() {
                    Some(total_amount / params.doses.unwrap_or(1) as i64)
                } else {
                    None
                }
            }),
            total_amount,
            params.doctor_id,
            params.doctor_name,
            "1",
            now,
            now
        ],
    );

    match result {
        Ok(_) => {
            let record_id = conn.last_insert_rowid() as i32;

            // 插入治疗项目记录
            if let Some(treatment_items) = params.treatment_items {
                for item in treatment_items {
                    let unit_price = item.unit_price.unwrap_or(0);  // 使用0而不是0.0
                    let total_price = unit_price * item.count as i64; // 计算总价（分）
                    
                    conn.execute(
                        "INSERT INTO medical_treatment_items (
                            medical_record_id, treatment_item_name, treatment_count, 
                            unit_price, total_price, created_at
                        ) VALUES (?, ?, ?, ?, ?, ?)",
                        params![
                            record_id,
                            item.treatment_item,
                            item.count,
                            unit_price,
                            total_price,
                            now
                        ],
                    ).map_err(|e| format!("插入治疗项目失败: {}", e))?;
                }
            }

            // 插入中药处方记录
            if let Some(prescription_items) = params.prescription_items {
                for item in prescription_items {
                    // 优先使用前端传递的价格数据（以分为单位）
                    let (unit_price, total_price) = if let (Some(up), Some(tp)) = (item.unit_price, item.total_price) {
                        (up, tp) // 直接使用，已经是分为单位
                    } else {
                        // 回退到查询数据库
                        let queried_unit_price: i64 = conn
                            .query_row(
                                "SELECT unit_price FROM chinese_medicines WHERE name = ? AND status = '1'",
                                params![item.medicine_name],
                                |row| row.get(0)
                            )
                            .unwrap_or(0); // 数据库中存储的就是分为单位
                        let calculated_total_price = (queried_unit_price as f64 * item.grams) as i64;
                        (queried_unit_price, calculated_total_price)
                    };
                    
                    conn.execute(
                        "INSERT INTO medical_prescription_items (
                            medical_record_id, medicine_name, grams, 
                            unit_price, total_price, created_at
                        ) VALUES (?, ?, ?, ?, ?, ?)",
                        params![
                            record_id,
                            item.medicine_name,
                            item.grams,
                            unit_price,
                            total_price,
                            now
                        ],
                    ).map_err(|e| format!("插入中药项目失败: {}", e))?;
                }
            }

            Ok(ApiResponse {
                code: 200,
                msg: "医疗记录创建成功".to_string(),
                data: record_id,
            })
        }
        Err(e) => Err(format!("创建医疗记录失败: {}", e)),
    }
}

// 查询医疗记录列表 (统一接口)
#[command]
pub async fn get_unified_medical_record_list(params: QueryMedicalRecordsParams) -> Result<ApiResponse<PagedMedicalRecords>, String> {
    let db = get_database();
    let conn = db.get_connection();

    // 构建查询条件
    let mut where_clauses = vec!["1=1".to_string()];
    let mut query_params = Vec::new();

    if let Some(record_type) = &params.record_type {
        where_clauses.push("record_type = ?".to_string());
        query_params.push(record_type.clone());
    }

    if let Some(customer_name) = &params.customer_name {
        if !customer_name.is_empty() {
            where_clauses.push("customer_name LIKE ?".to_string());
            query_params.push(format!("%{}%", customer_name));
        }
    }

    if let Some(telephone) = &params.telephone {
        if !telephone.is_empty() {
            where_clauses.push("telephone LIKE ?".to_string());
            query_params.push(format!("%{}%", telephone));
        }
    }

    if let Some(diagnosis) = &params.diagnosis {
        if !diagnosis.is_empty() {
            where_clauses.push("diagnosis LIKE ?".to_string());
            query_params.push(format!("%{}%", diagnosis));
        }
    }

    if let Some(start_date) = &params.start_date {
        if !start_date.is_empty() {
            where_clauses.push("date(time_of_visit) >= ?".to_string());
            query_params.push(start_date.clone());
        }
    }

    if let Some(end_date) = &params.end_date {
        if !end_date.is_empty() {
            where_clauses.push("date(time_of_visit) <= ?".to_string());
            query_params.push(end_date.clone());
        }
    }

    if let Some(gender) = &params.gender {
        if !gender.is_empty() {
            where_clauses.push("gender = ?".to_string());
            query_params.push(gender.clone());
        }
    }

    if let Some(min_age) = params.min_age {
        where_clauses.push("age >= ?".to_string());
        query_params.push(min_age.to_string());
    }

    if let Some(max_age) = params.max_age {
        where_clauses.push("age <= ?".to_string());
        query_params.push(max_age.to_string());
    }

    if let Some(trade_type) = &params.trade_type {
        if !trade_type.is_empty() {
            where_clauses.push("trade_type = ?".to_string());
            query_params.push(trade_type.clone());
        }
    }

    let where_clause = where_clauses.join(" AND ");

    // 查询总数
    let count_sql = format!("SELECT COUNT(*) FROM medical_records WHERE {}", where_clause);
    let total: i32 = conn
        .query_row(&count_sql, rusqlite::params_from_iter(query_params.iter()), |row| {
            row.get(0)
        })
        .map_err(|e| format!("查询总数失败: {}", e))?;

    // 查询数据
    let offset = (params.current - 1) * params.size;
    let query_sql = format!(
        "SELECT id, record_type, customer_name, gender, age, telephone, time_of_visit, 
         trade_type, diagnosis, note, chief_complaint, illness_history, physical_examination, 
         past_history, laboratory_examination, treatment_method, doses, single_dose_amount, 
         total_amount, doctor_id, doctor_name, status, created_at, updated_at
         FROM medical_records 
         WHERE {} 
         ORDER BY time_of_visit DESC 
         LIMIT ? OFFSET ?",
        where_clause
    );

    let mut query_params_with_limit = query_params;
    query_params_with_limit.push(params.size.to_string());
    query_params_with_limit.push(offset.to_string());

    let mut stmt = conn
        .prepare(&query_sql)
        .map_err(|e| format!("准备查询语句失败: {}", e))?;

    let records: Result<Vec<MedicalRecord>, _> = stmt
        .query_map(
            rusqlite::params_from_iter(query_params_with_limit.iter()),
            |row| {
                Ok(MedicalRecord {
                    id: Some(row.get(0)?),
                    record_type: row.get(1)?,
                    customer_name: row.get(2)?,
                    gender: row.get(3)?,
                    age: row.get(4)?,
                    telephone: row.get(5)?,
                    time_of_visit: row.get(6)?,
                    trade_type: row.get(7)?,
                    diagnosis: row.get(8)?,
                    note: row.get(9)?,
                    chief_complaint: row.get(10)?,
                    illness_history: row.get(11)?,
                    physical_examination: row.get(12)?,
                    past_history: row.get(13)?,
                    laboratory_examination: row.get(14)?,
                    treatment_method: row.get(15)?,
                    doses: row.get(16)?,
                    single_dose_amount: row.get(17)?,
                    total_amount: row.get(18)?,
                    doctor_id: row.get(19)?,
                    doctor_name: row.get(20)?,
                    status: row.get(21)?,
                    created_at: row.get(22)?,
                    updated_at: row.get(23)?,
                    treatment_items: None, // 延迟加载
                    prescription_items: None, // 延迟加载
                })
            },
        )
        .and_then(|mapped_rows| mapped_rows.collect());

    match records {
        Ok(mut records) => {
            // 为每个记录加载关联的治疗项目和中药项目
            for record in &mut records {
                if let Some(id) = record.id {
                    // 加载治疗项目
                    if record.record_type == "customer" || record.record_type == "medical" {
                        record.treatment_items = load_treatment_items(&conn, id).ok();
                    }
                    
                    // 加载中药项目
                    if record.record_type == "prescription" {
                        record.prescription_items = load_prescription_items(&conn, id).ok();
                    }
                }
            }

            Ok(ApiResponse {
                code: 200,
                msg: "查询成功".to_string(),
                data: PagedMedicalRecords {
                    records,
                    total,
                    current: params.current,
                    size: params.size,
                },
            })
        }
        Err(e) => Err(format!("查询医疗记录失败: {}", e)),
    }
}

// 加载治疗项目
fn load_treatment_items(conn: &rusqlite::Connection, medical_record_id: i32) -> Result<Vec<TreatmentItemRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, medical_record_id, treatment_item_name, treatment_count, unit_price, total_price, created_at 
         FROM medical_treatment_items 
         WHERE medical_record_id = ? 
         ORDER BY id"
    )?;

    let items: Result<Vec<TreatmentItemRecord>, _> = stmt
        .query_map(params![medical_record_id], |row| {
            Ok(TreatmentItemRecord {
                id: Some(row.get(0)?),
                medical_record_id: row.get(1)?,
                treatment_item_name: row.get(2)?,
                treatment_count: row.get(3)?,
                unit_price: row.get(4)?,
                total_price: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .and_then(|mapped_rows| mapped_rows.collect());

    items
}

// 加载中药项目
fn load_prescription_items(conn: &rusqlite::Connection, medical_record_id: i32) -> Result<Vec<PrescriptionItemRecord>, rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT id, medical_record_id, medicine_name, grams, unit_price, total_price, created_at 
         FROM medical_prescription_items 
         WHERE medical_record_id = ? 
         ORDER BY id"
    )?;

    let items: Result<Vec<PrescriptionItemRecord>, _> = stmt
        .query_map(params![medical_record_id], |row| {
            Ok(PrescriptionItemRecord {
                id: Some(row.get(0)?),
                medical_record_id: row.get(1)?,
                medicine_name: row.get(2)?,
                grams: row.get(3)?,
                unit_price: row.get(4)?,
                total_price: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .and_then(|mapped_rows| mapped_rows.collect());

    items
}

// 根据ID获取医疗记录详情
#[command]
pub async fn get_unified_medical_record_detail(id: i32) -> Result<ApiResponse<MedicalRecord>, String> {
    let db = get_database();
    let conn = db.get_connection();

    let query_sql = "SELECT id, record_type, customer_name, gender, age, telephone, time_of_visit, 
                     trade_type, diagnosis, note, chief_complaint, illness_history, physical_examination, 
                     past_history, laboratory_examination, treatment_method, doses, single_dose_amount, 
                     total_amount, doctor_id, doctor_name, status, created_at, updated_at
                     FROM medical_records 
                     WHERE id = ?";

    let record_result = conn.query_row(query_sql, params![id], |row| {
        Ok(MedicalRecord {
            id: Some(row.get(0)?),
            record_type: row.get(1)?,
            customer_name: row.get(2)?,
            gender: row.get(3)?,
            age: row.get(4)?,
            telephone: row.get(5)?,
            time_of_visit: row.get(6)?,
            trade_type: row.get(7)?,
            diagnosis: row.get(8)?,
            note: row.get(9)?,
            chief_complaint: row.get(10)?,
            illness_history: row.get(11)?,
            physical_examination: row.get(12)?,
            past_history: row.get(13)?,
            laboratory_examination: row.get(14)?,
            treatment_method: row.get(15)?,
            doses: row.get(16)?,
            single_dose_amount: row.get(17)?,
            total_amount: row.get(18)?,
            doctor_id: row.get(19)?,
            doctor_name: row.get(20)?,
            status: row.get(21)?,
            created_at: row.get(22)?,
            updated_at: row.get(23)?,
            treatment_items: None,
            prescription_items: None,
        })
    });

    match record_result {
        Ok(mut record) => {
            // 加载关联数据
            if record.record_type == "customer" || record.record_type == "medical" {
                record.treatment_items = load_treatment_items(&conn, id).ok();
            }
            
            if record.record_type == "prescription" {
                record.prescription_items = load_prescription_items(&conn, id).ok();
            }

            Ok(ApiResponse {
                code: 200,
                msg: "查询成功".to_string(),
                data: record,
            })
        }
        Err(e) => Err(format!("查询医疗记录详情失败: {}", e)),
    }
}

// 删除医疗记录 (统一接口)
#[command]
pub async fn delete_unified_medical_record(id: i32) -> Result<ApiResponse<bool>, String> {
    let db = get_database();
    let conn = db.get_connection();

    // 开始事务
    let tx = conn.unchecked_transaction().map_err(|e| format!("开始事务失败: {}", e))?;

    // 删除关联的治疗项目
    if let Err(e) = tx.execute(
        "DELETE FROM medical_treatment_items WHERE medical_record_id = ?",
        params![id],
    ) {
        return Err(format!("删除治疗项目失败: {}", e));
    }

    // 删除关联的处方项目
    if let Err(e) = tx.execute(
        "DELETE FROM medical_prescription_items WHERE medical_record_id = ?",
        params![id],
    ) {
        return Err(format!("删除处方项目失败: {}", e));
    }

    // 删除医疗记录主表
    let result = tx.execute(
        "DELETE FROM medical_records WHERE id = ?",
        params![id],
    );

    match result {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                // 提交事务
                tx.commit().map_err(|e| format!("提交事务失败: {}", e))?;
                Ok(ApiResponse {
                    code: 200,
                    msg: "医疗记录删除成功".to_string(),
                    data: true,
                })
            } else {
                Err("医疗记录不存在".to_string())
            }
        }
        Err(e) => Err(format!("删除医疗记录失败: {}", e)),
    }
}

// 批量删除医疗记录 (统一接口)
#[command]
pub async fn delete_unified_medical_records_batch(ids: Vec<i32>) -> Result<ApiResponse<i32>, String> {
    if ids.is_empty() {
        return Err("删除ID列表不能为空".to_string());
    }

    let db = get_database();
    let conn = db.get_connection();

    // 开始事务
    let tx = conn.unchecked_transaction().map_err(|e| format!("开始事务失败: {}", e))?;

    let mut deleted_count = 0;

    for id in ids {
        // 删除关联的治疗项目
        if let Err(e) = tx.execute(
            "DELETE FROM medical_treatment_items WHERE medical_record_id = ?",
            params![id],
        ) {
            return Err(format!("删除治疗项目失败 (ID: {}): {}", id, e));
        }

        // 删除关联的处方项目
        if let Err(e) = tx.execute(
            "DELETE FROM medical_prescription_items WHERE medical_record_id = ?",
            params![id],
        ) {
            return Err(format!("删除处方项目失败 (ID: {}): {}", id, e));
        }

        // 删除医疗记录主表
        match tx.execute(
            "DELETE FROM medical_records WHERE id = ?",
            params![id],
        ) {
            Ok(rows_affected) => {
                if rows_affected > 0 {
                    deleted_count += 1;
                }
            }
            Err(e) => return Err(format!("删除医疗记录失败 (ID: {}): {}", id, e)),
        }
    }

    // 提交事务
    tx.commit().map_err(|e| format!("提交事务失败: {}", e))?;

    Ok(ApiResponse {
        code: 200,
        msg: format!("批量删除成功，共删除 {} 条记录", deleted_count),
        data: deleted_count,
    })
}
