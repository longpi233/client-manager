/**
 * 门诊日志相关API
 * 统一获取散客登记、门诊病历、中药处方的汇总数据
 */
import {
  getUnifiedMedicalRecordList,
  deleteUnifiedMedicalRecord,
  deleteUnifiedMedicalRecordsBatch
} from './unifiedMedicalRecordApi'

/**
 * 门诊日志查询参数
 */
export interface OutpatientLogSearchParams {
  /** 当前页 */
  current: number
  /** 每页大小 */
  size: number
  /** 开始日期 */
  start_date?: string
  /** 结束日期 */
  end_date?: string
  /** 数据类型过滤 */
  data_type?: 'customer' | 'medical' | 'prescription'
  /** 结算类型过滤 */
  trade_type?: string
  /** 患者姓名 */
  customer_name?: string
}

/**
 * 统一的门诊日志数据项
 */
export interface OutpatientLogItem {
  /** 唯一标识 */
  id: string
  /** 数据类型 */
  data_type: 'customer' | 'medical' | 'prescription'
  /** 患者姓名 */
  customer_name: string
  /** 性别 */
  gender: string
  /** 年龄 */
  age: number
  /** 联系电话 */
  phone?: string
  telephone?: string
  /** 就诊时间 */
  time_of_visit: string
  /** 结算类型 */
  trade_type: string
  /** 诊断信息 */
  diagnosis: string
  /** 治疗项目 */
  treatment_items?: any[]
  /** 治疗项目摘要 */
  treatment_items_summary?: string
  /** 金额 */
  amount?: number
  /** 备注 */
  note?: string
  /** 原始数据 */
  raw_data: any
  /** 处方特有字段 */
  prescription?: any[]
  doses?: number
  single_dose_amount?: number
  total_amount?: number
  /** 门诊病历特有字段 */
  chief_complaint?: string
  illness_history?: string
  physical_examination?: string
  past_history?: string
  laboratory_examination?: string
  treatment_method?: string
  doctor_id?: string
  doctor_name?: string
}

/**
 * 门诊日志响应数据
 */
export interface OutpatientLogResponse {
  /** 数据列表 */
  records: OutpatientLogItem[]
  /** 总数 */
  total: number
  /** 当前页 */
  current: number
  /** 每页大小 */
  size: number
  /** 统计信息 */
  statistics: {
    total_count: number
    customer_count: number
    medical_count: number
    prescription_count: number
    total_amount: number
    cash_amount: number
    insurance_amount: number
    treatment_items_count: number
  }
}

/**
 * 获取门诊日志数据
 * 使用新的统一医疗记录API
 */
export const getOutpatientLogList = async (
  params: OutpatientLogSearchParams
): Promise<{ code: number; data: OutpatientLogResponse; message: string }> => {
  console.log('门诊日志API开始执行，参数:', params)
  try {
    const allData: OutpatientLogItem[] = []
    const allRecords: any[] = []

    // 获取散客登记数据
    if (!params.data_type || params.data_type === 'customer') {
      try {
        console.log('正在获取散客登记数据...')
        const customerResponse = await getUnifiedMedicalRecordList({
          current: 1,
          size: 1000,
          record_type: 'customer',
          start_date: params.start_date,
          end_date: params.end_date,
          customer_name: params.customer_name,
          trade_type: params.trade_type
        })
        console.log('散客登记数据响应:', customerResponse)
        if (customerResponse.code === 200 && customerResponse.data.records) {
          allRecords.push(
            ...customerResponse.data.records.map((record) => ({ ...record, data_type: 'customer' }))
          )
        }
      } catch (error) {
        console.error('获取散客数据失败:', error)
      }
    }

    // 获取门诊病历数据
    if (!params.data_type || params.data_type === 'medical') {
      try {
        const medicalResponse = await getUnifiedMedicalRecordList({
          current: 1,
          size: 1000,
          record_type: 'medical',
          start_date: params.start_date,
          end_date: params.end_date,
          customer_name: params.customer_name,
          trade_type: params.trade_type
        })
        if (medicalResponse.code === 200 && medicalResponse.data.records) {
          allRecords.push(
            ...medicalResponse.data.records.map((record) => ({ ...record, data_type: 'medical' }))
          )
        }
      } catch (error) {
        console.error('获取门诊病历数据失败:', error)
      }
    }

    // 获取中药处方数据
    if (!params.data_type || params.data_type === 'prescription') {
      try {
        const prescriptionResponse = await getUnifiedMedicalRecordList({
          current: 1,
          size: 1000,
          record_type: 'prescription',
          start_date: params.start_date,
          end_date: params.end_date,
          customer_name: params.customer_name,
          trade_type: params.trade_type
        })
        if (prescriptionResponse.code === 200 && prescriptionResponse.data.records) {
          allRecords.push(
            ...prescriptionResponse.data.records.map((record) => ({
              ...record,
              data_type: 'prescription'
            }))
          )
        }
      } catch (error) {
        console.error('获取中药处方数据失败:', error)
      }
    }

    // 转换为统一格式
    for (const record of allRecords) {
      const convertedItem = convertMedicalRecordToOutpatientLog(record)
      allData.push(convertedItem)
    }
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
 * 将统一医疗记录转换为门诊日志格式
 */
function convertMedicalRecordToOutpatientLog(record: any): OutpatientLogItem {
  const baseItem: OutpatientLogItem = {
    id: `${record.data_type}_${record.id}`,
    data_type: record.data_type,
    customer_name: record.customer_name,
    gender: record.gender,
    age: record.age,
    telephone: record.telephone,
    time_of_visit: record.time_of_visit,
    trade_type: record.trade_type,
    diagnosis: record.diagnosis,
    note: record.note,
    raw_data: record
  }

  // 处理治疗项目
  if (record.treatment_items && record.treatment_items.length > 0) {
    baseItem.treatment_items = record.treatment_items.map((item: any) => ({
      treatment_item: item.treatment_item_name,
      count: item.treatment_count,
      unit_price: item.unit_price,
      total_price: item.total_price
    }))

    // 生成治疗项目摘要
    baseItem.treatment_items_summary = record.treatment_items
      .map((item: any) => `${item.treatment_item_name}×${item.treatment_count}`)
      .join(', ')

    // 计算总金额
    baseItem.amount = record.treatment_items.reduce(
      (sum: number, item: any) => sum + (item.total_price || 0),
      0
    )
  }

  // 处理中药处方项目
  if (record.prescription_items && record.prescription_items.length > 0) {
    baseItem.prescription = record.prescription_items.map((item: any) => ({
      medicine_name: item.medicine_name,
      grams: item.grams,
      unit_price: item.unit_price,
      total_price: item.total_price
    }))

    // 生成治疗项目摘要（对于中药处方）
    baseItem.treatment_items_summary = record.prescription_items
      .map((item: any) => `${item.medicine_name} ${item.grams}g`)
      .join(', ')

    // 设置处方特有字段
    baseItem.doses = record.doses
    baseItem.single_dose_amount = record.single_dose_amount
    baseItem.total_amount = record.total_amount
    baseItem.amount = record.total_amount
  }

  // 设置门诊病历特有字段
  if (record.data_type === 'medical') {
    baseItem.chief_complaint = record.chief_complaint
    baseItem.illness_history = record.illness_history
    baseItem.physical_examination = record.physical_examination
    baseItem.past_history = record.past_history
    baseItem.laboratory_examination = record.laboratory_examination
    baseItem.treatment_method = record.treatment_method
  }

  // 设置医师信息
  baseItem.doctor_id = record.doctor_id
  baseItem.doctor_name = record.doctor_name

  return baseItem
}

/**
 * 计算统计信息
 */
function calculateStatistics(data: OutpatientLogItem[]) {
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
    if (item.amount) {
      stats.total_amount += item.amount

      // 按结算类型分类统计
      if (item.trade_type === '1') {
        stats.cash_amount += item.amount
      } else {
        stats.insurance_amount += item.amount
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
  data_type?: 'customer' | 'medical' | 'prescription'
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
 * 从门诊日志ID中提取数字ID
 * 门诊日志ID格式：{data_type}_{id}，如 "medical_5"
 */
function extractIdFromOutpatientLogId(outpatientLogId: string): number {
  const parts = outpatientLogId.split('_')
  if (parts.length !== 2) {
    throw new Error(`无效的门诊日志ID格式: ${outpatientLogId}`)
  }
  const numericId = parseInt(parts[1], 10)
  if (isNaN(numericId)) {
    throw new Error(`无法解析门诊日志ID中的数字部分: ${outpatientLogId}`)
  }
  return numericId
}

/**
 * 删除门诊日志记录
 */
export const deleteOutpatientLogItem = async (
  outpatientLogId: string
): Promise<{ code: number; message: string; data: boolean }> => {
  try {
    console.log('删除门诊日志记录:', outpatientLogId)

    // 从门诊日志ID中提取数字ID
    const numericId = extractIdFromOutpatientLogId(outpatientLogId)
    console.log('提取的数字ID:', numericId)

    const response = await deleteUnifiedMedicalRecord(numericId)
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
  outpatientLogIds: string[]
): Promise<{ code: number; message: string; data: number }> => {
  try {
    console.log('批量删除门诊日志记录:', outpatientLogIds)

    // 从门诊日志ID中提取数字ID
    const numericIds = outpatientLogIds.map((id) => {
      const numericId = extractIdFromOutpatientLogId(id)
      console.log(`提取ID: ${id} -> ${numericId}`)
      return numericId
    })
    console.log('提取的数字ID列表:', numericIds)

    const response = await deleteUnifiedMedicalRecordsBatch(numericIds)
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
