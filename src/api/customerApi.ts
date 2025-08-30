/**
 * 散客登记相关API
 */
import { invoke } from '@tauri-apps/api/core'

/** 获取散客列表 */
export const getCustomerList = (
  params: Api.Customer.SearchParams
): Promise<Api.Http.BaseResponse<Api.Customer.ListResponse>> => {
  return invoke('get_customer_list', { params })
}

/** 创建散客记录 */
export const createCustomer = (customer: any): Promise<Api.Http.BaseResponse<any>> => {
  return invoke('create_customer', { customer })
}

/** 获取散客信息 */
export const getCustomer = (id: number): Promise<Api.Http.BaseResponse<Api.Customer.Info>> => {
  return invoke('get_customer', { id })
}

/** 更新散客信息 */
export const updateCustomer = (
  id: number,
  data: Api.Customer.CreateParams
): Promise<Api.Http.BaseResponse<Api.Customer.Info>> => {
  return invoke('update_customer', { id, data })
}

/** 删除散客记录 */
export const deleteCustomer = (id: number): Promise<Api.Http.BaseResponse<boolean>> => {
  return invoke('delete_customer', { id })
}

/** 批量删除散客记录 */
export const batchDeleteCustomers = (ids: number[]): Promise<Api.Http.BaseResponse<number>> => {
  return invoke('batch_delete_customers', { ids })
}

/** 创建散客登记 */
export const createCustomerRegistration = (
  registration: any
): Promise<Api.Http.BaseResponse<any>> => {
  return invoke('create_customer_registration', { registration })
}

/** 获取散客登记列表 */
export const getCustomerRegistrations = (params: any): Promise<Api.Http.BaseResponse<any>> => {
  return invoke('get_customer_registrations', { params })
}

/** 获取散客的治疗项目 */
export const getCustomerTreatments = (customerId: number): Promise<Api.Http.BaseResponse<any>> => {
  return invoke('get_customer_treatments', { customerId })
}
