// 模块声明
pub mod modules;

// 重新导出所有模块的公共接口
pub use modules::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化示例数据
    modules::medical_records::init_sample_data();
    modules::healing_items::init_sample_data();
    modules::institution::init_sample_data();
    modules::customer::init_sample_data();
    modules::chinese_medicine::init_sample_data();
    modules::chinese_medicine_prescriptions::init_sample_data();

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // 康养项目管理
            get_healing_list,
            create_healing_item,
            update_healing_item,
            delete_healing_item,
            batch_delete_healing_items,
            get_healing_item,
            get_healing_statistics,
            batch_update_healing_status,
            // 机构管理
            get_institution_list,
            create_institution,
            update_institution,
            delete_institution,
            get_doctor_list,
            create_doctor,
            get_institution_info,
            update_institution_info,
            // 客户管理
            get_customer_list,
            create_customer,
            update_customer,
            delete_customer,
            create_customer_registration,
            get_customer_registrations,
            get_customer_treatments,
            get_customer_statistics,
            // 医疗记录管理
            get_medical_record_list,
            create_medical_record,
            get_medical_record,
            update_medical_record,
            delete_medical_record,
            generate_medical_record_pdf,
            generate_fee_list_pdf,
            // 中药管理
            get_chinese_medicine_list,
            create_chinese_medicine_item,
            update_chinese_medicine_item,
            delete_chinese_medicine_item,
            batch_delete_chinese_medicine_items,
            get_chinese_medicine_item,
            get_chinese_medicine_statistics,
            batch_update_chinese_medicine_status,
            // 中药处方管理
            get_chinese_medicine_prescription_list,
            create_chinese_medicine_prescription,
            get_chinese_medicine_prescription,
            update_chinese_medicine_prescription,
            delete_chinese_medicine_prescription
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
