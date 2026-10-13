/**
 * 门诊与医疗业务全局共享常量
 * 统一管理性别、结算类型、就诊状态及默认配置
 */

/** 患者性别选项 */
export const GENDER_OPTIONS = [
  { label: '男', value: '1' },
  { label: '女', value: '2' }
] as const

/** 结算类型选项 */
export const TRADE_TYPE_OPTIONS = [
  { label: '现金结算', value: '1' },
  { label: '省直医保', value: '2' },
  { label: '武汉医保', value: '3' }
] as const

/** 性别文本映射字典 */
export const GENDER_MAP: Record<string, string> = {
  '1': '男',
  '2': '女',
  Male: '男',
  Female: '女'
}

/** 结算类型文本映射字典 */
export const TRADE_TYPE_MAP: Record<string, string> = {
  '1': '现金结算',
  '2': '省直医保',
  '3': '武汉医保',
  Cash: '现金结算',
  ProvincialInsurance: '省直医保',
  WuHanInsurance: '武汉医保'
}

/** 默认分页大小 */
export const DEFAULT_PAGE_SIZE = 20

/** 默认金额换算率：万分位 (1元 = 10000) */
export const TEN_THOUSANDTHS_RATIO = 10000
