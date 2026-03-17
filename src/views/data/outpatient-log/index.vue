<template>
  <div class="outpatient-log-page">
    <!-- 页面标题 -->
    <div class="page-header">
      <h2>门诊日志</h2>
      <p>查看指定月份的门诊日志，包含散客登记、针灸理疗、中医中药等综合信息</p>
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
                v-model="queryConfig.queryType"
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
            <ElFormItem v-if="queryConfig.queryType === 'month'" label="查询月份">
              <ElDatePicker
                v-model="queryConfig.month"
                type="month"
                placeholder="请选择查询月份"
                format="YYYY年MM月"
                value-format="YYYY-MM"
                style="width: 100%"
                @change="handleSearch"
              />
            </ElFormItem>
            <ElFormItem v-else-if="queryConfig.queryType === 'year'" label="查询年份">
              <ElDatePicker
                v-model="queryConfig.year"
                type="year"
                placeholder="请选择查询年份"
                format="YYYY年"
                value-format="YYYY"
                style="width: 100%"
                @change="handleSearch"
              />
            </ElFormItem>
            <ElFormItem v-else-if="queryConfig.queryType === 'custom'" label="日期范围">
              <ElDatePicker
                v-model="queryConfig.dateRange"
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
                v-model="searchParams.data_type"
                placeholder="请选择数据类型"
                clearable
                style="width: 100%"
                @change="handleSearch"
              >
                <ElOption label="全部" value="" />
                <ElOption
                  v-for="option in RECORD_TYPE_OPTIONS"
                  :key="option.value"
                  :label="option.label"
                  :value="option.value"
                />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="结算类型">
              <ElSelect
                v-model="searchParams.trade_type"
                placeholder="请选择结算类型"
                clearable
                style="width: 100%"
                @change="handleSearch"
              >
                <ElOption label="全部" value="" />
                <ElOption
                  v-for="option in TRADE_TYPE_OPTIONS"
                  :key="option.value"
                  :label="option.label"
                  :value="option.value"
                />
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
          <ElButton
            type="danger"
            @click="handleBatchDelete"
            :disabled="selectedItems.length === 0"
            :loading="batchDeleting"
          >
            <ElIcon><Delete /></ElIcon>
            批量删除 ({{ selectedItems.length }})
          </ElButton>
          <ArtExcelExport
            :data="
              allSearchResults.map((item) => ({
                ...item,
                treatment_items: Array.isArray(item.treatment_items)
                  ? item.treatment_items
                      .map((i) => i.treatment_item || JSON.stringify(i))
                      .join(', ')
                  : item.treatment_items,
                prescription_items: Array.isArray(item.prescription_items)
                  ? item.prescription_items
                      .map((i) => i.medicine_name || JSON.stringify(i))
                      .join(', ')
                  : item.prescription_items
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

    <!-- 查询结果 -->
    <ElCard class="result-card" shadow="never">
      <template #header>
        <div class="card-header">
          <h4>门诊日志</h4>
          <div class="result-info">
            <ElTag v-if="loading" type="warning">查询中...</ElTag>
            <ElTag v-else-if="hasError" type="danger">查询失败</ElTag>
            <ElTag v-else type="success">找到 {{ totalCount }} 条记录</ElTag>
          </div>
        </div>
      </template>

      <!-- 门诊日志表格 -->
      <ElTable
        ref="tableRef"
        :data="tableData"
        :loading="loading"
        stripe
        border
        style="width: 100%"
        :default-sort="{ prop: 'time_of_visit', order: 'descending' }"
        @sort-change="handleSortChange"
        @selection-change="handleSelectionChange"
      >
        <ElTableColumn type="selection" width="50" />
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
            <span v-if="row.total_amount" class="amount-text">
              {{ formatAmountFromTenThousandths(row.total_amount) }}
            </span>
            <span v-else class="text-placeholder">-</span>
          </template>
        </ElTableColumn>
        <ElTableColumn label="操作" width="180" fixed="right">
          <template #default="{ row }">
            <ElButton type="primary" size="small" @click="handleViewDetail(row)">
              查看详情
            </ElButton>
            <ElButton type="danger" size="small" @click="handleDeleteItem(row)"> 删除 </ElButton>
          </template>
        </ElTableColumn>
      </ElTable>

      <!-- 分页 -->
      <div class="pagination-wrapper">
        <ElPagination
          v-model:current-page="searchParams.current"
          v-model:page-size="searchParams.size"
          :total="totalCount"
          :page-sizes="[10, 20, 50, 100]"
          layout="total, sizes, prev, pager, next, jumper"
          @size-change="handleSizeChange"
          @current-change="handleCurrentChange"
        />
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
            <div class="stat-label">针灸理疗</div>
          </div>
        </ElCol>
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value">{{ statistics.prescriptionCount }}</div>
            <div class="stat-label">中医中药</div>
          </div>
        </ElCol>
      </ElRow>
      <ElRow :gutter="24" style="margin-top: 16px">
        <ElCol :span="6">
          <div class="stat-item">
            <div class="stat-value amount">{{
              formatAmountFromTenThousandths(statistics.totalAmount)
            }}</div>
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
              margin-right="-100px"
              :show-tooltip="true"
              :show-legend="false"
              :smooth="true"
              :show-area="true"
              :show-label="true"
              label-position="top"
              label-formatter="(params) => Number(params.value || 0).toLocaleString() + '元'"
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
              :colors="['#67C23A', '#F56C6C', '#E6A23C']"
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

    <!-- 详情弹窗 -->
    <DataInquiryDetail v-model="detailVisible" :selected-item="selectedItem" />
  </div>
</template>

<script setup lang="ts">
  import { ref, reactive, computed, onMounted } from 'vue'
  import { ElMessage, ElMessageBox } from 'element-plus'
  import { Search, Refresh, Download, Delete } from '@element-plus/icons-vue'
  import {
    getOutpatientLogList,
    deleteOutpatientLogItem,
    deleteOutpatientLogItemsBatch
  } from '@/api/outpatientLogApi'
  import type { UnifiedMedicalRecord, UnifiedMedicalRecordSearchParams } from '@/types/medical'
  import { getInstitutionInfo } from '@/api/institutionApi'
  import ArtLineChart from '@/components/core/charts/art-line-chart/index.vue'
  import ArtRingChart from '@/components/core/charts/art-ring-chart/index.vue'
  import ArtExcelExport from '@/components/core/forms/art-excel-export/index.vue'
  import DataInquiryDetail from '@/components/custom/data-inquiry-detail/index.vue'
  import {
    formatDateTime,
    formatGender,
    formatAmountFromTenThousandths,
    getTradeTypeName,
    getDataTypeName,
    getTradeTypeTagType,
    getDataTypeTagType,
    generateExportFilename
  } from '@/utils/dataprocess/medicalFormatter'
  import { convertTenThousandthsToYuan } from '@/utils/dataprocess/format'
  import { RECORD_TYPE_OPTIONS, TRADE_TYPE_OPTIONS } from '@/types/medical'

  defineOptions({ name: 'OutpatientLog' })

  // 统一搜索参数
  const searchParams = reactive<UnifiedMedicalRecordSearchParams>({
    current: 1,
    size: 20,
    start_date: '',
    end_date: '',
    data_type: undefined,
    trade_type: ''
  })

  // 查询类型辅助参数
  const queryConfig = reactive({
    queryType: 'month', // month, year, custom
    month: '',
    year: '',
    dateRange: [] as string[]
  })

  // 加载状态
  const loading = ref(false)
  const hasError = ref(false)

  // 医师列表
  const doctorList = ref<{ id: number; name: string }[]>([])

  // 表格数据
  const tableData = ref<UnifiedMedicalRecord[]>([])

  // 所有搜索结果（用于统计）
  const allSearchResults = ref<UnifiedMedicalRecord[]>([])

  // 分页总数（从 searchParams 中获取 current 和 size）
  const totalCount = ref(0)

  // 详情弹窗
  const detailVisible = ref(false)
  const selectedItem = ref<UnifiedMedicalRecord | null>(null)

  // 选择和删除相关
  const selectedItems = ref<UnifiedMedicalRecord[]>([])
  const batchDeleting = ref(false)
  const tableRef = ref()

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
      if (item.total_amount) stats.totalAmount += item.total_amount

      // 统计结算类型（使用标准化函数）
      const normalizedTradeType = normalizeTradeType(item.trade_type)
      if (normalizedTradeType === '1') stats.cashCount++
      else if (normalizedTradeType === '2') stats.provincialInsuranceCount++
      else if (normalizedTradeType === '3') stats.wuhanInsuranceCount++

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
      if (item.total_amount && item.time_of_visit) {
        const date = item.time_of_visit.split(' ')[0] // 提取日期部分
        const currentAmount = dailyRevenue.get(date) || 0
        dailyRevenue.set(date, currentAmount + item.total_amount)
      }
    })

    // 转换为图表数据格式
    const sortedDates = Array.from(dailyRevenue.keys()).sort()
    // 将分转换为元用于图表显示
    const data = sortedDates.map((date) => convertToYuan(dailyRevenue.get(date) || 0))

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
      .reduce((sum, item) => sum + (item.total_amount || 0), 0)

    const medicalAmount = allSearchResults.value
      .filter((item) => item.data_type === 'medical')
      .reduce((sum, item) => sum + (item.total_amount || 0), 0)

    const prescriptionAmount = allSearchResults.value
      .filter((item) => item.data_type === 'prescription')
      .reduce((sum, item) => sum + (item.total_amount || 0), 0)

    return [
      { name: '散客登记', value: convertToYuan(customerAmount) },
      { name: '针灸理疗', value: convertToYuan(medicalAmount) },
      { name: '中医中药', value: convertToYuan(prescriptionAmount) }
    ].filter((item) => item.value > 0)
  })

  // 标准化结算类型函数
  const normalizeTradeType = (tradeType: string): string => {
    if (!tradeType) return ''

    // 统一转换为数字格式
    const normalMap: Record<string, string> = {
      '1': '1',
      '2': '2',
      '3': '3',
      cash: '1',
      Cash: '1',
      insurance: '2',
      ProvincialInsurance: '2',
      WuHanInsurance: '3'
    }

    return normalMap[tradeType] || tradeType
  }

  // 结算类型占比图数据（按营业额）
  const tradeTypeChartData = computed(() => {
    const cashAmount = allSearchResults.value
      .filter((item) => normalizeTradeType(item.trade_type) === '1')
      .reduce((sum, item) => sum + (item.total_amount || 0), 0)

    const provincialInsuranceAmount = allSearchResults.value
      .filter((item) => normalizeTradeType(item.trade_type) === '2')
      .reduce((sum, item) => sum + (item.total_amount || 0), 0)

    const wuhanInsuranceAmount = allSearchResults.value
      .filter((item) => normalizeTradeType(item.trade_type) === '3')
      .reduce((sum, item) => sum + (item.total_amount || 0), 0)

    return [
      { name: '现金结算', value: convertToYuan(cashAmount) },
      { name: '省直医保', value: convertToYuan(provincialInsuranceAmount) },
      { name: '武汉医保', value: convertToYuan(wuhanInsuranceAmount) }
    ].filter((item) => item.value > 0)
  })

  // 计算属性
  const hasData = computed(() => tableData.value.length > 0)

  // 使用标准化的价格转换函数（万分转元）
  const convertToYuan = convertTenThousandthsToYuan

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
      formatter: (value: any) => getDataTypeName(value)
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
      formatter: (value: any) => formatAmountFromTenThousandths(value as number)
    },
    note: {
      title: '备注'
    }
  }))

  // 生成导出文件名
  const generateOutpatientExportFilename = (): string => {
    let periodText = ''
    if (queryConfig.queryType === 'month' && queryConfig.month) {
      periodText = `${queryConfig.month}月份`
    } else if (queryConfig.queryType === 'year' && queryConfig.year) {
      periodText = `${queryConfig.year}年度`
    } else if (queryConfig.queryType === 'custom' && queryConfig.dateRange.length === 2) {
      periodText = `${queryConfig.dateRange[0]}_至_${queryConfig.dateRange[1]}`
    } else {
      periodText = '全部数据'
    }

    const conditions = ['门诊日志', periodText]
    if (searchParams.data_type) {
      conditions.push(getDataTypeName(searchParams.data_type))
    }
    if (searchParams.trade_type) {
      conditions.push(getTradeTypeName(searchParams.trade_type))
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
    queryConfig.month = ''
    queryConfig.year = ''
    queryConfig.dateRange = []
    tableData.value = []
    searchParams.current = 1
    totalCount.value = 0
  }

  // 执行搜索
  const handleSearch = async () => {
    // 根据查询类型验证参数
    if (queryConfig.queryType === 'month' && !queryConfig.month) {
      ElMessage.warning('请选择查询月份')
      return
    }
    if (queryConfig.queryType === 'year' && !queryConfig.year) {
      ElMessage.warning('请选择查询年份')
      return
    }
    if (
      queryConfig.queryType === 'custom' &&
      (!queryConfig.dateRange || queryConfig.dateRange.length !== 2)
    ) {
      ElMessage.warning('请选择日期范围')
      return
    }

    loading.value = true
    hasError.value = false

    try {
      // 根据查询类型构建日期范围
      if (queryConfig.queryType === 'month') {
        const [year, month] = queryConfig.month.split('-')
        searchParams.start_date = `${year}-${month}-01`
        // 修复月末日期计算：使用下个月的第0天来获取当前月的最后一天
        const lastDay = new Date(Number(year), Number(month), 0).getDate()
        searchParams.end_date = `${year}-${month}-${String(lastDay).padStart(2, '0')}`
      } else if (queryConfig.queryType === 'year') {
        searchParams.start_date = `${queryConfig.year}-01-01`
        searchParams.end_date = `${queryConfig.year}-12-31`
      } else if (queryConfig.queryType === 'custom') {
        searchParams.start_date = queryConfig.dateRange[0]
        searchParams.end_date = queryConfig.dateRange[1]
      } else {
        throw new Error('无效的查询类型')
      }

      const fullSearchParams: UnifiedMedicalRecordSearchParams = {
        ...searchParams,
        current: 1,
        size: 10000000
      }

      console.log('门诊日志查询参数:', fullSearchParams)

      // 使用统一的门诊日志API
      const response = await getOutpatientLogList(fullSearchParams)

      console.log('门诊日志API响应:', response)

      if (response.code === 200) {
        const { records, total } = response.data

        // 保存所有搜索结果
        allSearchResults.value = records

        // 按时间倒序排序（默认最新的在前面）
        allSearchResults.value.sort(
          (a, b) => new Date(b.time_of_visit).getTime() - new Date(a.time_of_visit).getTime()
        )

        // 重置分页到第一页
        searchParams.current = 1
        totalCount.value = total

        // 更新表格数据
        updateTableData()

        ElMessage.success(`查询完成，共找到 ${total} 条记录`)
      } else {
        ElMessage.error(response.message || '查询失败')
        tableData.value = []
        allSearchResults.value = []
        totalCount.value = 0
      }
    } catch (error) {
      console.error('查询失败:', error)
      hasError.value = true
      ElMessage.error('查询失败，请重试')
      tableData.value = []
      allSearchResults.value = []
      totalCount.value = 0
    } finally {
      loading.value = false
    }
  }

  // 重置搜索
  const handleReset = () => {
    Object.assign(queryConfig, {
      queryType: 'month',
      month: '',
      year: '',
      dateRange: []
    })

    Object.assign(searchParams, {
      current: 1,
      size: 20,
      start_date: '',
      end_date: '',
      data_type: undefined,
      trade_type: ''
    })

    tableData.value = []
    allSearchResults.value = []
    searchParams.current = 1
    totalCount.value = 0
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
    searchParams.size = size
    searchParams.current = 1
    updateTableData()
  }

  const handleCurrentChange = (current: number) => {
    searchParams.current = current
    updateTableData()
  }

  // 更新表格数据（仅分页，不重新搜索）
  const updateTableData = () => {
    const startIndex = (searchParams.current - 1) * searchParams.size
    const endIndex = startIndex + searchParams.size
    tableData.value = allSearchResults.value.slice(startIndex, endIndex)

    // 更新分页总数
    totalCount.value = allSearchResults.value.length
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
  const handleViewDetail = (row: UnifiedMedicalRecord) => {
    selectedItem.value = row
    detailVisible.value = true
  }

  // 选择变化处理
  const handleSelectionChange = (selection: UnifiedMedicalRecord[]) => {
    selectedItems.value = selection
  }

  // 删除单个记录
  const handleDeleteItem = async (row: UnifiedMedicalRecord) => {
    if (!row.id) {
      ElMessage.error('无效的记录ID')
      return
    }

    try {
      await ElMessageBox.confirm(
        `确定要删除患者 "${row.customer_name}" 的这条记录吗？删除后不可恢复。`,
        '确认删除',
        {
          confirmButtonText: '确定删除',
          cancelButtonText: '取消',
          type: 'warning',
          confirmButtonClass: 'el-button--danger'
        }
      )

      const response = await deleteOutpatientLogItem(row.id)

      if (response.code === 200) {
        ElMessage.success('删除成功')
        // 重新搜索数据
        await handleSearch()
      } else {
        ElMessage.error(response.message || '删除失败')
      }
    } catch (error) {
      if (error !== 'cancel') {
        console.error('删除失败:', error)
        ElMessage.error('删除失败，请重试')
      }
    }
  }

  // 批量删除
  const handleBatchDelete = async () => {
    if (selectedItems.value.length === 0) {
      ElMessage.warning('请先选择要删除的记录')
      return
    }

    for (const item of selectedItems.value) {
      if (!item.id) {
        ElMessage.error('无效的记录ID')
        return
      }
    }

    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedItems.value.length} 条记录吗？删除后不可恢复。`,
        '确认批量删除',
        {
          confirmButtonText: '确定删除',
          cancelButtonText: '取消',
          type: 'warning',
          confirmButtonClass: 'el-button--danger'
        }
      )

      batchDeleting.value = true

      const ids = selectedItems.value.map((item) => item.id as number)
      const response = await deleteOutpatientLogItemsBatch(ids)

      if (response.code === 200) {
        ElMessage.success(`批量删除成功，共删除 ${response.data} 条记录`)
        // 清空选择
        selectedItems.value = []
        tableRef.value?.clearSelection()
        // 重新搜索数据
        await handleSearch()
      } else {
        ElMessage.error(response.message || '批量删除失败')
      }
    } catch (error) {
      if (error !== 'cancel') {
        console.error('批量删除失败:', error)
        ElMessage.error('批量删除失败，请重试')
      }
    } finally {
      batchDeleting.value = false
    }
  }

  // 获取查询时间段文本
  const getQueryPeriodText = () => {
    if (queryConfig.queryType === 'month' && queryConfig.month) {
      return `${queryConfig.month}月份`
    } else if (queryConfig.queryType === 'year' && queryConfig.year) {
      return `${queryConfig.year}年全年`
    } else if (queryConfig.queryType === 'custom' && queryConfig.dateRange.length === 2) {
      return `${queryConfig.dateRange[0]} 至 ${queryConfig.dateRange[1]}`
    }
    return '当前查询'
  }

  // 初始化
  onMounted(() => {
    console.log('门诊日志页面初始化')
    loadDoctorList()

    // 设置默认月份为当前月份
    const now = new Date()
    queryConfig.month = `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`

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

          &.total_amount {
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

    .total_amount-text {
      font-weight: bold;
      color: var(--el-color-success);
    }

    .text-placeholder {
      color: var(--el-text-color-placeholder);
    }

    :deep(.el-table) {
      .total_amount-text {
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
