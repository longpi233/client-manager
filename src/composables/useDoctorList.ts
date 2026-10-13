import { ref } from 'vue'
import { getInstitutionInfo } from '@/api/institutionApi'

export interface DoctorOption {
  id: number | string
  name: string
}

/**
 * 医师列表共享组合式函数
 * 提供医师列表加载、缓存与统一容错逻辑
 */
export function useDoctorList() {
  const doctorList = ref<DoctorOption[]>([])
  const doctorLoading = ref(false)

  const defaultDoctors: DoctorOption[] = [
    { id: 1, name: '主治医师A' },
    { id: 2, name: '主治医师B' }
  ]

  const loadDoctorList = async () => {
    doctorLoading.value = true
    try {
      const response = await getInstitutionInfo()
      if (response.code === 200 && response.data?.doctor_management?.length) {
        doctorList.value = response.data.doctor_management.map((doctor, index) => ({
          id: index + 1,
          name: doctor.name
        }))
      } else {
        doctorList.value = [...defaultDoctors]
      }
    } catch (error) {
      console.warn('获取机构医师列表失败，启用默认医师列表:', error)
      doctorList.value = [...defaultDoctors]
    } finally {
      doctorLoading.value = false
    }
  }

  return {
    doctorList,
    doctorLoading,
    loadDoctorList
  }
}
