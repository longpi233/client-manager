<template>
  <div class="data-inquiry-page">
    <!-- 页面标题 -->
    <div class="page-header">
      <h2>数据查询</h2>
      <p>查询患者散客登记、门诊病历、中药处方信息</p>
    </div>

    <!-- 查询类型选择 -->
    <ElCard class="query-type-card" shadow="never">
      <template #header>
        <div class="card-header">
          <h4>查询类型</h4>
        </div>
      </template>
      <ElRadioGroup v-model="queryType" @change="handleQueryTypeChange">
        <ElRadio value="customer">
          <ElIcon><User /></ElIcon>
          散客登记
        </ElRadio>
        <ElRadio value="medical">
          <ElIcon><Document /></ElIcon>
          门诊病历
        </ElRadio>
        <ElRadio value="prescription">
          <ElIcon><Document /></ElIcon>
          中药处方
        </ElRadio>
      </ElRadioGroup>
    </ElCard>

    <!-- 搜索条件 -->
    <ElCard class="search-card" shadow="never">
      <template #header>
        <div class="card-header">
          <h4>搜索条件</h4>
        </div>
      </template>

      <!-- 散客登记搜索 -->
      <div v-if="queryType === 'customer'" class="search-form">
        <ElRow :gutter="20">
          <ElCol :span="6">
            <ElFormItem label="患者姓名">
              <ElInput
                v-model="currentSearch.customerName"
                placeholder="请输入患者姓名"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="联系电话">
              <ElInput
                v-model="currentSearch.telephone"
                placeholder="请输入联系电话"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="诊断信息">
              <ElInput
                v-model="currentSearch.diagnosis"
                placeholder="请输入诊断信息"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="就诊时间">
              <ElDatePicker
                v-model="currentSearch.dateRange"
                type="daterange"
                range-separator="至"
                start-placeholder="开始日期"
                end-placeholder="结束日期"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>
        <ElRow :gutter="20">
          <ElCol :span="6">
            <ElFormItem label="患者性别">
              <ElSelect
                v-model="currentSearch.gender"
                placeholder="请选择性别"
                clearable
                @change="handleSearch"
              >
                <ElOption label="男" value="1" />
                <ElOption label="女" value="2" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="12">
            <ElFormItem label="年龄范围">
              <div class="age-range-input">
                <ElInputNumber
                  v-model="currentSearch.minAge"
                  placeholder="最小年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
                <span class="age-separator">-</span>
                <ElInputNumber
                  v-model="currentSearch.maxAge"
                  placeholder="最大年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
              </div>
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="结算类型">
              <ElSelect
                v-model="currentSearch.tradeType"
                placeholder="请选择结算类型"
                clearable
                @change="handleSearch"
              >
                <ElOption label="现金结算" value="1" />
                <ElOption label="省直医保" value="2" />
                <ElOption label="武汉医保" value="3" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
        </ElRow>
      </div>

      <!-- 门诊病历搜索 -->
      <div v-if="queryType === 'medical'" class="search-form">
        <ElRow :gutter="20">
          <ElCol :span="6">
            <ElFormItem label="患者姓名">
              <ElInput
                v-model="currentSearch.customerName"
                placeholder="请输入患者姓名"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="联系电话">
              <ElInput
                v-model="currentSearch.telephone"
                placeholder="请输入联系电话"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="诊断信息">
              <ElInput
                v-model="currentSearch.diagnosis"
                placeholder="请输入诊断信息"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="就诊时间">
              <ElDatePicker
                v-model="currentSearch.dateRange"
                type="daterange"
                range-separator="至"
                start-placeholder="开始日期"
                end-placeholder="结束日期"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>
        <ElRow :gutter="20">
          <ElCol :span="6">
            <ElFormItem label="患者性别">
              <ElSelect
                v-model="currentSearch.gender"
                placeholder="请选择性别"
                clearable
                @change="handleSearch"
              >
                <ElOption label="男" value="1" />
                <ElOption label="女" value="2" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="12">
            <ElFormItem label="年龄范围">
              <div class="age-range-input">
                <ElInputNumber
                  v-model="currentSearch.minAge"
                  placeholder="最小年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
                <span class="age-separator">-</span>
                <ElInputNumber
                  v-model="currentSearch.maxAge"
                  placeholder="最大年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
              </div>
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="结算类型">
              <ElSelect
                v-model="currentSearch.tradeType"
                placeholder="请选择结算类型"
                clearable
                @change="handleSearch"
              >
                <ElOption label="现金结算" value="1" />
                <ElOption label="省直医保" value="2" />
                <ElOption label="武汉医保" value="3" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
        </ElRow>
      </div>

      <!-- 中药处方搜索 -->
      <div v-if="queryType === 'prescription'" class="search-form">
        <ElRow :gutter="20">
          <ElCol :span="6">
            <ElFormItem label="患者姓名">
              <ElInput
                v-model="currentSearch.customerName"
                placeholder="请输入患者姓名"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="联系电话">
              <ElInput
                v-model="currentSearch.telephone"
                placeholder="请输入联系电话"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="诊断信息">
              <ElInput
                v-model="currentSearch.diagnosis"
                placeholder="请输入诊断信息"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="就诊时间">
              <ElDatePicker
                v-model="currentSearch.dateRange"
                type="daterange"
                range-separator="至"
                start-placeholder="开始日期"
                end-placeholder="结束日期"
                format="YYYY-MM-DD"
                value-format="YYYY-MM-DD"
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>
        <ElRow :gutter="20">
          <ElCol :span="6">
            <ElFormItem label="患者性别">
              <ElSelect
                v-model="currentSearch.gender"
                placeholder="请选择性别"
                clearable
                @change="handleSearch"
              >
                <ElOption label="男" value="1" />
                <ElOption label="女" value="2" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="12">
            <ElFormItem label="年龄范围">
              <div class="age-range-input">
                <ElInputNumber
                  v-model="currentSearch.minAge"
                  placeholder="最小年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
                <span class="age-separator">-</span>
                <ElInputNumber
                  v-model="currentSearch.maxAge"
                  placeholder="最大年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
              </div>
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="结算类型">
              <ElSelect
                v-model="currentSearch.tradeType"
                placeholder="请选择结算类型"
                clearable
                @change="handleSearch"
              >
                <ElOption label="现金结算" value="1" />
                <ElOption label="省直医保" value="2" />
                <ElOption label="武汉医保" value="3" />
              </ElSelect>
            </ElFormItem>
          </ElCol>
        </ElRow>
      </div>

      <div class="search-actions">
        <ElButton type="primary" @click="handleSearch" :loading="loading">
          <ElIcon><Search /></ElIcon>
          查询
        </ElButton>
        <ElButton @click="handleReset">
          <ElIcon><Refresh /></ElIcon>
          重置
        </ElButton>
        <!-- <ElButton type="warning" @click="testUnifiedAPI">
          <ElIcon><Tools /></ElIcon>
          测试统一API
        </ElButton> -->
        <ArtExcelExport
          :data="
            tableData.map((item) => {
              const result = {
                ...item,
                treatment_items: Array.isArray(item.treatment_items)
                  ? item.treatment_items
                      .map((i) => i.treatment_item || JSON.stringify(i))
                      .join(', ')
                  : item.treatment_items,
                prescription: Array.isArray(item.prescription)
                  ? item.prescription.map((i) => i.medicine_name || JSON.stringify(i)).join(', ')
                  : item.prescription
              }
              return result
            })
          "
          :columns="exportColumns"
          :filename="generateDataInquiryExportFilename()"
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
    </ElCard>

    <!-- 查询结果 -->
    <ElCard class="result-card" shadow="never">
      <template #header>
        <div class="card-header">
          <h4>查询结果</h4>
          <div class="result-info">
            <ElTag v-if="loading" type="warning">查询中...</ElTag>
            <ElTag v-else-if="hasError" type="danger">查询失败</ElTag>
            <ElTag v-else type="success">找到 {{ tableData.length }} 条记录</ElTag>
          </div>
        </div>
      </template>

      <!-- 散客登记结果表格 -->
      <div v-if="queryType === 'customer'">
        <ElTable
          :data="tableData"
          :loading="loading"
          stripe
          border
          style="width: 100%"
          @row-click="handleRowClick"
        >
          <ElTableColumn type="index" label="序号" width="60" />
          <ElTableColumn prop="customer_name" label="患者姓名" width="100" />
          <ElTableColumn prop="gender" label="性别" width="80">
            <template #default="{ row }">
              <ElTag :type="row.gender === '1' ? 'primary' : 'success'">
                {{ formatGender(row.gender) }}
              </ElTag>
            </template>
          </ElTableColumn>
          <ElTableColumn prop="age" label="年龄" width="80" />
          <ElTableColumn prop="telephone" label="联系电话" width="130" />
          <ElTableColumn prop="diagnosis" label="诊断" min-width="150" show-overflow-tooltip />
          <!-- <ElTableColumn prop="trade_type" label="结算类型" width="100">
            <template #default="{ row }">
              {{ getTradeTypeName(row.trade_type || '1') }}
            </template>
          </ElTableColumn> -->
          <ElTableColumn prop="note" label="备注" min-width="120" show-overflow-tooltip />
          <ElTableColumn prop="time_of_visit" label="就诊时间" width="160">
            <template #default="{ row }">
              {{ formatDateTime(row.time_of_visit) }}
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
      </div>

      <!-- 门诊病历结果表格 -->
      <div v-if="queryType === 'medical'">
        <ElTable
          :data="tableData"
          :loading="loading"
          stripe
          border
          style="width: 100%"
          @row-click="handleRowClick"
        >
          <ElTableColumn type="index" label="序号" width="60" />
          <ElTableColumn prop="customer_name" label="患者姓名" width="100" />
          <ElTableColumn prop="gender" label="性别" width="80">
            <template #default="{ row }">
              <ElTag :type="row.gender === '1' ? 'primary' : 'success'">
                {{ formatGender(row.gender) }}
              </ElTag>
            </template>
          </ElTableColumn>
          <ElTableColumn prop="age" label="年龄" width="80" />
          <ElTableColumn prop="telephone" label="联系电话" width="130" />
          <ElTableColumn prop="diagnosis" label="诊断" min-width="150" show-overflow-tooltip />
          <ElTableColumn
            prop="chief_complaint"
            label="主诉"
            min-width="120"
            show-overflow-tooltip
          />
          <ElTableColumn prop="doctor_name" label="医师" width="100" />
          <ElTableColumn prop="note" label="备注" min-width="120" show-overflow-tooltip />
          <ElTableColumn prop="time_of_visit" label="就诊时间" width="160">
            <template #default="{ row }">
              {{ formatDateTime(row.time_of_visit) }}
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
      </div>

      <!-- 中药处方结果表格 -->
      <div v-if="queryType === 'prescription'">
        <ElTable
          :data="tableData"
          :loading="loading"
          stripe
          border
          style="width: 100%"
          @row-click="handleRowClick"
        >
          <ElTableColumn type="index" label="序号" width="60" />
          <ElTableColumn prop="customer_name" label="患者姓名" width="100" />
          <ElTableColumn prop="gender" label="性别" width="80">
            <template #default="{ row }">
              <ElTag :type="row.gender === '1' ? 'primary' : 'success'">
                {{ formatGender(row.gender) }}
              </ElTag>
            </template>
          </ElTableColumn>
          <ElTableColumn prop="age" label="年龄" width="80" />
          <ElTableColumn prop="telephone" label="联系电话" width="130" />
          <ElTableColumn prop="diagnosis" label="诊断" min-width="120" show-overflow-tooltip />
          <ElTableColumn prop="doses" label="付数" width="80">
            <template #default="{ row }"> {{ row.doses }}付 </template>
          </ElTableColumn>
          <!-- <ElTableColumn prop="total_amount" label="总金额" width="100">
            <template #default="{ row }">
              <span style="font-weight: bold; color: #f56c6c">
                {{ formatAmountFromTenThousandths(row.total_amount) }}
              </span>
            </template>
          </ElTableColumn> -->
          <ElTableColumn prop="doctor_name" label="医师" width="100" />
          <ElTableColumn prop="note" label="备注" min-width="120" show-overflow-tooltip />
          <ElTableColumn prop="time_of_visit" label="开方时间" width="160">
            <template #default="{ row }">
              {{ formatDateTime(row.time_of_visit) }}
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
      </div>

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
    <DataInquiryDetail v-model="detailVisible" :selected-item="selectedItem" />
  </div>
</template>

<script setup lang="ts">
  import { ref, reactive, computed, onMounted } from 'vue'
  import { ElMessage, ElInputNumber } from 'element-plus'
  import { User, Document, Search, Refresh, Download } from '@element-plus/icons-vue'
  import { getOutpatientLogList } from '@/api/outpatientLogApi'
  import { getInstitutionInfo } from '@/api/institutionApi'
  import ArtExcelExport from '@/components/core/forms/art-excel-export/index.vue'
  import DataInquiryDetail from '@/components/custom/data-inquiry-detail/index.vue'
  import type { OutpatientLogItem, OutpatientLogSearchParams } from '@/api/outpatientLogApi'
  import {
    formatDateTime,
    formatGender,
    formatAmountFromTenThousandths,
    getTradeTypeName,
    generateExportFilename
  } from '@/utils/dataprocess/medicalFormatter'
  import { RecordType } from '@/api/unifiedMedicalRecordApi'

  defineOptions({ name: 'DataInquiry' })

  // 查询类型
  const queryType = ref<RecordType>(RecordType.Customer)

  // 加载状态
  const loading = ref(false)
  const hasError = ref(false)

  // 医师列表
  const doctorList = ref<{ id: number; name: string }[]>([])

  // 散客搜索条件
  const currentSearch = reactive({
    customerName: '',
    telephone: '',
    diagnosis: '',
    dateRange: [] as string[],
    gender: '',
    minAge: undefined as number | undefined,
    maxAge: undefined as number | undefined,
    tradeType: ''
  })

  // // 门诊病历搜索条件
  // const currentSearch = reactive({
  //   customerName: '',
  //   telephone: '',
  //   diagnosis: '',
  //   dateRange: [] as string[],
  //   gender: '',
  //   minAge: undefined as number | undefined,
  //   maxAge: undefined as number | undefined,
  //   tradeType: ''
  // })

  // // 中药处方搜索条件
  // const currentSearch = reactive({
  //   customerName: '',
  //   telephone: '',
  //   diagnosis: '',
  //   dateRange: [] as string[],
  //   gender: '',
  //   minAge: undefined as number | undefined,
  //   maxAge: undefined as number | undefined,
  //   tradeType: ''
  // })

  // 表格数据
  const tableData = ref<OutpatientLogItem[]>([])

  // 分页信息
  const pagination = reactive({
    current: 1,
    size: 20,
    total: 0
  })

  // 详情弹窗
  const detailVisible = ref(false)
  const selectedItem = ref<OutpatientLogItem | null>(null)

  // 计算属性
  const hasData = computed(() => tableData.value.length > 0)

  // 导出列配置 - 根据查询类型动态生成
  const exportColumns = computed(() => {
    const baseColumns = {
      time_of_visit: {
        title: '就诊时间',
        formatter: (value: any) => formatDateTime(value as string)
      }
    }

    if (queryType.value === 'customer') {
      return {
        ...baseColumns,
        customer_name: { title: '患者姓名' },
        gender: {
          title: '性别',
          formatter: (value: any) => formatGender(value as string)
        },
        age: { title: '年龄' },
        telephone: { title: '联系电话' },
        diagnosis: { title: '诊断信息' },
        trade_type: {
          title: '结算类型',
          formatter: (value: any) => getTradeTypeName(value as string)
        },
        note: { title: '备注' }
      }
    } else if (queryType.value === 'medical') {
      return {
        ...baseColumns,
        customer_name: { title: '患者姓名' },
        gender: {
          title: '性别',
          formatter: (value: any) => formatGender(value as string)
        },
        age: { title: '年龄' },
        telephone: { title: '联系电话' },
        diagnosis: { title: '诊断' },
        chief_complaint: { title: '主诉' },
        doctor_name: { title: '医师' },
        note: { title: '备注' }
      }
    } else if (queryType.value === 'prescription') {
      return {
        ...baseColumns,
        customer_name: { title: '患者姓名' },
        gender: {
          title: '性别',
          formatter: (value: any) => formatGender(value as string)
        },
        age: { title: '年龄' },
        telephone: { title: '联系电话' },
        diagnosis: { title: '诊断' },
        doses: {
          title: '付数',
          formatter: (value: any) => `${value}付`
        },
        total_amount: {
          title: '总金额(元)',
          formatter: (value: any) => formatAmountFromTenThousandths(value as number)
        },
        doctor_name: { title: '医师' },
        note: { title: '备注' }
      }
    }
    return baseColumns
  })

  // 生成导出文件名
  const generateDataInquiryExportFilename = (): string => {
    const queryTypeNames = {
      customer: '散客登记',
      medical: '门诊病历',
      prescription: '中药处方'
    }

    const conditions = [queryTypeNames[queryType.value], '查询结果']

    if (currentSearch.customerName) {
      conditions.push(`姓名${currentSearch.customerName}`)
    }
    if (currentSearch.dateRange && currentSearch.dateRange.length === 2) {
      conditions.push(`${currentSearch.dateRange[0]}_至_${currentSearch.dateRange[1]}`)
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

  // 查询类型变化
  const handleQueryTypeChange = async () => {
    handleSearch()
  }

  // 执行搜索
  const handleSearch = async () => {
    loading.value = true
    hasError.value = false

    try {
      // 统一使用门诊日志API的查询参数
      let params: OutpatientLogSearchParams = {
        current: pagination.current,
        size: pagination.size,
        data_type: queryType.value // 根据查询类型过滤数据
      }

      // 构建通用查询参数
      if (currentSearch.customerName) {
        params.customer_name = currentSearch.customerName
      }
      if (currentSearch.telephone) {
        params.telephone = currentSearch.telephone
      }
      if (currentSearch.diagnosis) {
        params.diagnosis = currentSearch.diagnosis
      }
      if (currentSearch.gender) {
        params.gender = currentSearch.gender
      }
      if (currentSearch.minAge !== undefined) {
        params.min_age = currentSearch.minAge
      }
      if (currentSearch.maxAge !== undefined) {
        params.max_age = currentSearch.maxAge
      }
      if (currentSearch.tradeType) {
        params.trade_type = currentSearch.tradeType
      }

      // 日期范围
      if (currentSearch.dateRange && currentSearch.dateRange.length === 2) {
        params.start_date = currentSearch.dateRange[0]
        params.end_date = currentSearch.dateRange[1]
      }

      console.log('统一查询参数:', params)

      // 使用统一的门诊日志API
      const response = await getOutpatientLogList(params)

      if (response.code === 200) {
        // 统一的数据格式，无需转换
        tableData.value = response.data.records
        pagination.total = response.data.total

        console.log(`查询完成，共找到 ${response.data.total} 条记录`)
        ElMessage.success(`查询完成，共找到 ${response.data.total} 条记录`)
      } else {
        console.error('查询响应错误:', response)
        ElMessage.error(response.message || '查询失败')
        tableData.value = []
        pagination.total = 0
      }
    } catch (error) {
      console.error('查询失败:', error)
      hasError.value = true
      ElMessage.error('查询失败，请重试')
      tableData.value = []
      pagination.total = 0
    } finally {
      loading.value = false
    }
  }

  // 重置搜索
  const handleReset = () => {
    if (queryType.value === 'customer') {
      Object.assign(currentSearch, {
        customerName: '',
        telephone: '',
        diagnosis: '',
        dateRange: [],
        gender: '',
        minAge: undefined,
        maxAge: undefined,
        tradeType: ''
      })
    } else if (queryType.value === 'medical') {
      Object.assign(currentSearch, {
        customerName: '',
        telephone: '',
        diagnosis: '',
        dateRange: [],
        gender: '',
        minAge: undefined,
        maxAge: undefined,
        tradeType: ''
      })
    } else if (queryType.value === 'prescription') {
      Object.assign(currentSearch, {
        customerName: '',
        telephone: '',
        diagnosis: '',
        dateRange: [],
        gender: '',
        minAge: undefined,
        maxAge: undefined,
        tradeType: ''
      })
    }

    tableData.value = []
    pagination.current = 1
    pagination.total = 0
    hasError.value = false
  }

  // 导出数据处理
  const handleExportSuccess = (filename: string, rowCount: number) => {
    const queryTypeNames = {
      customer: '散客登记',
      medical: '门诊病历',
      prescription: '中药处方'
    }
    ElMessage.success(
      `成功导出 ${rowCount} 条${queryTypeNames[queryType.value]}记录到文件：${filename}`
    )
  }

  const handleExportError = (error: any) => {
    console.error('导出失败:', error)
    ElMessage.error(`导出失败：${error.message || '未知错误'}`)
  }

  // 分页处理
  const handleSizeChange = (size: number) => {
    pagination.size = size
    pagination.current = 1
    handleSearch()
  }

  const handleCurrentChange = (current: number) => {
    pagination.current = current
    handleSearch()
  }

  // 行点击
  const handleRowClick = (row: any) => {
    console.log('行点击:', row)
  }

  // 查看详情
  const handleViewDetail = async (row: any) => {
    console.log('查看详情 - 原始数据:', row)
    console.log('当前查询类型:', queryType.value)
    console.log('行数据的所有字段:', Object.keys(row))
    console.log('row.id值:', row.id, '类型:', typeof row.id)

    // 直接使用原始数据，DataInquiryDetail组件现在支持OutpatientLogItem格式
    selectedItem.value = row

    // 打开弹窗
    detailVisible.value = true
  }

  // 初始化
  onMounted(() => {
    console.log('页面初始化，加载医师列表')
    loadDoctorList()

    // 默认加载散客数据
    console.log('页面初始化，加载默认散客数据')
    handleSearch()
  })
</script>

<style lang="scss" scoped>
  .data-inquiry-page {
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

    .query-type-card,
    .search-card,
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

        .result-info {
          display: flex;
          gap: 8px;
        }
      }
    }

    .query-type-card {
      .el-radio-group {
        display: flex;
        gap: 24px;
        align-items: center;
        justify-content: space-between;

        .el-radio {
          display: flex;
          flex: 1;
          gap: 8px;
          align-items: center;
          justify-content: center;
          padding: 12px 20px;
          background: var(--el-bg-color);
          border: 2px solid var(--el-border-color-light);
          border-radius: 8px;
          transition: all 0.3s;

          &:hover {
            border-color: var(--el-color-primary);
          }

          &.is-checked {
            background: var(--el-color-primary-light-9);
            border-color: var(--el-color-primary);
          }

          .el-icon {
            font-size: 18px;
          }
        }
      }
    }

    .search-form {
      margin-bottom: 20px;

      .el-form-item {
        margin-bottom: 16px;

        :deep(.el-form-item__label) {
          font-weight: 500;
          color: var(--el-text-color-regular);
        }
      }

      .age-range-input {
        display: flex;
        gap: 8px;
        align-items: center;
        width: 100%;

        .age-input {
          flex: 1;
          min-width: 0;
        }

        .age-separator {
          flex-shrink: 0;
          font-weight: 500;
          color: var(--el-text-color-regular);
        }
      }
    }

    .search-actions {
      display: flex;
      gap: 12px;
      padding-top: 16px;
      border-top: 1px solid var(--el-border-color-light);
    }

    .pagination-wrapper {
      display: flex;
      justify-content: center;
      padding-top: 20px;
      margin-top: 20px;
      border-top: 1px solid var(--el-border-color-light);
    }
  }

  // 响应式设计
  @media (width <= 768px) {
    .data-inquiry-page {
      padding: 16px;

      .query-type-card .el-radio-group {
        flex-direction: column;
        gap: 12px;

        .el-radio {
          justify-content: flex-start;
        }
      }

      .search-form {
        .el-col {
          margin-bottom: 12px;
        }

        .age-range-input {
          flex-direction: column;
          gap: 8px;

          .age-separator {
            align-self: center;
          }

          .age-input {
            width: 100%;
          }
        }
      }

      .search-actions {
        flex-wrap: wrap;
      }
    }
  }

  // 中等屏幕优化
  @media (width <= 1024px) and (width >= 769px) {
    .search-form {
      .age-range-input {
        .age-input {
          min-width: 80px;
        }
      }
    }
  }
</style>
