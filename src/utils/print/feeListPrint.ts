/**
 * 费用清单打印模板
 */
import {
  type FeeListData,
  type PrintOptions,
  formatGender,
  formatTradeType,
  formatDateTime,
  formatAmount,
  getInstitutionInfo,
  printDocument
} from './printUtils'

/**
 * 生成费用清单打印内容
 */
export const generateFeeListHTML = async (data: FeeListData): Promise<string> => {
  const institution = await getInstitutionInfo()

  const treatmentItemsHTML = data.treatment_items?.length
    ? `
    <table class="treatment-table">
      <thead>
        <tr>
          <th></th>
          <th>项目名称</th>
          <th>单价</th>
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
          <td>${formatAmount(item.unit_price)}</td>
          <td>${item.count}</td>
        </tr>
        `
          )
          .join('')}
      </tbody>
    </table>
  `
    : '<div class="no-treatment">无收费项目</div>'

  const totalAmount = data.total_amount || 0

  return `
    <div class="print-header">
      <h1>${institution.name}</h1>
      <h2>费用清单</h2>
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
        <span class="form-label">结算类型：</span>
        <span class="form-value">${formatTradeType(data.trade_type)}</span>
      </div>

      <div class="form-row treatment-row">
        <span class="form-label">收费项目：</span>
        <div class="treatment-section">
          ${treatmentItemsHTML}
        </div>
      </div>

      <div class="form-row">
        <span class="form-label">合计：</span>
        <span class="form-value">${formatAmount(totalAmount)}</span>
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
 * 打印费用清单
 */
export const printFeeList = async (data: FeeListData, options?: PrintOptions): Promise<void> => {
  try {
    const htmlContent = await generateFeeListHTML(data)
    const filename = `费用清单-${data.customer_name}-${formatDateTime(data.time_of_visit).split(' ')[0]}`
    printDocument(htmlContent, `费用清单-${data.customer_name}`, {
      showPdfOption: true,
      filename,
      ...options
    })
  } catch (error) {
    console.error('打印费用清单失败:', error)
    throw error
  }
}
