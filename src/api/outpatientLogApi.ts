/**
 * 门诊日志相关API
 * 统一获取散客登记、门诊病历、中医中药的汇总数据
 */
import {
  getUnifiedMedicalRecordList,
  deleteUnifiedMedicalRecord,
  deleteUnifiedMedicalRecordsBatch
} from './unifiedMedicalRecordApi'

// 导入统一的医疗记录类型
import type {
  RecordType,
  UnifiedMedicalRecord,
  UnifiedMedicalRecordSearchParams,
  UnifiedMedicalRecordResponse
} from '@/types/medical'

/**
 * 获取门诊日志数据
 * 使用新的统一医疗记录API
 */
export const getOutpatientLogList = async (
  params: UnifiedMedicalRecordSearchParams
): Promise<{ code: number; data: UnifiedMedicalRecordResponse; message: string }> => {
  console.log('门诊日志API开始执行，参数:', params)
  try {
    const allData: UnifiedMedicalRecord[] = []
    // const allRecords: any[] = []

    try {
      console.log('正在获取数据...')
      const customerResponse = await getUnifiedMedicalRecordList({
        current: 1,
        size: 1000,
        data_type: params.data_type,
        start_date: params.start_date,
        end_date: params.end_date,
        customer_name: params.customer_name,
        trade_type: params.trade_type,
        telephone: params.telephone,
        diagnosis: params.diagnosis,
        gender: params.gender,
        min_age: params.min_age,
        max_age: params.max_age,
        doctor_id: params.doctor_id,
        doctor_name: params.doctor_name,
        chief_complaint: params.chief_complaint,
        illness_history: params.illness_history,
        physical_examination: params.physical_examination,
        past_history: params.past_history,
        laboratory_examination: params.laboratory_examination,
        treatment_method: params.treatment_method,
        doses: params.doses,
        min_amount: params.min_amount,
        max_amount: params.max_amount,
        note: params.note
      })
      console.log('获取数据响应:', customerResponse)
      if (customerResponse.code === 200 && customerResponse.data.records) {
        allData.push(...customerResponse.data.records)
      }
    } catch (error) {
      console.error('获取散客数据失败:', error)
    }

    // // 转换为统一格式
    // for (const record of allRecords) {
    //   const convertedItem = convertMedicalRecordToOutpatientLog(record)
    //   allData.push(convertedItem)
    // }
    console.log('转换后的门诊日志数据:', allData)

    // 按时间排序（降序）
    allData.sort(
      (a, b) => new Date(b.time_of_visit).getTime() - new Date(a.time_of_visit).getTime()
    )

    // 计算统计信息
    const statistics = calculateStatistics(allData)
    console.log('统计信息:', statistics)

    // 应用分页
    const startIndex = (params.current - 1) * params.size
    const endIndex = startIndex + params.size
    const records = allData.slice(startIndex, endIndex)

    console.log('门诊日志API执行完成，返回数据数量:', allData.length)
    return {
      code: 200,
      data: {
        records,
        total: allData.length,
        current: params.current,
        size: params.size,
        statistics
      },
      message: '查询成功'
    }
  } catch (error) {
    console.error('获取门诊日志失败:', error)
    return {
      code: 500,
      data: {
        records: [],
        total: 0,
        current: params.current,
        size: params.size,
        statistics: {
          total_count: 0,
          customer_count: 0,
          medical_count: 0,
          prescription_count: 0,
          total_amount: 0,
          cash_amount: 0,
          insurance_amount: 0,
          treatment_items_count: 0
        }
      },
      message: error instanceof Error ? error.message : '查询失败'
    }
  }
}

/**
 * 计算统计信息
 */
function calculateStatistics(data: UnifiedMedicalRecord[]) {
  const stats = {
    total_count: data.length,
    customer_count: 0,
    medical_count: 0,
    prescription_count: 0,
    total_amount: 0,
    cash_amount: 0,
    insurance_amount: 0,
    treatment_items_count: 0
  }

  data.forEach((item) => {
    // 统计数据类型
    if (item.data_type === 'customer') stats.customer_count++
    else if (item.data_type === 'medical') stats.medical_count++
    else if (item.data_type === 'prescription') stats.prescription_count++

    // 统计金额
    if (item.total_amount) {
      stats.total_amount += item.total_amount

      // 按结算类型分类统计
      if (item.trade_type === '1') {
        stats.cash_amount += item.total_amount
      } else {
        stats.insurance_amount += item.total_amount
      }
    }

    // 统计治疗项目数
    if (item.treatment_items) {
      stats.treatment_items_count += item.treatment_items.length
    }
  })

  return stats
}

/**
 * 获取门诊日志统计信息
 */
export const getOutpatientLogStatistics = async (params: {
  start_date?: string
  end_date?: string
  data_type?: RecordType
  trade_type?: string
}): Promise<{ code: number; data: any; message: string }> => {
  try {
    // 调用汇总接口获取所有数据
    const response = await getOutpatientLogList({
      current: 1,
      size: 1000, // 获取所有数据用于统计
      ...params
    })

    if (response.code === 200) {
      return {
        code: 200,
        data: response.data.statistics,
        message: '统计成功'
      }
    }

    return response
  } catch (error) {
    console.error('获取门诊日志统计失败:', error)
    return {
      code: 500,
      data: null,
      message: error instanceof Error ? error.message : '统计失败'
    }
  }
}

/**
 * 删除门诊日志记录
 */
export const deleteOutpatientLogItem = async (
  outpatientLogId: number
): Promise<{ code: number; message: string; data: boolean }> => {
  try {
    console.log('删除门诊日志记录:', outpatientLogId)

    const response = await deleteUnifiedMedicalRecord(outpatientLogId)
    return {
      code: response.code,
      message: response.msg || '删除成功',
      data: response.data
    }
  } catch (error) {
    console.error('删除门诊日志记录失败:', error)
    return {
      code: 500,
      message: error instanceof Error ? error.message : '删除失败',
      data: false
    }
  }
}

/**
 * 批量删除门诊日志记录
 */
export const deleteOutpatientLogItemsBatch = async (
  outpatientLogIds: number[]
): Promise<{ code: number; message: string; data: number }> => {
  try {
    console.log('批量删除门诊日志记录:', outpatientLogIds)

    const response = await deleteUnifiedMedicalRecordsBatch(outpatientLogIds)
    return {
      code: response.code,
      message: response.msg || '批量删除成功',
      data: response.data
    }
  } catch (error) {
    console.error('批量删除门诊日志记录失败:', error)
    return {
      code: 500,
      message: error instanceof Error ? error.message : '批量删除失败',
      data: 0
    }
  }
}
