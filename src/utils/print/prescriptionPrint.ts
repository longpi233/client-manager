/**
 * 中医中药打印模板
 */
import { formatAmountFromTenThousandths, getTradeTypeName } from '../dataprocess'
import {
  type PrescriptionData,
  type PrintOptions,
  formatGender,
  formatDateTime,
  getInstitutionInfo,
  printDocument
} from './printUtils'

/**
 * 生成中医中药打印内容
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
  const totalAmount = data.total_amount || 0

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
        <span class="form-label">结算类型：</span>
        <span class="form-value">${getTradeTypeName(data.trade_type)}</span>
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
        <span class="form-label">用法：</span>
        <span class="form-value">水煎分二次服</span>
      </div>

      <div class="form-row">
        <span class="form-label">药费：</span>
        <span class="form-value">${formatAmountFromTenThousandths(totalAmount)}</span>
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
 * 打印中医中药
 */
export const printPrescription = async (
  data: PrescriptionData,
  options?: PrintOptions
): Promise<void> => {
  try {
    const htmlContent = await generatePrescriptionHTML(data)
    const filename = `中医中药-${data.customer_name}-${formatDateTime(data.time_of_visit).split(' ')[0]}`
    printDocument(htmlContent, `中医中药-${data.customer_name}`, {
      showPdfOption: true,
      filename,
      ...options
    })
  } catch (error) {
    console.error('打印中医中药失败:', error)
    throw error
  }
}

/**
 * 生成中医病历打印内容
 */
export const generateChineseMedicalRecordHTML = async (data: PrescriptionData): Promise<string> => {
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

  return `
    <div class="print-header">
      <h1>${institution.name}</h1>
      <h2>中医病历</h2>
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
        <span class="form-label">舌象脉象：</span>
        <span class="form-value">${data.physical_examination || ''}</span>
      </div>

      <div class="form-row">
        <span class="form-label">诊断：</span>
        <span class="form-value">${data.diagnosis || ''}</span>
      </div>

      <div class="form-row">
        <span class="form-label">治法治则：</span>
        <span class="form-value">${data.treatment_method || ''}</span>
      </div>

      <div class="form-row treatment-row">
        <span class="form-label">中药处方：</span>
        <div class="treatment-section">
          ${prescriptionItemsHTML}
        </div>
      </div>

      <div class="form-row">
        <span class="form-label">剂数：</span>
        <span class="form-value">${data.doses || '1'}付</span>
        <span class="form-label">用法：</span>
        <span class="form-value">水煎分二次服</span>
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
 * 打印中医病历
 */
export const printChineseMedicalRecord = async (
  data: PrescriptionData,
  options?: PrintOptions
): Promise<void> => {
  try {
    const htmlContent = await generateChineseMedicalRecordHTML(data)
    const filename = `中医病历-${data.customer_name}-${formatDateTime(data.time_of_visit).split(' ')[0]}`
    printDocument(htmlContent, `中医病历-${data.customer_name}`, {
      showPdfOption: true,
      filename,
      ...options
    })
  } catch (error) {
    console.error('打印中医病历失败:', error)
    throw error
  }
}
