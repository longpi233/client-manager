/**
 * 医疗系统专用格式化工具函数
 */

import {
  RecordType,
  GENDER_LABEL_MAP,
  TRADE_TYPE_LABEL_MAP,
  RECORD_TYPE_LABEL_MAP,
  GENDER_TAG_TYPE_MAP,
  TRADE_TYPE_TAG_TYPE_MAP,
  RECORD_TYPE_TAG_TYPE_MAP,
  type TagType
} from '@/types/medical'
import {
  formatDateTime as baseFormatDateTime,
  formatGender as baseFormatGender,
  formatAmount as baseFormatAmount,
  formatAmountFromTenThousandths,
  convertYuanToTenThousandths,
  convertTenThousandthsToYuan
} from './format'

// 重新导出基础格式化函数
export {
  baseFormatDateTime as formatDateTime,
  baseFormatGender as formatGender,
  baseFormatAmount as formatAmount,
  formatAmountFromTenThousandths,
  convertYuanToTenThousandths,
  convertTenThousandthsToYuan
}

/**
 * 获取性别名称
 * @param gender 性别值 ('1' | '2' | string)
 * @returns 性别中文名称
 */
export function getGenderName(gender: string | number): string {
  if (!gender && gender !== 0) return '-'
  const genderStr = String(gender)
  return GENDER_LABEL_MAP[genderStr as keyof typeof GENDER_LABEL_MAP] || genderStr
}

/**
 * 标准化结算类型值
 * @param type 原始结算类型值
 * @returns 标准化后的结算类型值 ('1' | '2' | '3')
 */
function normalizeTradeType(type: string): string {
  if (!type) return ''

  const normalMap: Record<string, string> = {
    '1': '1',
    '2': '2',
    '3': '3',
    cash: '1',
    Cash: '1',
    insurance: '2',
    ProvincialInsurance: '2',
    WuHanInsurance: '3'
  }

  return normalMap[type] || type
}

/**
 * 获取结算类型名称
 * @param type 结算类型值
 * @returns 结算类型中文名称
 */
export function getTradeTypeName(type: string): string {
  if (!type) return '-'

  // 先标准化，再查询映射
  const normalizedType = normalizeTradeType(type)

  // 标准映射表
  const standardMap: Record<string, string> = {
    '1': '现金结算',
    '2': '省直医保',
    '3': '武汉医保'
  }

  return (
    standardMap[normalizedType] ||
    TRADE_TYPE_LABEL_MAP[type as keyof typeof TRADE_TYPE_LABEL_MAP] ||
    type
  )
}

/**
 * 获取数据类型名称
 * @param type 数据类型值
 * @returns 数据类型中文名称
 */
export function getDataTypeName(type: RecordType): string {
  if (!type) return '-'
  return RECORD_TYPE_LABEL_MAP[type] || type
}

/**
 * 获取查询类型名称
 * @param type 查询类型值
 * @returns 查询类型中文名称
 */
export function getQueryTypeName(type: string): string {
  if (!type) return '-'

  const queryTypeMap: Record<string, string> = {
    month: '按月份查询',
    year: '按年份查询',
    custom: '自定义日期'
  }

  return queryTypeMap[type] || type
}

/**
 * 获取性别标签类型
 * @param gender 性别值
 * @returns Element Plus 标签类型
 */
export function getGenderTagType(gender: string): TagType {
  return GENDER_TAG_TYPE_MAP[gender as keyof typeof GENDER_TAG_TYPE_MAP] || 'info'
}

/**
 * 获取结算类型标签类型
 * @param type 结算类型值
 * @returns Element Plus 标签类型
 */
export function getTradeTypeTagType(type: string): TagType {
  return TRADE_TYPE_TAG_TYPE_MAP[type as keyof typeof TRADE_TYPE_TAG_TYPE_MAP] || 'info'
}

/**
 * 获取数据类型标签类型
 * @param type 数据类型值
 * @returns Element Plus 标签类型
 */
export function getDataTypeTagType(type: string): TagType {
  return RECORD_TYPE_TAG_TYPE_MAP[type as keyof typeof RECORD_TYPE_TAG_TYPE_MAP] || 'info'
}

/**
 * 格式化付数
 * @param doses 付数
 * @returns 格式化后的付数字符串
 */
export function formatDoses(doses: number | string): string {
  if (!doses && doses !== 0) return '-'
  return `${doses}付`
}

/**
 * 格式化年龄
 * @param age 年龄
 * @returns 格式化后的年龄字符串
 */
export function formatAge(age: number | string): string {
  if (!age && age !== 0) return '-'
  return `${age}岁`
}

/**
 * 生成查询时间段文本
 * @param queryType 查询类型
 * @param params 查询参数对象
 * @returns 查询时间段的中文描述
 */
export function generateQueryPeriodText(
  queryType: string,
  params: {
    month?: string
    year?: string
    dateRange?: string[]
  }
): string {
  if (queryType === 'month' && params.month) {
    return `${params.month}月份`
  } else if (queryType === 'year' && params.year) {
    return `${params.year}年全年`
  } else if (queryType === 'custom' && params.dateRange?.length === 2) {
    return `${params.dateRange[0]} 至 ${params.dateRange[1]}`
  }
  return '当前查询'
}

/**
 * 生成导出文件名
 * @param prefix 文件名前缀
 * @param conditions 查询条件
 * @returns 格式化的文件名
 */
export function generateExportFilename(prefix: string, conditions: string[] = []): string {
  const now = new Date()
  const timestamp = now.toISOString().replace(/[:.]/g, '-').slice(0, 19)

  let conditionText = ''
  if (conditions.length > 0) {
    conditionText = '_' + conditions.join('_')
  }

  return `${prefix}${conditionText}_${timestamp}`
}
