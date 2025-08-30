/**
 * 康复项目相关常量定义
 */

/** 康复项目状态选项 */
export const HEALING_STATUS_OPTIONS = [
  { label: '全部', value: '' },
  { label: '启用', value: '1' },
  { label: '禁用', value: '2' }
] as const

/** 康复项目类别选项 */
export const HEALING_CATEGORY_OPTIONS = [
  { label: '全部', value: '' },
  { label: '理疗', value: '理疗' },
  { label: '针灸', value: '针灸' },
  { label: '按摩', value: '按摩' },
  { label: '康复训练', value: '康复训练' },
  { label: '中医调理', value: '中医调理' }
] as const

/** 康复项目状态映射 */
export const HEALING_STATUS_MAP: Record<
  Api.Healing.Status,
  { type: 'success' | 'danger'; text: string }
> = {
  '1': { type: 'success', text: '启用' },
  '2': { type: 'danger', text: '禁用' }
} as const

/** 康复项目类别颜色映射 */
export const HEALING_CATEGORY_COLOR_MAP: Record<Api.Healing.Category, string> = {
  理疗: '#409EFF',
  针灸: '#67C23A',
  按摩: '#E6A23C',
  康复训练: '#F56C6C',
  中医调理: '#909399'
} as const

/** 默认搜索参数 */
export const DEFAULT_HEALING_SEARCH_PARAMS: Api.Healing.SearchParams = {
  current: 1,
  size: 20,
  name: '',
  category: undefined,
  status: undefined
} as const

/** 默认创建参数 */
export const DEFAULT_HEALING_CREATE_PARAMS: Omit<Api.Healing.CreateParams, 'name' | 'price'> = {
  description: '',
  category: '理疗',
  status: '1'
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
  { prop: 'category', label: '类别', width: 100 },
  { prop: 'status', label: '状态', width: 80 },
  { prop: 'operation', label: '操作', width: 190, fixed: 'right' }
] as const

/** 导出列配置 */
export const HEALING_EXPORT_COLUMNS = {
  name: { title: '治疗项目', width: 30 },
  price: { title: '单价（元）', width: 20 },
  category: { title: '类别', width: 15 },
  status: { title: '状态', width: 10 },
  description: { title: '描述', width: 40 },
  create_time: { title: '创建时间', width: 20 },
  update_time: { title: '更新时间', width: 20 }
} as const
