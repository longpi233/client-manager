/**
 * 打印功能测试
 * 验证各种打印模板是否正常工作
 */

import {
  printMedicalRecord,
  printFeeList,
  printPrescription,
  type MedicalRecordData,
  type FeeListData,
  type PrescriptionData
} from '@/utils/print'

// 测试数据
const testMedicalRecord: MedicalRecordData = {
  customer_name: '张三',
  gender: '1',
  age: 35,
  telephone: '13812345678',
  time_of_visit: '2024-01-15 10:30:00',
  trade_type: '1',
  chief_complaint: '头痛三天，伴有发热',
  illness_history: '患者3天前开始出现头痛，持续性钝痛，伴有轻度发热',
  physical_examination: '体温38.2℃，血压120/80mmHg，神清语明',
  past_history: '既往体健，无药物过敏史',
  laboratory_examination: '血常规：白细胞计数略高',
  diagnosis: '感冒，头痛',
  treatment_method: '清热解毒，止痛',
  note: '注意休息，多饮水',
  treatment_items: [
    {
      treatment_item: '针灸治疗',
      count: 1,
      unit_price: 50,
      total_price: 50
    },
    {
      treatment_item: '中药煎剂',
      count: 3,
      unit_price: 25,
      total_price: 75
    }
  ],
  doctor_name: '李医生'
}

const testFeeList: FeeListData = {
  customer_name: '李四',
  gender: '2',
  age: 28,
  telephone: '13987654321',
  time_of_visit: '2024-01-15 14:20:00',
  trade_type: '2',
  diagnosis: '腰椎间盘突出',
  note: '建议定期复查',
  treatment_items: [
    {
      treatment_item: '推拿按摩',
      count: 1,
      unit_price: 80,
      total_price: 80
    },
    {
      treatment_item: '理疗',
      count: 1,
      unit_price: 60,
      total_price: 60
    }
  ],
  total_amount: 140,
  doctor_name: '王医生'
}

const testPrescription: PrescriptionData = {
  customer_name: '王五',
  gender: '1',
  age: 45,
  telephone: '13566778899',
  time_of_visit: '2024-01-15 16:45:00',
  trade_type: '1',
  diagnosis: '失眠，神经衰弱',
  note: '睡前服用，忌食辛辣',
  prescription_items: [
    {
      medicine_name: '酸枣仁',
      grams: 15,
      unit_price: 0.8,
      total_price: 12
    },
    {
      medicine_name: '远志',
      grams: 10,
      unit_price: 1.2,
      total_price: 12
    },
    {
      medicine_name: '当归',
      grams: 12,
      unit_price: 1.5,
      total_price: 18
    }
  ],
  doses: 7,
  single_dose_amount: 42,
  total_amount: 294,
  doctor_name: '赵医生'
}

// 测试函数
export function testPrintFunctions() {
  console.log('开始测试打印功能...')

  try {
    // 测试门诊病历打印
    console.log('测试门诊病历打印...')
    printMedicalRecord(testMedicalRecord)

    // 测试费用清单打印
    console.log('测试费用清单打印...')
    printFeeList(testFeeList)

    // 测试中药处方打印
    console.log('测试中药处方打印...')
    printPrescription(testPrescription)

    console.log('所有打印功能测试完成！')
    return true
  } catch (error) {
    console.error('打印功能测试失败:', error)
    return false
  }
}

// 如果在浏览器环境中运行，添加到全局对象用于调试
if (typeof window !== 'undefined') {
  ;(window as any).testPrintFunctions = testPrintFunctions
  ;(window as any).testData = {
    medicalRecord: testMedicalRecord,
    feeList: testFeeList,
    prescription: testPrescription
  }
}
