/**
 * 通用常量配置
 * 用于门诊病历、中药处方、散客登记等所有医疗相关页面
 */

/** 性别枚举 */
export enum Gender {
  Male = '1',
  Female = '2'
}

/** 结算类型枚举 */
export enum TradeType {
  Cash = '1',
  ProvincialInsurance = '2',
  WuHanInsurance = '3'
}

/** 性别选项 */
export const GENDER_OPTIONS = [
  { label: '男', value: Gender.Male },
  { label: '女', value: Gender.Female }
] as const

/** 结算类型选项 */
export const TRADE_TYPE_OPTIONS = [
  { label: '现金结算', value: TradeType.Cash },
  { label: '省直医保', value: TradeType.ProvincialInsurance },
  { label: '武汉医保', value: TradeType.WuHanInsurance }
] as const

/** 性别标签映射 */
export const GENDER_LABEL_MAP = {
  [Gender.Male]: '男',
  [Gender.Female]: '女'
} as const

/** 结算类型标签映射 */
export const TRADE_TYPE_LABEL_MAP = {
  [TradeType.Cash]: '现金结算',
  [TradeType.ProvincialInsurance]: '省直医保',
  [TradeType.WuHanInsurance]: '武汉医保'
} as const
