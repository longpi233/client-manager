use serde::{Deserialize, Serialize};
use tauri::command;
use std::collections::HashMap;
use std::sync::LazyLock;
use std::sync::Mutex;
use crate::modules::types::ApiResponse;
use crate::modules::utils::contains_pinyin;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionSearchParams {
    pub current: i32,
    pub size: i32,
    pub name: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InstitutionListResponse {
    pub records: Vec<Institution>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoctorSearchParams {
    pub current: i32,
    pub size: i32,
    pub name: Option<String>,
    pub speciality: Option<String>,
    pub institution_id: Option<i32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DoctorListResponse {
    pub records: Vec<Doctor>,
    pub current: i32,
    pub size: i32,
    pub total: i32,
}

// 模拟数据存储（实际项目中应该使用数据库）
static INSTITUTIONS: LazyLock<Mutex<HashMap<i32, Institution>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static DOCTORS: LazyLock<Mutex<HashMap<i32, Doctor>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));
static INSTITUTION_INFO: LazyLock<Mutex<Option<InstitutionInfo>>> =
    LazyLock::new(|| Mutex::new(None));
static NEXT_INSTITUTION_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));
static NEXT_DOCTOR_ID: LazyLock<Mutex<i32>> = LazyLock::new(|| Mutex::new(1));

#[command]
pub async fn get_institution_list(
    params: InstitutionSearchParams,
) -> Result<ApiResponse<InstitutionListResponse>, String> {
    let institutions = INSTITUTIONS.lock().unwrap();

    // 模拟搜索过滤，支持拼音查询
    let mut filtered_institutions: Vec<Institution> = institutions
        .values()
        .filter(|institution| {
            if let Some(ref name) = params.name {
                if !name.is_empty() && !contains_pinyin(&institution.name, name) {
                    return false;
                }
            }
            if let Some(ref status) = params.status {
                if !status.is_empty() && institution.status != *status {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // 排序
    filtered_institutions.sort_by(|a, b| b.id.cmp(&a.id));

    let total = filtered_institutions.len() as i32;
    let start = ((params.current - 1) * params.size) as usize;
    let end = std::cmp::min(start + params.size as usize, filtered_institutions.len());

    let records = if start < filtered_institutions.len() {
        filtered_institutions[start..end].to_vec()
    } else {
        Vec::new()
    };

    let response = InstitutionListResponse {
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
pub async fn create_institution(institution: Institution) -> Result<ApiResponse<Institution>, String> {
    let mut institutions = INSTITUTIONS.lock().unwrap();
    let mut next_id = NEXT_INSTITUTION_ID.lock().unwrap();

    let mut new_institution = institution;
    new_institution.id = Some(*next_id);
    new_institution.create_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    new_institution.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

    institutions.insert(*next_id, new_institution.clone());
    *next_id += 1;

    Ok(ApiResponse {
        code: 200,
        msg: "创建成功".to_string(),
        data: new_institution,
    })
}

#[command]
pub async fn update_institution(institution: Institution) -> Result<ApiResponse<Institution>, String> {
    let mut institutions = INSTITUTIONS.lock().unwrap();

    if let Some(id) = institution.id {
        if let Some(existing_institution) = institutions.get_mut(&id) {
            existing_institution.name = institution.name;
            existing_institution.address = institution.address;
            existing_institution.phone = institution.phone;
            existing_institution.email = institution.email;
            existing_institution.status = institution.status;
            existing_institution.description = institution.description;
            existing_institution.update_time =
                Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

            Ok(ApiResponse {
                code: 200,
                msg: "更新成功".to_string(),
                data: existing_institution.clone(),
            })
        } else {
            Err("机构不存在".to_string())
        }
    } else {
        Err("无效的ID".to_string())
    }
}

#[command]
pub async fn delete_institution(id: i32) -> Result<ApiResponse<bool>, String> {
    let mut institutions = INSTITUTIONS.lock().unwrap();

    if institutions.remove(&id).is_some() {
        Ok(ApiResponse {
            code: 200,
            msg: "删除成功".to_string(),
            data: true,
        })
    } else {
        Err("机构不存在".to_string())
    }
}

// 新增：获取机构信息
#[command]
pub async fn get_institution_info() -> Result<ApiResponse<InstitutionInfo>, String> {
    let info = INSTITUTION_INFO.lock().unwrap();
    
    match &*info {
        Some(institution_info) => {
            Ok(ApiResponse {
                code: 200,
                msg: "获取成功".to_string(),
                data: institution_info.clone(),
            })
        }
        None => {
            // 如果没有数据，返回默认数据
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
            
            Ok(ApiResponse {
                code: 200,
                msg: "获取成功".to_string(),
                data: default_info,
            })
        }
    }
}

// 新增：更新机构信息
#[command]
pub async fn update_institution_info(data: InstitutionUpdateParams) -> Result<ApiResponse<InstitutionInfo>, String> {
    let mut info = INSTITUTION_INFO.lock().unwrap();
    
    let updated_info = InstitutionInfo {
        id: Some(1),
        institution_name: data.institution_name,
        department_settings: data.department_settings,
        doctor_management: data.doctor_management,
        create_time: info.as_ref()
            .and_then(|i| i.create_time.clone())
            .or_else(|| Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string())),
        update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
    };
    
    *info = Some(updated_info.clone());
    
    Ok(ApiResponse {
        code: 200,
        msg: "更新成功".to_string(),
        data: updated_info,
    })
}

#[command]
pub async fn get_doctor_list(
    params: DoctorSearchParams,
) -> Result<ApiResponse<DoctorListResponse>, String> {
    let doctors = DOCTORS.lock().unwrap();

    // 模拟搜索过滤，支持拼音查询
    let mut filtered_doctors: Vec<Doctor> = doctors
        .values()
        .filter(|doctor| {
            if let Some(ref name) = params.name {
                if !name.is_empty() && !contains_pinyin(&doctor.name, name) {
                    return false;
                }
            }
            if let Some(ref speciality) = params.speciality {
                if !speciality.is_empty() && !contains_pinyin(&doctor.speciality, speciality) {
                    return false;
                }
            }
            if let Some(institution_id) = params.institution_id {
                if doctor.institution_id != institution_id {
                    return false;
                }
            }
            if let Some(ref status) = params.status {
                if !status.is_empty() && doctor.status != *status {
                    return false;
                }
            }
            true
        })
        .cloned()
        .collect();

    // 排序
    filtered_doctors.sort_by(|a, b| b.id.cmp(&a.id));

    let total = filtered_doctors.len() as i32;
    let start = ((params.current - 1) * params.size) as usize;
    let end = std::cmp::min(start + params.size as usize, filtered_doctors.len());

    let records = if start < filtered_doctors.len() {
        filtered_doctors[start..end].to_vec()
    } else {
        Vec::new()
    };

    let response = DoctorListResponse {
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
pub async fn create_doctor(doctor: Doctor) -> Result<ApiResponse<Doctor>, String> {
    let mut doctors = DOCTORS.lock().unwrap();
    let mut next_id = NEXT_DOCTOR_ID.lock().unwrap();

    let mut new_doctor = doctor;
    new_doctor.id = Some(*next_id);
    new_doctor.create_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());
    new_doctor.update_time = Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string());

    doctors.insert(*next_id, new_doctor.clone());
    *next_id += 1;

    Ok(ApiResponse {
        code: 200,
        msg: "创建成功".to_string(),
        data: new_doctor,
    })
}

// 初始化示例数据
pub fn init_sample_data() {
    let mut institutions = INSTITUTIONS.lock().unwrap();
    let mut doctors = DOCTORS.lock().unwrap();
    let mut next_institution_id = NEXT_INSTITUTION_ID.lock().unwrap();
    let mut next_doctor_id = NEXT_DOCTOR_ID.lock().unwrap();

    if institutions.is_empty() {
        let sample_institutions = vec![
            Institution {
                id: Some(1),
                name: "东城区中医院".to_string(),
                address: "北京市东城区安定门大街".to_string(),
                phone: "010-12345678".to_string(),
                email: Some("dczyy@163.com".to_string()),
                status: "1".to_string(),
                description: Some("专业中医诊疗机构".to_string()),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            Institution {
                id: Some(2),
                name: "西城区康复中心".to_string(),
                address: "北京市西城区复兴门大街".to_string(),
                phone: "010-23456789".to_string(),
                email: Some("xckfzx@126.com".to_string()),
                status: "1".to_string(),
                description: Some("专业康复治疗".to_string()),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
        ];

        for institution in sample_institutions {
            if let Some(id) = institution.id {
                institutions.insert(id, institution);
                if id >= *next_institution_id {
                    *next_institution_id = id + 1;
                }
            }
        }

        println!("已初始化 {} 条示例机构数据", institutions.len());
    }

    if doctors.is_empty() {
        let sample_doctors = vec![
            Doctor {
                id: Some(1),
                name: "李明".to_string(),
                phone: "13800138001".to_string(),
                email: Some("liming@163.com".to_string()),
                speciality: "中医内科".to_string(),
                title: "主任医师".to_string(),
                institution_id: 1,
                status: "1".to_string(),
                description: Some("从事中医内科临床工作20年".to_string()),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            Doctor {
                id: Some(2),
                name: "王华".to_string(),
                phone: "13800138002".to_string(),
                email: Some("wanghua@126.com".to_string()),
                speciality: "康复医学".to_string(),
                title: "副主任医师".to_string(),
                institution_id: 2,
                status: "1".to_string(),
                description: Some("专业康复治疗师".to_string()),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            Doctor {
                id: Some(3),
                name: "张丽".to_string(),
                phone: "13800138003".to_string(),
                email: Some("zhangli@qq.com".to_string()),
                speciality: "针灸推拿".to_string(),
                title: "主治医师".to_string(),
                institution_id: 1,
                status: "1".to_string(),
                description: Some("精通针灸推拿技术".to_string()),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
            Doctor {
                id: Some(4),
                name: "刘强".to_string(),
                phone: "13800138004".to_string(),
                email: Some("liuqiang@163.com".to_string()),
                speciality: "骨科".to_string(),
                title: "主任医师".to_string(),
                institution_id: 2,
                status: "2".to_string(), // 休假状态
                description: Some("骨科专家".to_string()),
                create_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
                update_time: Some(chrono::Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
        ];

        for doctor in sample_doctors {
            if let Some(id) = doctor.id {
                doctors.insert(id, doctor);
                if id >= *next_doctor_id {
                    *next_doctor_id = id + 1;
                }
            }
        }

        println!("已初始化 {} 条示例医生数据", doctors.len());
    }
}
