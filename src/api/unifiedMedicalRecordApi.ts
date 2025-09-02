import { invoke } from '@tauri-apps/api/core'

// 导入统一的医疗记录类型
import { RecordType } from '@/types/medical'
import type {
  UnifiedMedicalRecord,
  UnifiedMedicalRecordSearchParams,
  UnifiedMedicalRecordResponse,
  MedicalApiResponse
} from '@/types/medical'

export interface ApiResponse<T> {
  code: number
  msg: string
  data: T
}

export async function createUnifiedMedicalRecord(
  params: UnifiedMedicalRecord
): Promise<MedicalApiResponse<number>> {
  console.log('创建统一医疗记录:', params)
  return invoke('create_unified_medical_record', { params })
}

export async function getUnifiedMedicalRecordList(
  params: UnifiedMedicalRecordSearchParams
): Promise<MedicalApiResponse<UnifiedMedicalRecordResponse>> {
  console.log('查询统一医疗记录列表:', params)
  return invoke('get_unified_medical_record_list', { params })
}

export async function getUnifiedMedicalRecordDetail(
  id: number
): Promise<MedicalApiResponse<UnifiedMedicalRecord>> {
  console.log('获取统一医疗记录详情:', id)
  return invoke('get_unified_medical_record_detail', { id })
}

// 便捷方法
export function createCustomerRecord(
  params: Omit<UnifiedMedicalRecord, 'data_type'>
): Promise<MedicalApiResponse<number>> {
  return createUnifiedMedicalRecord({ ...params, data_type: RecordType.Customer })
}

export function createMedicalRecord(
  params: Omit<UnifiedMedicalRecord, 'data_type'>
): Promise<MedicalApiResponse<number>> {
  return createUnifiedMedicalRecord({ ...params, data_type: RecordType.Medical })
}

export function createPrescriptionRecord(
  params: Omit<UnifiedMedicalRecord, 'data_type'>
): Promise<MedicalApiResponse<number>> {
  return createUnifiedMedicalRecord({ ...params, data_type: RecordType.Prescription })
}

export function getCustomerRecordList(
  params: Omit<UnifiedMedicalRecordSearchParams, 'data_type'>
): Promise<MedicalApiResponse<UnifiedMedicalRecordResponse>> {
  return getUnifiedMedicalRecordList({ ...params, data_type: RecordType.Customer })
}

export function getMedicalRecordList(
  params: Omit<UnifiedMedicalRecordSearchParams, 'data_type'>
): Promise<MedicalApiResponse<UnifiedMedicalRecordResponse>> {
  return getUnifiedMedicalRecordList({ ...params, data_type: RecordType.Medical })
}

export function getPrescriptionRecordList(
  params: Omit<UnifiedMedicalRecordSearchParams, 'data_type'>
): Promise<MedicalApiResponse<UnifiedMedicalRecordResponse>> {
  return getUnifiedMedicalRecordList({ ...params, data_type: RecordType.Prescription })
}

// 删除医疗记录
export async function deleteUnifiedMedicalRecord(id: number): Promise<ApiResponse<boolean>> {
  console.log('删除统一医疗记录:', id)
  return invoke('delete_unified_medical_record', { id })
}

// 批量删除医疗记录
export async function deleteUnifiedMedicalRecordsBatch(
  ids: number[]
): Promise<ApiResponse<number>> {
  console.log('批量删除统一医疗记录:', ids)
  return invoke('delete_unified_medical_records_batch', { ids })
}
