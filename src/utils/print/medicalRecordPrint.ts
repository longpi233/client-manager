/**
 * 针灸理疗打印模板
 */
import {
  type MedicalRecordData,
  type PrintOptions,
  formatGender,
  formatDateTime,
  getInstitutionInfo,
  printDocument
} from './printUtils'

/**
 * 生成门诊病历打印内容
 */
export const generateMedicalRecordHTML = async (data: MedicalRecordData): Promise<string> => {
  const institution = await getInstitutionInfo()

  const treatmentItemsHTML = data.treatment_items?.length
    ? `
    <table class="treatment-table">
      <thead>
        <tr>
          <th></th>
          <th>治疗项目</th>
          <th>数量</th>
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
      <h2>针灸理疗</h2>
    </div>

    <div class="form-content">
      <div class="form-row">
        <span class="form-label">患者姓名：</span>
        <span class="form-value">${data.customer_name}</span>
        <span class="form-label">性别：</span>
        <span class="form-value">${formatGender(data.gender)}</span>
        <span class="form-label">年龄：</span>
        <span class="form-value">${data.age}岁</span>
      </div>

      <div class="form-row">
        <span class="form-label">主诉：</span>
        <span class="form-value">${data.chief_complaint || ''}</span>
      </div>

      <div class="form-row">
        <span class="form-label">现病史：</span>
        <span class="form-value">${data.illness_history || ''}</span>
      </div>

      <div class="form-row">
        <span class="form-label">既往史：</span>
        <span class="form-value">${data.past_history || ''}</span>
      </div>

      <div class="form-row">
        <span class="form-label">体格检查：</span>
        <span class="form-value">${data.physical_examination || ''}</span>
      </div>

      <div class="form-row">
        <span class="form-label">实验室检查：</span>
        <span class="form-value">${data.laboratory_examination || ''}</span>
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

      <div class="form-row">
        <span class="form-label">医师：</span>
        <span class="form-value">${data.doctor_name || ''}</span>
        <span class="form-label">时间：</span>
        <span class="form-value">${formatDateTime(data.time_of_visit)}</span>
      </div>
    </div>
  `
}

/**
 * 打印门诊病历
 */
export const printMedicalRecord = async (
  data: MedicalRecordData,
  options?: PrintOptions
): Promise<void> => {
  try {
    const htmlContent = await generateMedicalRecordHTML(data)
    const filename = `针灸理疗-${data.customer_name}-${formatDateTime(data.time_of_visit).split(' ')[0]}`
    printDocument(htmlContent, `针灸理疗-${data.customer_name}`, {
      showPdfOption: true,
      filename,
      ...options
    })
  } catch (error) {
    console.error('打印门诊病历失败:', error)
    throw error
  }
}
