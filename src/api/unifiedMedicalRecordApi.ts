import { invoke } from '@tauri-apps/api/core'

export enum RecordType {
  Customer = 'customer',
  Medical = 'medical',
  Prescription = 'prescription'
}

export interface MedicalRecord {
  id?: number
  record_type: string
  customer_name: string
  gender: string
  age: number
  telephone?: string
  time_of_visit: string
  trade_type: string
  diagnosis: string
  note?: string
  chief_complaint?: string
  illness_history?: string
  physical_examination?: string
  past_history?: string
  laboratory_examination?: string
  treatment_method?: string
  doses?: number
  single_dose_amount?: number
  total_amount: number
  doctor_id?: number
  doctor_name?: string
  status: string
  created_at: string
  updated_at: string
  treatment_items?: TreatmentItemRecord[]
  prescription_items?: PrescriptionItemRecord[]
}

export interface TreatmentItemRecord {
  id?: number
  medical_record_id: number
  treatment_item_name: string
  treatment_count: number
  unit_price: number
  total_price: number
  created_at: string
}

export interface PrescriptionItemRecord {
  id?: number
  medical_record_id: number
  medicine_name: string
  grams: number
  unit_price: number
  total_price: number
  created_at: string
}

export interface TreatmentFormItem {
  treatment_item: string
  count: number
  unit_price?: number
  total_price?: number
}

export interface PrescriptionFormItem {
  medicine_name: string
  grams: number
  unit_price?: number // 单价（以分为单位）
  total_price?: number // 总价（以分为单位）
}

export interface CreateMedicalRecordParams {
  record_type: string
  customer_name: string
  gender: string
  age: number
  telephone?: string
  time_of_visit: string
  trade_type: string
  diagnosis: string
  note?: string
  chief_complaint?: string
  illness_history?: string
  physical_examination?: string
  past_history?: string
  laboratory_examination?: string
  treatment_method?: string
  doses?: number
  single_dose_amount?: number // 单付金额（以分为单位）
  total_amount?: number // 总金额（以分为单位）
  treatment_items?: TreatmentFormItem[]
  prescription_items?: PrescriptionFormItem[]
  doctor_id?: number
  doctor_name?: string
}

export interface QueryMedicalRecordsParams {
  current: number
  size: number
  record_type?: string
  customer_name?: string
  telephone?: string
  diagnosis?: string
  start_date?: string
  end_date?: string
  gender?: string
  min_age?: number
  max_age?: number
  trade_type?: string
}

export interface PagedMedicalRecords {
  records: MedicalRecord[]
  total: number
  current: number
  size: number
}

export interface ApiResponse<T> {
  code: number
  msg: string
  data: T
}

export async function createUnifiedMedicalRecord(
  params: CreateMedicalRecordParams
): Promise<ApiResponse<number>> {
  console.log('创建统一医疗记录:', params)
  return invoke('create_unified_medical_record', { params })
}

export async function getUnifiedMedicalRecordList(
  params: QueryMedicalRecordsParams
): Promise<ApiResponse<PagedMedicalRecords>> {
  console.log('查询统一医疗记录列表:', params)
  return invoke('get_unified_medical_record_list', { params })
}

export async function getUnifiedMedicalRecordDetail(
  id: number
): Promise<ApiResponse<MedicalRecord>> {
  console.log('获取统一医疗记录详情:', id)
  return invoke('get_unified_medical_record_detail', { id })
}

// 便捷方法
export function createCustomerRecord(
  params: Omit<CreateMedicalRecordParams, 'record_type'>
): Promise<ApiResponse<number>> {
  return createUnifiedMedicalRecord({ ...params, record_type: RecordType.Customer })
}

export function createMedicalRecordLegacy(
  params: Omit<CreateMedicalRecordParams, 'record_type'>
): Promise<ApiResponse<number>> {
  return createUnifiedMedicalRecord({ ...params, record_type: RecordType.Medical })
}

export function createPrescriptionRecord(
  params: Omit<CreateMedicalRecordParams, 'record_type'>
): Promise<ApiResponse<number>> {
  return createUnifiedMedicalRecord({ ...params, record_type: RecordType.Prescription })
}

export function getCustomerRecordList(
  params: Omit<QueryMedicalRecordsParams, 'record_type'>
): Promise<ApiResponse<PagedMedicalRecords>> {
  return getUnifiedMedicalRecordList({ ...params, record_type: RecordType.Customer })
}

export function getMedicalRecordListLegacy(
  params: Omit<QueryMedicalRecordsParams, 'record_type'>
): Promise<ApiResponse<PagedMedicalRecords>> {
  return getUnifiedMedicalRecordList({ ...params, record_type: RecordType.Medical })
}

export function getPrescriptionRecordList(
  params: Omit<QueryMedicalRecordsParams, 'record_type'>
): Promise<ApiResponse<PagedMedicalRecords>> {
  return getUnifiedMedicalRecordList({ ...params, record_type: RecordType.Prescription })
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
