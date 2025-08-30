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
                v-model="customerSearch.customerName"
                placeholder="请输入患者姓名"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="联系电话">
              <ElInput
                v-model="customerSearch.telephone"
                placeholder="请输入联系电话"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="诊断信息">
              <ElInput
                v-model="customerSearch.diagnosis"
                placeholder="请输入诊断信息"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="就诊时间">
              <ElDatePicker
                v-model="customerSearch.dateRange"
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
                v-model="customerSearch.gender"
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
                  v-model="customerSearch.minAge"
                  placeholder="最小年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
                <span class="age-separator">-</span>
                <ElInputNumber
                  v-model="customerSearch.maxAge"
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
                v-model="customerSearch.tradeType"
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
                v-model="medicalSearch.customerName"
                placeholder="请输入患者姓名"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="联系电话">
              <ElInput
                v-model="medicalSearch.telephone"
                placeholder="请输入联系电话"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="诊断信息">
              <ElInput
                v-model="medicalSearch.diagnosis"
                placeholder="请输入诊断信息"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="就诊时间">
              <ElDatePicker
                v-model="medicalSearch.dateRange"
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
                v-model="medicalSearch.gender"
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
                  v-model="medicalSearch.minAge"
                  placeholder="最小年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
                <span class="age-separator">-</span>
                <ElInputNumber
                  v-model="medicalSearch.maxAge"
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
                v-model="medicalSearch.tradeType"
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
                v-model="prescriptionSearch.customerName"
                placeholder="请输入患者姓名"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="联系电话">
              <ElInput
                v-model="prescriptionSearch.telephone"
                placeholder="请输入联系电话"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="诊断信息">
              <ElInput
                v-model="prescriptionSearch.diagnosis"
                placeholder="请输入诊断信息"
                clearable
                @change="handleSearch"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="6">
            <ElFormItem label="就诊时间">
              <ElDatePicker
                v-model="prescriptionSearch.dateRange"
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
                v-model="prescriptionSearch.gender"
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
                  v-model="prescriptionSearch.minAge"
                  placeholder="最小年龄"
                  :min="0"
                  :max="200"
                  class="age-input"
                  @change="handleSearch"
                />
                <span class="age-separator">-</span>
                <ElInputNumber
                  v-model="prescriptionSearch.maxAge"
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
                v-model="prescriptionSearch.tradeType"
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
          :data="tableData"
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
                {{ formatAmountFromCents(row.total_amount) }}
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
    <ElDialog
      v-model="detailVisible"
      :title="`${queryType === 'customer' ? '散客信息' : queryType === 'medical' ? '病历' : '处方'}详情`"
      width="80%"
      :before-close="handleCloseDetail"
    >
      <div class="detail-content">
        <!-- 散客详情 -->
        <div v-if="queryType === 'customer' && selectedItem">
          <ElDescriptions title="基本信息" :column="3" border style="margin-bottom: 20px">
            <ElDescriptionsItem label="患者姓名">{{
              selectedItem.customer_name
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="患者性别">
              {{ formatGender(selectedItem.gender) }}
            </ElDescriptionsItem>
            <ElDescriptionsItem label="年龄">{{ selectedItem.age }}岁</ElDescriptionsItem>
            <ElDescriptionsItem label="联系电话">{{ selectedItem.telephone }}</ElDescriptionsItem>
            <ElDescriptionsItem label="就诊时间">{{
              formatDateTime(selectedItem.time_of_visit)
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="结算类型">{{
              getTradeTypeName(selectedItem.trade_type || '1')
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="诊断信息" :span="2">{{
              selectedItem.diagnosis || '-'
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="备注" :span="3">{{
              selectedItem.note || '-'
            }}</ElDescriptionsItem>
          </ElDescriptions>

          <!-- 治疗项目明细 -->
          <h4 style="margin: 20px 0 10px">治疗项目明细</h4>
          <ElTable
            :data="selectedItem.treatmentItems || []"
            border
            style="width: 100%"
            :empty-text="
              selectedItem.treatmentItems === undefined
                ? '加载中...'
                : selectedItem.treatmentItems.length === 0
                  ? '暂无治疗项目'
                  : ''
            "
          >
            <ElTableColumn type="index" label="序号" width="60" />
            <ElTableColumn prop="treatment_item_name" label="治疗项目" min-width="150" />
            <ElTableColumn prop="treatment_count" label="数量" width="100" align="center">
              <template #default="{ row }">{{ row.treatment_count }}</template>
            </ElTableColumn>
            <ElTableColumn prop="unit_price" label="单价(元)" width="120" align="right">
              <template #default="{ row }">{{
                formatAmountFromCents(row.unit_price || 0)
              }}</template>
            </ElTableColumn>
            <ElTableColumn prop="total_price" label="小计(元)" width="120" align="right">
              <template #default="{ row }">
                <span style="font-weight: bold; color: #f56c6c">{{
                  formatAmountFromCents(row.total_price || 0)
                }}</span>
              </template>
            </ElTableColumn>
          </ElTable>

          <!-- 费用汇总 -->
          <div
            v-if="selectedItem.treatmentItems && selectedItem.treatmentItems.length > 0"
            style="margin-top: 16px"
          >
            <h5 style="margin: 0 0 16px; color: var(--el-text-color-primary)">费用汇总</h5>
            <ElDescriptions :column="4" border>
              <ElDescriptionsItem label="治疗项目">
                {{ selectedItem.treatmentItems.length }}项
              </ElDescriptionsItem>
              <ElDescriptionsItem label="总数量">
                {{
                  selectedItem.treatmentItems.reduce(
                    (sum: number, item: any) => sum + (item.treatment_count || 0),
                    0
                  )
                }}次
              </ElDescriptionsItem>
              <ElDescriptionsItem label="应收金额">
                {{
                  formatAmountFromCents(
                    selectedItem.treatmentItems.reduce(
                      (sum: number, item: any) => sum + (item.total_price || 0),
                      0
                    )
                  )
                }}
              </ElDescriptionsItem>
              <ElDescriptionsItem label="支付状态">
                <ElTag type="success">已支付</ElTag>
              </ElDescriptionsItem>
            </ElDescriptions>
          </div>
        </div>

        <!-- 病历详情 -->
        <div v-if="queryType === 'medical' && selectedItem">
          <ElDescriptions title="基本信息" :column="3" border style="margin-bottom: 20px">
            <ElDescriptionsItem label="患者姓名">{{
              selectedItem.customer_name
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="性别">
              {{ formatGender(selectedItem.gender) }}
            </ElDescriptionsItem>
            <ElDescriptionsItem label="年龄">{{ selectedItem.age }}岁</ElDescriptionsItem>
            <ElDescriptionsItem label="就诊时间">{{
              formatDateTime(selectedItem.time_of_visit)
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="医师">{{ selectedItem.doctor_name }}</ElDescriptionsItem>
            <ElDescriptionsItem label="结算类型">{{
              getTradeTypeName(selectedItem.trade_type)
            }}</ElDescriptionsItem>
          </ElDescriptions>

          <ElDescriptions title="诊疗信息" :column="1" border>
            <ElDescriptionsItem label="主诉">{{ selectedItem.chief_complaint }}</ElDescriptionsItem>
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
            <ElDescriptionsItem label="诊断">{{ selectedItem.diagnosis }}</ElDescriptionsItem>
            <ElDescriptionsItem label="治法治则">{{
              selectedItem.treatment_method || '-'
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="备注">{{ selectedItem.note || '-' }}</ElDescriptionsItem>
          </ElDescriptions>

          <!-- 治疗项目明细 -->
          <h4 style="margin: 20px 0 10px">治疗项目明细</h4>
          <ElTable
            :data="selectedItem.treatmentItems || []"
            border
            style="width: 100%"
            :empty-text="
              selectedItem.treatmentItems === undefined
                ? '加载中...'
                : selectedItem.treatmentItems.length === 0
                  ? '暂无治疗项目'
                  : ''
            "
          >
            <ElTableColumn type="index" label="序号" width="60" />
            <ElTableColumn prop="treatment_item_name" label="治疗项目" min-width="150" />
            <ElTableColumn prop="treatment_count" label="次数" width="100" align="center">
              <template #default="{ row }">{{ row.treatment_count }}</template>
            </ElTableColumn>
            <ElTableColumn prop="unit_price" label="单价(元)" width="120" align="right">
              <template #default="{ row }">{{
                formatAmountFromCents(row.unit_price || 0)
              }}</template>
            </ElTableColumn>
            <ElTableColumn prop="total_price" label="小计(元)" width="120" align="right">
              <template #default="{ row }">
                <span style="font-weight: bold; color: #f56c6c">{{
                  formatAmountFromCents(row.total_price || 0)
                }}</span>
              </template>
            </ElTableColumn>
          </ElTable>

          <!-- 费用汇总 -->
          <div
            v-if="selectedItem.treatmentItems && selectedItem.treatmentItems.length > 0"
            style="margin-top: 16px"
          >
            <h5 style="margin: 0 0 16px; color: var(--el-text-color-primary)">费用汇总</h5>
            <ElDescriptions :column="4" border>
              <ElDescriptionsItem label="治疗项目">
                {{ selectedItem.treatmentItems.length }}项
              </ElDescriptionsItem>
              <ElDescriptionsItem label="总次数">
                {{
                  selectedItem.treatmentItems.reduce(
                    (sum: number, item: any) => sum + (item.treatment_count || 0),
                    0
                  )
                }}次
              </ElDescriptionsItem>
              <ElDescriptionsItem label="总费用">
                {{
                  formatAmountFromCents(
                    selectedItem.treatmentItems.reduce(
                      (sum: number, item: any) => sum + (item.total_price || 0),
                      0
                    )
                  )
                }}
              </ElDescriptionsItem>
              <ElDescriptionsItem label="结算状态">
                <ElTag type="success">已结算</ElTag>
              </ElDescriptionsItem>
            </ElDescriptions>
          </div>
        </div>

        <!-- 处方详情 -->
        <div v-if="queryType === 'prescription' && selectedItem">
          <ElDescriptions title="基本信息" :column="3" border style="margin-bottom: 20px">
            <ElDescriptionsItem label="患者姓名">{{
              selectedItem.customer_name
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="性别">
              {{ formatGender(selectedItem.gender) }}
            </ElDescriptionsItem>
            <ElDescriptionsItem label="年龄">{{ selectedItem.age }}岁</ElDescriptionsItem>
            <ElDescriptionsItem label="开方时间">{{
              formatDateTime(selectedItem.time_of_visit)
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="医师">{{ selectedItem.doctor_name }}</ElDescriptionsItem>
            <ElDescriptionsItem label="付数">{{ selectedItem.doses }}付</ElDescriptionsItem>
            <ElDescriptionsItem label="诊断" :span="3">{{
              selectedItem.diagnosis
            }}</ElDescriptionsItem>
          </ElDescriptions>

          <ElDescriptions title="处方信息" :column="1" border style="margin-bottom: 20px">
            <ElDescriptionsItem label="单付金额">{{
              formatAmountFromCents(selectedItem.single_dose_amount || 0)
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="总金额">{{
              formatAmountFromCents(selectedItem.total_amount || 0)
            }}</ElDescriptionsItem>
            <ElDescriptionsItem label="备注">{{ selectedItem.note || '-' }}</ElDescriptionsItem>
          </ElDescriptions>

          <h4 style="margin: 20px 0 10px">处方明细</h4>
          <ElTable :data="selectedItem.prescription_items || []" border style="width: 100%">
            <ElTableColumn type="index" label="序号" width="60" />
            <ElTableColumn prop="medicine_name" label="中药名称" />
            <ElTableColumn prop="grams" label="克数" width="100">
              <template #default="{ row }"> {{ row.grams }}g </template>
            </ElTableColumn>
            <ElTableColumn prop="unit_price" label="单价(元/g)" width="120">
              <template #default="{ row }">
                {{ formatAmountFromCents(row.unit_price || 0) }}
              </template>
            </ElTableColumn>
            <ElTableColumn prop="total_price" label="金额(元)" width="100">
              <template #default="{ row }">
                {{ formatAmountFromCents((row.grams || 0) * (row.unit_price || 0)) }}
              </template>
            </ElTableColumn>
          </ElTable>
        </div>
      </div>

      <template #footer>
        <ElButton @click="detailVisible = false">关闭</ElButton>
        <!-- 散客登记：暂不支持打印 -->
        <template v-if="queryType === 'customer'">
          <ElButton type="info" disabled>散客登记暂不支持打印</ElButton>
        </template>
        <!-- 门诊病历：支持打印病历和费用清单 -->
        <template v-if="queryType === 'medical'">
          <ElButton type="success" @click="handlePrintMedicalRecord">打印门诊病历</ElButton>
          <ElButton type="warning" @click="handlePrintMedicalFeeList">打印费用清单</ElButton>
        </template>
        <!-- 中药处方：支持打印处方和费用清单 -->
        <template v-if="queryType === 'prescription'">
          <ElButton type="success" @click="handlePrintPrescription">打印中药处方</ElButton>
          <ElButton type="warning" @click="handlePrintPrescriptionFeeList">打印费用清单</ElButton>
        </template>
      </template>
    </ElDialog>
  </div>
</template>

<script setup lang="ts">
  import { ref, reactive, computed, onMounted } from 'vue'
  import { ElMessage, ElInputNumber } from 'element-plus'
  import { User, Document, Search, Refresh, Download } from '@element-plus/icons-vue'
  import {
    getCustomerRecordList,
    getMedicalRecordListLegacy,
    getPrescriptionRecordList
  } from '@/api/unifiedMedicalRecordApi'
  import { getInstitutionInfo } from '@/api/institutionApi'
  import ArtExcelExport from '@/components/core/forms/art-excel-export/index.vue'
  import {
    formatDateTime,
    formatGender,
    formatAmountFromCents,
    getTradeTypeName,
    generateExportFilename
  } from '@/utils/dataprocess/medicalFormatter'
  import {
    printMedicalRecord,
    printFeeList,
    printPrescription,
    type MedicalRecordData,
    type FeeListData,
    type PrescriptionData
  } from '@/utils/print'

  defineOptions({ name: 'DataInquiry' })

  // 查询类型
  const queryType = ref<'customer' | 'medical' | 'prescription'>('customer')

  // 加载状态
  const loading = ref(false)
  const hasError = ref(false)

  // 医师列表
  const doctorList = ref<{ id: number; name: string }[]>([])

  // 散客搜索条件
  const customerSearch = reactive({
    customerName: '',
    telephone: '',
    diagnosis: '',
    dateRange: [] as string[],
    gender: '',
    minAge: undefined as number | undefined,
    maxAge: undefined as number | undefined,
    tradeType: ''
  })

  // 门诊病历搜索条件
  const medicalSearch = reactive({
    customerName: '',
    telephone: '',
    diagnosis: '',
    dateRange: [] as string[],
    gender: '',
    minAge: undefined as number | undefined,
    maxAge: undefined as number | undefined,
    tradeType: ''
  })

  // 中药处方搜索条件
  const prescriptionSearch = reactive({
    customerName: '',
    telephone: '',
    diagnosis: '',
    dateRange: [] as string[],
    gender: '',
    minAge: undefined as number | undefined,
    maxAge: undefined as number | undefined,
    tradeType: ''
  })

  // 表格数据
  const tableData = ref<any[]>([])

  // 分页信息
  const pagination = reactive({
    current: 1,
    size: 20,
    total: 0
  })

  // 详情弹窗
  const detailVisible = ref(false)
  const selectedItem = ref<any>(null)

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
          formatter: (value: any) => formatAmountFromCents(value as number)
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

    const currentSearch =
      queryType.value === 'customer'
        ? customerSearch
        : queryType.value === 'medical'
          ? medicalSearch
          : prescriptionSearch

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
  const handleQueryTypeChange = () => {
    tableData.value = []
    pagination.current = 1
    pagination.total = 0
    hasError.value = false
  }

  // 执行搜索
  const handleSearch = async () => {
    loading.value = true
    hasError.value = false

    try {
      let params: any = {
        current: pagination.current,
        size: pagination.size
      }

      if (queryType.value === 'customer') {
        // 散客查询
        params = {
          ...params,
          name: customerSearch.customerName || '',
          phone: customerSearch.telephone || '',
          diagnosis: customerSearch.diagnosis || '',
          gender: customerSearch.gender || '',
          min_age: customerSearch.minAge,
          max_age: customerSearch.maxAge
        }

        if (customerSearch.dateRange && customerSearch.dateRange.length === 2) {
          params.start_date = customerSearch.dateRange[0]
          params.end_date = customerSearch.dateRange[1]
        }

        const response = await getCustomerRecordList(params)
        if (response.code === 200) {
          tableData.value = response.data.records
          pagination.total = response.data.total
        }
      } else if (queryType.value === 'medical') {
        // 门诊病历查询
        params = {
          ...params,
          customer_name: medicalSearch.customerName || '',
          telephone: medicalSearch.telephone || '',
          diagnosis: medicalSearch.diagnosis || '',
          gender: medicalSearch.gender || '',
          min_age: medicalSearch.minAge,
          max_age: medicalSearch.maxAge
        }

        if (medicalSearch.dateRange && medicalSearch.dateRange.length === 2) {
          params.start_date = medicalSearch.dateRange[0]
          params.end_date = medicalSearch.dateRange[1]
        }

        const response = await getMedicalRecordListLegacy(params)
        if (response.code === 200) {
          tableData.value = response.data.records.map((item) => ({
            ...item,
            doctor_name:
              doctorList.value.find((d) => d.id === Number(item.doctor_id))?.name || '未知医师'
          }))
          pagination.total = response.data.total
        }
      } else if (queryType.value === 'prescription') {
        // 中药处方查询
        params = {
          ...params,
          customer_name: prescriptionSearch.customerName || '',
          telephone: prescriptionSearch.telephone || '',
          diagnosis: prescriptionSearch.diagnosis || '',
          gender: prescriptionSearch.gender || '',
          min_age: prescriptionSearch.minAge,
          max_age: prescriptionSearch.maxAge
        }

        if (prescriptionSearch.dateRange && prescriptionSearch.dateRange.length === 2) {
          params.start_date = prescriptionSearch.dateRange[0]
          params.end_date = prescriptionSearch.dateRange[1]
        }

        const response = await getPrescriptionRecordList(params)
        if (response.code === 200) {
          tableData.value = response.data.records
          pagination.total = response.data.total
        }
      }
    } catch (error) {
      console.error('查询失败:', error)
      hasError.value = true
      ElMessage.error('查询失败，请重试')
    } finally {
      loading.value = false
    }
  }

  // 重置搜索
  const handleReset = () => {
    if (queryType.value === 'customer') {
      Object.assign(customerSearch, {
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
      Object.assign(medicalSearch, {
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
      Object.assign(prescriptionSearch, {
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

    // 先设置基础数据，治疗项目设为undefined表示加载中
    selectedItem.value = {
      ...row,
      treatmentItems: undefined
    }

    // 打开弹窗
    detailVisible.value = true

    // 如果是散客详情，加载治疗项目
    if (queryType.value === 'customer' && row.id) {
      try {
        // 通过统一医疗记录API，治疗项目已包含在主记录中
        console.log('散客记录详情:', row)
        selectedItem.value = {
          ...row,
          treatmentItems: row.treatment_items || []
        }
        console.log('已设置治疗项目数据:', selectedItem.value.treatmentItems)
        console.log('治疗项目数量:', selectedItem.value.treatmentItems?.length)
      } catch (error) {
        console.error('处理散客记录详情失败:', error)
        selectedItem.value = {
          ...row,
          treatmentItems: []
        }
      }
    }

    // 如果是门诊病历详情，直接使用治疗项目数据
    if (queryType.value === 'medical') {
      try {
        console.log('门诊病历数据包含治疗项目:', row.treatment_items)
        selectedItem.value = {
          ...row,
          treatmentItems: row.treatment_items || []
        }
        console.log('已设置门诊病历治疗项目数据:', selectedItem.value.treatmentItems)
      } catch (error) {
        console.error('处理门诊病历治疗项目失败:', error)
        selectedItem.value = {
          ...row,
          treatmentItems: []
        }
      }
    }

    // 如果是中药处方详情，处理处方项目数据
    if (queryType.value === 'prescription') {
      try {
        console.log('中药处方数据包含处方项目:', row.prescription_items)
        selectedItem.value = {
          ...row,
          prescription_items: row.prescription_items || []
        }
        console.log('已设置中药处方项目数据:', selectedItem.value.prescription_items)
      } catch (error) {
        console.error('处理中药处方项目失败:', error)
        selectedItem.value = {
          ...row,
          prescription_items: []
        }
      }
    }
  }

  // 关闭详情
  const handleCloseDetail = () => {
    detailVisible.value = false
    selectedItem.value = null
  }

  // 打印门诊病历
  const handlePrintMedicalRecord = async () => {
    if (!selectedItem.value) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建门诊病历打印数据
      const medicalData: MedicalRecordData = {
        customer_name: selectedItem.value.customer_name,
        gender: selectedItem.value.gender,
        age: selectedItem.value.age,
        telephone: selectedItem.value.telephone,
        time_of_visit: selectedItem.value.time_of_visit,
        trade_type: selectedItem.value.trade_type,
        chief_complaint: selectedItem.value.chief_complaint,
        illness_history: selectedItem.value.illness_history,
        physical_examination: selectedItem.value.physical_examination,
        past_history: selectedItem.value.past_history,
        laboratory_examination: selectedItem.value.laboratory_examination,
        diagnosis: selectedItem.value.diagnosis,
        treatment_method: selectedItem.value.treatment_method,
        note: selectedItem.value.note,
        treatment_items: (selectedItem.value.treatmentItems || []).map((item: any) => ({
          treatment_item: item.treatment_item_name,
          count: item.treatment_count,
          unit_price: item.unit_price,
          total_price: item.total_price
        })),
        doctor_name: selectedItem.value.doctor_name
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
    if (!selectedItem.value) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建费用清单打印数据
      const feeListData: FeeListData = {
        customer_name: selectedItem.value.customer_name,
        gender: selectedItem.value.gender,
        age: selectedItem.value.age,
        telephone: selectedItem.value.telephone,
        time_of_visit: selectedItem.value.time_of_visit,
        trade_type: selectedItem.value.trade_type,
        diagnosis: selectedItem.value.diagnosis,
        note: selectedItem.value.note,
        treatment_items: (selectedItem.value.treatmentItems || []).map((item: any) => ({
          treatment_item: item.treatment_item_name,
          count: item.treatment_count,
          unit_price: item.unit_price,
          total_price: item.total_price
        })),
        total_amount: (selectedItem.value.treatmentItems || []).reduce(
          (sum: number, item: any) => sum + (item.total_price || 0),
          0
        ),
        doctor_name: selectedItem.value.doctor_name
      }

      await printFeeList(feeListData)
      ElMessage.success('费用清单打印成功')
    } catch (error) {
      console.error('打印费用清单失败:', error)
      ElMessage.error('打印失败，请重试')
    }
  }

  // 打印中药处方
  const handlePrintPrescription = async () => {
    if (!selectedItem.value) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建中药处方打印数据
      const prescriptionData: PrescriptionData = {
        customer_name: selectedItem.value.customer_name,
        gender: selectedItem.value.gender,
        age: selectedItem.value.age,
        telephone: selectedItem.value.telephone,
        time_of_visit: selectedItem.value.time_of_visit,
        trade_type: selectedItem.value.trade_type,
        diagnosis: selectedItem.value.diagnosis,
        note: selectedItem.value.note,
        prescription_items: (selectedItem.value.prescription_items || []).map((item: any) => ({
          medicine_name: item.medicine_name,
          grams: item.grams,
          unit_price: item.unit_price,
          total_price: item.total_price
        })),
        doses: selectedItem.value.doses,
        single_dose_amount: selectedItem.value.single_dose_amount,
        total_amount: selectedItem.value.total_amount,
        doctor_name: selectedItem.value.doctor_name
      }

      await printPrescription(prescriptionData)
      ElMessage.success('中药处方打印成功')
    } catch (error) {
      console.error('打印中药处方失败:', error)
      ElMessage.error('打印失败，请重试')
    }
  }

  // 打印中药处方费用清单
  const handlePrintPrescriptionFeeList = async () => {
    if (!selectedItem.value) {
      ElMessage.warning('请先选择要打印的记录')
      return
    }

    try {
      // 构建费用清单打印数据（基于中药处方）
      const feeListData: FeeListData = {
        customer_name: selectedItem.value.customer_name,
        gender: selectedItem.value.gender,
        age: selectedItem.value.age,
        telephone: selectedItem.value.telephone,
        time_of_visit: selectedItem.value.time_of_visit,
        trade_type: selectedItem.value.trade_type,
        diagnosis: selectedItem.value.diagnosis,
        note: selectedItem.value.note,
        treatment_items: (selectedItem.value.prescription_items || []).map((item: any) => {
          const singleTotal = item.grams * item.unit_price
          return {
            treatment_item: `${item.medicine_name} (${item.grams}g)`,
            count: selectedItem.value.doses || 1,
            unit_price: singleTotal,
            total_price: singleTotal * (selectedItem.value.doses || 1)
          }
        }),
        total_amount: selectedItem.value.total_amount,
        doctor_name: selectedItem.value.doctor_name
      }

      await printFeeList(feeListData)
      ElMessage.success('费用清单打印成功')
    } catch (error) {
      console.error('打印费用清单失败:', error)
      ElMessage.error('打印失败，请重试')
    }
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

    .detail-content {
      max-height: 70vh;
      overflow-y: auto;

      .el-descriptions {
        margin-bottom: 20px;
      }

      h4 {
        padding-bottom: 8px;
        color: var(--el-text-color-primary);
        border-bottom: 2px solid var(--el-color-primary);
      }
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
