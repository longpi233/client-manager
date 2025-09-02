<template>
  <div class="customer-detail">
    <ElDescriptions title="基本信息" :column="3" border style="margin-bottom: 20px">
      <ElDescriptionsItem label="患者姓名">{{ data.customer_name }}</ElDescriptionsItem>
      <ElDescriptionsItem label="患者性别">
        {{ formatGender(data.gender) }}
      </ElDescriptionsItem>
      <ElDescriptionsItem label="年龄">{{ data.age }}岁</ElDescriptionsItem>
      <ElDescriptionsItem label="联系电话">{{ data.telephone }}</ElDescriptionsItem>
      <ElDescriptionsItem label="就诊时间">{{
        formatDateTime(data.time_of_visit)
      }}</ElDescriptionsItem>
      <ElDescriptionsItem label="结算类型">{{
        getTradeTypeName(data.trade_type || '1')
      }}</ElDescriptionsItem>
      <ElDescriptionsItem label="诊断信息" :span="2">{{
        data.diagnosis || '-'
      }}</ElDescriptionsItem>
      <ElDescriptionsItem label="备注" :span="3">{{ data.note || '-' }}</ElDescriptionsItem>
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
      <ElTableColumn prop="count" label="数量" width="100" align="center">
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
        <ElDescriptionsItem label="总数量">
          {{
            data.treatment_items.reduce((sum: number, item: any) => sum + (item.count || 0), 0)
          }}次
        </ElDescriptionsItem>
        <ElDescriptionsItem label="总金额">
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

  defineOptions({ name: 'CustomerDetail' })

  interface Props {
    data: UnifiedMedicalRecord
  }

  defineProps<Props>()
</script>
