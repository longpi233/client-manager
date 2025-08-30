/**
 * 中药处方打印模板
 */
import {
  type PrescriptionData,
  type PrintOptions,
  formatGender,
  formatDateTime,
  getInstitutionInfo,
  printDocument
} from './printUtils'

/**
 * 生成中药处方打印内容
 */
export const generatePrescriptionHTML = async (data: PrescriptionData): Promise<string> => {
  const institution = await getInstitutionInfo()

  const prescriptionItemsHTML = data.prescription_items?.length
    ? `
    <table class="prescription-table">
      <tbody>
        ${data.prescription_items
          .reduce(
            (rows, item, index) => {
              if (index % 4 === 0) {
                rows.push([])
              }
              rows[rows.length - 1].push(item)
              return rows
            },
            [] as Array<Array<any>>
          )
          .map(
            (row) => `
        <tr>
          ${row
            .map(
              (item) => `
            <td>${item.medicine_name} ${item.grams}g</td>
          `
            )
            .join('')}
          ${row.length < 4 ? '<td></td>'.repeat(4 - row.length) : ''}
        </tr>
        `
          )
          .join('')}
      </tbody>
    </table>
  `
    : '<div class="no-treatment">无处方药品</div>'

  const doses = data.doses || 1

  return `
    <div class="print-header">
      <h1>${institution.name}</h1>
      <h2>门诊处方</h2>
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
        <span class="form-label">诊断：</span>
        <span class="form-value">${data.diagnosis}</span>
      </div>

      <div class="form-row treatment-row">
        <span class="form-label">处方：</span>
        <div class="treatment-section">
          ${prescriptionItemsHTML}
        </div>
      </div>

      <div class="form-row">
        <span class="form-label">剂数：</span>
        <span class="form-value">${doses}付</span>
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
 * 打印中药处方
 */
export const printPrescription = async (
  data: PrescriptionData,
  options?: PrintOptions
): Promise<void> => {
  try {
    const htmlContent = await generatePrescriptionHTML(data)
    const filename = `中药处方-${data.customer_name}-${formatDateTime(data.time_of_visit).split(' ')[0]}`
    printDocument(htmlContent, `中药处方-${data.customer_name}`, {
      showPdfOption: true,
      filename,
      ...options
    })
  } catch (error) {
    console.error('打印中药处方失败:', error)
    throw error
  }
}
