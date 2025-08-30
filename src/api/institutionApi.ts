/**
 * 机构信息相关API
 */
import { invoke } from '@tauri-apps/api/core'

/** 获取机构信息 */
export const getInstitutionInfo = (): Promise<Api.Http.BaseResponse<Api.Institution.Info>> => {
  return invoke('get_institution_info')
}

/** 更新机构信息 */
export const updateInstitutionInfo = (
  data: Api.Institution.UpdateParams
): Promise<Api.Http.BaseResponse<Api.Institution.Info>> => {
  return invoke('update_institution_info', { data })
}

/** 添加医师 */
export const addDoctor = (
  doctor: Omit<Api.Institution.Doctor, 'id'>
): Promise<Api.Http.BaseResponse<Api.Institution.Doctor>> => {
  return invoke('add_doctor', { doctor })
}

/** 更新医师信息 */
export const updateDoctor = (
  id: number,
  doctor: Partial<Api.Institution.Doctor>
): Promise<Api.Http.BaseResponse<Api.Institution.Doctor>> => {
  return invoke('update_doctor', { id, doctor })
}

/** 删除医师 */
export const deleteDoctor = (id: number): Promise<Api.Http.BaseResponse<boolean>> => {
  return invoke('delete_doctor', { id })
}

/** 获取医师列表 */
export const getDoctorList = (): Promise<Api.Http.BaseResponse<Api.Institution.Doctor[]>> => {
  return invoke('get_doctor_list')
}
