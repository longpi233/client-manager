/**
 * 门诊病历相关API
 */
import { invoke } from '@tauri-apps/api/core'

/** 获取门诊病历列表 */
export const getMedicalRecordList = (
  params: Api.MedicalRecord.SearchParams
): Promise<Api.Http.BaseResponse<Api.MedicalRecord.ListResponse>> => {
  return invoke('get_medical_record_list', { params })
}

/** 创建门诊病历 */
export const createMedicalRecord = (
  data: Api.MedicalRecord.CreateParams
): Promise<Api.Http.BaseResponse<Api.MedicalRecord.Info>> => {
  return invoke('create_medical_record', { data })
}

/** 获取门诊病历信息 */
export const getMedicalRecord = (
  id: number
): Promise<Api.Http.BaseResponse<Api.MedicalRecord.Info>> => {
  return invoke('get_medical_record', { id })
}

/** 更新门诊病历信息 */
export const updateMedicalRecord = (
  id: number,
  data: Api.MedicalRecord.CreateParams
): Promise<Api.Http.BaseResponse<Api.MedicalRecord.Info>> => {
  return invoke('update_medical_record', { id, data })
}

/** 删除门诊病历记录 */
export const deleteMedicalRecord = (id: number): Promise<Api.Http.BaseResponse<boolean>> => {
  return invoke('delete_medical_record', { id })
}

/** 批量删除门诊病历记录 */
export const batchDeleteMedicalRecords = (
  ids: number[]
): Promise<Api.Http.BaseResponse<number>> => {
  return invoke('batch_delete_medical_records', { ids })
}

/** 生成门诊病历PDF */
export const generateMedicalRecordPdf = (id: number): Promise<Api.Http.BaseResponse<string>> => {
  return invoke('generate_medical_record_pdf', { id })
}

/** 生成费用清单PDF */
export const generateFeeListPdf = (id: number): Promise<Api.Http.BaseResponse<string>> => {
  return invoke('generate_fee_list_pdf', { id })
}
