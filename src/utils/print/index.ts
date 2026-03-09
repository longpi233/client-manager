/**
 * 统一打印入口
 * 导出所有打印功能
 */

// 导出类型定义
export type {
  PrintableItem,
  MedicalRecordData,
  PrescriptionData,
  FeeListData,
  TreatmentPrescriptionData,
  PrintOptions
} from './printUtils'

// 导出工具函数
export {
  formatGender,
  formatTradeType,
  formatDateTime,
  formatDateChinese,
  formatAmount,
  getInstitutionInfo,
  printDocument
} from './printUtils'

// 导出门诊病历打印
export { generateMedicalRecordHTML, printMedicalRecord } from './medicalRecordPrint'

// 导出费用清单打印
export { generateFeeListHTML, printFeeList } from './feeListPrint'

// 导出中医中药打印
export {
  generatePrescriptionHTML,
  printPrescription,
  generateChineseMedicalRecordHTML,
  printChineseMedicalRecord
} from './prescriptionPrint'

// 导出治疗处方单打印
export {
  generateTreatmentPrescriptionHTML,
  printTreatmentPrescription
} from './treatmentPrescriptionPrint'

/**
 * 统一打印函数
 * 根据数据类型自动选择对应的打印模板
 */
import { printMedicalRecord } from './medicalRecordPrint'
import { printFeeList } from './feeListPrint'
import { printPrescription } from './prescriptionPrint'
import type { MedicalRecordData, PrescriptionData, FeeListData } from './printUtils'

export interface UnifiedPrintData {
  type: 'medical' | 'fee' | 'prescription'
  data: MedicalRecordData | PrescriptionData | FeeListData
}

/**
 * 统一打印方法
 */
export const unifiedPrint = (printData: UnifiedPrintData): void => {
  try {
    switch (printData.type) {
      case 'medical':
        printMedicalRecord(printData.data as MedicalRecordData)
        break
      case 'fee':
        printFeeList(printData.data as FeeListData)
        break
      case 'prescription':
        printPrescription(printData.data as PrescriptionData)
        break
      default:
        throw new Error(`不支持的打印类型: ${printData.type}`)
    }
  } catch (error) {
    console.error('统一打印失败:', error)
    throw error
  }
}

/**
 * 批量打印
 */
export const batchPrint = (printDataList: UnifiedPrintData[]): void => {
  try {
    printDataList.forEach((printData, index) => {
      setTimeout(() => {
        unifiedPrint(printData)
      }, index * 500) // 每个打印任务间隔500ms，避免弹窗冲突
    })
  } catch (error) {
    console.error('批量打印失败:', error)
    throw error
  }
}
