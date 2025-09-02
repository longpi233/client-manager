<template>
  <div class="medical-detail">
    <ElDescriptions title="基本信息" :column="3" border style="margin-bottom: 20px">
      <ElDescriptionsItem label="患者姓名">{{ data.customer_name }}</ElDescriptionsItem>
      <ElDescriptionsItem label="性别">
        {{ formatGender(data.gender) }}
      </ElDescriptionsItem>
      <ElDescriptionsItem label="年龄">{{ data.age }}岁</ElDescriptionsItem>
      <ElDescriptionsItem label="就诊时间">{{
        formatDateTime(data.time_of_visit)
      }}</ElDescriptionsItem>
      <ElDescriptionsItem label="医师">{{ data.doctor_name }}</ElDescriptionsItem>
      <ElDescriptionsItem label="结算类型">{{
        getTradeTypeName(data.trade_type)
      }}</ElDescriptionsItem>
    </ElDescriptions>

    <ElDescriptions title="诊疗信息" :column="1" border>
      <ElDescriptionsItem label="主诉">{{ data.chief_complaint }}</ElDescriptionsItem>
      <ElDescriptionsItem label="现病史">{{ data.illness_history || '-' }}</ElDescriptionsItem>
      <ElDescriptionsItem label="体格检查">{{
        data.physical_examination || '-'
      }}</ElDescriptionsItem>
      <ElDescriptionsItem label="既往史">{{ data.past_history || '-' }}</ElDescriptionsItem>
      <ElDescriptionsItem label="实验室检查">{{
        data.laboratory_examination || '-'
      }}</ElDescriptionsItem>
      <ElDescriptionsItem label="诊断">{{ data.diagnosis }}</ElDescriptionsItem>
      <ElDescriptionsItem label="治法治则">{{ data.treatment_method || '-' }}</ElDescriptionsItem>
      <ElDescriptionsItem label="备注">{{ data.note || '-' }}</ElDescriptionsItem>
    </ElDescriptions>

    <!-- 治疗项目明细 -->
    <h4 style="margin: 20px 0 10px">治疗项目明细</h4>
    <ElTable
      :data="data.treatment_items || []"
      border
      style="width: 100%"
      :empty-text="
        data.treatment_items === undefined
          ? '加载中...'
          : data.treatment_items.length === 0
            ? '暂无治疗项目'
            : ''
      "
    >
      <ElTableColumn type="index" label="序号" width="60" />
      <ElTableColumn prop="treatment_item" label="治疗项目" min-width="150" />
      <ElTableColumn prop="count" label="次数" width="100" align="center">
        <template #default="{ row }">{{ row.count }}</template>
      </ElTableColumn>
      <ElTableColumn prop="unit_price" label="单价(元)" width="120" align="right">
        <template #default="{ row }">{{
          formatAmountFromTenThousandths(row.unit_price || 0)
        }}</template>
      </ElTableColumn>
      <ElTableColumn prop="total_price" label="小计(元)" width="120" align="right">
        <template #default="{ row }">
          <span style="font-weight: bold; color: #f56c6c">{{
            formatAmountFromTenThousandths(row.total_price || 0)
          }}</span>
        </template>
      </ElTableColumn>
    </ElTable>

    <!-- 费用汇总 -->
    <div v-if="data.treatment_items && data.treatment_items.length > 0" style="margin-top: 16px">
      <h5 style="margin: 0 0 16px; color: var(--el-text-color-primary)">费用汇总</h5>
      <ElDescriptions :column="4" border>
        <ElDescriptionsItem label="治疗项目">
          {{ data.treatment_items.length }}项
        </ElDescriptionsItem>
        <ElDescriptionsItem label="总次数">
          {{
            data.treatment_items.reduce((sum: number, item: any) => sum + (item.count || 0), 0)
          }}次
        </ElDescriptionsItem>
        <ElDescriptionsItem label="总费用">
          {{
            formatAmountFromTenThousandths(
              data.treatment_items.reduce(
                (sum: number, item: any) => sum + (item.total_price || 0),
                0
              )
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
    formatAmountFromTenThousandths,
    getTradeTypeName
  } from '@/utils/dataprocess/medicalFormatter'
  import type { UnifiedMedicalRecord } from '@/types/medical'

  defineOptions({ name: 'MedicalDetail' })

  interface Props {
    data: UnifiedMedicalRecord
  }

  defineProps<Props>()
</script>
