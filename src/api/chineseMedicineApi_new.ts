import { invoke } from '@tauri-apps/api/core'

/**
 * 中药管理API
 * 使用 Tauri invoke 调用后端 Rust 函数
 */

/**
 * 获取中药列表
 */
export const getChineseMedicineList = async (
  params: Api.ChineseMedicine.SearchParams
): Promise<Api.Http.BaseResponse<Api.ChineseMedicine.ListResponse>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.ChineseMedicine.ListResponse>>(
      'get_chinese_medicine_list',
      { params }
    )
    return response
  } catch (error) {
    console.error('获取中药列表失败:', error)
    throw error
  }
}

/**
 * 创建中药项目
 */
export const createChineseMedicineItem = async (
  data: Api.ChineseMedicine.CreateParams
): Promise<Api.Http.BaseResponse<Api.ChineseMedicine.Item>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.ChineseMedicine.Item>>(
      'create_chinese_medicine_item',
      { item: data }
    )
    return response
  } catch (error) {
    console.error('创建中药项目失败:', error)
    throw error
  }
}

/**
 * 更新中药项目
 */
export const updateChineseMedicineItem = async (
  data: Api.ChineseMedicine.UpdateParams
): Promise<Api.Http.BaseResponse<Api.ChineseMedicine.Item>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.ChineseMedicine.Item>>(
      'update_chinese_medicine_item',
      { item: data }
    )
    return response
  } catch (error) {
    console.error('更新中药项目失败:', error)
    throw error
  }
}

/**
 * 删除中药项目
 */
export const deleteChineseMedicineItem = async (
  id: number
): Promise<Api.Http.BaseResponse<boolean>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<boolean>>('delete_chinese_medicine_item', {
      id
    })
    return response
  } catch (error) {
    console.error('删除中药项目失败:', error)
    throw error
  }
}

/**
 * 批量删除中药项目
 */
export const batchDeleteChineseMedicineItems = async (
  ids: number[]
): Promise<Api.Http.BaseResponse<number>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<number>>(
      'batch_delete_chinese_medicine_items',
      { ids }
    )
    return response
  } catch (error) {
    console.error('批量删除中药项目失败:', error)
    throw error
  }
}

/**
 * 获取中药项目详情
 */
export const getChineseMedicineItem = async (
  id: number
): Promise<Api.Http.BaseResponse<Api.ChineseMedicine.Item>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.ChineseMedicine.Item>>(
      'get_chinese_medicine_item',
      { id }
    )
    return response
  } catch (error) {
    console.error('获取中药项目详情失败:', error)
    throw error
  }
}

/**
 * 获取中药统计信息
 */
export const getChineseMedicineStatistics = async (): Promise<
  Api.Http.BaseResponse<Api.ChineseMedicine.Statistics>
> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.ChineseMedicine.Statistics>>(
      'get_chinese_medicine_statistics'
    )
    return response
  } catch (error) {
    console.error('获取中药统计信息失败:', error)
    throw error
  }
}

/**
 * 批量更新中药状态
 */
export const batchUpdateChineseMedicineStatus = async (
  ids: number[],
  status: string
): Promise<Api.Http.BaseResponse<number>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<number>>(
      'batch_update_chinese_medicine_status',
      { ids, status }
    )
    return response
  } catch (error) {
    console.error('批量更新中药状态失败:', error)
    throw error
  }
}
