<template>
  <div class="medical-record-card-list">
    <div class="form-wrap">
      <ElForm
        :model="form"
        :rules="formRules"
        ref="formRef"
        label-width="120px"
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
              <ElSelect v-model="form.gender" placeholder="请选择性别" class="full-width">
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
                class="full-width"
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
                class="full-width"
                format="YYYY-MM-DD HH:mm"
                value-format="YYYY-MM-DD HH:mm:ss"
              />
            </ElFormItem>
          </ElCol>
          <ElCol :span="8">
            <ElFormItem label="结算类型" prop="tradeType">
              <ElSelect v-model="form.tradeType" placeholder="请选择结算类型" class="full-width">
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

        <!-- 病史信息区域 -->
        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="主诉" prop="chiefComplaint">
              <ElInput
                type="textarea"
                placeholder="请输入主诉"
                v-model="form.chiefComplaint"
                show-word-limit
                maxlength="50"
                :rows="2"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="现病史" prop="illnessHistory">
              <ElInput
                type="textarea"
                placeholder="请输入现病史"
                v-model="form.illnessHistory"
                show-word-limit
                maxlength="200"
                :rows="3"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="体格检查" prop="physicalExamination">
              <ElInput
                type="textarea"
                placeholder="请输入体格检查"
                v-model="form.physicalExamination"
                show-word-limit
                maxlength="200"
                :rows="3"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="既往史" prop="pastHistory">
              <ElInput
                type="textarea"
                placeholder="请输入既往史"
                v-model="form.pastHistory"
                show-word-limit
                maxlength="200"
                :rows="3"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="实验室检查" prop="laboratoryExamination">
              <ElInput
                type="textarea"
                placeholder="请输入实验室检查"
                v-model="form.laboratoryExamination"
                show-word-limit
                maxlength="100"
                :rows="2"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <!-- 诊断和治疗区域 -->
        <ElRow :gutter="20">
          <ElCol :span="12">
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
          <ElCol :span="12">
            <ElFormItem label="治法治则" prop="treatmentMethod">
              <ElInput
                type="textarea"
                placeholder="请输入治法治则"
                v-model="form.treatmentMethod"
                show-word-limit
                maxlength="100"
                :rows="3"
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
                        class="full-width"
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
                    <div class="price-display">{{ getUnitPrice(index) }}</div>
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
                    <div class="price-display">{{ getItemTotalPrice(index) }}</div>
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

        <!-- 总金额显示 -->
        <ElRow :gutter="20">
          <ElCol :span="24">
            <div class="total-amount">
              <span class="total-label">总金额：</span>
              <span class="total-value"
                >¥{{ convertTenThousandthsToYuan(totalTenThousandthsAmount) }}</span
              >
            </div>
          </ElCol>
        </ElRow>

        <!-- 操作按钮区域 -->
        <ElRow :gutter="20">
          <ElCol :span="8">
            <ElFormItem label="医师" prop="doctorId">
              <ElSelect v-model="form.doctorId" placeholder="请选择医师" class="full-width">
                <ElOption
                  v-for="doctor in doctorList"
                  :key="doctor.id"
                  :label="doctor.name"
                  :value="String(doctor.id)"
                />
              </ElSelect>
            </ElFormItem>
          </ElCol>
          <ElCol :span="16">
            <div class="form-actions">
              <ElButton @click="resetForm">重置</ElButton>
              <ElButton type="primary" @click="submitRegistration" :loading="loading">
                提交登记
              </ElButton>
              <ElButton type="success" @click="printRecord" plain> 生成门诊病历 </ElButton>
              <ElButton type="warning" @click="printFeeList" plain> 生成费用清单 </ElButton>
            </div>
          </ElCol>
        </ElRow>
      </ElForm>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { computed, nextTick, onMounted, reactive, ref } from 'vue'
  import {
    ElButton,
    ElForm,
    ElFormItem,
    ElRow,
    ElCol,
    ElDatePicker,
    ElInput,
    ElMessage,
    ElMessageBox,
    ElOption,
    ElSelect,
    ElIcon,
    type FormRules,
    type FormInstance
  } from 'element-plus'
  import { Plus } from '@element-plus/icons-vue'
  import { getInstitutionInfo } from '@/api/institutionApi'
  import { getHealingList } from '@/api/healingApi'
  import { createMedicalRecord as createMedicalRecord } from '@/api/unifiedMedicalRecordApi'
  import { GENDER_OPTIONS, TRADE_TYPE_OPTIONS } from '@/config/common'
  import {
    printMedicalRecord as printMedicalRecordTemplate,
    printFeeList as printFeeListTemplate,
    type MedicalRecordData,
    type FeeListData
  } from '@/utils/print'
  import {
    formatAmountFromTenThousandths,
    convertTenThousandthsToYuan
  } from '@/utils/dataprocess/format'

  // 移除缺失的依赖项调用
  // const { initColors } = useTheme()
  // const { initFastEnterKey } = useFastEnter()

  // 表单数据
  interface TreatmentFormItem {
    treatmentItem: string
    count: number
  }

  const form = reactive({
    customerName: '',
    gender: '',
    age: null as number | null,
    telephone: '',
    timeOfVisit: new Date() as Date | string,
    tradeType: '',
    chiefComplaint: '',
    illnessHistory: '',
    physicalExamination: '',
    pastHistory: '',
    laboratoryExamination: '',
    diagnosis: '',
    treatmentMethod: '',
    note: '',
    treatment: [{ treatmentItem: '', count: 1 }] as TreatmentFormItem[],
    doctorId: ''
  })

  // 表单验证规则
  const formRules: FormRules = {
    customerName: [{ required: true, message: '请输入患者姓名', trigger: 'blur' }],
    gender: [{ required: true, message: '请选择性别', trigger: 'change' }],
    age: [
      { required: true, message: '请输入年龄', trigger: 'blur' },
      { type: 'number', min: 0, max: 200, message: '年龄应在0-200之间', trigger: 'blur' }
    ],
    telephone: [{ pattern: /^1[3-9]\d{9}$/, message: '请输入正确的手机号格式', trigger: 'blur' }],
    timeOfVisit: [{ required: true, message: '请选择就诊时间', trigger: 'change' }],
    tradeType: [{ required: true, message: '请选择结算类型', trigger: 'change' }],
    chiefComplaint: [{ required: true, message: '请输入主诉', trigger: 'blur' }],
    diagnosis: [{ required: true, message: '请输入诊断信息', trigger: 'blur' }],
    doctorId: [{ required: true, message: '请选择医师', trigger: 'change' }]
  }

  // 表单引用
  const formRef = ref<FormInstance>()

  // 状态管理
  const loading = ref(false)
  const healingItemsLoading = ref(false)
  const isSubmitting = ref(false) // 防止重复提交

  // 医师数据
  const doctorList = ref<{ id: number; name: string }[]>([])

  // 加载医师列表
  const loadDoctorList = async () => {
    try {
      const res = await getInstitutionInfo()
      if (res.code === 200) {
        // 从机构信息中获取医师列表
        doctorList.value = res.data.doctor_management.map((doctor, index) => ({
          id: index + 1,
          name: doctor.name
        }))
      } else {
        console.error('获取机构信息失败:', res.msg)
        // 如果API失败，使用默认数据
        doctorList.value = [
          { id: 1, name: '李明' },
          { id: 2, name: '张丽' }
        ]
      }
    } catch (error) {
      console.error('加载医师列表失败:', error)
      // 使用默认数据
      doctorList.value = [
        { id: 1, name: '李明' },
        { id: 2, name: '张丽' }
      ]
    }
  }

  // 治疗项目数据
  const healingItems = ref<Api.Healing.Item[]>([])
  const filteredHealingItems = ref<Api.Healing.Item[]>([])

  // 加载治疗项目
  const loadHealingItems = async () => {
    try {
      const res = await getHealingList({
        current: 1,
        size: 100,
        name: ''
      })
      if (res.code === 200) {
        healingItems.value = res.data.records
        filteredHealingItems.value = res.data.records
      } else {
        console.error('获取康养项目失败:', res.msg)
        // 使用默认数据作为备选
        const mockHealingItems: Api.Healing.Item[] = [
          {
            id: 1,
            name: '针灸',
            price: 100
          },
          {
            id: 2,
            name: '推拿',
            price: 150
          },
          {
            id: 3,
            name: '拔罐',
            price: 80
          },
          {
            id: 4,
            name: '艾灸',
            price: 120
          },
          {
            id: 5,
            name: '中药熏蒸',
            price: 200
          }
        ]
        healingItems.value = mockHealingItems
        filteredHealingItems.value = mockHealingItems
      }
    } catch (error) {
      console.error('加载治疗项目失败:', error)
      // 使用默认数据
      const mockHealingItems: Api.Healing.Item[] = [
        {
          id: 1,
          name: '针灸',
          price: 100
        },
        {
          id: 2,
          name: '推拿',
          price: 150
        },
        {
          id: 3,
          name: '拔罐',
          price: 80
        },
        {
          id: 4,
          name: '艾灸',
          price: 120
        },
        {
          id: 5,
          name: '中药熏蒸',
          price: 200
        }
      ]
      healingItems.value = mockHealingItems
      filteredHealingItems.value = mockHealingItems
      ElMessage.error('加载治疗项目失败')
    }
  }

  // 搜索治疗项目
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
        filteredHealingItems.value = res.data.records
      }
    } catch (error) {
      console.error('搜索治疗项目失败:', error)
      // 本地过滤搜索作为备选
      const filtered = healingItems.value.filter((item) =>
        item.name.toLowerCase().includes(query.toLowerCase())
      )
      filteredHealingItems.value = filtered
    } finally {
      healingItemsLoading.value = false
    }
  }

  // 处理治疗项目变化
  const handleTreatmentItemChange = () => {
    calculateTotalPrice()
  }

  // 获取单价（显示为元）
  const getUnitPrice = (index: number): string => {
    const treatmentName = form.treatment[index]?.treatmentItem
    if (!treatmentName) return '¥0.0000'

    const healingItem = healingItems.value.find((item) => item.name === treatmentName)
    if (!healingItem) return '¥0.0000'

    // healingItem.price 假设是以万分为单位存储，需要转换为元显示
    return formatAmountFromTenThousandths(healingItem.price)
  }

  // 获取项目总价（显示为元）
  const getItemTotalPrice = (index: number): string => {
    const treatmentName = form.treatment[index]?.treatmentItem
    const count = form.treatment[index]?.count || 0

    if (!treatmentName || !count) return '¥0.0000'

    const healingItem = healingItems.value.find((item) => item.name === treatmentName)
    if (!healingItem) return '¥0.0000'

    // healingItem.price 以万分为单位，计算总价（万分）
    const totalPriceInTenThousandths = healingItem.price * count
    // 转换为元显示
    return formatAmountFromTenThousandths(totalPriceInTenThousandths)
  }

  // 计算总金额（显示为元）
  const totalTenThousandthsAmount = computed(() => {
    let totalTenThousandths = 0

    form.treatment.forEach((item) => {
      if (item.treatmentItem && item.count) {
        const healingItem = healingItems.value.find((hi) => hi.name === item.treatmentItem)
        if (healingItem) {
          // healingItem.price 以万分为单位，乘以数量得到总价（万分）
          totalTenThousandths += healingItem.price * item.count
        }
      }
    })

    // 转换为元显示
    return totalTenThousandths
  })

  // 计算总价
  const calculateTotalPrice = () => {
    // 触发响应式更新
    nextTick()
  }

  // 添加治疗项目
  const addTreatmentItem = () => {
    // 检查最后一项是否完整
    const lastItem = form.treatment[form.treatment.length - 1]
    if (!lastItem.treatmentItem || !lastItem.count) {
      ElMessage.warning('请先完善当前治疗项目信息')
      return
    }
    form.treatment.push({ treatmentItem: '', count: 1 })
  }

  // 删除治疗项目
  const removeTreatmentItem = (index: number) => {
    if (form.treatment.length > 1) {
      form.treatment.splice(index, 1)
      calculateTotalPrice()
    }
  }

  // 重置表单
  const resetForm = () => {
    formRef.value?.resetFields()
    form.treatment = [{ treatmentItem: '', count: 1 }]
    form.timeOfVisit = new Date()
  }

  // 提交登记
  const submitRegistration = async () => {
    if (loading.value) return

    const isValid = (await formRef.value?.validate().catch(() => false)) || false
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

    try {
      loading.value = true
      isSubmitting.value = true
      await formRef.value?.validate()

      // 构建门诊病历数据 (统一API)
      const medicalRecordData = {
        customer_name: form.customerName,
        gender: form.gender,
        age: form.age || 0,
        telephone: form.telephone,
        time_of_visit:
          typeof form.timeOfVisit === 'string'
            ? form.timeOfVisit
            : new Date(form.timeOfVisit).toISOString().slice(0, 19).replace('T', ' '),
        trade_type: form.tradeType,
        chief_complaint: form.chiefComplaint,
        illness_history: form.illnessHistory,
        physical_examination: form.physicalExamination,
        past_history: form.pastHistory,
        laboratory_examination: form.laboratoryExamination,
        diagnosis: form.diagnosis,
        treatment_method: form.treatmentMethod,
        note: form.note,
        treatment_items: form.treatment.map((item) => {
          const healingItem = healingItems.value.find((hi) => hi.name === item.treatmentItem)
          const unitPrice = healingItem ? healingItem.price : 0
          return {
            treatment_item: item.treatmentItem,
            count: item.count,
            unit_price: unitPrice,
            total_price: unitPrice * item.count
          }
        }),
        doctor_id: parseInt(form.doctorId),
        doctor_name: doctorList.value.find((doctor) => doctor.id === parseInt(form.doctorId))?.name
      }

      // 调用后端API创建门诊病历 (统一API)
      const response = await createMedicalRecord(medicalRecordData)

      if (response.code === 200) {
        ElMessage.success('门诊病历登记成功')
        // 不重置表单，保持当前数据
        // resetForm() // 注释掉重置操作
      } else {
        ElMessage.error(response.msg || '门诊病历登记失败')
      }
    } catch (error) {
      console.error('提交门诊病历登记失败:', error)
      ElMessage.error('提交门诊病历登记失败')
    } finally {
      loading.value = false
      // 延迟重置提交状态，防止快速重复点击
      setTimeout(() => {
        isSubmitting.value = false
      }, 2000)
    }
  }

  // 生成门诊病历（打印功能）
  const printRecord = async () => {
    try {
      await formRef.value?.validate()

      // 构建门诊病历打印数据
      const printData: MedicalRecordData = {
        customer_name: form.customerName,
        gender: form.gender,
        age: form.age || 0,
        telephone: form.telephone,
        time_of_visit:
          typeof form.timeOfVisit === 'string'
            ? form.timeOfVisit
            : form.timeOfVisit.toISOString().slice(0, 19).replace('T', ' '),
        trade_type: form.tradeType,
        chief_complaint: form.chiefComplaint,
        illness_history: form.illnessHistory,
        physical_examination: form.physicalExamination,
        past_history: form.pastHistory,
        laboratory_examination: form.laboratoryExamination,
        diagnosis: form.diagnosis,
        treatment_method: form.treatmentMethod,
        note: form.note,
        treatment_items: form.treatment.map((item) => {
          const healingItem = healingItems.value.find((hi) => hi.name === item.treatmentItem)
          const unitPrice = healingItem ? healingItem.price : 0
          return {
            treatment_item: item.treatmentItem,
            count: item.count,
            unit_price: unitPrice,
            total_price: unitPrice * item.count
          }
        }),
        doctor_name: doctorList.value.find((doctor) => doctor.id === parseInt(form.doctorId))?.name
      }

      printMedicalRecordTemplate(printData)
      ElMessage.success('门诊病历打印成功')
    } catch (error) {
      console.error('打印门诊病历失败:', error)
      ElMessage.error('请先完善必填信息')
    }
  }

  // 生成费用清单（打印功能）
  const printFeeList = async () => {
    try {
      await formRef.value?.validate()

      // 构建费用清单打印数据
      const printData: FeeListData = {
        customer_name: form.customerName,
        gender: form.gender,
        age: form.age || 0,
        telephone: form.telephone,
        time_of_visit:
          typeof form.timeOfVisit === 'string'
            ? form.timeOfVisit
            : form.timeOfVisit.toISOString().slice(0, 19).replace('T', ' '),
        trade_type: form.tradeType,
        diagnosis: form.diagnosis,
        note: form.note,
        treatment_items: form.treatment.map((item) => {
          const healingItem = healingItems.value.find((hi) => hi.name === item.treatmentItem)
          const unitPrice = healingItem ? healingItem.price : 0
          return {
            treatment_item: item.treatmentItem,
            count: item.count,
            unit_price: unitPrice,
            total_price: unitPrice * item.count
          }
        }),
        total_amount: totalTenThousandthsAmount.value,
        doctor_name: doctorList.value.find((doctor) => doctor.id === parseInt(form.doctorId))?.name
      }

      printFeeListTemplate(printData)
      ElMessage.success('费用清单打印成功')
    } catch (error) {
      console.error('打印费用清单失败:', error)
      ElMessage.error('请先完善必填信息')
    }
  }

  // 初始化
  onMounted(() => {
    // 移除缺失的函数调用
    // initColors()
    // initFastEnterKey()
    loadHealingItems()
    loadDoctorList()
    // 初始化时设置当前时间
    form.timeOfVisit = new Date()
  })
</script>

<style lang="scss" scoped>
  .medical-record-card-list {
    width: 100%;
    height: 100%;
    min-height: calc(100vh - 64px);
    padding: 24px;
    background: var(--el-bg-color-page);

    .form-wrap {
      max-width: 1200px;
      padding: 24px;
      margin: 0 auto;
      background: var(--el-bg-color);
      border-radius: 8px;
      box-shadow: 0 2px 8px rgb(0 0 0 / 10%);

      .el-form {
        .el-form-item {
          margin-bottom: 16px;

          .el-input,
          .el-select,
          .el-date-picker {
            width: 100%;
          }
        }

        .full-width {
          width: 100%;
        }
      }
    }
  }

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

    .treatment-items {
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
    display: flex;
    gap: 10px;
    align-items: center;
    justify-content: flex-end;
    padding-top: 20px;

    .el-button {
      padding: 12px 30px;
      font-size: 16px;
    }
  }

  // 响应式设计
  @media (width <= 768px) {
    .medical-record-card-list {
      padding: 12px;

      .form-wrap {
        padding: 16px;

        .el-col {
          margin-bottom: 8px;
        }
      }
    }

    .treatment-section {
      .treatment-header,
      .treatment-items {
        .el-col {
          margin-bottom: 8px;
        }
      }
    }

    .form-actions {
      flex-direction: column;
      align-items: stretch;

      .el-button {
        width: 100%;
      }
    }
  }
</style>
