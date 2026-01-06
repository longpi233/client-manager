/**
 * 治疗处方单打印模板
 */
import {
  type PrintOptions,
  formatGender,
  formatDateChinese,
  formatTradeType,
  getInstitutionInfo,
  printDocument
} from './printUtils'

/**
 * 治疗处方单数据接口
 */
export interface TreatmentPrescriptionData {
  customer_name: string
  gender: string
  age: number
  trade_type: string
  diagnosis: string
  treatment_method?: string // 治法治则
  treatment_items?: Array<{
    treatment_item: string
    count: number
  }>
  note?: string
  time_of_visit: string
}

/**
 * 生成治疗处方单打印内容
 */
export const generateTreatmentPrescriptionHTML = async (
  data: TreatmentPrescriptionData
): Promise<string> => {
  const institution = await getInstitutionInfo()

  const treatmentItemsHTML = data.treatment_items?.length
    ? `
    <table class="treatment-table">
      <thead>
        <tr>
          <th></th>
          <th>治疗项目</th>
          <th>治疗次数</th>
        </tr>
      </thead>
      <tbody>
        ${data.treatment_items
          .map(
            (item, index) => `
        <tr>
          <td>${index + 1}.</td>
          <td>${item.treatment_item}</td>
          <td>${item.count}</td>
        </tr>
        `
          )
          .join('')}
      </tbody>
    </table>
  `
    : '<div class="no-treatment">无治疗项目</div>'

  return `
    <div class="print-header">
      <h1>${institution.name}</h1>
      <h2>治疗处方单</h2>
    </div>

    <div class="form-content">
      <div class="form-row">
        <span class="form-label">患者姓名：</span>
        <span class="form-value">${data.customer_name}</span>
        <span class="form-label">性别：</span>
        <span class="form-value">${formatGender(data.gender)}</span>
        <span class="form-label">年龄：</span>
        <span class="form-value">${data.age}岁</span>
        <span class="form-label">结算类别：</span>
        <span class="form-value">${formatTradeType(data.trade_type)}</span>
      </div>

      <div class="form-row">
        <span class="form-label">诊断：</span>
        <span class="form-value">${data.diagnosis}</span>
      </div>

      <div class="form-row">
        <span class="form-label">治法治则：</span>
        <span class="form-value">${data.treatment_method || ''}</span>
      </div>

      <div class="form-row treatment-row">
        <span class="form-label">处理意见：</span>
        <div class="treatment-section">
          ${treatmentItemsHTML}
        </div>
      </div>

      <div class="form-row">
        <span class="form-label">备注：</span>
        <span class="form-value">${data.note || ''}</span>
      </div>

      <div class="blank-space-small"></div>

      <div class="form-row">
        <span class="form-label">处方时间：</span>
        <span class="form-value">${formatDateChinese(data.time_of_visit)}</span>
      </div>

      <div class="form-row signature-row">
        <div class="signature-item">
          <span class="form-label">医师签名：</span>
          <span class="form-value signature-space"></span>
        </div>
        <div class="signature-item">
          <span class="form-label">收费盖章：</span>
          <span class="form-value signature-space"></span>
        </div>
      </div>

      <div class="blank-space-large"></div>
    </div>
  `
}

/**
 * 打印治疗处方单
 */
export const printTreatmentPrescription = async (
  data: TreatmentPrescriptionData,
  options?: PrintOptions
): Promise<void> => {
  try {
    const htmlContent = await generateTreatmentPrescriptionHTML(data)
    const filename = `治疗处方单-${data.customer_name}-${formatDateChinese(data.time_of_visit)}`
    printDocument(htmlContent, `治疗处方单-${data.customer_name}`, {
      showPdfOption: true,
      filename,
      ...options
    })
  } catch (error) {
    console.error('打印治疗处方单失败:', error)
    throw error
  }
}
