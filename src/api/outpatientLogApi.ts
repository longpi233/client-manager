/**
 * 门诊日志相关API
 * 统一获取散客登记、门诊病历、中药处方的汇总数据
 */
import {
  getUnifiedMedicalRecordList,
  deleteUnifiedMedicalRecord,
  deleteUnifiedMedicalRecordsBatch,
  MedicalRecord,
  RecordType
} from './unifiedMedicalRecordApi'

/**
 * 治疗项目接口
 */
export interface TreatmentItem {
  /** 治疗项目名称 */
  treatment_item: string
  /** 数量/次数 */
  count: number
  /** 单价（万分为单位） */
  unit_price: number
  /** 总价（万分为单位） */
  total_price: number
}

/**
 * 处方项目接口
 */
export interface PrescriptionItem {
  /** 药物名称 */
  medicine_name: string
  /** 克数 */
  grams: number
  /** 单价（万分为单位，元/克） */
  unit_price: number
  /** 总价（万分为单位） */
  total_price: number
}

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
  data_type?: RecordType
  /** 结算类型过滤 */
  trade_type?: string
  /** 患者姓名 */
  customer_name?: string
  /** 联系电话 */
  telephone?: string
  /** 诊断信息 */
  diagnosis?: string
  /** 患者性别 */
  gender?: string
  /** 最小年龄 */
  min_age?: number
  /** 最大年龄 */
  max_age?: number
  /** 医师ID */
  doctor_id?: number
  /** 医师姓名 */
  doctor_name?: string
  /** 主诉（门诊病历） */
  chief_complaint?: string
  /** 病史（门诊病历） */
  illness_history?: string
  /** 体格检查（门诊病历） */
  physical_examination?: string
  /** 既往史（门诊病历） */
  past_history?: string
  /** 实验室检查（门诊病历） */
  laboratory_examination?: string
  /** 治疗方法（门诊病历） */
  treatment_method?: string
  /** 付数（中药处方） */
  doses?: number
  /** 最小总金额 */
  min_amount?: number
  /** 最大总金额 */
  max_amount?: number
  /** 备注 */
  note?: string
}

/**
 * 统一的门诊日志数据项
 */
export interface OutpatientLogItem {
  /** 唯一标识 */
  id: string
  /** 数据类型 */
  data_type: RecordType
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
  treatment_items?: TreatmentItem[]
  /** 治疗项目摘要 */
  treatment_items_summary?: string
  /** 金额 */
  amount?: number
  /** 备注 */
  note?: string
  /** 原始数据 */
  // raw_data: any
  /** 处方特有字段 */
  prescription?: PrescriptionItem[]
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
  doctor_id?: number
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
    // const allRecords: any[] = []

    try {
      console.log('正在获取数据...')
      const customerResponse = await getUnifiedMedicalRecordList({
        current: 1,
        size: 1000,
        record_type: params.data_type,
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
      console.log('散客登记数据响应:', customerResponse)
      if (customerResponse.code === 200 && customerResponse.data.records) {
        allData.push(...convertMedicalRecordToOutpatientLog(customerResponse.data.records))
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
 * 将统一医疗记录转换为门诊日志格式
 */
function convertMedicalRecordToOutpatientLog(record: MedicalRecord[]): OutpatientLogItem[] {
  return record.map((item) => {
    const baseItem: OutpatientLogItem = {
      id: `${item.record_type}_${item.id}`,
      data_type: item.record_type,
      customer_name: item.customer_name,
      gender: item.gender,
      age: item.age,
      telephone: item.telephone,
      time_of_visit: item.time_of_visit,
      trade_type: item.trade_type,
      diagnosis: item.diagnosis,
      note: item.note
    }

    // 处理治疗项目
    if (item.treatment_items && item.treatment_items.length > 0) {
      baseItem.treatment_items = item.treatment_items.map((item: any) => ({
        treatment_item: item.treatment_item_name,
        count: item.treatment_count,
        unit_price: item.unit_price,
        total_price: item.total_price
      }))

      // 生成治疗项目摘要
      baseItem.treatment_items_summary = item.treatment_items
        .map((item: any) => `${item.treatment_item_name}×${item.treatment_count}`)
        .join(', ')

      // 计算总金额
      baseItem.amount = item.treatment_items.reduce(
        (sum: number, item: any) => sum + (item.total_price || 0),
        0
      )
    }

    // 处理中药处方项目
    if (item.prescription_items && item.prescription_items.length > 0) {
      baseItem.prescription = item.prescription_items.map((item: any) => ({
        medicine_name: item.medicine_name,
        grams: item.grams,
        unit_price: item.unit_price,
        total_price: item.total_price
      }))

      // 生成治疗项目摘要（对于中药处方）
      baseItem.treatment_items_summary = item.prescription_items
        .map((item: any) => `${item.medicine_name} ${item.grams}g`)
        .join(', ')

      // 设置处方特有字段
      baseItem.doses = item.doses
      baseItem.single_dose_amount = item.single_dose_amount
      baseItem.total_amount = item.total_amount
      baseItem.amount = item.total_amount
    }

    // 设置门诊病历特有字段
    if (item.record_type === 'medical') {
      baseItem.chief_complaint = item.chief_complaint
      baseItem.illness_history = item.illness_history
      baseItem.physical_examination = item.physical_examination
      baseItem.past_history = item.past_history
      baseItem.laboratory_examination = item.laboratory_examination
      baseItem.treatment_method = item.treatment_method
    }

    // 设置医师信息
    baseItem.doctor_id = item.doctor_id
    baseItem.doctor_name = item.doctor_name

    return baseItem
  })
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
