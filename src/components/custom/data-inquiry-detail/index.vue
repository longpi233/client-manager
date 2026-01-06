<template>
  <ElDialog
    v-model="visible"
    :title="`${queryType === 'customer' ? '散客信息' : queryType === 'medical' ? '病历' : '处方'}详情`"
    width="80%"
    :before-close="handleClose"
  >
    <div class="detail-content">
      <!-- 散客详情 -->
      <CustomerDetail v-if="queryType === 'customer' && selectedItem" :data="selectedItem" />

      <!-- 病历详情 -->
      <MedicalDetail v-if="queryType === 'medical' && selectedItem" :data="selectedItem" />

      <!-- 处方详情 -->
      <PrescriptionDetail
        v-if="queryType === 'prescription' && selectedItem"
        :data="selectedItem"
      />
    </div>

    <template #footer>
      <ElButton @click="handleClose">关闭</ElButton>
      <!-- 散客登记：暂不支持打印 -->
      <template v-if="queryType === 'customer'">
        <ElButton type="info" disabled>散客登记暂不支持打印</ElButton>
      </template>
      <!-- 门诊病历：支持打印病历、费用清单和治疗处方单 -->
      <template v-if="queryType === 'medical'">
        <ElButton type="success" @click="handlePrintMedicalRecord">打印门诊病历</ElButton>
        <ElButton type="warning" @click="handlePrintMedicalFeeList">打印费用清单</ElButton>
        <ElButton type="primary" @click="handlePrintTreatmentPrescription">打印治疗处方单</ElButton>
      </template>
      <!-- 中药处方：支持打印处方和费用清单 -->
      <template v-if="queryType === 'prescription'">
        <ElButton type="success" @click="handlePrintPrescription">打印中药处方</ElButton>
        <!-- <ElButton type="warning" @click="handlePrintPrescriptionFeeList">打印费用清单</ElButton> -->
      </template>
    </template>
  </ElDialog>
</template>

<script setup lang="ts">
  import { ref, watch, computed } from 'vue'
  import { ElMessage } from 'element-plus'
  import CustomerDetail from './components/CustomerDetail.vue'
  import MedicalDetail from './components/MedicalDetail.vue'
  import PrescriptionDetail from './components/PrescriptionDetail.vue'
  import {
    printMedicalRecord,
    printFeeList,
    printPrescription,
    printTreatmentPrescription,
    type MedicalRecordData,
    type FeeListData,
    type PrescriptionData,
    type TreatmentPrescriptionData
  } from '@/utils/print'
  import type { UnifiedMedicalRecord } from '@/types/medical'

  defineOptions({ name: 'DataInquiryDetail' })

  interface Props {
    modelValue: boolean
    selectedItem: UnifiedMedicalRecord | null
  }

  interface Emits {
    (e: 'update:modelValue', value: boolean): void
  }

  const props = defineProps<Props>()
  const emit = defineEmits<Emits>()

  const queryType = computed(() => {
    return props.selectedItem ? props.selectedItem.data_type : null
  })

  const visible = ref(props.modelValue)

  // 监听外部状态变化
  watch(
    () => props.modelValue,
    (newValue) => {
      visible.value = newValue
    }
  )

  // 监听内部状态变化
  watch(visible, (newValue) => {
    emit('update:modelValue', newValue)
  })

  // 关闭弹窗
  const handleClose = () => {
    visible.value = false
  }

  // 打印门诊病历
  const handlePrintMedicalRecord = async () => {
    if (!props.selectedItem) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建门诊病历打印数据
      const medicalData: MedicalRecordData = {
        customer_name: props.selectedItem.customer_name,
        gender: props.selectedItem.gender,
        age: props.selectedItem.age,
        telephone: props.selectedItem.telephone,
        time_of_visit: props.selectedItem.time_of_visit,
        trade_type: props.selectedItem.trade_type,
        chief_complaint: props.selectedItem.chief_complaint,
        illness_history: props.selectedItem.illness_history,
        physical_examination: props.selectedItem.physical_examination,
        past_history: props.selectedItem.past_history,
        laboratory_examination: props.selectedItem.laboratory_examination,
        diagnosis: props.selectedItem.diagnosis,
        treatment_method: props.selectedItem.treatment_method,
        note: props.selectedItem.note,
        treatment_items: (props.selectedItem.treatment_items || []).map((item: any) => ({
          treatment_item: item.treatment_item || item.treatment_item_name,
          count: item.count || item.treatment_count,
          unit_price: item.unit_price,
          total_price: item.total_price
        })),
        doctor_name: props.selectedItem.doctor_name
      }

      await printMedicalRecord(medicalData)
      ElMessage.success('门诊病历打印成功')
    } catch (error) {
      console.error('打印门诊病历失败:', error)
      ElMessage.error('打印失败，请重试')
    }
  }

  // 打印门诊费用清单
  const handlePrintMedicalFeeList = async () => {
    if (!props.selectedItem) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建费用清单打印数据
      const feeListData: FeeListData = {
        customer_name: props.selectedItem.customer_name,
        gender: props.selectedItem.gender,
        age: props.selectedItem.age,
        telephone: props.selectedItem.telephone,
        time_of_visit: props.selectedItem.time_of_visit,
        trade_type: props.selectedItem.trade_type,
        diagnosis: props.selectedItem.diagnosis,
        note: props.selectedItem.note,
        treatment_items: (props.selectedItem.treatment_items || []).map((item: any) => ({
          treatment_item: item.treatment_item || item.treatment_item_name,
          count: item.count || item.treatment_count,
          unit_price: item.unit_price,
          total_price: item.total_price
        })),
        total_amount: (props.selectedItem.treatment_items || []).reduce(
          (sum: number, item: any) => sum + (item.total_price || 0),
          0
        ),
        doctor_name: props.selectedItem.doctor_name
      }

      await printFeeList(feeListData)
      ElMessage.success('费用清单打印成功')
    } catch (error) {
      console.error('打印费用清单失败:', error)
      ElMessage.error('打印失败，请重试')
    }
  }

  // 打印治疗处方单
  const handlePrintTreatmentPrescription = async () => {
    if (!props.selectedItem) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建治疗处方单打印数据
      const treatmentPrescriptionData: TreatmentPrescriptionData = {
        customer_name: props.selectedItem.customer_name,
        gender: props.selectedItem.gender,
        age: props.selectedItem.age,
        trade_type: props.selectedItem.trade_type,
        diagnosis: props.selectedItem.diagnosis,
        treatment_method: props.selectedItem.treatment_method,
        treatment_items: (props.selectedItem.treatment_items || []).map((item: any) => ({
          treatment_item: item.treatment_item || item.treatment_item_name,
          count: item.count || item.treatment_count
        })),
        note: props.selectedItem.note,
        time_of_visit: props.selectedItem.time_of_visit
      }

      await printTreatmentPrescription(treatmentPrescriptionData)
      ElMessage.success('治疗处方单打印成功')
    } catch (error) {
      console.error('打印治疗处方单失败:', error)
      ElMessage.error('打印失败，请重试')
    }
  }

  // 打印中药处方
  const handlePrintPrescription = async () => {
    if (!props.selectedItem) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建中药处方打印数据
      const prescriptionData: PrescriptionData = {
        customer_name: props.selectedItem.customer_name,
        gender: props.selectedItem.gender,
        age: props.selectedItem.age,
        telephone: props.selectedItem.telephone,
        time_of_visit: props.selectedItem.time_of_visit,
        trade_type: props.selectedItem.trade_type,
        diagnosis: props.selectedItem.diagnosis,
        note: props.selectedItem.note,
        prescription_items: (props.selectedItem.prescription_items || []).map((item: any) => ({
          medicine_name: item.medicine_name,
          grams: item.grams,
          unit_price: item.unit_price,
          total_price: item.total_price
        })),
        doses: props.selectedItem.doses,
        single_dose_amount: props.selectedItem.single_dose_amount,
        total_amount: props.selectedItem.total_amount,
        doctor_name: props.selectedItem.doctor_name
      }

      await printPrescription(prescriptionData)
      ElMessage.success('中药处方打印成功')
    } catch (error) {
      console.error('打印中药处方失败:', error)
      ElMessage.error('打印失败，请重试')
    }
  }

  //   // 打印中药处方费用清单
  //   const handlePrintPrescriptionFeeList = async () => {
  //     if (!props.selectedItem) {
  //       ElMessage.warning('请先选择要打印的记录')
  //       return
  //     }

  //     try {
  //       // 构建费用清单打印数据（基于中药处方）
  //       const feeListData: FeeListData = {
  //         customer_name: props.selectedItem.customer_name,
  //         gender: props.selectedItem.gender,
  //         age: props.selectedItem.age,
  //         telephone: props.selectedItem.telephone,
  //         time_of_visit: props.selectedItem.time_of_visit,
  //         trade_type: props.selectedItem.trade_type,
  //         diagnosis: props.selectedItem.diagnosis,
  //         note: props.selectedItem.note,
  //         treatment_items: (props.selectedItem.prescription || []).map((item: any) => {
  //           const singleTotal = item.grams * item.unit_price
  //           return {
  //             treatment_item: `${item.medicine_name} (${item.grams}g)`,
  //             count: props.selectedItem?.doses || 1,
  //             unit_price: singleTotal,
  //             total_price: singleTotal * (props.selectedItem?.doses || 1)
  //           }
  //         }),
  //         total_amount: props.selectedItem.total_amount,
  //         doctor_name: props.selectedItem.doctor_name
  //       }

  //       await printFeeList(feeListData)
  //       ElMessage.success('费用清单打印成功')
  //     } catch (error) {
  //       console.error('打印费用清单失败:', error)
  //       ElMessage.error('打印失败，请重试')
  //     }
  //   }
</script>

<style lang="scss" scoped>
  .detail-content {
    max-height: 70vh;
    overflow-y: auto;

    :deep(.el-descriptions) {
      margin-bottom: 20px;
    }

    :deep(h4) {
      padding-bottom: 8px;
      color: var(--el-text-color-primary);
      border-bottom: 2px solid var(--el-color-primary);
    }
  }
</style>
