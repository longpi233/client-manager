/**
 * 统一医疗记录数据结构
 * 适用于散客登记、门诊病历、中医中药等所有医疗记录类型
 */

/**
 * 记录类型枚举
 */
export enum RecordType {
  Customer = 'customer',
  Medical = 'medical',
  Prescription = 'prescription'
}

/**
 * 性别枚举
 */
export enum Gender {
  Male = '1',
  Female = '2'
}

/**
 * 结算类型枚举
 */
export enum TradeType {
  Cash = '1',
  ProvincialInsurance = '2',
  WuHanInsurance = '3'
}

/**
 * 标签类型枚举（Element Plus）
 */
export enum TagType {
  Primary = 'primary',
  Success = 'success',
  Warning = 'warning',
  Info = 'info',
  Danger = 'danger'
}

/**
 * 性别选项配置
 */
export const GENDER_OPTIONS = [
  { label: '男', value: '1' },
  { label: '女', value: '2' }
] as const

/**
 * 结算类型选项配置
 */
export const TRADE_TYPE_OPTIONS = [
  { label: '现金结算', value: '1' },
  { label: '省直医保', value: '2' },
  { label: '武汉医保', value: '3' }
] as const

/**
 * 记录类型选项配置
 */
export const RECORD_TYPE_OPTIONS = [
  { label: '散客登记', value: 'customer' },
  { label: '门诊病历', value: 'medical' },
  { label: '中医中药', value: 'prescription' }
] as const

/**
 * 性别标签映射
 */
export const GENDER_LABEL_MAP = {
  '1': '男',
  '2': '女'
} as const

/**
 * 结算类型标签映射
 */
export const TRADE_TYPE_LABEL_MAP = {
  '1': '现金结算',
  '2': '省直医保',
  '3': '武汉医保'
} as const

/**
 * 记录类型标签映射
 */
export const RECORD_TYPE_LABEL_MAP = {
  customer: '散客登记',
  medical: '门诊病历',
  prescription: '中医中药'
} as const

/**
 * 性别标签类型映射
 */
export const GENDER_TAG_TYPE_MAP = {
  '1': 'primary' as TagType,
  '2': 'success' as TagType
} as const

/**
 * 结算类型标签类型映射
 */
export const TRADE_TYPE_TAG_TYPE_MAP = {
  '1': 'success' as TagType,
  '2': 'warning' as TagType,
  '3': 'info' as TagType
} as const

/**
 * 记录类型标签类型映射
 */
export const RECORD_TYPE_TAG_TYPE_MAP = {
  customer: 'primary' as TagType,
  medical: 'success' as TagType,
  prescription: 'warning' as TagType
} as const

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
 * 统一医疗记录接口
 * 包含所有类型医疗记录的完整字段
 */
export interface UnifiedMedicalRecord {
  /** 原始数据库ID 唯一标识 */
  id?: number
  /** 记录类型 */
  data_type: RecordType

  /** ===== 基础患者信息 ===== */
  /** 患者姓名 */
  customer_name: string
  /** 性别 */
  gender: string
  /** 年龄 */
  age: number
  /** 联系电话 */
  telephone?: string
  /** 就诊时间 */
  time_of_visit: string
  /** 结算类型 */
  trade_type: string
  /** 诊断信息 */
  diagnosis: string
  /** 备注 */
  note?: string

  /** ===== 通用金额字段 ===== */
  /** 总金额（万分为单位） */
  total_amount: number

  /** ===== 医师信息 ===== */
  /** 医师ID */
  doctor_id?: number
  /** 医师姓名 */
  doctor_name?: string

  /** ===== 系统字段 ===== */
  /** 创建时间 */
  created_at?: string
  /** 更新时间 */
  updated_at?: string

  /** ===== 门诊病历特有字段 ===== */
  /** 主诉 */
  chief_complaint?: string
  /** 现病史 */
  illness_history?: string
  /** 体格检查 */
  physical_examination?: string
  /** 既往史 */
  past_history?: string
  /** 实验室检查 */
  laboratory_examination?: string
  /** 治疗方法 */
  treatment_method?: string

  /** ===== 中医中药特有字段 ===== */
  /** 付数 */
  doses?: number
  /** 单付金额（万分为单位） */
  single_dose_amount?: number

  /** ===== 关联项目列表 ===== */
  /** 治疗项目列表 */
  treatment_items?: TreatmentItem[]
  /** 处方项目列表 */
  prescription_items?: PrescriptionItem[]
}

/**
 * 统一医疗记录查询参数
 */
export interface UnifiedMedicalRecordSearchParams {
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
  /** 付数（中医中药） */
  doses?: number
  /** 最小总金额 */
  min_amount?: number
  /** 最大总金额 */
  max_amount?: number
  /** 备注 */
  note?: string
}

/**
 * 统一医疗记录响应数据
 */
export interface UnifiedMedicalRecordResponse {
  /** 数据列表 */
  records: UnifiedMedicalRecord[]
  /** 总数 */
  total: number
  /** 当前页 */
  current: number
  /** 每页大小 */
  size: number
  /** 统计信息 */
  statistics?: {
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
 * API响应接口
 */
export interface MedicalApiResponse<T> {
  code: number
  msg: string
  data: T
}
