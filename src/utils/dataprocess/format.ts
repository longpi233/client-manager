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

// 格式化金额（支持分转元）
export function formatAmount(amount: number | string): string {
  if (!amount || amount === 0) return '¥0.00'

  const num = typeof amount === 'string' ? parseFloat(amount) : amount
  if (isNaN(num)) return '¥0.00'

  return `¥${num.toFixed(2)}`
}

// 格式化金额（从分转为元显示）
// 格式化金额（从万分转为元显示）
export function formatAmountFromTenThousandths(tenThousandths: number): string {
  if (!tenThousandths || tenThousandths === 0) return '¥0.0000'
  // 万分转元：除以10000
  const yuan = tenThousandths / 10000
  return `¥${yuan.toFixed(4)}`
}

// 元转分（用于前端向后端发送数据）
// 元转万分（用于前端向后端发送数据，1元=10000单位）
export function convertYuanToTenThousandths(yuan: number | string): number {
  if (!yuan || yuan === 0) return 0

  const yuanNum = typeof yuan === 'string' ? parseFloat(yuan) : yuan
  if (isNaN(yuanNum)) return 0

  // 元转万分：乘以10000并四舍五入
  return Math.round(yuanNum * 10000)
}

// 分转元（用于后端数据转为前端显示）
// 万分转元（用于后端数据转为前端显示，1元=10000单位）
export function convertTenThousandthsToYuan(tenThousandths: number | string): number {
  if (!tenThousandths || tenThousandths === 0) return 0

  const num = typeof tenThousandths === 'string' ? parseInt(tenThousandths) : tenThousandths
  if (isNaN(num)) return 0

  // 万分转元：除以10000
  return num / 10000
}

// 格式化百分比
export function formatPercent(value: number, decimals: number = 2): string {
  if (typeof value !== 'number' || isNaN(value)) return '-'
  return `${(value * 100).toFixed(decimals)}%`
}
