/**
 * 机构信息相关常量配置
 */

/** 医师状态选项 */
export const DOCTOR_STATUS_OPTIONS = [
  { label: '启用', value: '1', color: 'success' },
  { label: '禁用', value: '2', color: 'danger' }
] as const

/** 医师职称选项 */
export const DOCTOR_TITLE_OPTIONS = [
  { label: '主任医师', value: 'chief' },
  { label: '副主任医师', value: 'associate_chief' },
  { label: '主治医师', value: 'attending' },
  { label: '住院医师', value: 'resident' },
  { label: '医师', value: 'doctor' }
] as const

/** 科室选项 */
export const DEPARTMENT_OPTIONS = [
  { label: '内科', value: 'internal' },
  { label: '外科', value: 'surgery' },
  { label: '妇科', value: 'gynecology' },
  { label: '儿科', value: 'pediatrics' },
  { label: '中医科', value: 'tcm' },
  { label: '康复科', value: 'rehabilitation' },
  { label: '理疗科', value: 'physiotherapy' }
] as const

/** 表单验证规则 */
export const INSTITUTION_FORM_RULES = {
  institutionName: [
    { required: true, message: '请输入机构名称', trigger: 'blur' },
    { max: 50, message: '机构名称长度不能超过50个字符', trigger: 'blur' }
  ],
  departmentSettings: [
    { required: true, message: '请输入科室设置', trigger: 'blur' },
    { max: 100, message: '科室设置长度不能超过100个字符', trigger: 'blur' }
  ]
} as const

/** 医师表单验证规则 */
export const DOCTOR_FORM_RULES = {
  name: [
    { required: true, message: '请输入医师名称', trigger: 'blur' },
    { max: 20, message: '医师名称长度不能超过20个字符', trigger: 'blur' }
  ]
} as const

/** 状态标签映射 */
export const STATUS_TAG_MAP = {
  '1': { type: 'success', text: '启用' },
  '2': { type: 'danger', text: '禁用' }
} as const

/** 职称标签映射 */
export const TITLE_LABEL_MAP = {
  chief: '主任医师',
  associate_chief: '副主任医师',
  attending: '主治医师',
  resident: '住院医师',
  doctor: '医师'
} as const

/** 科室标签映射 */
export const DEPARTMENT_LABEL_MAP = {
  internal: '内科',
  surgery: '外科',
  gynecology: '妇科',
  pediatrics: '儿科',
  tcm: '中医科',
  rehabilitation: '康复科',
  physiotherapy: '理疗科'
} as const
