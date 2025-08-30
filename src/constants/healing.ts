/**
 * 康复项目相关常量定义
 */

/** 默认搜索参数 */
export const DEFAULT_HEALING_SEARCH_PARAMS: Api.Healing.SearchParams = {
  current: 1,
  size: 20,
  name: ''
} as const

/** 默认创建参数 */
export const DEFAULT_HEALING_CREATE_PARAMS: Omit<Api.Healing.CreateParams, 'name' | 'price'> = {
  description: ''
} as const

/** 价格输入规则 */
export const PRICE_INPUT_RULES = {
  min: 0,
  max: 99999,
  precision: 2,
  step: 10
} as const

/** 表格列配置 */
export const HEALING_TABLE_COLUMNS = [
  { type: 'selection', width: 50 },
  { type: 'globalIndex', width: 60, label: '序号' },
  { prop: 'name', label: '治疗项目', sortable: true, minWidth: 200 },
  { prop: 'price', label: '单价（元）', sortable: true, width: 120 },
  { prop: 'operation', label: '操作', width: 190, fixed: 'right' }
] as const

/** 导出列配置 */
export const HEALING_EXPORT_COLUMNS = {
  name: { title: '治疗项目', width: 30 },
  price: { title: '单价（元）', width: 20 },
  description: { title: '描述', width: 40 },
  create_time: { title: '创建时间', width: 20 },
  update_time: { title: '更新时间', width: 20 }
} as const
