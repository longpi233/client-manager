pub mod types;
pub mod utils;
pub mod medical_records;
pub mod healing_items;
pub mod institution;
pub mod customer;
pub mod chinese_medicine;
pub mod chinese_medicine_prescriptions;

// 重新导出模块中的公共函数和类型
pub use types::*;
pub use utils::*;

// 有选择地导出命令函数以避免冲突
pub use medical_records::{
    get_medical_record_list, create_medical_record, get_medical_record,
    update_medical_record, delete_medical_record, generate_medical_record_pdf,
    generate_fee_list_pdf
};

pub use healing_items::{
    get_healing_list, create_healing_item, update_healing_item,
    delete_healing_item, batch_delete_healing_items, get_healing_item,
    get_healing_statistics, batch_update_healing_status
};

pub use institution::{
    get_institution_list, create_institution, update_institution,
    delete_institution, get_doctor_list, create_doctor,
    get_institution_info, update_institution_info
};

pub use customer::{
    get_customer_list, create_customer, update_customer, delete_customer,
    create_customer_registration, get_customer_registrations, get_customer_treatments, get_customer_statistics
};

pub use chinese_medicine::{
    get_chinese_medicine_list, create_chinese_medicine_item, update_chinese_medicine_item,
    delete_chinese_medicine_item, batch_delete_chinese_medicine_items, get_chinese_medicine_item,
    get_chinese_medicine_statistics, batch_update_chinese_medicine_status
};

pub use chinese_medicine_prescriptions::{
    get_chinese_medicine_prescription_list, create_chinese_medicine_prescription,
    get_chinese_medicine_prescription, update_chinese_medicine_prescription,
    delete_chinese_medicine_prescription
};
