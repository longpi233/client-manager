<template>
  <div class="prescription-detail">
    <ElDescriptions title="基本信息" :column="3" border style="margin-bottom: 20px">
      <ElDescriptionsItem label="患者姓名">{{ data.customer_name }}</ElDescriptionsItem>
      <ElDescriptionsItem label="性别">
        {{ formatGender(data.gender) }}
      </ElDescriptionsItem>
      <ElDescriptionsItem label="年龄">{{ data.age }}岁</ElDescriptionsItem>
      <ElDescriptionsItem label="开方时间">{{
        formatDateTime(data.time_of_visit)
      }}</ElDescriptionsItem>
      <ElDescriptionsItem label="医师">{{ data.doctor_name }}</ElDescriptionsItem>
      <ElDescriptionsItem label="付数">{{ data.doses }}付</ElDescriptionsItem>
      <ElDescriptionsItem label="诊断" :span="3">{{ data.diagnosis }}</ElDescriptionsItem>
      <ElDescriptionsItem label="备注">{{ data.note || '-' }}</ElDescriptionsItem>
    </ElDescriptions>

    <h4 style="margin: 20px 0 10px">处方明细</h4>
    <ElTable :data="data.prescription_items || []" border style="width: 100%">
      <ElTableColumn type="index" label="序号" width="60" />
      <ElTableColumn prop="medicine_name" label="中药名称" />
      <ElTableColumn prop="grams" label="克数" width="100">
        <template #default="{ row }"> {{ row.grams }}g </template>
      </ElTableColumn>
      <ElTableColumn prop="unit_price" label="单价(元/g)" width="120">
        <template #default="{ row }">
          {{ formatAmountFromTenThousandths(row.unit_price || 0) }}
        </template>
      </ElTableColumn>
      <ElTableColumn prop="total_price" label="金额(元)" width="100">
        <template #default="{ row }">
          {{ formatAmountFromTenThousandths((row.grams || 0) * (row.unit_price || 0)) }}
        </template>
      </ElTableColumn>
    </ElTable>

    <!-- 费用汇总 -->
    <div
      v-if="data.prescription_items && data.prescription_items.length > 0"
      style="margin-top: 16px"
    >
      <h5 style="margin: 0 0 16px; color: var(--el-text-color-primary)">费用汇总</h5>
      <ElDescriptions :column="4" border>
        <ElDescriptionsItem label="单付金额">
          {{
            formatAmountFromTenThousandths(
              data.prescription_items.reduce(
                (sum: number, item: any) => sum + (item.total_price || 0),
                0
              )
            )
          }}
        </ElDescriptionsItem>
        <ElDescriptionsItem label="总金额">
          {{
            formatAmountFromTenThousandths(
              data.prescription_items.reduce(
                (sum: number, item: any) => sum + (item.total_price || 0),
                0
              ) * (data?.doses || 1)
            )
          }}
        </ElDescriptionsItem>
      </ElDescriptions>
    </div>
  </div>
</template>

<script setup lang="ts">
  import {
    formatDateTime,
    formatGender,
    formatAmountFromTenThousandths
  } from '@/utils/dataprocess/medicalFormatter'
  import type { UnifiedMedicalRecord } from '@/types/medical'

  defineOptions({ name: 'PrescriptionDetail' })

  interface Props {
    data: UnifiedMedicalRecord
  }

  defineProps<Props>()
</script>
