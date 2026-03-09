<template>
  <div class="customer-card-list">
    <div class="form-wrap">
      <ElForm
        :model="form"
        :rules="formRules"
        ref="formRef"
        label-width="100px"
        size="default"
        require-asterisk-position="right"
      >
        <!-- 基本信息区域 -->
        <ElRow :gutter="20">
          <ElCol :span="8">
            <ElFormItem label="患者姓名" prop="customerName">
              <ElInput
                placeholder="请输入患者姓名"
                v-model="form.customerName"
                maxlength="20"
                show-word-limit
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="8">
            <ElFormItem label="患者性别" prop="gender">
              <ElSelect v-model="form.gender" placeholder="请选择性别" style="width: 100%">
                <ElOption
                  v-for="option in GENDER_OPTIONS"
                  :key="option.value"
                  :label="option.label"
                  :value="option.value"
                />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="8">
            <ElFormItem label="年龄" prop="age">
              <ElInput
                v-model.number="form.age"
                type="number"
                min="0"
                max="200"
                placeholder="请输入年龄"
                style="width: 100%"
              >
                <template #append>岁</template>
              </ElInput>
            </ElFormItem>
          </ElCol>
        </ElRow>

        <ElRow :gutter="20">
          <ElCol :span="8">
            <ElFormItem label="联系电话" prop="telephone">
              <ElInput
                placeholder="请输入联系电话"
                v-model="form.telephone"
                type="tel"
                maxlength="11"
                show-word-limit
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="8">
            <ElFormItem label="就诊时间" prop="timeOfVisit">
              <ElDatePicker
                v-model="form.timeOfVisit"
                type="datetime"
                placeholder="请选择就诊时间"
                style="width: 100%"
                format="YYYY-MM-DD HH:mm"
                value-format="YYYY-MM-DD HH:mm:ss"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="8">
            <ElFormItem label="结算类型" prop="tradeType">
              <ElSelect v-model="form.tradeType" placeholder="请选择结算类型" style="width: 100%">
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

        <!-- 诊断信息区域 -->
        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="诊断信息" prop="diagnosis">
              <ElInput
                type="textarea"
                placeholder="请输入诊断信息"
                v-model="form.diagnosis"
                show-word-limit
                maxlength="200"
                :rows="3"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="备注" prop="note">
              <ElInput
                type="textarea"
                placeholder="请输入备注"
                v-model="form.note"
                show-word-limit
                maxlength="100"
                :rows="2"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <!-- 治疗项目区域 -->
        <ElFormItem label="治疗项目">
          <div class="treatment-section">
            <div class="treatment-header">
              <ElRow :gutter="10" align="middle">
                <ElCol :span="8"><span class="header-text">项目名称</span></ElCol>
                <ElCol :span="4"><span class="header-text">单价(元)</span></ElCol>
                <ElCol :span="4"><span class="header-text">数量</span></ElCol>
                <ElCol :span="4"><span class="header-text">金额(元)</span></ElCol>
                <ElCol :span="4"><span class="header-text">操作</span></ElCol>
              </ElRow>
            </div>

            <div class="treatment-items">
              <div v-for="(item, index) in form.treatment" :key="index" class="treatment-item">
                <ElRow :gutter="10" align="middle">
                  <ElCol :span="8">
                    <ElFormItem
                      :prop="`treatment.${index}.treatmentItem`"
                      :rules="[{ required: true, message: '请选择治疗项目', trigger: 'change' }]"
                    >
                      <ElSelect
                        v-model="item.treatmentItem"
                        filterable
                        remote
                        :remote-method="searchHealingItems"
                        :loading="healingItemsLoading"
                        placeholder="请输入项目名称"
                        style="width: 100%"
                        @change="handleTreatmentItemChange"
                      >
                        <ElOption
                          v-for="healingItem in filteredHealingItems"
                          :key="healingItem.id"
                          :label="healingItem.name"
                          :value="healingItem.name"
                          :data-price="healingItem.price"
                        />
                      </ElSelect>
                    </ElFormItem>
                  </ElCol>
                  <ElCol :span="4">
                    <div class="price-display">{{
                      formatAmountFromTenThousandths(getUnitPriceTenThousandths(item.treatmentItem))
                    }}</div>
                  </ElCol>
                  <ElCol :span="4">
                    <ElFormItem
                      :prop="`treatment.${index}.count`"
                      :rules="[
                        { required: true, message: '请输入数量', trigger: 'blur' },
                        { type: 'number', min: 1, message: '数量不能小于1', trigger: 'blur' }
                      ]"
                    >
                      <ElInput
                        v-model.number="item.count"
                        type="number"
                        min="1"
                        placeholder="数量"
                        @input="calculateTotalPrice"
                      />
                    </ElFormItem>
                  </ElCol>
                  <ElCol :span="4">
                    <div class="price-display">{{
                      formatAmountFromTenThousandths(
                        getUnitPriceTenThousandths(item.treatmentItem) * (item.count || 0)
                      )
                    }}</div>
                  </ElCol>
                  <ElCol :span="4">
                    <div class="action-button-wrapper">
                      <ElButton
                        type="danger"
                        size="small"
                        @click="removeTreatmentItem(index)"
                        :disabled="form.treatment.length === 1"
                      >
                        删除
                      </ElButton>
                    </div>
                  </ElCol>
                </ElRow>
              </div>
            </div>

            <div class="treatment-actions">
              <ElButton type="primary" @click="addTreatmentItem">
                <ElIcon><Plus /></ElIcon>
                添加项目
              </ElButton>
            </div>
          </div>
        </ElFormItem>

        <!-- 总金额显示 -->
        <ElRow :gutter="20">
          <ElCol :span="24">
            <div class="total-amount">
              <span class="total-label">总金额：</span>
              <span class="total-value">{{
                formatAmountFromTenThousandths(totalAmountTenThousandths)
              }}</span>
            </div>
          </ElCol>
        </ElRow>

        <!-- 操作按钮 -->
        <ElRow :gutter="20">
          <ElCol :span="24">
            <div class="form-actions">
              <ElButton @click="resetForm">重置</ElButton>
              <ElButton type="primary" @click="submitForm" :loading="loading"> 提交登记 </ElButton>
            </div>
          </ElCol>
        </ElRow>
      </ElForm>
    </div>
  </div>
</template>

<script setup lang="ts">
  import {
    ElFormItem,
    ElInput,
    ElSelect,
    ElOption,
    ElMessage,
    ElMessageBox,
    FormInstance,
    ElIcon
  } from 'element-plus'
  import { Plus } from '@element-plus/icons-vue'
  import { ref, computed, onMounted } from 'vue'
  import { createCustomerRecord } from '@/api/unifiedMedicalRecordApi'
  import { getHealingList } from '@/api/healingApi'
  import { GENDER_OPTIONS, TRADE_TYPE_OPTIONS } from '@/types/medical'
  import { formatAmountFromTenThousandths } from '@/utils/dataprocess/format'

  // 前端表单数据接口
  interface CustomerForm {
    customerName: string
    gender: string
    age: number
    telephone: string
    timeOfVisit: string
    tradeType: string
    diagnosis: string
    note: string
    treatment: {
      treatmentItem: string
      count: number
    }[]
  }

  // 表单引用
  const formRef = ref<FormInstance>()

  // 状态管理
  const loading = ref(false)
  const healingItemsLoading = ref(false)
  const isSubmitting = ref(false) // 防止重复提交

  // 康复项目数据
  const healingItems = ref<Api.Healing.Item[]>([])
  const filteredHealingItems = ref<Api.Healing.Item[]>([])

  // 获取当前时间的格式化字符串
  const getCurrentDateTime = () => {
    const now = new Date()
    const year = now.getFullYear()
    const month = String(now.getMonth() + 1).padStart(2, '0')
    const day = String(now.getDate()).padStart(2, '0')
    const hours = String(now.getHours()).padStart(2, '0')
    const minutes = String(now.getMinutes()).padStart(2, '0')
    const seconds = String(now.getSeconds()).padStart(2, '0')
    return `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`
  }

  // 表单数据
  const form = ref<CustomerForm>({
    customerName: '',
    gender: '',
    age: 0,
    telephone: '',
    timeOfVisit: getCurrentDateTime(),
    tradeType: '',
    diagnosis: '',
    note: '',
    treatment: [
      {
        treatmentItem: '',
        count: 1
      }
    ]
  })

  // 表单验证规则
  const formRules = {
    customerName: [
      { required: true, message: '请输入患者姓名', trigger: 'blur' },
      { max: 20, message: '患者姓名长度不能超过20个字符', trigger: 'blur' }
    ],
    gender: [{ required: true, message: '请选择性别', trigger: 'change' }],
    age: [
      { required: true, message: '请输入年龄', trigger: 'blur' },
      {
        type: 'number' as const,
        min: 0,
        max: 200,
        message: '年龄必须在0到200之间',
        trigger: 'blur'
      }
    ],
    telephone: [{ pattern: /^1[3-9]\d{9}$/, message: '请输入正确的手机号格式', trigger: 'blur' }],
    timeOfVisit: [{ required: true, message: '请选择就诊时间', trigger: 'change' }],
    tradeType: [{ required: true, message: '请选择结算类型', trigger: 'change' }],
    diagnosis: [
      { required: true, message: '请输入诊断信息', trigger: 'blur' },
      { max: 200, message: '诊断信息长度不能超过200个字符', trigger: 'blur' }
    ],
    note: [{ max: 100, message: '备注信息长度不能超过100个字符', trigger: 'blur' }]
  }

  // 获取康复项目列表
  const fetchHealingItems = async () => {
    try {
      const res = await getHealingList({
        current: 1,
        size: 100,
        name: ''
      })
      if (res.code === 200) {
        healingItems.value = res.data.records || []
        filteredHealingItems.value = res.data.records || []
        console.log('成功加载康复项目列表，共', healingItems.value.length, '条记录')

        // 数据验证：检查是否有价格为0的项目（可能提示数据库问题）
        const zeroPrice = healingItems.value.filter((item) => !item.price || item.price === 0)
        if (zeroPrice.length > 0) {
          const zeroPriceNames = zeroPrice.map((item) => item.name)
          console.warn('发现', zeroPrice.length, '条康复项目单价为0或未定义', zeroPriceNames)
        }
      } else {
        console.error('获取康复项目失败:', res.msg)
        ElMessage.error(`获取康复项目失败: ${res.msg}`)
      }
    } catch (error) {
      console.error('获取康复项目失败:', error)
      ElMessage.error('加载康复项目失败，请检查网络连接')
    }
  }

  // 搜索康复项目（支持拼音）
  const searchHealingItems = async (query: string) => {
    if (!query) {
      filteredHealingItems.value = healingItems.value
      return
    }

    healingItemsLoading.value = true
    try {
      const res = await getHealingList({
        current: 1,
        size: 100,
        name: query
      })
      if (res.code === 200) {
        const records = res.data.records
        // 关键修复：同时更新 filteredHealingItems 和 healingItems
        // 确保搜索结果也被加入主列表，避免后续 getUnitPriceTenThousandths 查找失败
        filteredHealingItems.value = records

        // 合并新搜索结果到主列表（避免重复）
        records.forEach((newItem) => {
          if (!healingItems.value.find((item) => item.id === newItem.id)) {
            healingItems.value.push(newItem)
          }
        })

        console.log('搜索康复项目成功，共获得', records.length, '条记录')
      } else {
        console.error('搜索康复项目API返回错误:', res.msg)
        ElMessage.warning(`搜索康复项目失败: ${res.msg}`)
        // 降级处理：本地过滤搜索
        const filtered = healingItems.value.filter((item) =>
          item.name.toLowerCase().includes(query.toLowerCase())
        )
        filteredHealingItems.value = filtered
      }
    } catch (error) {
      console.error('搜索康复项目请求失败:', error)
      ElMessage.error('搜索康复项目请求失败，请检查网络连接')
      // 本地过滤搜索作为备选
      const filtered = healingItems.value.filter((item) =>
        item.name.toLowerCase().includes(query.toLowerCase())
      )
      filteredHealingItems.value = filtered
    } finally {
      healingItemsLoading.value = false
    }
  }

  // 获取单价（关键函数：务必确保能查找到价格）
  const getUnitPriceTenThousandths = (name: string): number => {
    if (!name) {
      console.warn('getUnitPriceTenThousandths: 康复项目名称为空')
      return 0
    }

    const item = healingItems.value.find((item) => item.name === name)
    if (!item) {
      console.warn(`getUnitPriceTenThousandths: 未找到康复项目 "${name}"，可能原因：`, {
        healingItemsCount: healingItems.value.length,
        itemNameList: healingItems.value.map((item) => item.name).slice(0, 5)
      })
      return 0
    }

    const price = item.price || 0
    if (price === 0) {
      console.warn(`getUnitPriceTenThousandths: 康复项目 "${name}" 的单价为0或未定义`, {
        item
      })
    }

    return price
  }

  // 计算总金额（显示为元）
  const totalAmountTenThousandths = computed(() => {
    let totalTenThousandths = 0

    form.value.treatment.forEach((treatmentItem) => {
      totalTenThousandths +=
        getUnitPriceTenThousandths(treatmentItem.treatmentItem) * (treatmentItem.count || 0)
    })

    // 转换为元显示
    return totalTenThousandths
  })

  // 处理治疗项目变更
  const handleTreatmentItemChange = () => {
    // 重新计算总价
    calculateTotalPrice()
  }

  // 计算总价
  const calculateTotalPrice = () => {
    // 触发计算属性重新计算
    form.value = { ...form.value }
  }

  // 添加治疗项目
  const addTreatmentItem = () => {
    // 检查最后一项是否完整
    const lastItem = form.value.treatment[form.value.treatment.length - 1]
    if (!lastItem.treatmentItem || !lastItem.count) {
      ElMessage.warning('请先完善当前治疗项目信息')
      return
    }

    form.value.treatment.push({
      treatmentItem: '',
      count: 1
    })
  }

  // 删除治疗项目
  const removeTreatmentItem = (index: number) => {
    if (form.value.treatment.length === 1) {
      ElMessage.warning('至少需要保留一个治疗项目')
      return
    }
    form.value.treatment.splice(index, 1)
    calculateTotalPrice()
  }

  // 重置表单
  const resetForm = () => {
    formRef.value?.resetFields()
    form.value = {
      customerName: '',
      gender: '',
      age: 0,
      telephone: '',
      timeOfVisit: getCurrentDateTime(),
      tradeType: '',
      diagnosis: '',
      note: '',
      treatment: [
        {
          treatmentItem: '',
          count: 1
        }
      ]
    }
  }

  // 提交表单
  const submitForm = async () => {
    if (!formRef.value) return

    const isValid = await formRef.value.validate().catch(() => false)
    if (!isValid) {
      ElMessage.warning('请完善所有必填信息后再提交')
      return
    }

    // 防止重复提交
    if (isSubmitting.value) {
      // 二次确认
      const confirmed = await ElMessageBox.confirm(
        '检测到正在提交或刚刚提交过，是否确认重新提交？',
        '重复提交确认',
        {
          confirmButtonText: '确认提交',
          cancelButtonText: '取消',
          type: 'warning'
        }
      ).catch(() => false)

      if (!confirmed) {
        return
      }
    }

    // 验证治疗项目
    const hasIncompleteTreatment = form.value.treatment.some(
      (item) => !item.treatmentItem || !item.count || item.count < 1
    )
    if (hasIncompleteTreatment) {
      ElMessage.warning('请完善治疗项目信息')
      return
    }

    try {
      loading.value = true
      isSubmitting.value = true

      // 使用新的统一医疗记录API
      const unifiedRecordData = {
        customer_name: form.value.customerName,
        gender: form.value.gender,
        age: form.value.age,
        telephone: form.value.telephone,
        time_of_visit: form.value.timeOfVisit,
        trade_type: form.value.tradeType,
        diagnosis: form.value.diagnosis,
        note: form.value.note,
        total_amount: totalAmountTenThousandths.value,
        treatment_items: form.value.treatment.map((item) => {
          const unitPrice = getUnitPriceTenThousandths(item.treatmentItem)

          // 添加调试日志
          if (unitPrice === 0) {
            console.error(`警告：康复项目 "${item.treatmentItem}" 的单价为0`, {
              count: item.count,
              calculatedTotalPrice: unitPrice * item.count
            })
          }

          return {
            treatment_item: item.treatmentItem,
            count: item.count,
            unit_price: unitPrice,
            total_price: unitPrice * item.count
          }
        })
      }

      console.log('准备创建散客登记记录 (统一API):', unifiedRecordData)

      // 数据验证
      const hasPriceZero = unifiedRecordData.treatment_items.some((item) => item.unit_price === 0)
      if (hasPriceZero) {
        const confirmation = await ElMessageBox.confirm(
          '检测到部分康复项目单价为0，可能是查询失败。是否强制提交？',
          '数据验证警告',
          {
            confirmButtonText: '强制提交',
            cancelButtonText: '取消',
            type: 'warning'
          }
        ).catch(() => false)

        if (!confirmation) return
      }

      const response = await createCustomerRecord(unifiedRecordData)
      console.log('散客登记响应 (统一API):', response)

      if (response.code === 200) {
        ElMessage.success('散客登记成功！')
        // 不重置表单，保持当前数据
      } else {
        ElMessage.error(response.msg || '登记失败')
      }
    } catch (error) {
      console.error('提交散客登记失败:', error)
      ElMessage.error('登记失败，请重试')
    } finally {
      loading.value = false
      // 延迟重置提交状态，防止快速重复点击
      setTimeout(() => {
        isSubmitting.value = false
      }, 2000)
    }
  }

  // 组件挂载时获取数据
  onMounted(() => {
    fetchHealingItems()
  })
</script>

<style lang="scss" scoped>
  .customer-card-list {
    max-width: 1200px;
    margin: 20px auto;
    border: 1px solid var(--art-border-color);
    border-radius: calc(var(--custom-radius) / 2 + 2px) !important;

    .form-wrap {
      padding: 30px;
      background-color: var(--art-main-bg-color);

      .treatment-section {
        width: 100%;

        .treatment-header {
          padding: 10px 0;
          margin-bottom: 10px;
          background-color: var(--el-fill-color-light);
          border-radius: 4px;

          .header-text {
            display: block;
            font-weight: 600;
            color: var(--el-text-color-primary);
            text-align: center;
          }
        }

        .treatment-item {
          padding: 10px;
          margin-bottom: 15px;
          background-color: var(--el-bg-color);
          border: 1px solid var(--el-border-color-light);
          border-radius: 4px;

          &:hover {
            border-color: var(--el-color-primary);
            box-shadow: 0 2px 4px rgb(64 158 255 / 10%);
          }
        }

        .treatment-actions {
          padding: 10px;
          margin-top: 15px;
          text-align: center;
          background-color: var(--el-fill-color-blank);
          border: 2px dashed var(--el-border-color);
          border-radius: 4px;

          &:hover {
            background-color: var(--el-color-primary-light-9);
            border-color: var(--el-color-primary);
          }
        }
      }

      .price-display {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 32px;
        padding: 8px 12px;
        font-weight: 600;
        color: var(--el-color-primary);
        text-align: center;
        background-color: var(--el-color-primary-light-9);
        border-radius: 4px;
      }

      .action-button-wrapper {
        display: flex;
        align-items: center;
        justify-content: center;
        min-height: 32px;
      }

      .total-amount {
        padding: 20px 0;
        margin-top: 20px;
        text-align: right;
        border-top: 2px solid var(--el-border-color);

        .total-label {
          margin-right: 10px;
          font-size: 18px;
          font-weight: 600;
          color: var(--el-text-color-primary);
        }

        .total-value {
          font-size: 24px;
          font-weight: 700;
          color: var(--el-color-danger);
        }
      }

      .form-actions {
        padding-top: 20px;
        margin-top: 20px;
        text-align: center;
        border-top: 1px solid var(--el-border-color-light);

        .el-button {
          padding: 12px 30px;
          margin: 0 10px;
          font-size: 16px;
        }
      }
    }
  }

  // 响应式布局
  @media (width <= 768px) {
    .customer-card-list {
      margin: 10px;

      .form-wrap {
        padding: 20px;
      }
    }
  }
</style>
