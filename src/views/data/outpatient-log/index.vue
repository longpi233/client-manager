<template>
  <div class="outpatient-log-page">
    <!-- 页面标题 -->
    <div class="page-header">
      <h2>门诊日志</h2>
      <p>查看指定月份的门诊日志，包含散客登记、门诊病历、中药处方等综合信息</p>
    </div>

    <!-- 查询条件 -->
    <ElCard class="search-card" shadow="never">
      <template #header>
        <div class="card-header">
          <h4>查询条件</h4>
        </div>
      </template>

      <div class="search-form">
        <ElRow :gutter="20">
          <ElCol :span="6">
            <ElFormItem label="查询类型">
              <ElSelect
                v-model="searchParams.queryType"
                placeholder="请选择查询类型"
                style="width: 100%"
                @change="handleQueryTypeChange"
              >
                <ElOption label="按月份查询" value="month" />
                <ElOption label="按年份查询" value="year" />
                <ElOption label="自定义日期" value="custom" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem v-if="searchParams.queryType === 'month'" label="查询月份">
              <ElDatePicker
                v-model="searchParams.month"
                type="month"
                placeholder="请选择查询月份"
                format="YYYY年MM月"
                value-format="YYYY-MM"
                style="width: 100%"
                @change="handleSearch"
              />
            </ElFormItem>
            <ElFormItem v-else-if="searchParams.queryType === 'year'" label="查询年份">
              <ElDatePicker
                v-model="searchParams.year"
                type="year"
                placeholder="请选择查询年份"
                format="YYYY年"
                value-format="YYYY"
                style="width: 100%"
                @change="handleSearch"
              />
            </ElFormItem>
            <ElFormItem v-else-if="searchParams.queryType === 'custom'" label="日期范围">
              <ElDatePicker
                v-model="searchParams.dateRange"
                type="daterange"
                range-separator="至"
                start-placeholder="开始日期"
                end-placeholder="结束日期"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                style="width: 100%"
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="数据类型">
              <ElSelect
                v-model="searchParams.dataType"
                placeholder="请选择数据类型"
                clearable
                style="width: 100%"
                @change="handleSearch"
              >
                <ElOption label="全部" value="" />
                <ElOption label="散客登记" value="customer" />
                <ElOption label="门诊病历" value="medical" />
                <ElOption label="中药处方" value="prescription" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="结算类型">
              <ElSelect
                v-model="searchParams.tradeType"
                placeholder="请选择结算类型"
                clearable
                style="width: 100%"
                @change="handleSearch"
              >
                <ElOption label="全部" value="" />
                <ElOption label="现金结算" value="1" />
                <ElOption label="省直医保" value="2" />
                <ElOption label="武汉医保" value="3" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
        </ElRow>

        <div class="search-actions">
          <ElButton type="primary" @click="handleSearch" :loading="loading">
            <ElIcon><Search /></ElIcon>
            查询
          </ElButton>
          <ElButton @click="handleReset">
            <ElIcon><Refresh /></ElIcon>
            重置
          </ElButton>
          <ArtExcelExport
            :data="
              allSearchResults.map((item) => ({
                ...item,
                treatment_items: undefined,
                raw_data: undefined,
                prescription: undefined
              }))
            "
            :columns="exportColumns"
            :filename="generateOutpatientExportFilename()"
            button-text="导出数据"
            :disabled="!hasData"
            @export-success="handleExportSuccess"
            @export-error="handleExportError"
          >
            <template #default>
              <ElIcon><Download /></ElIcon>
              导出数据
            </template>
          </ArtExcelExport>
        </div>
      </div>
    </ElCard>

    <!-- 统计信息 -->
    <ElCard class="stats-card" shadow="never" v-if="hasData">
      <template #header>
        <div class="card-header">
          <h4>统计信息</h4>
          <div class="stats-info">
            <ElTag type="info"> {{ getQueryPeriodText() }}统计 </ElTag>
          </div>
        </div>
      </template>

      <ElRow :gutter="24">
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.total }}</div>
            <div class="stat-label">总记录数</div>
          </div>
        </ElCol>
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.customerCount }}</div>
            <div class="stat-label">散客登记</div>
          </div>
        </ElCol>
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.medicalCount }}</div>
            <div class="stat-label">门诊病历</div>
          </div>
        </ElCol>
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.prescriptionCount }}</div>
            <div class="stat-label">中药处方</div>
          </div>
        </ElCol>
      </ElRow>
      <ElRow :gutter="24" style="margin-top: 16px">
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value amount">¥{{ statistics.totalAmount.toFixed(2) }}</div>
            <div class="stat-label">总金额</div>
          </div>
        </ElCol>
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.cashCount }}</div>
            <div class="stat-label">现金结算</div>
          </div>
        </ElCol>
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.provincialInsuranceCount }}</div>
            <div class="stat-label">省直医保</div>
          </div>
        </ElCol>
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.wuhanInsuranceCount }}</div>
            <div class="stat-label">武汉医保</div>
          </div>
        </ElCol>
      </ElRow>
      <ElRow :gutter="24" style="margin-top: 16px">
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.treatmentItemsCount }}</div>
            <div class="stat-label">治疗项目数</div>
          </div>
        </ElCol>
        <ElCol :span="18">
          <!-- 空白占位 -->
        </ElCol>
      </ElRow>
    </ElCard>

    <!-- 图表展示 -->
    <ElCard class="charts-card" shadow="never" v-if="hasData">
      <template #header>
        <div class="card-header">
          <h4>数据分析</h4>
        </div>
      </template>

      <ElRow :gutter="24">
        <!-- 营业额趋势图 -->
        <ElCol :span="24">
          <div class="chart-container">
            <h5>营业额趋势</h5>
            <ArtLineChart
              :data="revenueChartData.data"
              :x-axis-data="revenueChartData.xAxisData"
              :colors="['#409EFF']"
              height="20rem"
              :show-tooltip="true"
              :show-legend="false"
              :smooth="true"
              :show-area="true"
              tooltip-formatter="function(params) { return params[0].name + '<br/>' + params[0].marker + '营业额: ¥' + params[0].value.toLocaleString() + '元'; }"
              y-axis-formatter="function(value) { return '¥' + value.toLocaleString(); }"
            />
          </div>
        </ElCol>
      </ElRow>

      <ElRow :gutter="24" style="margin-top: 24px">
        <!-- 数据类型占比饼图 -->
        <ElCol :span="12">
          <div class="chart-container">
            <h5>数据类型营业额占比</h5>
            <ArtRingChart
              :data="dataTypeChartData"
              :colors="['#409EFF', '#67C23A', '#E6A23C']"
              height="20rem"
              :show-tooltip="true"
              :show-legend="true"
              legend-position="bottom"
              :radius="['40%', '70%']"
              center-text="数据类型"
              tooltip-formatter="function(params) { return params.name + ': ¥' + params.value.toLocaleString() + '元 (' + params.percent + '%)'; }"
            />
          </div>
        </ElCol>
        <!-- 结算类型占比饼图 -->
        <ElCol :span="12">
          <div class="chart-container">
            <h5>结算类型营业额占比</h5>
            <ArtRingChart
              :data="tradeTypeChartData"
              :colors="['#67C23A', '#F56C6C', '#E6A23C', '#909399']"
              height="20rem"
              :show-tooltip="true"
              :show-legend="true"
              legend-position="bottom"
              :radius="['40%', '70%']"
              center-text="结算类型"
              tooltip-formatter="function(params) { return params.name + ': ¥' + params.value.toLocaleString() + '元 (' + params.percent + '%)'; }"
            />
          </div>
        </ElCol>
      </ElRow>
    </ElCard>

    <!-- 查询结果 -->
    <ElCard class="result-card" shadow="never">
      <template #header>
        <div class="card-header">
          <h4>门诊日志</h4>
          <div class="result-info">
            <ElTag v-if="loading" type="warning">查询中...</ElTag>
            <ElTag v-else-if="hasError" type="danger">查询失败</ElTag>
            <ElTag v-else type="success">找到 {{ tableData.length }} 条记录</ElTag>
          </div>
        </div>
      </template>

      <!-- 门诊日志表格 -->
      <ElTable
        :data="tableData"
        :loading="loading"
        stripe
        border
        style="width: 100%"
        :default-sort="{ prop: 'time_of_visit', order: 'descending' }"
        @sort-change="handleSortChange"
      >
        <ElTableColumn type="index" label="序号" width="60" />
        <ElTableColumn
          prop="time_of_visit"
          label="时间"
          width="160"
          sortable="custom"
          :sort-orders="['descending', 'ascending']"
        >
          <template #default="{ row }">
            {{ formatDateTime(row.time_of_visit) }}
          </template>
        </ElTableColumn>
        <ElTableColumn prop="customer_name" label="患者姓名" width="120" />
        <ElTableColumn prop="trade_type" label="结算类别" width="100">
          <template #default="{ row }">
            <ElTag :type="getTradeTypeTagType(row.trade_type)">
              {{ getTradeTypeName(row.trade_type) }}
            </ElTag>
          </template>
        </ElTableColumn>
        <ElTableColumn prop="data_type" label="数据类型" width="100">
          <template #default="{ row }">
            <ElTag :type="getDataTypeTagType(row.data_type)">
              {{ getDataTypeName(row.data_type) }}
            </ElTag>
          </template>
        </ElTableColumn>
        <ElTableColumn prop="diagnosis" label="诊断" min-width="150" show-overflow-tooltip />
        <ElTableColumn
          prop="treatment_items"
          label="治疗项目"
          min-width="200"
          show-overflow-tooltip
        >
          <template #default="{ row }">
            <span v-if="row.treatment_items_summary">
              {{ row.treatment_items_summary }}
            </span>
            <span v-else class="text-placeholder">-</span>
          </template>
        </ElTableColumn>
        <ElTableColumn prop="amount" label="结算金额" width="120" align="right">
          <template #default="{ row }">
            <span v-if="row.amount" class="amount-text"> ¥{{ row.amount.toFixed(2) }} </span>
            <span v-else class="text-placeholder">-</span>
          </template>
        </ElTableColumn>
        <ElTableColumn label="操作" width="120" fixed="right">
          <template #default="{ row }">
            <ElButton type="primary" size="small" @click="handleViewDetail(row)">
              查看详情
            </ElButton>
          </template>
        </ElTableColumn>
      </ElTable>

      <!-- 分页 -->
      <div class="pagination-wrapper">
        <ElPagination
          v-model:current-page="pagination.current"
          v-model:page-size="pagination.size"
          :total="pagination.total"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
      </div>
    </ElCard>

    <!-- 详情弹窗 -->
    <ElDialog
      v-model="detailVisible"
      :title="`${getDataTypeName(selectedItem?.data_type || '')}详情`"
      width="80%"
      :before-close="handleCloseDetail"
    >
      <div class="detail-content" v-if="selectedItem">
        <!-- 基本信息 -->
        <ElDescriptions title="基本信息" :column="3" border style="margin-bottom: 20px">
          <ElDescriptionsItem label="患者姓名">{{ selectedItem.customer_name }}</ElDescriptionsItem>
          <ElDescriptionsItem label="性别">
            {{ formatGender(selectedItem.gender) }}
          </ElDescriptionsItem>
          <ElDescriptionsItem label="年龄">{{ selectedItem.age }}岁</ElDescriptionsItem>
          <ElDescriptionsItem label="联系电话">{{
            selectedItem.phone || selectedItem.telephone
          }}</ElDescriptionsItem>
          <ElDescriptionsItem label="就诊时间">{{
            formatDateTime(selectedItem.time_of_visit)
          }}</ElDescriptionsItem>
          <ElDescriptionsItem label="结算类型">{{
            getTradeTypeName(selectedItem.trade_type)
          }}</ElDescriptionsItem>
          <ElDescriptionsItem label="数据类型" :span="2">
            <ElTag :type="getDataTypeTagType(selectedItem.data_type)">
              {{ getDataTypeName(selectedItem.data_type) }}
            </ElTag>
          </ElDescriptionsItem>
          <ElDescriptionsItem label="诊断信息" :span="3">{{
            selectedItem.diagnosis || '-'
          }}</ElDescriptionsItem>
        </ElDescriptions>

        <!-- 治疗项目详情 -->
        <div v-if="selectedItem.treatment_items && selectedItem.treatment_items.length > 0">
          <h4 style="margin: 20px 0 10px">治疗项目明细</h4>
          <ElTable :data="selectedItem.treatment_items" border style="width: 100%">
            <ElTableColumn type="index" label="序号" width="60" />
            <ElTableColumn prop="treatment_item" label="治疗项目" min-width="150" />
            <ElTableColumn prop="count" label="数量/次数" width="100" align="center" />
            <ElTableColumn prop="unit_price" label="单价(元)" width="120" align="right">
              <template #default="{ row }">¥{{ row.unit_price || 0 }}</template>
            </ElTableColumn>
            <ElTableColumn prop="total_price" label="小计(元)" width="120" align="right">
              <template #default="{ row }">
                <span class="amount-text">¥{{ row.total_price || 0 }}</span>
              </template>
            </ElTableColumn>
          </ElTable>
        </div>

        <!-- 处方详情（仅中药处方） -->
        <div
          v-if="
            selectedItem.data_type === 'prescription' &&
            selectedItem.prescription &&
            selectedItem.prescription.length > 0
          "
        >
          <h4 style="margin: 20px 0 10px">处方明细</h4>
          <ElTable :data="selectedItem.prescription" border style="width: 100%">
            <ElTableColumn type="index" label="序号" width="60" />
            <ElTableColumn prop="medicine_name" label="中药名称" />
            <ElTableColumn prop="grams" label="克数" width="100">
              <template #default="{ row }">{{ row.grams }}g</template>
            </ElTableColumn>
            <ElTableColumn prop="unit_price" label="单价(元/g)" width="120" />
            <ElTableColumn prop="total_price" label="金额(元)" width="100">
              <template #default="{ row }">
                ¥{{ (row.grams * row.unit_price).toFixed(2) }}
              </template>
            </ElTableColumn>
          </ElTable>

          <ElDescriptions title="处方信息" :column="2" border style="margin-top: 20px">
            <ElDescriptionsItem label="付数">{{ selectedItem.doses }}付</ElDescriptionsItem>
            <ElDescriptionsItem label="单付金额"
              >¥{{ selectedItem.single_dose_amount }}</ElDescriptionsItem
            >
            <ElDescriptionsItem label="总金额" :span="2">
              <span class="amount-text">¥{{ selectedItem.total_amount }}</span>
            </ElDescriptionsItem>
          </ElDescriptions>
        </div>

        <!-- 其他详细信息 -->
        <div v-if="selectedItem.data_type === 'medical'">
          <ElDescriptions title="诊疗信息" :column="1" border style="margin-top: 20px">
            <ElDescriptionsItem label="主诉">{{
              selectedItem.chief_complaint || '-'
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="现病史">{{
              selectedItem.illness_history || '-'
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="体格检查">{{
              selectedItem.physical_examination || '-'
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="既往史">{{
              selectedItem.past_history || '-'
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="实验室检查">{{
              selectedItem.laboratory_examination || '-'
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="治法治则">{{
              selectedItem.treatment_method || '-'
            }}</ElDescriptionsItem>
          </ElDescriptions>
        </div>

        <ElDescriptions :column="1" border style="margin-top: 20px">
          <ElDescriptionsItem label="备注">{{ selectedItem.note || '-' }}</ElDescriptionsItem>
        </ElDescriptions>
      </div>

      <template #footer>
        <ElButton @click="detailVisible = false">关闭</ElButton>
        <ElButton type="primary" @click="handlePrint">打印</ElButton>
      </template>
    </ElDialog>
  </div>
</template>

<script setup lang="ts">
  import { ref, reactive, computed, onMounted } from 'vue'
  import { ElMessage } from 'element-plus'
  import { Search, Refresh, Download } from '@element-plus/icons-vue'
  import { getOutpatientLogList, type OutpatientLogItem } from '@/api/outpatientLogApi'
  import { getInstitutionInfo } from '@/api/institutionApi'
  import ArtLineChart from '@/components/core/charts/art-line-chart/index.vue'
  import ArtRingChart from '@/components/core/charts/art-ring-chart/index.vue'
  import ArtExcelExport from '@/components/core/forms/art-excel-export/index.vue'
  import {
    formatDateTime,
    formatGender,
    formatAmount,
    getTradeTypeName,
    getDataTypeName,
    getTradeTypeTagType,
    getDataTypeTagType,
    generateExportFilename
  } from '@/utils/dataprocess/medicalFormatter'

  defineOptions({ name: 'OutpatientLog' })

  // 搜索参数
  const searchParams = reactive({
    queryType: 'month', // month, year, custom
    month: '',
    year: '',
    dateRange: [] as string[],
    dataType: '',
    tradeType: ''
  })

  // 加载状态
  const loading = ref(false)
  const hasError = ref(false)

  // 医师列表
  const doctorList = ref<{ id: number; name: string }[]>([])

  // 表格数据
  const tableData = ref<OutpatientLogItem[]>([])

  // 所有搜索结果（用于统计）
  const allSearchResults = ref<OutpatientLogItem[]>([])

  // 分页信息
  const pagination = reactive({
    current: 1,
    size: 20,
    total: 0
  })

  // 详情弹窗
  const detailVisible = ref(false)
  const selectedItem = ref<OutpatientLogItem | null>(null)

  // 统计信息
  const statistics = computed(() => {
    const stats = {
      total: allSearchResults.value.length,
      customerCount: 0,
      medicalCount: 0,
      prescriptionCount: 0,
      totalAmount: 0,
      cashCount: 0,
      provincialInsuranceCount: 0,
      wuhanInsuranceCount: 0,
      treatmentItemsCount: 0
    }

    allSearchResults.value.forEach((item) => {
      // 统计数据类型
      if (item.data_type === 'customer') stats.customerCount++
      else if (item.data_type === 'medical') stats.medicalCount++
      else if (item.data_type === 'prescription') stats.prescriptionCount++

      // 统计金额
      if (item.amount) stats.totalAmount += item.amount

      // 统计结算类型
      if (item.trade_type === '1') stats.cashCount++
      else if (item.trade_type === '2') stats.provincialInsuranceCount++
      else if (item.trade_type === '3') stats.wuhanInsuranceCount++

      // 统计治疗项目数
      if (item.treatment_items) stats.treatmentItemsCount += item.treatment_items.length
    })

    return stats
  })

  // 营业额趋势图数据
  const revenueChartData = computed(() => {
    if (allSearchResults.value.length === 0) {
      return { data: [], xAxisData: [] }
    }

    // 按日期分组统计营业额
    const dailyRevenue = new Map<string, number>()

    allSearchResults.value.forEach((item) => {
      if (item.amount && item.time_of_visit) {
        const date = item.time_of_visit.split(' ')[0] // 提取日期部分
        const currentAmount = dailyRevenue.get(date) || 0
        dailyRevenue.set(date, currentAmount + item.amount)
      }
    })

    // 转换为图表数据格式
    const sortedDates = Array.from(dailyRevenue.keys()).sort()
    const data = sortedDates.map((date) => dailyRevenue.get(date) || 0)

    // 优化 x 轴标签显示，跨年时显示年份
    const xAxisData = sortedDates.map((date) => {
      const dateObj = new Date(date)
      const year = dateObj.getFullYear()
      const month = dateObj.getMonth() + 1
      const day = dateObj.getDate()

      // 如果数据跨年，显示年/月/日格式
      const years = new Set(sortedDates.map((d) => new Date(d).getFullYear()))
      if (years.size > 1) {
        return `${year}/${month}/${day}`
      } else {
        // 单年内显示月/日格式
        return `${month}/${day}`
      }
    })

    return { data, xAxisData }
  })

  // 数据类型占比图数据（按营业额）
  const dataTypeChartData = computed(() => {
    const customerAmount = allSearchResults.value
      .filter((item) => item.data_type === 'customer')
      .reduce((sum, item) => sum + (item.amount || 0), 0)

    const medicalAmount = allSearchResults.value
      .filter((item) => item.data_type === 'medical')
      .reduce((sum, item) => sum + (item.amount || 0), 0)

    const prescriptionAmount = allSearchResults.value
      .filter((item) => item.data_type === 'prescription')
      .reduce((sum, item) => sum + (item.amount || 0), 0)

    return [
      { name: '散客登记', value: customerAmount },
      { name: '门诊病历', value: medicalAmount },
      { name: '中药处方', value: prescriptionAmount }
    ].filter((item) => item.value > 0)
  })

  // 结算类型占比图数据（按营业额）
  const tradeTypeChartData = computed(() => {
    const cashAmount = allSearchResults.value
      .filter((item) => item.trade_type === '1')
      .reduce((sum, item) => sum + (item.amount || 0), 0)

    const provincialInsuranceAmount = allSearchResults.value
      .filter((item) => item.trade_type === '2')
      .reduce((sum, item) => sum + (item.amount || 0), 0)

    const wuhanInsuranceAmount = allSearchResults.value
      .filter((item) => item.trade_type === '3')
      .reduce((sum, item) => sum + (item.amount || 0), 0)

    const data = [
      { name: '现金结算', value: cashAmount },
      { name: '省直医保', value: provincialInsuranceAmount },
      { name: '武汉医保', value: wuhanInsuranceAmount }
    ].filter((item) => item.value > 0)

    // 如果有其他结算类型，也添加进去
    const totalKnownAmount = cashAmount + provincialInsuranceAmount + wuhanInsuranceAmount
    const otherAmount = statistics.value.totalAmount - totalKnownAmount
    if (otherAmount > 0) {
      data.push({ name: '其他结算', value: otherAmount })
    }

    return data
  })

  // 计算属性
  const hasData = computed(() => tableData.value.length > 0)

  // 导出列配置
  const exportColumns = computed(() => ({
    time_of_visit: {
      title: '就诊时间',
      formatter: (value: any) => formatDateTime(value as string)
    },
    customer_name: {
      title: '患者姓名'
    },
    gender: {
      title: '性别',
      formatter: (value: any) => formatGender(value as string)
    },
    age: {
      title: '年龄'
    },
    data_type: {
      title: '数据类型',
      formatter: (value: any) => getDataTypeName(value as string)
    },
    trade_type: {
      title: '结算类别',
      formatter: (value: any) => getTradeTypeName(value as string)
    },
    diagnosis: {
      title: '诊断信息'
    },
    treatment_items_summary: {
      title: '治疗项目'
    },
    amount: {
      title: '结算金额(元)',
      formatter: (value: any) => formatAmount(value as number)
    },
    note: {
      title: '备注'
    }
  }))

  // 生成导出文件名
  const generateOutpatientExportFilename = (): string => {
    let periodText = ''
    if (searchParams.queryType === 'month' && searchParams.month) {
      periodText = `${searchParams.month}月份`
    } else if (searchParams.queryType === 'year' && searchParams.year) {
      periodText = `${searchParams.year}年度`
    } else if (searchParams.queryType === 'custom' && searchParams.dateRange.length === 2) {
      periodText = `${searchParams.dateRange[0]}_至_${searchParams.dateRange[1]}`
    } else {
      periodText = '全部数据'
    }

    const conditions = ['门诊日志', periodText]
    if (searchParams.dataType) {
      conditions.push(getDataTypeName(searchParams.dataType))
    }
    if (searchParams.tradeType) {
      conditions.push(getTradeTypeName(searchParams.tradeType))
    }

    return generateExportFilename('', conditions)
  }

  // 加载医师列表
  const loadDoctorList = async () => {
    try {
      const res = await getInstitutionInfo()
      if (res.code === 200) {
        doctorList.value = res.data.doctor_management.map((doctor, index) => ({
          id: index + 1,
          name: doctor.name
        }))
      }
    } catch (error) {
      console.error('加载医师列表失败:', error)
      doctorList.value = [
        { id: 1, name: '李明' },
        { id: 2, name: '张丽' }
      ]
    }
  }

  // 查询类型变化处理
  const handleQueryTypeChange = () => {
    // 清空其他查询条件
    searchParams.month = ''
    searchParams.year = ''
    searchParams.dateRange = []
    tableData.value = []
    pagination.current = 1
    pagination.total = 0
  }

  // 执行搜索
  const handleSearch = async () => {
    // 根据查询类型验证参数
    if (searchParams.queryType === 'month' && !searchParams.month) {
      ElMessage.warning('请选择查询月份')
      return
    }
    if (searchParams.queryType === 'year' && !searchParams.year) {
      ElMessage.warning('请选择查询年份')
      return
    }
    if (
      searchParams.queryType === 'custom' &&
      (!searchParams.dateRange || searchParams.dateRange.length !== 2)
    ) {
      ElMessage.warning('请选择日期范围')
      return
    }

    loading.value = true
    hasError.value = false

    try {
      let startDate: string
      let endDate: string

      // 根据查询类型构建日期范围
      if (searchParams.queryType === 'month') {
        const [year, month] = searchParams.month.split('-')
        startDate = `${year}-${month}-01`
        // 修复月末日期计算：使用下个月的第0天来获取当前月的最后一天
        const lastDay = new Date(Number(year), Number(month), 0).getDate()
        endDate = `${year}-${month}-${String(lastDay).padStart(2, '0')}`
      } else if (searchParams.queryType === 'year') {
        startDate = `${searchParams.year}-01-01`
        endDate = `${searchParams.year}-12-31`
      } else if (searchParams.queryType === 'custom') {
        startDate = searchParams.dateRange[0]
        endDate = searchParams.dateRange[1]
      } else {
        throw new Error('无效的查询类型')
      }

      // 调用统一的门诊日志API
      const response = await getOutpatientLogList({
        current: 1, // 始终从第一页开始获取所有数据
        size: 10000, // 设置足够大的数量以获取所有数据
        start_date: startDate,
        end_date: endDate,
        data_type: (searchParams.dataType as 'customer' | 'medical' | 'prescription') || undefined,
        trade_type: searchParams.tradeType || undefined
      })

      if (response.code === 200) {
        const { records, total } = response.data

        // 保存所有搜索结果
        allSearchResults.value = records

        // 按时间倒序排序（默认最新的在前面）
        allSearchResults.value.sort(
          (a, b) => new Date(b.time_of_visit).getTime() - new Date(a.time_of_visit).getTime()
        )

        // 重置分页到第一页
        pagination.current = 1
        pagination.total = total

        // 更新表格数据
        updateTableData()

        ElMessage.success(`查询完成，共找到 ${total} 条记录`)
      } else {
        ElMessage.error(response.message || '查询失败')
        tableData.value = []
        allSearchResults.value = []
        pagination.total = 0
      }
    } catch (error) {
      console.error('查询失败:', error)
      hasError.value = true
      ElMessage.error('查询失败，请重试')
      tableData.value = []
      allSearchResults.value = []
      pagination.total = 0
    } finally {
      loading.value = false
    }
  }

  // 重置搜索
  const handleReset = () => {
    Object.assign(searchParams, {
      queryType: 'month',
      month: '',
      year: '',
      dateRange: [],
      dataType: '',
      tradeType: ''
    })

    tableData.value = []
    allSearchResults.value = []
    pagination.current = 1
    pagination.total = 0
    hasError.value = false
  }

  // 导出数据处理
  const handleExportSuccess = (filename: string, rowCount: number) => {
    ElMessage.success(`成功导出 ${rowCount} 条门诊日志记录到文件：${filename}`)
  }

  const handleExportError = (error: any) => {
    console.error('导出失败:', error)
    ElMessage.error(`导出失败：${error.message || '未知错误'}`)
  }

  // 分页处理
  const handleSizeChange = (size: number) => {
    pagination.size = size
    pagination.current = 1
    updateTableData()
  }

  const handleCurrentChange = (current: number) => {
    pagination.current = current
    updateTableData()
  }

  // 更新表格数据（仅分页，不重新搜索）
  const updateTableData = () => {
    const startIndex = (pagination.current - 1) * pagination.size
    const endIndex = startIndex + pagination.size
    tableData.value = allSearchResults.value.slice(startIndex, endIndex)

    // 更新分页总数
    pagination.total = allSearchResults.value.length
  }

  // 排序处理
  const handleSortChange = ({ prop, order }: any) => {
    if (prop === 'time_of_visit') {
      // 对所有搜索结果进行排序，而不仅仅是当前页数据
      if (order === 'ascending') {
        allSearchResults.value.sort(
          (a, b) => new Date(a.time_of_visit).getTime() - new Date(b.time_of_visit).getTime()
        )
      } else if (order === 'descending') {
        allSearchResults.value.sort(
          (a, b) => new Date(b.time_of_visit).getTime() - new Date(a.time_of_visit).getTime()
        )
      }

      // 重新计算当前页数据
      updateTableData()
    }
  }

  // 查看详情
  const handleViewDetail = (row: OutpatientLogItem) => {
    selectedItem.value = row
    detailVisible.value = true
  }

  // 关闭详情
  const handleCloseDetail = () => {
    detailVisible.value = false
    selectedItem.value = null
  }

  // 打印
  const handlePrint = () => {
    ElMessage.info('打印功能开发中...')
  }

  // 获取查询时间段文本
  const getQueryPeriodText = () => {
    if (searchParams.queryType === 'month' && searchParams.month) {
      return `${searchParams.month}月份`
    } else if (searchParams.queryType === 'year' && searchParams.year) {
      return `${searchParams.year}年全年`
    } else if (searchParams.queryType === 'custom' && searchParams.dateRange.length === 2) {
      return `${searchParams.dateRange[0]} 至 ${searchParams.dateRange[1]}`
    }
    return '当前查询'
  }

  // 初始化
  onMounted(() => {
    console.log('门诊日志页面初始化')
    loadDoctorList()

    // 设置默认月份为当前月份
    const now = new Date()
    searchParams.month = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`

    // 默认加载当前月份数据
    handleSearch()
  })
</script>

<style lang="scss" scoped>
  .outpatient-log-page {
    min-height: calc(100vh - 64px);
    padding: 24px;
    background: var(--el-bg-color-page);

    .page-header {
      margin-bottom: 24px;

      h2 {
        margin: 0 0 8px;
        color: var(--el-text-color-primary);
      }

      p {
        margin: 0;
        color: var(--el-text-color-regular);
      }
    }

    .search-card,
    .stats-card,
    .charts-card,
    .result-card {
      margin-bottom: 24px;

      .card-header {
        display: flex;
        align-items: center;
        justify-content: space-between;

        h4 {
          margin: 0;
          color: var(--el-text-color-primary);
        }

        .result-info,
        .stats-info {
          display: flex;
          gap: 8px;
        }
      }
    }

    .search-form {
      .search-actions {
        display: flex;
        gap: 12px;
        padding-top: 16px;
        border-top: 1px solid var(--el-border-color-light);
      }
    }

    .stats-card {
      .stat-item {
        padding: 16px;
        text-align: center;
        background: var(--el-bg-color);
        border: 1px solid var(--el-border-color-light);
        border-radius: 8px;

        .stat-value {
          margin-bottom: 8px;
          font-size: 24px;
          font-weight: bold;
          color: var(--el-color-primary);

          &.amount {
            color: var(--el-color-success);
          }
        }

        .stat-label {
          font-size: 14px;
          color: var(--el-text-color-regular);
        }
      }
    }

    .charts-card {
      .chart-container {
        padding: 16px;
        background: var(--el-bg-color);
        border: 1px solid var(--el-border-color-light);
        border-radius: 8px;

        h5 {
          margin: 0 0 16px;
          font-size: 16px;
          font-weight: 600;
          color: var(--el-text-color-primary);
          text-align: center;
        }
      }
    }

    .pagination-wrapper {
      display: flex;
      justify-content: center;
      padding-top: 20px;
      margin-top: 20px;
      border-top: 1px solid var(--el-border-color-light);
    }

    .detail-content {
      max-height: 70vh;
      overflow-y: auto;

      h4 {
        padding-bottom: 8px;
        color: var(--el-text-color-primary);
        border-bottom: 2px solid var(--el-color-primary);
      }
    }

    .amount-text {
      font-weight: bold;
      color: var(--el-color-success);
    }

    .text-placeholder {
      color: var(--el-text-color-placeholder);
    }

    :deep(.el-table) {
      .amount-text {
        font-weight: bold;
        color: var(--el-color-success);
      }
    }
  }

  // 响应式设计
  @media (width <= 768px) {
    .outpatient-log-page {
      padding: 16px;

      .search-form {
        .search-actions {
          flex-wrap: wrap;
        }
      }

      .stats-card {
        .el-row {
          .el-col {
            margin-bottom: 12px;
          }
        }

        .stat-item {
          padding: 12px;

          .stat-value {
            font-size: 20px;
          }
        }
      }
    }
  }
</style>
