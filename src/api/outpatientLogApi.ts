/**
 * 门诊日志相关API
 * 统一获取散客登记、门诊病历、中药处方的汇总数据
 */
import { getCustomerList, getCustomerTreatments } from './customerApi'
import { getMedicalRecordList } from './medicalRecordApi'
import { getChineseMedicinePrescriptionList } from './chineseMedicinePrescriptionApi'

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
 * 统一从散客登记、门诊病历、中药处方获取数据
 */
export const getOutpatientLogList = async (
  params: OutpatientLogSearchParams
): Promise<{ code: number; data: OutpatientLogResponse; message: string }> => {
  try {
    const allData: OutpatientLogItem[] = []

    // 构建通用查询参数
    const baseParams = {
      current: 1,
      size: 1000, // 获取足够多的数据用于汇总
      start_date: params.start_date,
      end_date: params.end_date,
      customer_name: params.customer_name
    }

    // 获取散客登记数据
    if (!params.data_type || params.data_type === 'customer') {
      try {
        const customerResponse = await getCustomerList(baseParams)
        if (customerResponse.code === 200 && customerResponse.data.records) {
          const customerData = await convertCustomerData(customerResponse.data.records)
          allData.push(...customerData)
        }
      } catch (error) {
        console.error('获取散客数据失败:', error)
      }
    }

    // 获取门诊病历数据
    if (!params.data_type || params.data_type === 'medical') {
      try {
        const medicalResponse = await getMedicalRecordList(baseParams)
        if (medicalResponse.code === 200 && medicalResponse.data.records) {
          const medicalData = convertMedicalData(medicalResponse.data.records)
          allData.push(...medicalData)
        }
      } catch (error) {
        console.error('获取门诊病历数据失败:', error)
      }
    }

    // 获取中药处方数据
    if (!params.data_type || params.data_type === 'prescription') {
      try {
        const prescriptionResponse = await getChineseMedicinePrescriptionList(baseParams)
        if (prescriptionResponse.code === 200 && prescriptionResponse.data.records) {
          const prescriptionData = convertPrescriptionData(prescriptionResponse.data.records)
          allData.push(...prescriptionData)
        }
      } catch (error) {
        console.error('获取中药处方数据失败:', error)
      }
    }

    // 应用结算类型过滤
    let filteredData = allData
    if (params.trade_type) {
      filteredData = allData.filter((item) => item.trade_type === params.trade_type)
    }

    // 按时间排序（降序）
    filteredData.sort(
      (a, b) => new Date(b.time_of_visit).getTime() - new Date(a.time_of_visit).getTime()
    )

    // 计算统计信息
    const statistics = calculateStatistics(filteredData)

    // 应用分页
    const startIndex = (params.current - 1) * params.size
    const endIndex = startIndex + params.size
    const records = filteredData.slice(startIndex, endIndex)

    return {
      code: 200,
      data: {
        records,
        total: filteredData.length,
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
 * 转换散客登记数据
 */
async function convertCustomerData(customerData: any[]): Promise<OutpatientLogItem[]> {
  const results: OutpatientLogItem[] = []

  for (const customer of customerData) {
    try {
      // 获取治疗项目
      const treatmentResponse = await getCustomerTreatments(customer.id)
      const treatmentItems = treatmentResponse.code === 200 ? treatmentResponse.data : []

      // 计算金额
      const amount = treatmentItems.reduce(
        (sum: number, item: any) => sum + (item.total_price || 0),
        0
      )

      // 生成治疗项目摘要
      const treatmentSummary =
        treatmentItems.length > 0
          ? treatmentItems
              .map((item: any) => {
                const name = item.treatment_item || item.name || '治疗项目'
                const count = item.count ? `×${item.count}` : ''
                const price = item.total_price ? `(¥${item.total_price})` : ''
                return `${name}${count}${price}`
              })
              .join('、')
          : '无治疗项目'

      results.push({
        id: `customer_${customer.id}`,
        data_type: 'customer',
        customer_name: customer.name,
        gender: customer.gender,
        age: customer.age,
        phone: customer.phone,
        time_of_visit: customer.time_of_visit,
        trade_type: customer.trade_type || '1',
        diagnosis: customer.diagnosis || '',
        treatment_items: treatmentItems,
        treatment_items_summary: treatmentSummary,
        amount: amount,
        note: customer.note,
        raw_data: customer
      })
    } catch (error) {
      console.error(`获取客户 ${customer.id} 治疗项目失败:`, error)
      // 即使获取治疗项目失败，也要包含基本信息
      results.push({
        id: `customer_${customer.id}`,
        data_type: 'customer',
        customer_name: customer.name,
        gender: customer.gender,
        age: customer.age,
        phone: customer.phone,
        time_of_visit: customer.time_of_visit,
        trade_type: customer.trade_type || '1',
        diagnosis: customer.diagnosis || '',
        treatment_items_summary: '无治疗项目',
        amount: 0,
        note: customer.note,
        raw_data: customer
      })
    }
  }

  return results
}

/**
 * 转换门诊病历数据
 */
function convertMedicalData(medicalData: any[]): OutpatientLogItem[] {
  return medicalData.map((medical) => {
    // 使用内嵌的治疗项目数据
    const treatmentItems = medical.treatment || []
    const amount = treatmentItems.reduce(
      (sum: number, item: any) => sum + (item.total_price || 0),
      0
    )
    const treatmentSummary =
      treatmentItems.length > 0
        ? treatmentItems
            .map((item: any) => {
              const name = item.treatment_item || item.name || '治疗项目'
              const count = item.count ? `×${item.count}` : ''
              const price = item.total_price ? `(¥${item.total_price})` : ''
              return `${name}${count}${price}`
            })
            .join('、')
        : '无治疗项目'

    return {
      id: `medical_${medical.id}`,
      data_type: 'medical',
      customer_name: medical.customer_name,
      gender: medical.gender,
      age: medical.age,
      telephone: medical.telephone,
      time_of_visit: medical.time_of_visit,
      trade_type: medical.trade_type || '1',
      diagnosis: medical.diagnosis || '',
      treatment_items: treatmentItems,
      treatment_items_summary: treatmentSummary,
      amount: amount,
      note: medical.note,
      raw_data: medical,
      // 门诊病历特有字段
      chief_complaint: medical.chief_complaint,
      illness_history: medical.illness_history,
      physical_examination: medical.physical_examination,
      past_history: medical.past_history,
      laboratory_examination: medical.laboratory_examination,
      treatment_method: medical.treatment_method,
      doctor_id: medical.doctor_id,
      doctor_name: medical.doctor_name
    }
  })
}

/**
 * 转换中药处方数据
 */
function convertPrescriptionData(prescriptionData: any[]): OutpatientLogItem[] {
  return prescriptionData.map((prescription) => {
    // 处方明细作为治疗项目摘要
    const prescriptionSummary =
      prescription.prescription && prescription.prescription.length > 0
        ? prescription.prescription
            .map((item: any) => `${item.medicine_name}(${item.grams}g)`)
            .join('、')
        : '无处方明细'

    return {
      id: `prescription_${prescription.id}`,
      data_type: 'prescription',
      customer_name: prescription.customer_name,
      gender: prescription.gender,
      age: prescription.age,
      phone: prescription.phone,
      time_of_visit: prescription.time_of_visit,
      trade_type: prescription.trade_type || '1',
      diagnosis: prescription.diagnosis || '',
      treatment_items_summary: prescriptionSummary,
      amount: prescription.total_amount || 0,
      note: prescription.note,
      raw_data: prescription,
      // 处方特有字段
      prescription: prescription.prescription,
      doses: prescription.doses,
      single_dose_amount: prescription.single_dose_amount,
      total_amount: prescription.total_amount,
      doctor_id: prescription.doctor_id,
      doctor_name: prescription.doctor_name
    }
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
