use serde::{Deserialize, Serialize};
use tauri::command;
use rusqlite::params;
use crate::modules::types::ApiResponse;
use crate::modules::database::get_database;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Institution {
    pub id: Option<i32>,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub email: Option<String>,
    pub status: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Doctor {
    pub id: Option<i32>,
    pub name: String,
    pub phone: String,
    pub email: Option<String>,
    pub speciality: String,
    pub title: String,
    pub institution_id: i32,
    pub status: String,
    pub description: Option<String>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

// 机构信息页面专用的数据结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InstitutionInfo {
    pub id: Option<i32>,
    pub institution_name: String,
    pub department_settings: String,
    pub doctor_management: Vec<DoctorBasic>,
    pub create_time: Option<String>,
    pub update_time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoctorBasic {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionUpdateParams {
    pub institution_name: String,
    pub department_settings: String,
    pub doctor_management: Vec<DoctorBasic>,
}

// 获取机构信息
#[command]
pub async fn get_institution_info() -> Result<ApiResponse<InstitutionInfo>, String> {
    let db = get_database();
    let conn = db.get_connection();
    
    let query = "SELECT id, name, description, doctor_management, created_at, updated_at 
                 FROM institutions WHERE id = 1 LIMIT 1";
    
    match conn.query_row(query, [], |row| {
        let doctor_management_json: String = row.get("doctor_management").unwrap_or_default();
        let doctor_management: Vec<DoctorBasic> = if doctor_management_json.is_empty() {
            vec![
                DoctorBasic { name: "李明".to_string() },
                DoctorBasic { name: "张丽".to_string() },
            ]
        } else {
            serde_json::from_str(&doctor_management_json).unwrap_or_else(|_| vec![
                DoctorBasic { name: "李明".to_string() },
                DoctorBasic { name: "张丽".to_string() },
            ])
        };
        
        Ok(InstitutionInfo {
            id: Some(row.get("id")?),
            institution_name: row.get("name")?,
            department_settings: row.get("description").unwrap_or_else(|_| "中医科、针灸科、推拿科".to_string()),
            doctor_management,
            create_time: row.get("created_at").ok(),
            update_time: row.get("updated_at").ok(),
        })
    }) {
        Ok(institution_info) => {
            Ok(ApiResponse {
                code: 200,
                msg: "获取成功".to_string(),
                data: institution_info,
            })
        }
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            // 如果没有数据，创建默认数据
            let default_info = InstitutionInfo {
                id: Some(1),
                institution_name: "默认健康诊所".to_string(),
                department_settings: "中医科、针灸科、推拿科".to_string(),
                doctor_management: vec![
                    DoctorBasic { name: "李明".to_string() },
                    DoctorBasic { name: "张丽".to_string() },
                ],
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            };
            
            // 插入默认数据到数据库
            let doctor_management_json = serde_json::to_string(&default_info.doctor_management)
                .unwrap_or_default();
            let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
            
            let insert_query = "INSERT INTO institutions 
                               (id, name, description, doctor_management, created_at, updated_at) 
                               VALUES (1, ?, ?, ?, ?, ?)";
            
            match conn.execute(insert_query, params![
                &default_info.institution_name,
                &default_info.department_settings,
                &doctor_management_json,
                &now,
                &now
            ]) {
                Ok(_) => {
                    Ok(ApiResponse {
                        code: 200,
                        msg: "获取成功".to_string(),
                        data: default_info,
                    })
                }
                Err(e) => Err(format!("创建默认机构信息失败: {}", e))
            }
        }
        Err(e) => Err(format!("获取机构信息失败: {}", e))
    }
}

// 更新机构信息
#[command]
pub async fn update_institution_info(data: InstitutionUpdateParams) -> Result<ApiResponse<InstitutionInfo>, String> {
    let db = get_database();
    let conn = db.get_connection();
    
    let doctor_management_json = serde_json::to_string(&data.doctor_management)
        .map_err(|e| format!("序列化医师信息失败: {}", e))?;
    let now = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
    
    // 检查是否存在记录
    let check_query = "SELECT COUNT(*) FROM institutions WHERE id = 1";
    let exists: i32 = conn.query_row(check_query, [], |row| row.get(0))
        .map_err(|e| format!("检查记录失败: {}", e))?;
    
    if exists > 0 {
        // 更新现有记录
        let update_query = "UPDATE institutions 
                           SET name = ?, description = ?, doctor_management = ?, updated_at = ? 
                           WHERE id = 1";
        
        conn.execute(update_query, params![
            &data.institution_name,
            &data.department_settings,
            &doctor_management_json,
            &now
        ]).map_err(|e| format!("更新机构信息失败: {}", e))?;
    } else {
        // 插入新记录
        let insert_query = "INSERT INTO institutions 
                           (id, name, description, doctor_management, created_at, updated_at) 
                           VALUES (1, ?, ?, ?, ?, ?)";
        
        conn.execute(insert_query, params![
            &data.institution_name,
            &data.department_settings,
            &doctor_management_json,
            &now,
            &now
        ]).map_err(|e| format!("插入机构信息失败: {}", e))?;
    }
    
    // 获取更新后的数据
    let get_query = "SELECT created_at FROM institutions WHERE id = 1";
    let create_time: Option<String> = conn.query_row(get_query, [], |row| row.get("created_at")).ok();
    
    let updated_info = InstitutionInfo {
        id: Some(1),
        institution_name: data.institution_name,
        department_settings: data.department_settings,
        doctor_management: data.doctor_management,
        create_time,
        update_time: Some(now),
    };
    
    Ok(ApiResponse {
        code: 200,
        msg: "更新成功".to_string(),
        data: updated_info,
    })
}