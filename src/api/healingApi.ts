import { invoke } from '@tauri-apps/api/core'

/**
 * 康复项目管理API
 * 使用 Tauri invoke 调用后端 Rust 函数
 */

/**
 * 获取康复项目列表
 * @param params 搜索参数
 * @returns 康复项目列表响应
 */
export const getHealingList = async (
  params: Api.Healing.SearchParams
): Promise<Api.Http.BaseResponse<Api.Healing.ListResponse>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.Healing.ListResponse>>(
      'get_healing_list',
      { params }
    )
    return response
  } catch (error) {
    console.error('获取康复项目列表失败:', error)
    throw new Error(error as string)
  }
}

/**
 * 获取单个康复项目详情
 * @param id 项目ID
 * @returns 康复项目详情
 */
export const getHealingItem = async (
  id: number
): Promise<Api.Http.BaseResponse<Api.Healing.Item>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.Healing.Item>>('get_healing_item', {
      id
    })
    return response
  } catch (error) {
    console.error('获取康复项目详情失败:', error)
    throw new Error(error as string)
  }
}

/**
 * 创建康复项目
 * @param item 创建参数
 * @returns 创建的康复项目
 */
export const createHealingItem = async (
  item: Api.Healing.CreateParams
): Promise<Api.Http.BaseResponse<Api.Healing.Item>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.Healing.Item>>('create_healing_item', {
      item
    })
    return response
  } catch (error) {
    console.error('创建康复项目失败:', error)
    throw new Error(error as string)
  }
}

/**
 * 更新康复项目
 * @param item 更新参数
 * @returns 更新后的康复项目
 */
export const updateHealingItem = async (
  item: Api.Healing.UpdateParams
): Promise<Api.Http.BaseResponse<Api.Healing.Item>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<Api.Healing.Item>>('update_healing_item', {
      item
    })
    return response
  } catch (error) {
    console.error('更新康复项目失败:', error)
    throw new Error(error as string)
  }
}

/**
 * 删除康复项目
 * @param id 项目ID
 * @returns 删除结果
 */
export const deleteHealingItem = async (id: number): Promise<Api.Http.BaseResponse<boolean>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<boolean>>('delete_healing_item', {
      id
    })
    return response
  } catch (error) {
    console.error('删除康复项目失败:', error)
    throw new Error(error as string)
  }
}

/**
 * 批量删除康复项目
 * @param ids 项目ID数组
 * @returns 删除的项目数量
 */
export const batchDeleteHealingItems = async (
  ids: number[]
): Promise<Api.Http.BaseResponse<number>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<number>>('batch_delete_healing_items', {
      ids
    })
    return response
  } catch (error) {
    console.error('批量删除康复项目失败:', error)
    throw new Error(error as string)
  }
}

/**
 * 获取康复项目统计信息
 * @returns 统计信息
 */
export const getHealingStatistics = async (): Promise<
  Api.Http.BaseResponse<Api.Healing.Statistics>
> => {
  try {
    const response =
      await invoke<Api.Http.BaseResponse<Api.Healing.Statistics>>('get_healing_statistics')
    return response
  } catch (error) {
    console.error('获取康复项目统计失败:', error)
    throw new Error(error as string)
  }
}

/**
 * 批量更新康复项目状态
 * @param ids 项目ID数组
 * @param status 新状态
 * @returns 更新的项目数量
 */
export const batchUpdateStatus = async (
  ids: number[],
  status: Api.Healing.Status
): Promise<Api.Http.BaseResponse<number>> => {
  try {
    const response = await invoke<Api.Http.BaseResponse<number>>('batch_update_healing_status', {
      ids,
      status
    })
    return response
  } catch (error) {
    console.error('批量更新状态失败:', error)
    throw new Error(error as string)
  }
}

// 导出康复项目服务对象（可选，方便统一管理）
export const HealingService = {
  getList: getHealingList,
  getItem: getHealingItem,
  create: createHealingItem,
  update: updateHealingItem,
  delete: deleteHealingItem,
  batchDelete: batchDeleteHealingItems,
  getStatistics: getHealingStatistics,
  batchUpdateStatus
} as const
