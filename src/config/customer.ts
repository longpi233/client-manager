/**
 * 散客登记相关常量配置
 */

/** 性别选项 */
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

/** 性别标签映射 */
export const GENDER_LABEL_MAP = {
  '1': '男',
  '2': '女'
} as const

/** 结算类型标签映射 */
export const TRADE_TYPE_LABEL_MAP = {
  '1': '现金结算',
  '2': '省直医保',
  '3': '武汉医保'
} as const

/** 表单验证规则 */
export const CUSTOMER_FORM_RULES = {
  customer_name: [
    { required: true, message: '请输入患者姓名', trigger: 'blur' },
    { max: 20, message: '患者姓名长度不能超过20个字符', trigger: 'blur' }
  ],
  gender: [{ required: true, message: '请选择性别', trigger: 'change' }],
  age: [
    { required: true, message: '请输入年龄', trigger: 'blur' },
    { type: 'number', min: 0, max: 200, message: '年龄必须在0到200之间', trigger: 'blur' }
  ],
  telephone: [
    { required: true, message: '请输入联系电话', trigger: 'blur' },
    { pattern: /^1[3-9]\d{9}$/, message: '请输入正确的手机号格式', trigger: 'blur' }
  ],
  time_of_visit: [{ required: true, message: '请选择就诊时间', trigger: 'change' }],
  trade_type: [{ required: true, message: '请选择结算类型', trigger: 'change' }],
  diagnosis: [
    { required: true, message: '请输入诊断信息', trigger: 'blur' },
    { max: 200, message: '诊断信息长度不能超过200个字符', trigger: 'blur' }
  ],
  note: [{ max: 100, message: '备注信息长度不能超过100个字符', trigger: 'blur' }]
} as const

/** 治疗项目表单验证规则 */
export const TREATMENT_FORM_RULES = {
  treatmentItem: [{ required: true, message: '请选择治疗项目', trigger: 'change' }],
  count: [
    { required: true, message: '请输入数量', trigger: 'blur' },
    { type: 'number', min: 1, message: '数量不能小于1', trigger: 'blur' }
  ]
} as const
