/**
 * 统一打印工具类
 * 支持针灸理疗、费用清单、中医中药、治疗处方单的打印功能
 */
import type { UnifiedMedicalRecord, TreatmentItem, PrescriptionItem } from '@/types/medical'

// 为了向后兼容，保留原有接口定义
export interface PrintableItem {
  id?: number
  customer_name: string
  gender: string
  age: number
  telephone?: string
  time_of_visit: string
  trade_type: string
  diagnosis: string
  doctor_name?: string
  note?: string
}

export interface MedicalRecordData extends PrintableItem {
  chief_complaint?: string
  illness_history?: string
  physical_examination?: string
  past_history?: string
  laboratory_examination?: string
  treatment_method?: string
  treatment_items?: TreatmentItem[]
}

export interface PrescriptionData extends PrintableItem {
  chief_complaint?: string
  illness_history?: string
  past_history?: string
  physical_examination?: string
  treatment_method?: string
  prescription_items?: PrescriptionItem[]
  doses?: number
  single_dose_amount?: number // 单付金额（分）
  total_amount?: number // 总金额（分）
}

export interface FeeListData extends PrintableItem {
  treatment_items?: TreatmentItem[]
  total_amount?: number // 总金额（分）
}

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

// 新增：统一医疗记录的打印适配器
export type UnifiedPrintableRecord = UnifiedMedicalRecord

/**
 * 格式化性别显示
 */
export const formatGender = (gender: string): string => {
  const genderMap: Record<string, string> = {
    '1': '男',
    '2': '女',
    Male: '男',
    Female: '女'
  }
  return genderMap[gender] || gender
}

/**
 * 格式化结算类型
 */
export const formatTradeType = (tradeType: string): string => {
  const tradeTypeMap: Record<string, string> = {
    '1': '现金结算',
    '2': '省直医保',
    '3': '武汉医保',
    Cash: '现金结算',
    ProvincialInsurance: '省直医保',
    WuHanInsurance: '武汉医保'
  }
  return tradeTypeMap[tradeType] || tradeType
}

/**
 * 格式化日期时间
 */
export const formatDateTime = (dateTime: string | Date): string => {
  const date = typeof dateTime === 'string' ? new Date(dateTime) : dateTime
  return date.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit'
    // hour: '2-digit',
    // minute: '2-digit'
  })
}

/**
 * 格式化日期为 xxxx年xx月xx日 格式
 */
export const formatDateChinese = (dateTime: string | Date): string => {
  const date = typeof dateTime === 'string' ? new Date(dateTime) : dateTime
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  return `${year}年${month}月${day}日`
}

/**
 * 格式化金额（从万分转为元显示）
 */
export const formatAmount = (tenThousandths: number): string => {
  if (!tenThousandths || tenThousandths === 0) return '¥0.0000'
  if (isNaN(tenThousandths)) return '¥0.0000'

  // 万分转元：除以10000
  const yuan = tenThousandths / 10000
  return `¥${yuan.toFixed(4)}`
}

/**
 * 元转万分（用于前端计算）
 */
export const convertYuanToTenThousandths = (yuan: number): number => {
  if (!yuan || yuan === 0) return 0
  if (isNaN(yuan)) return 0

  // 元转万分：乘以10000并四舍五入
  return Math.round(yuan * 10000)
}

/**
 * 获取机构信息
 */
export const getInstitutionInfo = async (): Promise<{ name: string }> => {
  try {
    // 动态导入API函数以避免循环依赖
    const { getInstitutionInfo: getInstitutionInfoApi } = await import('@/api/institutionApi')
    const response = await getInstitutionInfoApi()
    console.log('机构信息:', response)

    if (response.code === 200 && response.data) {
      return {
        name: response.data.institution_name || '健康诊所'
      }
    } else {
      // 如果API调用失败，返回默认值
      return {
        name: '健康诊所'
      }
    }
  } catch (error) {
    console.error('获取机构信息失败:', error)
    // 如果出现错误，返回默认值
    return {
      name: '健康诊所'
    }
  }
}

/**
 * 打印选项接口
 */
export interface PrintOptions {
  showPdfOption?: boolean
  filename?: string
}

/**
 * 通用打印函数
 */
export const printDocument = (
  htmlContent: string,
  title: string = '打印文档',
  options: PrintOptions = {}
): void => {
  const { showPdfOption = true, filename = title } = options

  const printContent = `
    <!DOCTYPE html>
    <html>
    <head>
      <meta charset="utf-8">
      <title>${title}</title>
      <style>
        body {
          font-family: 'Microsoft YaHei', Arial, sans-serif;
          font-size: 14px;
          line-height: 1.5;
          margin: 20px;
          color: #333;
        }
        .print-header {
          text-align: center;
          margin-bottom: 30px;
          border-bottom: 2px solid #333;
          padding-bottom: 15px;
        }
        .print-header h1 {
          font-size: 24px;
          font-weight: bold;
          margin: 0 0 10px 0;
        }
        .print-header h2 {
          font-size: 18px;
          margin: 0 0 5px 0;
          color: #666;
        }
        .form-content {
          margin: 20px 0;
        }
        .form-row {
          margin-bottom: 15px;
          display: flex;
          align-items: flex-start;
          min-height: 25px;
        }
        .form-label {
          font-weight: bold;
          margin-right: 10px;
          min-width: 80px;
          color: #333;
        }
        .form-value {
          flex: 1;
          border-bottom: 1px solid #ccc;
          padding: 2px 5px;
          min-height: 20px;
          margin-right: 20px;
        }
        .treatment-items {
          flex: 1;
          margin-left: 10px;
        }
        .treatment-item {
          margin-bottom: 5px;
          padding: 2px 0;
        }
        .institution-info {
          font-size: 12px;
          color: #888;
          margin-top: 5px;
        }
        .patient-info {
          margin-bottom: 20px;
          display: flex;
          flex-wrap: wrap;
          gap: 20px;
        }
        .info-item {
          display: flex;
          align-items: center;
          min-width: 200px;
        }
        .info-label {
          font-weight: bold;
          margin-right: 10px;
          min-width: 80px;
        }
        .info-value {
          border-bottom: 1px solid #ccc;
          flex: 1;
          padding: 2px 5px;
        }
        .section {
          margin-bottom: 20px;
        }
        .section-title {
          font-weight: bold;
          font-size: 16px;
          margin-bottom: 10px;
          color: #333;
          border-left: 4px solid #007bff;
          padding-left: 10px;
        }
        .content-area {
          border: 1px solid #ccc;
          padding: 10px;
          min-height: 60px;
          background-color: #fafafa;
        }
        .items-table {
          width: 100%;
          border-collapse: collapse;
          margin-top: 10px;
        }
        .items-table th,
        .items-table td {
          border: 1px solid #ccc;
          padding: 8px;
          text-align: center;
        }
        .items-table th {
          background-color: #f5f5f5;
          font-weight: bold;
        }
        .items-table td:first-child {
          text-align: left;
        }
        .total-section {
          margin-top: 20px;
          text-align: right;
          font-size: 16px;
          font-weight: bold;
        }
        .signature-section {
          margin-top: 40px;
          display: flex;
          justify-content: space-between;
        }
        .signature-item {
          text-align: center;
          min-width: 150px;
        }
        .signature-line {
          border-bottom: 1px solid #333;
          margin-bottom: 5px;
          height: 30px;
        }
        .footer {
          margin-top: 40px;
          text-align: center;
          font-size: 12px;
          color: #666;
          border-top: 1px solid #ccc;
          padding-top: 10px;
        }
        .action-buttons {
          text-align: center;
          margin: 20px;
          gap: 10px;
          display: flex;
          justify-content: center;
          flex-wrap: wrap;
        }
        .btn {
          padding: 10px 20px;
          font-size: 16px;
          border: none;
          border-radius: 4px;
          cursor: pointer;
          transition: background-color 0.3s;
        }
        .btn-primary {
          background: #007bff;
          color: white;
        }
        .btn-primary:hover {
          background: #0056b3;
        }
        .btn-success {
          background: #28a745;
          color: white;
        }
        .btn-success:hover {
          background: #1e7e34;
        }
        .btn-secondary {
          background: #6c757d;
          color: white;
        }
        .btn-secondary:hover {
          background: #545b62;
        }
        .treatment-section {
          margin: 15px 0;
          flex: 1;
        }
        .treatment-row {
          align-items: flex-start;
        }
        .treatment-row .treatment-section {
          margin: 0;
          margin-left: 10px;
        }
        .treatment-table {
          width: 100%;
          border-collapse: collapse;
          margin-top: 10px;
          margin-left: -10px;
        }
        .treatment-table th,
        .treatment-table td {
          border: none;
          padding: 8px 12px;
          text-align: left;
        }
        .treatment-table th {
          background-color: transparent;
          font-weight: bold;
          border-bottom: none;
        }
        .treatment-table td:first-child {
          text-align: left;
          width: 40px;
        }
        .treatment-table th:first-child {
          width: 40px;
        }
        .treatment-table td:last-child {
          text-align: center;
          width: 80px;
        }
        .no-treatment {
          color: #666;
          font-style: italic;
          padding: 10px 0;
        }
        .prescription-table {
          width: 100%;
          border-collapse: collapse;
          margin-top: 10px;
          margin-left: -10px;
        }
        .prescription-table td {
          border: none;
          padding: 8px 12px;
          text-align: left;
          width: 25%;
          vertical-align: top;
        }
        /* 治疗处方单样式 */
        .signature-row {
          display: flex;
          gap: 40px;
        }
        .signature-item {
          display: flex;
          align-items: flex-start;
          flex: 1;
          min-width: 250px;
        }
        .signature-item .form-label {
          min-width: 80px;
          margin-right: 10px;
          flex-shrink: 0;
        }
        .signature-space {
          border-bottom: none;
          padding: 2px 5px;
          min-height: 50px;
          flex: 1;
          display: inline-block;
          margin-right: 20px;
        }
        /* 空白区域样式 */
        .blank-space-small {
          height: 60px;
          margin-bottom: 15px;
        }
        .blank-space-large {
          height: 150px;
          margin-top: 20px;
        }
        @media print {
          body { margin: 0; }
          .no-print { display: none; }
          @page {
            margin: 0.5in;
            size: A4;
            @top-left { content: ""; }
            @top-center { content: ""; }
            @top-right { content: ""; }
            @bottom-left { content: ""; }
            @bottom-center { content: ""; }
            @bottom-right { content: ""; }
          }
        }
      </style>
    </head>
    <body>
      ${htmlContent}
      ${
        showPdfOption
          ? `
      <div class="no-print action-buttons">
        <button onclick="handlePrint()" class="btn btn-primary">
          🖨️ 打印
        </button>
        <button onclick="saveToPdf()" class="btn btn-success">
          📄 保存为PDF
        </button>
        <button onclick="window.close()" class="btn btn-secondary">
          ❌ 关闭
        </button>
      </div>
      <script>
        function handlePrint() {
          // 尝试设置打印选项，隐藏页眉页脚
          if (window.chrome) {
            // Chrome浏览器的打印设置
            const printSettings = {
              shouldPrintBackgrounds: false,
              shouldPrintSelectionOnly: false,
              marginsType: 1, // NO_MARGINS
              isHeaderFooterEnabled: false
            };
          }
          window.print();
        }
        
        function saveToPdf() {
          // 隐藏按钮
          const buttons = document.querySelector('.action-buttons');
          if (buttons) buttons.style.display = 'none';
          
          // 调用浏览器打印对话框，用户可以选择"保存为PDF"
          const originalTitle = document.title;
          document.title = '${filename}';
          
          // 尝试设置打印选项
          if (window.chrome) {
            const printSettings = {
              shouldPrintBackgrounds: false,
              shouldPrintSelectionOnly: false,
              marginsType: 1,
              isHeaderFooterEnabled: false
            };
          }
          
          window.print();
          
          // 恢复按钮显示
          setTimeout(() => {
            if (buttons) buttons.style.display = 'flex';
            document.title = originalTitle;
          }, 1000);
        }

        // 页面加载完成后移除可能的页眉页脚样式
        window.addEventListener('DOMContentLoaded', function() {
          // 创建样式来隐藏页眉页脚
          const style = document.createElement('style');
          style.textContent = \`
            @media print {
              @page { margin: 0; }
              body { margin: 1cm; }
            }
          \`;
          document.head.appendChild(style);
        });
      </script>
      `
          : `
      <div class="no-print action-buttons">
        <button onclick="handleSimplePrint()" class="btn btn-primary">🖨️ 打印</button>
        <button onclick="window.close()" class="btn btn-secondary">❌ 关闭</button>
      </div>
      <script>
        function handleSimplePrint() {
          // 尝试设置打印选项，隐藏页眉页脚
          if (window.chrome) {
            const printSettings = {
              shouldPrintBackgrounds: false,
              shouldPrintSelectionOnly: false,
              marginsType: 1,
              isHeaderFooterEnabled: false
            };
          }
          window.print();
        }

        // 页面加载完成后移除可能的页眉页脚样式
        window.addEventListener('DOMContentLoaded', function() {
          const style = document.createElement('style');
          style.textContent = \`
            @media print {
              @page { margin: 0; }
              body { margin: 1cm; }
            }
          \`;
          document.head.appendChild(style);
        });
      </script>
      `
      }
    </body>
    </html>
  `

  // 使用 iframe 方法替代 window.open，避免被弹窗拦截
  const iframe = document.createElement('iframe')
  iframe.style.display = 'none'
  document.body.appendChild(iframe)

  try {
    const iframeDoc = iframe.contentDocument || iframe.contentWindow?.document
    if (!iframeDoc) {
      throw new Error('无法访问iframe的document对象')
    }

    // 写入内容到 iframe
    iframeDoc.open()
    iframeDoc.write(printContent)
    iframeDoc.close()

    // 等待内容加载完成后打印
    iframe.onload = () => {
      try {
        iframe.contentWindow?.print()
      } catch (error) {
        console.error('打印失败:', error)
        throw new Error('打印窗口加载失败，请重试')
      }
    }

    // 设置超时，如果加载太慢就直接尝试打印
    const timeout = setTimeout(() => {
      try {
        iframe.contentWindow?.print()
      } catch (error) {
        console.error('打印超时:', error)
      }
    }, 2000)

    // 页面加载完成后清除超时
    iframe.addEventListener('load', () => {
      clearTimeout(timeout)
    })

    // 打印完成或用户取消后删除 iframe
    iframe.contentWindow?.addEventListener('afterprint', () => {
      document.body.removeChild(iframe)
    })

    // 备用：延迟删除 iframe（如果 afterprint 事件未触发）
    setTimeout(() => {
      if (iframe.parentNode) {
        document.body.removeChild(iframe)
      }
    }, 5000)
  } catch (error) {
    document.body.removeChild(iframe)
    console.error('创建打印窗口失败:', error)
    throw new Error('无法打开打印窗口，请检查浏览器设置')
  }
}
