use crate::modules::database::get_database;
use crate::modules::types::{
    ApiResponse, MedicalRecord, PrescriptionItemRecord, TreatmentItemRecord,
};
use chrono::Local;
use rusqlite::params;
use serde::{Deserialize, Serialize};
use tauri::command;

// 查询参数
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryMedicalRecordsParams {
    pub current: i64,
    pub size: i64,
    pub data_type: Option<String>,
    pub customer_name: Option<String>,
    pub telephone: Option<String>,
    pub diagnosis: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub gender: Option<String>,
    pub min_age: Option<i64>,
    pub max_age: Option<i64>,
    pub trade_type: Option<String>,
    // 新增搜索参数
    pub doctor_id: Option<i64>,
    pub doctor_name: Option<String>,
    pub chief_complaint: Option<String>,
    pub illness_history: Option<String>,
    pub physical_examination: Option<String>,
    pub past_history: Option<String>,
    pub laboratory_examination: Option<String>,
    pub treatment_method: Option<String>,
    pub doses: Option<i64>,
    pub min_amount: Option<i64>,
    pub max_amount: Option<i64>,
    pub note: Option<String>,
}

// 分页查询结果
#[derive(Debug, Serialize, Deserialize)]
pub struct PagedMedicalRecords {
    pub records: Vec<MedicalRecord>,
    pub total: i64,
    pub current: i64,
    pub size: i64,
}

// 创建医疗记录 (统一接口)
#[command]
pub async fn create_unified_medical_record(
    params: MedicalRecord,
) -> Result<ApiResponse<i64>, String> {
    println!("Creating medical record: {:?}", params);
    let db = get_database();
    let mut conn = db.get_connection();
    let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    // 验证必要字段
    if params.customer_name.trim().is_empty() {
        return Err("患者姓名不能为空".to_string());
    }
    if params.diagnosis.trim().is_empty() {
        return Err("诊断信息不能为空".to_string());
    }

    println!(
        "开始原子事务插入医疗记录与明细，患者: {}",
        params.customer_name
    );

    let tx = conn
        .transaction()
        .map_err(|e| format!("开启事务失败: {}", e))?;

    // 插入主记录
    let insert_result = tx.execute(
        "INSERT INTO medical_records (
            data_type, customer_name, gender, age, telephone, time_of_visit, 
            trade_type, diagnosis, note, total_amount, doctor_id, doctor_name, 
            created_at, updated_at, chief_complaint, illness_history, 
            physical_examination, past_history, laboratory_examination, 
            treatment_method, doses, single_dose_amount
        ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            params.data_type,
            params.customer_name,
            params.gender,
            params.age,
            params.telephone,
            params.time_of_visit,
            params.trade_type,
            params.diagnosis,
            params.note,
            params.total_amount,
            params.doctor_id,
            params.doctor_name,
            now,
            now,
            params.chief_complaint,
            params.illness_history,
            params.physical_examination,
            params.past_history,
            params.laboratory_examination,
            params.treatment_method,
            params.doses,
            params.single_dose_amount,
        ],
    );

    if let Err(e) = insert_result {
        return Err(format!(
            "插入主医疗记录失败 - 患者: '{}', 诊断: '{}', 记录类型: '{}', 医生: '{}', 数据库错误: {}",
            params.customer_name,
            params.diagnosis,
            params.data_type,
            params.doctor_name.as_deref().unwrap_or("未指定"),
            e
        ));
    }

    let record_id = tx.last_insert_rowid();

    // 插入治疗项目记录
    if let Some(treatment_items) = params.treatment_items {
        for (index, item) in treatment_items.iter().enumerate() {
            println!("插入治疗项目 #{}: {:?}", index + 1, item);
            let res = tx.execute(
                "INSERT INTO medical_treatment_items (
                    medical_record_id, treatment_item, count, 
                    unit_price, total_price, created_at
                ) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    record_id,
                    item.treatment_item,
                    item.count,
                    item.unit_price,
                    item.total_price,
                    now
                ],
            );
            if let Err(e) = res {
                return Err(format!(
                    "插入治疗项目失败 - 项目名称: '{}', 数量: {}, 单价: {}, 总价: {}, 错误: {}",
                    item.treatment_item, item.count, item.unit_price, item.total_price, e
                ));
            }
        }
    }

    // 插入中医中药处方明细
    if let Some(prescription_items) = params.prescription_items {
        for (index, item) in prescription_items.iter().enumerate() {
            println!("插入中医中药项目 #{}: {:?}", index + 1, item);
            let res = tx.execute(
                "INSERT INTO medical_prescription_items (
                    medical_record_id, medicine_name, grams, 
                    unit_price, total_price, created_at
                ) VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    record_id,
                    item.medicine_name,
                    item.grams,
                    item.unit_price,
                    item.total_price,
                    now
                ],
            );
            if let Err(e) = res {
                return Err(format!(
                    "插入中医中药项目失败 - 项目名称: '{}', 剂量: {}, 单价: {}, 总价: {}, 错误: {}",
                    item.medicine_name, item.grams, item.unit_price, item.total_price, e
                ));
            }
        }
    }

    tx.commit().map_err(|e| format!("提交事务失败: {}", e))?;

    Ok(ApiResponse {
        code: 200,
        msg: "医疗记录创建成功".to_string(),
        data: record_id,
    })
}

// 查询医疗记录列表 (统一接口)
#[command]
pub async fn get_unified_medical_record_list(
    params: QueryMedicalRecordsParams,
) -> Result<ApiResponse<PagedMedicalRecords>, String> {
    let db = get_database();
    let conn = db.get_connection();

    // 构建查询条件
    let mut where_clauses = vec!["1=1".to_string()];
    let mut query_params = Vec::new();

    if let Some(data_type) = &params.data_type {
        where_clauses.push("data_type = ?".to_string());
        query_params.push(data_type.clone());
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

    // 新增搜索参数
    if let Some(doctor_id) = params.doctor_id {
        where_clauses.push("doctor_id = ?".to_string());
        query_params.push(doctor_id.to_string());
    }

    if let Some(doctor_name) = &params.doctor_name {
        if !doctor_name.is_empty() {
            where_clauses.push("doctor_name LIKE ?".to_string());
            query_params.push(format!("%{}%", doctor_name));
        }
    }

    if let Some(chief_complaint) = &params.chief_complaint {
        if !chief_complaint.is_empty() {
            where_clauses.push("chief_complaint LIKE ?".to_string());
            query_params.push(format!("%{}%", chief_complaint));
        }
    }

    if let Some(illness_history) = &params.illness_history {
        if !illness_history.is_empty() {
            where_clauses.push("illness_history LIKE ?".to_string());
            query_params.push(format!("%{}%", illness_history));
        }
    }

    if let Some(physical_examination) = &params.physical_examination {
        if !physical_examination.is_empty() {
            where_clauses.push("physical_examination LIKE ?".to_string());
            query_params.push(format!("%{}%", physical_examination));
        }
    }

    if let Some(past_history) = &params.past_history {
        if !past_history.is_empty() {
            where_clauses.push("past_history LIKE ?".to_string());
            query_params.push(format!("%{}%", past_history));
        }
    }

    if let Some(laboratory_examination) = &params.laboratory_examination {
        if !laboratory_examination.is_empty() {
            where_clauses.push("laboratory_examination LIKE ?".to_string());
            query_params.push(format!("%{}%", laboratory_examination));
        }
    }

    if let Some(treatment_method) = &params.treatment_method {
        if !treatment_method.is_empty() {
            where_clauses.push("treatment_method LIKE ?".to_string());
            query_params.push(format!("%{}%", treatment_method));
        }
    }

    if let Some(doses) = params.doses {
        where_clauses.push("doses = ?".to_string());
        query_params.push(doses.to_string());
    }

    if let Some(min_amount) = params.min_amount {
        where_clauses.push("total_amount >= ?".to_string());
        query_params.push(min_amount.to_string());
    }

    if let Some(max_amount) = params.max_amount {
        where_clauses.push("total_amount <= ?".to_string());
        query_params.push(max_amount.to_string());
    }

    if let Some(note) = &params.note {
        if !note.is_empty() {
            where_clauses.push("note LIKE ?".to_string());
            query_params.push(format!("%{}%", note));
        }
    }

    let where_clause = where_clauses.join(" AND ");

    // 查询总数
    let count_sql = format!(
        "SELECT COUNT(*) FROM medical_records WHERE {}",
        where_clause
    );
    let total: i64 = conn
        .query_row(
            &count_sql,
            rusqlite::params_from_iter(query_params.iter()),
            |row| row.get(0),
        )
        .map_err(|e| format!("查询总数失败: {}", e))?;

    // 查询数据
    let offset = (params.current - 1) * params.size;
    let query_sql = format!(
        "SELECT id, data_type, customer_name, gender, age, telephone, time_of_visit, 
         trade_type, diagnosis, note, total_amount, doctor_id, doctor_name, created_at,
         updated_at, chief_complaint, illness_history, physical_examination, 
         past_history, laboratory_examination, treatment_method, doses, single_dose_amount
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
                    id: row.get(0)?,
                    data_type: row.get(1)?,
                    customer_name: row.get(2)?,
                    gender: row.get(3)?,
                    age: row.get(4)?,
                    telephone: row.get(5)?,
                    time_of_visit: row.get(6)?,
                    trade_type: row.get(7)?,
                    diagnosis: row.get(8)?,
                    note: row.get(9)?,
                    total_amount: row.get(10)?,
                    doctor_id: row.get(11)?,
                    doctor_name: row.get(12)?,
                    created_at: row.get(13)?,
                    updated_at: row.get(14)?,
                    chief_complaint: row.get(15)?,
                    illness_history: row.get(16)?,
                    physical_examination: row.get(17)?,
                    past_history: row.get(18)?,
                    laboratory_examination: row.get(19)?,
                    treatment_method: row.get(20)?,
                    doses: row.get(21)?,
                    single_dose_amount: row.get(22)?,
                    treatment_items: None,    // 延迟加载
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
                    if record.data_type == "customer" || record.data_type == "medical" {
                        println!("正在为医疗记录 ID: {} 加载治疗项目", id);
                        match load_treatment_items(&conn, id) {
                            Ok(items) => {
                                println!("成功加载到 {} 个治疗项目", items.len());
                                record.treatment_items = Some(items);
                            }
                            Err(e) => {
                                eprintln!("加载治疗项目失败: {}", e);
                                record.treatment_items = None;
                            }
                        }
                    }

                    // 加载中药项目
                    if record.data_type == "prescription" {
                        println!("正在为中医中药记录 ID: {} 加载处方项目", id);
                        match load_prescription_items(&conn, id) {
                            Ok(items) => {
                                println!("成功加载到 {} 个处方项目", items.len());
                                record.prescription_items = Some(items);
                            }
                            Err(e) => {
                                eprintln!("加载处方项目失败: {}", e);
                                record.prescription_items = None;
                            }
                        }
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
fn load_treatment_items(
    conn: &rusqlite::Connection,
    medical_record_id: i64,
) -> Result<Vec<TreatmentItemRecord>, rusqlite::Error> {
    println!("查询治疗项目，医疗记录ID: {}", medical_record_id);
    let mut stmt = conn.prepare(
        "SELECT id, medical_record_id, treatment_item, count, unit_price, total_price, created_at 
         FROM medical_treatment_items 
         WHERE medical_record_id = ? 
         ORDER BY id",
    )?;

    let items: Result<Vec<TreatmentItemRecord>, _> = stmt
        .query_map(params![medical_record_id], |row| {
            Ok(TreatmentItemRecord {
                id: Some(row.get(0)?),
                medical_record_id: row.get(1)?,
                treatment_item: row.get(2)?,
                count: row.get(3)?,
                unit_price: row.get(4)?,
                total_price: row.get(5)?,
                created_at: row.get(6)?,
            })
        })
        .and_then(|mapped_rows| mapped_rows.collect());

    items
}

// 加载中药项目
fn load_prescription_items(
    conn: &rusqlite::Connection,
    medical_record_id: i64,
) -> Result<Vec<PrescriptionItemRecord>, rusqlite::Error> {
    println!("查询中医中药项目，医疗记录ID: {}", medical_record_id);

    let mut stmt = conn.prepare(
        "SELECT id, medical_record_id, medicine_name, grams, unit_price, total_price, created_at 
         FROM medical_prescription_items 
         WHERE medical_record_id = ? 
         ORDER BY id",
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
pub async fn get_unified_medical_record_detail(
    id: i64,
) -> Result<ApiResponse<MedicalRecord>, String> {
    let db = get_database();
    let conn = db.get_connection();

    let query_sql = "SELECT id, data_type, customer_name, gender, age, telephone, time_of_visit, 
                     trade_type, diagnosis, note, total_amount, doctor_id, doctor_name, created_at, updated_at,
                     chief_complaint, illness_history, physical_examination, 
                     past_history, laboratory_examination, treatment_method, doses, single_dose_amount
                     FROM medical_records 
                     WHERE id = ?";

    let record_result = conn.query_row(query_sql, params![id], |row| {
        Ok(MedicalRecord {
            id: row.get(0)?,
            data_type: row.get(1)?,
            customer_name: row.get(2)?,
            gender: row.get(3)?,
            age: row.get(4)?,
            telephone: row.get(5)?,
            time_of_visit: row.get(6)?,
            trade_type: row.get(7)?,
            diagnosis: row.get(8)?,
            note: row.get(9)?,
            total_amount: row.get(10)?,
            doctor_id: row.get(11)?,
            doctor_name: row.get(12)?,
            created_at: row.get(13)?,
            updated_at: row.get(14)?,
            chief_complaint: row.get(15)?,
            illness_history: row.get(16)?,
            physical_examination: row.get(17)?,
            past_history: row.get(18)?,
            laboratory_examination: row.get(19)?,
            treatment_method: row.get(20)?,
            doses: row.get(21)?,
            single_dose_amount: row.get(22)?,
            treatment_items: None,
            prescription_items: None,
        })
    });

    match record_result {
        Ok(mut record) => {
            // 加载关联数据
            if record.data_type == "customer" || record.data_type == "medical" {
                record.treatment_items = load_treatment_items(&conn, id).ok();
            }

            if record.data_type == "prescription" {
                println!("详情查询 - 正在为中医中药记录 ID: {} 加载处方项目", id);
                match load_prescription_items(&conn, id) {
                    Ok(items) => {
                        println!("详情查询 - 成功加载到 {} 个处方项目", items.len());
                        record.prescription_items = Some(items);
                    }
                    Err(e) => {
                        eprintln!("详情查询 - 加载处方项目失败: {}", e);
                        record.prescription_items = None;
                    }
                }
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
pub async fn delete_unified_medical_record(id: i64) -> Result<ApiResponse<bool>, String> {
    let db = get_database();
    let conn = db.get_connection();

    // 开始事务
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("开始事务失败: {}", e))?;

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
    let result = tx.execute("DELETE FROM medical_records WHERE id = ?", params![id]);

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
pub async fn delete_unified_medical_records_batch(
    ids: Vec<i64>,
) -> Result<ApiResponse<i64>, String> {
    if ids.is_empty() {
        return Err("删除ID列表不能为空".to_string());
    }

    let db = get_database();
    let conn = db.get_connection();

    // 开始事务
    let tx = conn
        .unchecked_transaction()
        .map_err(|e| format!("开始事务失败: {}", e))?;

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
        match tx.execute("DELETE FROM medical_records WHERE id = ?", params![id]) {
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
