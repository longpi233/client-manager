/**
 * 数据格式化相关工具函数
 */

// 时间戳转时间
export function timestampToTime(timestamp: number = Date.now(), isMs: boolean = true): string {
  const date = new Date(isMs ? timestamp : timestamp * 1000)
  return date.toISOString().replace('T', ' ').slice(0, 19)
}

// 数字格式化（千位分隔符）
export function commafy(num: number): string {
  return num.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ',')
}

// 生成随机数
export function randomNum(min: number, max?: number): number {
  if (max === undefined) {
    max = min
    min = 0
  }
  return Math.floor(Math.random() * (max - min + 1)) + min
}

// 移除HTML标签
export function removeHtmlTags(str: string = ''): string {
  return str.replace(/<[^>]*>/g, '')
}

// 格式化日期时间 - 通用格式化函数
export function formatDateTime(dateStr: string | Date | number): string {
  if (!dateStr) return '-'

  let date: Date
  if (typeof dateStr === 'string') {
    date = new Date(dateStr)
  } else if (typeof dateStr === 'number') {
    date = new Date(dateStr)
  } else {
    date = dateStr
  }

  // 检查日期是否有效
  if (isNaN(date.getTime())) return '-'

  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit'
  })
}

// 格式化性别
export function formatGender(gender: string | number): string {
  if (!gender && gender !== 0) return '-'

  const genderStr = String(gender)
  return genderStr === '1' ? '男' : genderStr === '2' ? '女' : genderStr
}

// 格式化金额
export function formatAmount(amount: number | string): string {
  if (!amount || amount === 0) return '-'

  const num = typeof amount === 'string' ? parseFloat(amount) : amount
  if (isNaN(num)) return '-'

  return `¥${num.toFixed(2)}`
}

// 格式化百分比
export function formatPercent(value: number, decimals: number = 2): string {
  if (typeof value !== 'number' || isNaN(value)) return '-'
  return `${(value * 100).toFixed(decimals)}%`
}
