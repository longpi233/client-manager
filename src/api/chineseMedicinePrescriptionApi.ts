import { invoke } from '@tauri-apps/api/core'

/**
 * 中药处方记录API
 * 使用 Tauri invoke 调用后端 Rust 函数
 */

/**
 * 中药处方项目
 */
export interface ChineseMedicinePrescriptionItem {
  medicine_name: string
  unit_price: number
  grams: number
  total_price: number
}

/**
 * 中药处方记录
 */
export interface ChineseMedicinePrescriptionRecord {
  id?: number
  customer_id: number
  customer_name: string
  gender: string
  age: number
  phone: string
  diagnosis: string
  prescription: ChineseMedicinePrescriptionItem[]
  doses: number
  single_dose_amount: number
  total_amount: number
  doctor_id: string
  doctor_name: string
  time_of_visit: string
  note?: string
  create_time?: string
  update_time?: string
}

/**
 * 中药处方搜索参数
 */
export interface ChineseMedicinePrescriptionSearchParams {
  current: number
  size: number
  customer_name?: string
  medicine_name?: string
  doctor_id?: string
  start_date?: string
  end_date?: string
}

/**
 * 中药处方列表响应
 */
export interface ChineseMedicinePrescriptionListResponse {
  records: ChineseMedicinePrescriptionRecord[]
  current: number
  size: number
  total: number
}

/**
 * 获取中药处方列表
 * @param params 搜索参数
 * @returns 中药处方列表响应
 */
export const getChineseMedicinePrescriptionList = async (
  params: ChineseMedicinePrescriptionSearchParams
): Promise<Api.Http.BaseResponse<ChineseMedicinePrescriptionListResponse>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<ChineseMedicinePrescriptionListResponse>>(
      'get_chinese_medicine_prescription_list',
      { params }
    )
    return response
  } catch (error) {
    console.error('获取中药处方列表失败:', error)
    throw error
  }
}

/**
 * 创建中药处方记录
 * @param record 处方记录数据
 * @returns 创建结果
 */
export const createChineseMedicinePrescription = async (
  record: ChineseMedicinePrescriptionRecord
): Promise<Api.Http.BaseResponse<ChineseMedicinePrescriptionRecord>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<ChineseMedicinePrescriptionRecord>>(
      'create_chinese_medicine_prescription',
      { record }
    )
    return response
  } catch (error) {
    console.error('创建中药处方记录失败:', error)
    throw error
  }
}

/**
 * 获取中药处方记录详情
 * @param id 记录ID
 * @returns 处方记录详情
 */
export const getChineseMedicinePrescription = async (
  id: number
): Promise<Api.Http.BaseResponse<ChineseMedicinePrescriptionRecord>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<ChineseMedicinePrescriptionRecord>>(
      'get_chinese_medicine_prescription',
      { id }
    )
    return response
  } catch (error) {
    console.error('获取中药处方记录失败:', error)
    throw error
  }
}

/**
 * 更新中药处方记录
 * @param record 处方记录数据
 * @returns 更新结果
 */
export const updateChineseMedicinePrescription = async (
  record: ChineseMedicinePrescriptionRecord
): Promise<Api.Http.BaseResponse<ChineseMedicinePrescriptionRecord>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<ChineseMedicinePrescriptionRecord>>(
      'update_chinese_medicine_prescription',
      { record }
    )
    return response
  } catch (error) {
    console.error('更新中药处方记录失败:', error)
    throw error
  }
}

/**
 * 删除中药处方记录
 * @param id 记录ID
 * @returns 删除结果
 */
export const deleteChineseMedicinePrescription = async (
  id: number
): Promise<Api.Http.BaseResponse<string>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<string>>(
      'delete_chinese_medicine_prescription',
      { id }
    )
    return response
  } catch (error) {
    console.error('删除中药处方记录失败:', error)
    throw error
  }
}
