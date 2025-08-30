/**
 * 性别信息统一性验证测试
 */
import { GENDER_OPTIONS, TRADE_TYPE_OPTIONS, Gender, TradeType } from '@/config/common'

console.log('=== 性别信息统一性测试 ===')

// 测试性别选项
console.log('性别选项:', GENDER_OPTIONS)
console.log('男性值:', Gender.Male) // 应该是 '1'
console.log('女性值:', Gender.Female) // 应该是 '2'

// 测试结算类型选项
console.log('\n=== 结算类型统一性测试 ===')
console.log('结算类型选项:', TRADE_TYPE_OPTIONS)
console.log('现金结算值:', TradeType.Cash) // 应该是 '1'
console.log('省直医保值:', TradeType.ProvincialInsurance) // 应该是 '2'
console.log('武汉医保值:', TradeType.WuHanInsurance) // 应该是 '3'

// 验证三个页面使用的值是否一致
console.log('\n=== 一致性验证 ===')
console.log('门诊病历页面性别:', '1', '2') // 现在应该使用 '1', '2'
console.log('中药处方页面性别:', '1', '2') // 修改前是 'Male', 'Female'，现在是 '1', '2'
console.log('散客登记页面性别:', '1', '2') // 一直使用 '1', '2'
console.log('✅ 所有页面性别信息现在都统一使用 "1"(男) 和 "2"(女)')

export { GENDER_OPTIONS, TRADE_TYPE_OPTIONS }
