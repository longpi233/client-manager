<template>
  <div class="chinese-medicine-record-card-list">
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
            <ElFormItem label="舌象脉象" prop="tonguePulseSign">
              <ElInput
                type="textarea"
                placeholder="请输入舌象脉象"
                v-model="form.tonguePulseSign"
                show-word-limit
                maxlength="200"
                :rows="3"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <!-- 诊断信息区域 -->
        <ElRow :gutter="20">
          <ElCol :span="24">
            <ElFormItem label="诊断" prop="diagnosis">
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
            <ElFormItem label="治法治则" prop="treatmentMethod">
              <ElInput
                type="textarea"
                placeholder="请输入治法治则"
                v-model="form.treatmentMethod"
                show-word-limit
                maxlength="100"
                :rows="2"
              />
            </ElFormItem>
          </ElCol>
        </ElRow>

        <!-- 中药处方区域 -->
        <ElFormItem label="中药处方">
          <div class="prescription-section">
            <div class="prescription-header">
              <ElRow :gutter="10" align="middle">
                <ElCol :span="8"><span class="header-text">中药名称</span></ElCol>
                <ElCol :span="4"><span class="header-text">单价(元/克)</span></ElCol>
                <ElCol :span="4"><span class="header-text">克数(g)</span></ElCol>
                <ElCol :span="4"><span class="header-text">金额(元)</span></ElCol>
                <ElCol :span="4"><span class="header-text">操作</span></ElCol>
              </ElRow>
            </div>

            <div class="prescription-items">
              <div
                v-for="(item, index) in form.prescription"
                :key="index"
                class="prescription-item"
              >
                <ElRow :gutter="10" align="middle">
                  <ElCol :span="8">
                    <ElFormItem
                      :prop="`prescription.${index}.medicineName`"
                      :rules="[{ required: true, message: '请选择中药', trigger: 'change' }]"
                    >
                      <ElSelect
                        v-model="item.medicineName"
                        filterable
                        remote
                        :remote-method="searchChineseMedicines"
                        :loading="chineseMedicinesLoading"
                        placeholder="请输入中药名称"
                        class="full-width"
                        @change="handleMedicineChange(index)"
                      >
                        <ElOption
                          v-for="medicine in getAvailableChineseMedicines(index)"
                          :key="medicine.id"
                          :label="medicine.name"
                          :value="medicine.name"
                          :data-price="medicine.unit_price"
                        />
                      </ElSelect>
                    </ElFormItem>
                  </ElCol>
                  <ElCol :span="4">
                    <div class="price-display">{{
                      formatAmountFromTenThousandths(getUnitPriceTenThousandths(item.medicineName))
                    }}</div>
                  </ElCol>
                  <ElCol :span="4">
                    <ElFormItem
                      :prop="`prescription.${index}.grams`"
                      :rules="[
                        { required: true, message: '请输入克数', trigger: 'blur' },
                        { type: 'number', min: 1, message: '克数不能小于1', trigger: 'blur' }
                      ]"
                    >
                      <ElInput
                        v-model.number="item.grams"
                        type="number"
                        min="1"
                        step="1"
                        placeholder="克数"
                        @input="calculateTotalPrice"
                      />
                    </ElFormItem>
                  </ElCol>
                  <ElCol :span="4">
                    <div class="price-display">{{
                      formatAmountFromTenThousandths(
                        getUnitPriceTenThousandths(item.medicineName) * item.grams
                      )
                    }}</div>
                  </ElCol>
                  <ElCol :span="4">
                    <div class="action-button-wrapper">
                      <ElButton
                        type="danger"
                        size="small"
                        @click="removePrescriptionItem(index)"
                        :disabled="form.prescription.length === 1"
                      >
                        删除
                      </ElButton>
                    </div>
                  </ElCol>
                </ElRow>
              </div>
            </div>

            <div class="prescription-actions">
              <ElButton type="primary" @click="addPrescriptionItem">
                <ElIcon><Plus /></ElIcon>
                添加中药
              </ElButton>
            </div>
          </div>
        </ElFormItem>

        <!-- 付数和总价 -->
        <ElRow :gutter="20">
          <ElCol :span="8">
            <ElFormItem label="付数" prop="doses">
              <ElInput
                v-model.number="form.doses"
                type="number"
                min="1"
                placeholder="请输入付数"
                class="full-width"
                @input="calculateTotalPrice"
              >
                <template #append>付</template>
              </ElInput>
            </ElFormItem>
          </ElCol>
          <ElCol :span="16">
            <div class="total-amount">
              <span class="total-label">单付金额：</span>
              <span class="total-value">{{
                formatAmountFromTenThousandths(singleDoseAmountTenThousandths)
              }}</span>
              <span class="total-label" style="margin-left: 20px">总金额：</span>
              <span class="total-value">{{
                formatAmountFromTenThousandths(totalAmountTenThousandths)
              }}</span>
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
              <ElButton type="warning" @click="printChineseMedicalRecord" plain>
                生成中医病历
              </ElButton>
              <ElButton type="success" @click="printPrescription" plain> 生成中药处方 </ElButton>
              <!-- <ElButton type="warning" @click="printFeeList" plain> 生成费用清单 </ElButton> -->
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
  import { getChineseMedicineList } from '@/api/chineseMedicineApi'
  import { createPrescriptionRecord } from '@/api/unifiedMedicalRecordApi'
  import { GENDER_OPTIONS, TRADE_TYPE_OPTIONS } from '@/types/medical'
  import {
    printPrescription as printPrescriptionTemplate,
    printChineseMedicalRecord as printChineseMedicalRecordTemplate,
    // printFeeList as printFeeListTemplate,
    type PrescriptionData
    // type FeeListData
  } from '@/utils/print'
  import {
    formatAmountFromTenThousandths
    // convertTenThousandthsToYuan
  } from '@/utils/dataprocess/format'
  import { filterByNamePinyin, mergeUniqueBy } from '@/utils/dataprocess'

  // 表单数据
  interface PrescriptionFormItem {
    medicineName: string
    grams: number
  }

  const form = reactive({
    customerName: '',
    gender: '',
    age: null as number | null,
    telephone: '',
    timeOfVisit: new Date(),
    tradeType: '',
    chiefComplaint: '',
    illnessHistory: '',
    pastHistory: '',
    tonguePulseSign: '',
    diagnosis: '',
    treatmentMethod: '',
    prescription: [{ medicineName: '', grams: 10 }] as PrescriptionFormItem[],
    doses: null as number | null,
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
    illnessHistory: [{ required: true, message: '请输入现病史', trigger: 'blur' }],
    pastHistory: [{ required: true, message: '请输入既往史', trigger: 'blur' }],
    tonguePulseSign: [{ required: true, message: '请输入舌象脉象', trigger: 'blur' }],
    diagnosis: [{ required: true, message: '请输入诊断', trigger: 'blur' }],
    treatmentMethod: [{ required: true, message: '请输入治法治则', trigger: 'blur' }],
    doses: [
      { required: true, message: '请输入付数', trigger: 'blur' },
      { type: 'number', min: 1, message: '付数不能小于1', trigger: 'blur' }
    ],
    doctorId: [{ required: true, message: '请选择医师', trigger: 'change' }]
  }

  // 表单引用
  const formRef = ref<FormInstance>()

  // 状态管理
  const loading = ref(false)
  const chineseMedicinesLoading = ref(false)
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
      }
    } catch (error) {
      console.error('加载医师列表失败:', error)
    }
  }

  // 中药数据
  const chineseMedicines = ref<Api.ChineseMedicine.Item[]>([])
  const filteredChineseMedicines = ref<Api.ChineseMedicine.Item[]>([])

  // 加载中药列表
  const loadChineseMedicines = async () => {
    try {
      const res = await getChineseMedicineList({
        current: 1,
        size: 100,
        name: ''
      })
      if (res.code === 200) {
        chineseMedicines.value = res.data.records || []
        filteredChineseMedicines.value = res.data.records || []
        console.log('成功加载中药列表，共', chineseMedicines.value.length, '条记录')

        // 数据验证：检查是否有价格为0的药品（可能提示数据库问题）
        const zeroPrice = chineseMedicines.value.filter((m) => !m.unit_price || m.unit_price === 0)
        if (zeroPrice.length > 0) {
          const zeroPriceNames = zeroPrice.map((m) => m.name)
          console.warn('发现', zeroPrice.length, '条药品单价为0或未定义', zeroPriceNames)
        }
      } else {
        console.error('获取中药列表失败:', res.msg)
        ElMessage.error(`获取中药列表失败: ${res.msg}`)
      }
    } catch (error) {
      console.error('加载中药列表失败:', error)
      ElMessage.error('加载中药列表失败，请检查网络连接')
    }
  }

  // 搜索中药
  const searchChineseMedicines = async (query: string) => {
    if (!query) {
      filteredChineseMedicines.value = chineseMedicines.value
      return
    }

    chineseMedicinesLoading.value = true
    try {
      const res = await getChineseMedicineList({
        current: 1,
        size: 100,
        name: query
      })
      const localMatched = filterByNamePinyin(chineseMedicines.value, (item) => item.name, query)

      if (res.code === 200) {
        const records = res.data.records
        // 合并新搜索结果到主列表（避免重复）
        chineseMedicines.value = mergeUniqueBy(
          chineseMedicines.value,
          records,
          (item) => `${item.id ?? item.name}`
        )

        // 合并 API 与本地拼音匹配结果
        filteredChineseMedicines.value = mergeUniqueBy(
          records,
          localMatched,
          (item) => `${item.id ?? item.name}`
        )

        console.log('搜索中药成功，共获得', records.length, '条记录')
      } else {
        console.error('搜索中药API返回错误:', res.msg)
        ElMessage.warning(`搜索中药失败: ${res.msg}`)
        // 降级处理：本地拼音过滤搜索
        filteredChineseMedicines.value = localMatched
      }
    } catch (error) {
      console.error('搜索中药请求失败:', error)
      ElMessage.error('搜索中药请求失败，请检查网络连接')
      // 本地拼音过滤搜索作为备选
      filteredChineseMedicines.value = filterByNamePinyin(
        chineseMedicines.value,
        (item) => item.name,
        query
      )
    } finally {
      chineseMedicinesLoading.value = false
    }
  }

  // 处理中药变化
  const handleMedicineChange = (changedIndex?: number) => {
    if (typeof changedIndex === 'number') {
      const selectedName = form.prescription[changedIndex]?.medicineName
      if (selectedName) {
        const duplicateIndex = form.prescription.findIndex(
          (item, index) => index !== changedIndex && item.medicineName === selectedName
        )

        if (duplicateIndex !== -1) {
          ElMessage.warning('该中药已添加，请勿重复选择')
          form.prescription[changedIndex].medicineName = ''
        }
      }
    }

    calculateTotalPrice()
  }

  const getAvailableChineseMedicines = (currentIndex: number) => {
    const selectedByOthers = new Set(
      form.prescription
        .map((item, index) => (index === currentIndex ? '' : item.medicineName))
        .filter(Boolean)
    )

    return filteredChineseMedicines.value.filter((item) => !selectedByOthers.has(item.name))
  }

  // 获取单价（关键函数：务必确保能查找到价格）
  const getUnitPriceTenThousandths = (medicineName: string): number => {
    if (!medicineName) {
      console.warn('getUnitPriceTenThousandths: 药品名称为空')
      return 0
    }

    const medicine = chineseMedicines.value.find((item) => item.name === medicineName)
    if (!medicine) {
      console.warn(`getUnitPriceTenThousandths: 未找到药品 "${medicineName}"，可能原因：`, {
        chineseMedicinesCount: chineseMedicines.value.length,
        medicineNameList: chineseMedicines.value.map((m) => m.name).slice(0, 5)
      })
      return 0
    }

    const unitPrice = medicine.unit_price || 0
    if (unitPrice === 0) {
      console.warn(`getUnitPriceTenThousandths: 药品 "${medicineName}" 的单价为0或未定义`, {
        medicine
      })
    }

    // medicine.unit_price 以万分为单位存储
    return unitPrice
  }

  // 计算单付金额（显示为元）
  const singleDoseAmountTenThousandths = computed(() => {
    let totalTenThousandths = 0

    form.prescription.forEach((item) => {
      const unitPrice = getUnitPriceTenThousandths(item.medicineName)
      if (unitPrice && item.grams) {
        totalTenThousandths += unitPrice * item.grams
      }
    })

    return totalTenThousandths
  })

  // 计算总金额
  const totalAmountTenThousandths = computed(() => {
    return singleDoseAmountTenThousandths.value * (form.doses || 1)
  })

  // 计算总价
  const calculateTotalPrice = () => {
    // 触发响应式更新
    nextTick()
  }

  // 时间格式转换工具函数
  const formatDateTime = (dateTime: Date | string): string => {
    if (!dateTime) return ''
    if (typeof dateTime === 'string') return dateTime
    return dateTime.toISOString().slice(0, 19).replace('T', ' ')
  }

  // 添加中药项目
  const addPrescriptionItem = () => {
    const lastItem = form.prescription[form.prescription.length - 1]
    if (!lastItem.medicineName || !lastItem.grams) {
      ElMessage.warning('请先完善当前中药项目信息')
      return
    }
    form.prescription.push({ medicineName: '', grams: 10 })
  }

  // 删除中药项目
  const removePrescriptionItem = (index: number) => {
    if (form.prescription.length > 1) {
      form.prescription.splice(index, 1)
      calculateTotalPrice()
    }
  }

  // 重置表单
  const resetForm = () => {
    formRef.value?.resetFields()
    form.prescription = [{ medicineName: '', grams: 10 }]
    form.doses = 7
    form.timeOfVisit = new Date()
  }

  // 提交登记
  const submitRegistration = async () => {
    if (loading.value || isSubmitting.value) return

    const isValid = (await formRef.value?.validate().catch(() => false)) || false
    if (!isValid) {
      ElMessage.warning('请完善所有必填信息后再提交')
      return
    }

    // 重复提交确认
    if (isSubmitting.value) {
      try {
        await ElMessageBox.confirm('检测到正在提交中，是否确认再次提交？', '确认提交', {
          confirmButtonText: '确认',
          cancelButtonText: '取消',
          type: 'warning'
        })
      } catch {
        return // 用户取消
      }
    }

    try {
      loading.value = true
      isSubmitting.value = true
      await formRef.value?.validate()

      // 获取选中医师的姓名
      const selectedDoctor = doctorList.value.find((doctor) => doctor.id === Number(form.doctorId))
      const doctorName = selectedDoctor?.name || '未知医师'

      // 构造中医中药数据 (统一API)
      const prescriptionData = {
        customer_name: form.customerName,
        gender: form.gender,
        age: form.age!,
        telephone: form.telephone,
        chief_complaint: form.chiefComplaint || undefined,
        illness_history: form.illnessHistory || undefined,
        past_history: form.pastHistory || undefined,
        physical_examination: form.tonguePulseSign || undefined,
        diagnosis: form.diagnosis,
        treatment_method: form.treatmentMethod || undefined,
        prescription_items: form.prescription.map((item) => {
          const unitPrice = getUnitPriceTenThousandths(item.medicineName)
          const totalPrice = Math.round(unitPrice * item.grams)

          // 添加调试日志
          if (unitPrice === 0) {
            console.error(`警告：药品 "${item.medicineName}" 的单价为0`, {
              grams: item.grams,
              calculatedTotalPrice: totalPrice
            })
          }

          return {
            medicine_name: item.medicineName,
            grams: item.grams,
            unit_price: unitPrice, // 单价(万分)
            total_price: totalPrice // 总价(万分)
          }
        }),
        doses: form.doses!,
        // 计算单付金额和总金额（以万分为单位），使用 convertYuanToTenThousandths 避免浮点精度问题
        single_dose_amount: singleDoseAmountTenThousandths.value, // 转换为万分
        total_amount: totalAmountTenThousandths.value, // 转换为万分
        time_of_visit: formatDateTime(form.timeOfVisit),
        trade_type: form.tradeType,
        doctor_id: form.doctorId ? Number(form.doctorId) : undefined,
        doctor_name: doctorName
      }

      console.log('准备发送的处方数据 (统一API):', prescriptionData)

      // 数据验证
      const hasPriceZero = prescriptionData.prescription_items.some((item) => item.unit_price === 0)
      if (hasPriceZero) {
        const confirmation = await ElMessageBox.confirm(
          '检测到部分药品单价为0，可能是查询失败。是否强制提交？',
          '数据验证警告',
          {
            confirmButtonText: '强制提交',
            cancelButtonText: '取消',
            type: 'warning'
          }
        ).catch(() => false)

        if (!confirmation) return
      }

      // 调用后端API保存处方 (统一API)
      const response = await createPrescriptionRecord(prescriptionData)

      if (response.code === 200) {
        ElMessage.success('中医中药登记成功！')
        console.log('保存成功的处方记录:', response.data)

        // 保留表单数据，不自动重置
        // resetForm()
      } else {
        ElMessage.error(`登记失败: ${response.msg}`)
      }
    } catch (error) {
      console.error('提交中医中药登记失败:', error)
      ElMessage.error('提交中医中药登记失败，请重试')
    } finally {
      loading.value = false
      // 2秒后重置提交状态
      setTimeout(() => {
        isSubmitting.value = false
      }, 2000)
    }
  }

  // 生成中药处方（打印功能）
  const printPrescription = async () => {
    try {
      await formRef.value?.validate()

      // 构建中医中药打印数据
      const printData: PrescriptionData = {
        customer_name: form.customerName,
        gender: form.gender,
        age: form.age || 0,
        telephone: form.telephone,
        time_of_visit: formatDateTime(form.timeOfVisit),
        trade_type: form.tradeType,
        chief_complaint: form.chiefComplaint || undefined,
        illness_history: form.illnessHistory || undefined,
        past_history: form.pastHistory || undefined,
        physical_examination: form.tonguePulseSign || undefined,
        diagnosis: form.diagnosis,
        treatment_method: form.treatmentMethod || undefined,
        prescription_items: form.prescription.map((item) => {
          const unitPriceInTenThousandths = getUnitPriceTenThousandths(item.medicineName) // 价格以万分为单位
          return {
            medicine_name: item.medicineName,
            grams: item.grams,
            unit_price: unitPriceInTenThousandths, // 价格以万分为单位
            total_price: unitPriceInTenThousandths * item.grams // 价格以万分为单位
          }
        }),
        doses: form.doses || 1,
        single_dose_amount: singleDoseAmountTenThousandths.value,
        total_amount: totalAmountTenThousandths.value,
        doctor_name: doctorList.value.find((doctor) => doctor.id === parseInt(form.doctorId))?.name
      }

      printPrescriptionTemplate(printData)
      ElMessage.success('中医中药打印成功')

      ElMessageBox.confirm('是否确认提交中医中药登记？', '提交登记', {
        confirmButtonText: '确认',
        cancelButtonText: '取消',
        type: 'warning'
      })
        .then(() => {
          submitRegistration()
        })
        .catch(() => {
          // 用户取消提交
        })
    } catch (error) {
      console.error('打印中药处方失败:', error)
      ElMessage.error('请先完善必填信息')
    }
  }

  // 生成中医病历（打印功能）
  const printChineseMedicalRecord = async () => {
    try {
      await formRef.value?.validate()

      const printData: PrescriptionData = {
        customer_name: form.customerName,
        gender: form.gender,
        age: form.age || 0,
        telephone: form.telephone,
        time_of_visit: formatDateTime(form.timeOfVisit),
        trade_type: form.tradeType,
        chief_complaint: form.chiefComplaint || undefined,
        illness_history: form.illnessHistory || undefined,
        past_history: form.pastHistory || undefined,
        physical_examination: form.tonguePulseSign || undefined,
        diagnosis: form.diagnosis,
        treatment_method: form.treatmentMethod || undefined,
        prescription_items: form.prescription.map((item) => {
          const unitPriceInTenThousandths = getUnitPriceTenThousandths(item.medicineName)
          return {
            medicine_name: item.medicineName,
            grams: item.grams,
            unit_price: unitPriceInTenThousandths,
            total_price: unitPriceInTenThousandths * item.grams
          }
        }),
        doses: form.doses || 1,
        single_dose_amount: singleDoseAmountTenThousandths.value,
        total_amount: totalAmountTenThousandths.value,
        doctor_name: doctorList.value.find((doctor) => doctor.id === parseInt(form.doctorId))?.name
      }

      await printChineseMedicalRecordTemplate(printData)
      ElMessage.success('中医病历打印成功')
    } catch (error) {
      console.error('打印中医病历失败:', error)
      ElMessage.error('请先完善必填信息')
    }
  }

  // 生成费用清单（打印功能）
  // const printFeeList = async () => {
  //   try {
  //     await formRef.value?.validate()

  //     // 构建费用清单打印数据
  //     const feeListData: FeeListData = {
  //       customer_name: form.customerName,
  //       gender: form.gender,
  //       age: form.age || 0,
  //       telephone: form.telephone,
  //       time_of_visit:
  //         typeof form.timeOfVisit === 'string'
  //           ? form.timeOfVisit
  //           : form.timeOfVisit.toISOString().slice(0, 19).replace('T', ' '),
  //       trade_type: form.tradeType,
  //       diagnosis: form.diagnosis,
  //       note: form.note,
  //       treatment_items: form.prescription.map((item) => {
  //         const unitPriceInTenThousandths = getUnitPriceTenThousandths(item.medicineName) // 价格以万分为单位
  //         const singleTotalInYuanTenThousandths = unitPriceInTenThousandths * item.grams // 单付总价(元)
  //         return {
  //           treatment_item: `${item.medicineName} (${item.grams}g)`,
  //           count: form.doses || 1,
  //           unit_price: singleTotalInYuanTenThousandths, // 单付价格(元)
  //           total_price: singleTotalInYuanTenThousandths * (form.doses || 1) // 总价格(元)
  //         }
  //       }),
  //       total_amount: totalAmountTenThousandths.value,
  //       doctor_name: doctorList.value.find((doctor) => doctor.id === parseInt(form.doctorId))?.name
  //     }

  //     printFeeListTemplate(feeListData)
  //     ElMessage.success('费用清单打印成功')
  //   } catch (error) {
  //     console.error('打印费用清单失败:', error)
  //     ElMessage.error('请先完善必填信息')
  //   }
  // }

  // 初始化
  onMounted(() => {
    loadChineseMedicines()
    loadDoctorList()
    // 初始化时设置当前时间
    form.timeOfVisit = new Date()
  })
</script>

<style lang="scss" scoped>
  .chinese-medicine-record-card-list {
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

  .prescription-section {
    width: 100%;

    .prescription-header {
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

    .prescription-items {
      .prescription-item {
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
          color: var(--el-color-success);
          text-align: center;
          background-color: var(--el-color-success-light-9);
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

    .prescription-actions {
      padding: 10px;
      margin-top: 15px;
      text-align: center;
      background-color: var(--el-fill-color-blank);
      border: 2px dashed var(--el-border-color);
      border-radius: 4px;

      &:hover {
        background-color: var(--el-color-success-light-9);
        border-color: var(--el-color-success);
      }
    }
  }

  .total-amount {
    display: flex;
    align-items: center;
    justify-content: flex-end;
    padding: 20px 0;
    margin-top: 20px;
    border-top: 2px solid var(--el-border-color);

    .total-label {
      margin-right: 10px;
      font-size: 16px;
      font-weight: 600;
      color: var(--el-text-color-primary);
    }

    .total-value {
      font-size: 20px;
      font-weight: 700;
      color: var(--el-color-success);
    }
  }

  .form-actions {
    display: flex;
    gap: 5px;
    align-items: center;
    justify-content: flex-end;
    padding-top: 20px;

    .el-button {
      padding: 12px;
      font-size: 16px;
    }
  }

  // 响应式设计
  @media (width <= 768px) {
    .chinese-medicine-record-card-list {
      padding: 12px;

      .form-wrap {
        padding: 16px;

        .el-col {
          margin-bottom: 8px;
        }
      }
    }

    .prescription-section {
      .prescription-header,
      .prescription-items {
        .el-col {
          margin-bottom: 8px;
        }
      }
    }

    .total-amount {
      flex-direction: column;
      align-items: stretch;
      text-align: center;

      .total-label {
        margin-bottom: 5px;
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
