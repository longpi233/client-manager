/**
 * 医疗系统相关枚举
 */

// 性别枚举
export enum GenderEnum {
  MALE = '1',
  FEMALE = '2'
}

// 性别映射
export const GenderMap = {
  [GenderEnum.MALE]: '男',
  [GenderEnum.FEMALE]: '女'
}

// 结算类型枚举
export enum TradeTypeEnum {
  CASH = '1',
  PROVINCIAL_INSURANCE = '2',
  WUHAN_INSURANCE = '3'
}

// 结算类型映射 (支持多种格式)
export const TradeTypeMap = {
  // 数字格式
  [TradeTypeEnum.CASH]: '现金结算',
  [TradeTypeEnum.PROVINCIAL_INSURANCE]: '省直医保',
  [TradeTypeEnum.WUHAN_INSURANCE]: '武汉医保',
  // 英文格式
  cash: '现金结算',
  insurance: '医保结算',
  Cash: '现金结算',
  ProvincialInsurance: '省直医保',
  WuHanInsurance: '武汉医保'
} as const

// 数据类型枚举
export enum DataTypeEnum {
  CUSTOMER = 'customer',
  MEDICAL = 'medical',
  PRESCRIPTION = 'prescription'
}

// 数据类型映射
export const DataTypeMap = {
  [DataTypeEnum.CUSTOMER]: '散客登记',
  [DataTypeEnum.MEDICAL]: '门诊病历',
  [DataTypeEnum.PRESCRIPTION]: '中药处方'
} as const

// 查询类型枚举
export enum QueryTypeEnum {
  MONTH = 'month',
  YEAR = 'year',
  CUSTOM = 'custom'
}

// 查询类型映射
export const QueryTypeMap = {
  [QueryTypeEnum.MONTH]: '按月份查询',
  [QueryTypeEnum.YEAR]: '按年份查询',
  [QueryTypeEnum.CUSTOM]: '自定义日期'
} as const

// 标签类型枚举 (Element Plus)
export enum TagTypeEnum {
  PRIMARY = 'primary',
  SUCCESS = 'success',
  WARNING = 'warning',
  INFO = 'info',
  DANGER = 'danger'
}

// 性别标签类型映射
export const GenderTagTypeMap = {
  [GenderEnum.MALE]: TagTypeEnum.PRIMARY,
  [GenderEnum.FEMALE]: TagTypeEnum.SUCCESS
} as const

// 结算类型标签类型映射
export const TradeTypeTagTypeMap = {
  [TradeTypeEnum.CASH]: TagTypeEnum.SUCCESS,
  [TradeTypeEnum.PROVINCIAL_INSURANCE]: TagTypeEnum.WARNING,
  [TradeTypeEnum.WUHAN_INSURANCE]: TagTypeEnum.INFO
} as const

// 数据类型标签类型映射
export const DataTypeTagTypeMap = {
  [DataTypeEnum.CUSTOMER]: TagTypeEnum.PRIMARY,
  [DataTypeEnum.MEDICAL]: TagTypeEnum.SUCCESS,
  [DataTypeEnum.PRESCRIPTION]: TagTypeEnum.WARNING
} as const
