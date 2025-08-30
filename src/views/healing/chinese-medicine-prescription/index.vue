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
                        @change="handleMedicineChange"
                      >
                        <ElOption
                          v-for="medicine in filteredChineseMedicines"
                          :key="medicine.id"
                          :label="medicine.name"
                          :value="medicine.name"
                          :data-price="medicine.price"
                        />
                      </ElSelect>
                    </ElFormItem>
                  </ElCol>
                  <ElCol :span="4">
                    <div class="price-display">{{ getUnitPrice(index) }}</div>
                  </ElCol>
                  <ElCol :span="4">
                    <ElFormItem
                      :prop="`prescription.${index}.grams`"
                      :rules="[
                        { required: true, message: '请输入克数', trigger: 'blur' },
                        { type: 'number', min: 0.1, message: '克数不能小于0.1', trigger: 'blur' }
                      ]"
                    >
                      <ElInput
                        v-model.number="item.grams"
                        type="number"
                        min="0.1"
                        step="0.1"
                        placeholder="克数"
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
              <span class="total-value">¥{{ singleDoseAmount }}</span>
              <span class="total-label" style="margin-left: 20px">总金额：</span>
              <span class="total-value">¥{{ totalAmount }}</span>
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
              <ElButton type="success" @click="generatePrescription" :loading="loading">
                生成中药处方
              </ElButton>
              <ElButton type="warning" @click="generateFeeList" :loading="loading">
                生成费用清单
              </ElButton>
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
    ElOption,
    ElSelect,
    ElIcon,
    type FormRules,
    type FormInstance
  } from 'element-plus'
  import { Plus } from '@element-plus/icons-vue'
  import { getInstitutionInfo } from '@/api/institutionApi'
  import { getChineseMedicineList } from '@/api/chineseMedicineApi'
  import { createChineseMedicinePrescription } from '@/api/chineseMedicinePrescriptionApi'

  // 枚举定义
  enum Gender {
    Male = 'Male',
    Female = 'Female'
  }

  enum TradeType {
    Cash = 'Cash',
    ProvincialInsurance = 'ProvincialInsurance',
    WuHanInsurance = 'WuHanInsurance'
  }

  // 选项配置
  const GENDER_OPTIONS = [
    { label: '男', value: Gender.Male },
    { label: '女', value: Gender.Female }
  ]

  const TRADE_TYPE_OPTIONS = [
    { label: '现金结算', value: TradeType.Cash },
    { label: '省直医保', value: TradeType.ProvincialInsurance },
    { label: '武汉医保', value: TradeType.WuHanInsurance }
  ]

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
    diagnosis: '',
    note: '',
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
    diagnosis: [{ required: true, message: '请输入诊断', trigger: 'blur' }],
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

  // 中药数据
  const chineseMedicines = ref<Api.ChineseMedicine.Item[]>([])
  const filteredChineseMedicines = ref<Api.ChineseMedicine.Item[]>([])

  // 加载中药列表
  const loadChineseMedicines = async () => {
    try {
      const res = await getChineseMedicineList({
        current: 1,
        size: 100,
        name: '',
        status: '1' // 只获取启用的中药
      })
      if (res.code === 200) {
        chineseMedicines.value = res.data.records
        filteredChineseMedicines.value = res.data.records
      } else {
        console.error('获取中药列表失败:', res.msg)
        // 使用默认数据作为备选
        const mockChineseMedicines: Api.ChineseMedicine.Item[] = [
          {
            id: 1,
            name: '人参',
            price: 0.5,
            status: '1' as Api.ChineseMedicine.Status
          },
          {
            id: 2,
            name: '当归',
            price: 0.3,
            status: '1' as Api.ChineseMedicine.Status
          },
          {
            id: 3,
            name: '黄芪',
            price: 0.2,
            status: '1' as Api.ChineseMedicine.Status
          },
          {
            id: 4,
            name: '川芎',
            price: 0.25,
            status: '1' as Api.ChineseMedicine.Status
          },
          {
            id: 5,
            name: '白芍',
            price: 0.4,
            status: '1' as Api.ChineseMedicine.Status
          }
        ]
        chineseMedicines.value = mockChineseMedicines
        filteredChineseMedicines.value = mockChineseMedicines
      }
    } catch (error) {
      console.error('加载中药列表失败:', error)
      // 使用默认数据
      const mockChineseMedicines: Api.ChineseMedicine.Item[] = [
        {
          id: 1,
          name: '人参',
          price: 0.5,
          status: '1' as Api.ChineseMedicine.Status
        },
        {
          id: 2,
          name: '当归',
          price: 0.3,
          status: '1' as Api.ChineseMedicine.Status
        },
        {
          id: 3,
          name: '黄芪',
          price: 0.2,
          status: '1' as Api.ChineseMedicine.Status
        },
        {
          id: 4,
          name: '川芎',
          price: 0.25,
          status: '1' as Api.ChineseMedicine.Status
        },
        {
          id: 5,
          name: '白芍',
          price: 0.4,
          status: '1' as Api.ChineseMedicine.Status
        }
      ]
      chineseMedicines.value = mockChineseMedicines
      filteredChineseMedicines.value = mockChineseMedicines
      ElMessage.error('加载中药列表失败')
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
        name: query,
        status: '1'
      })
      if (res.code === 200) {
        filteredChineseMedicines.value = res.data.records
      }
    } catch (error) {
      console.error('搜索中药失败:', error)
      // 本地过滤搜索作为备选
      const filtered = chineseMedicines.value.filter((item) =>
        item.name.toLowerCase().includes(query.toLowerCase())
      )
      filteredChineseMedicines.value = filtered
    } finally {
      chineseMedicinesLoading.value = false
    }
  }

  // 处理中药变化
  const handleMedicineChange = () => {
    calculateTotalPrice()
  }

  // 获取单价
  const getUnitPrice = (index: number): string => {
    const medicineName = form.prescription[index]?.medicineName
    if (!medicineName) return '¥0.00'

    const medicine = chineseMedicines.value.find((item) => item.name === medicineName)
    return medicine ? `¥${medicine.price}` : '¥0.00'
  }

  // 获取项目总价
  const getItemTotalPrice = (index: number): string => {
    const medicineName = form.prescription[index]?.medicineName
    const grams = form.prescription[index]?.grams || 0

    if (!medicineName || !grams) return '¥0.00'

    const medicine = chineseMedicines.value.find((item) => item.name === medicineName)
    const unitPrice = medicine ? medicine.price : 0

    return `¥${(unitPrice * grams).toFixed(2)}`
  }

  // 计算单付金额
  const singleDoseAmount = computed(() => {
    return form.prescription
      .reduce((total, item) => {
        const medicine = chineseMedicines.value.find((med) => med.name === item.medicineName)
        const unitPrice = medicine ? medicine.price : 0
        return total + unitPrice * (item.grams || 0)
      }, 0)
      .toFixed(2)
  })

  // 计算总金额
  const totalAmount = computed(() => {
    const singleAmount = parseFloat(singleDoseAmount.value)
    const doses = form.doses || 1
    return (singleAmount * doses).toFixed(2)
  })

  // 计算总价
  const calculateTotalPrice = () => {
    // 触发响应式更新
    nextTick()
  }

  // 添加中药项目
  const addPrescriptionItem = () => {
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

  // 生成中药处方
  const generatePrescription = async () => {
    if (loading.value) return

    try {
      loading.value = true
      await formRef.value?.validate()

      // 获取选中医师的姓名
      const selectedDoctor = doctorList.value.find((doctor) => doctor.id === Number(form.doctorId))
      const doctorName = selectedDoctor?.name || '未知医师'

      // 构造中药处方数据
      const prescriptionData = {
        customer_id: 1, // 暂时使用固定值，实际应该根据患者信息获取
        customer_name: form.customerName,
        gender: form.gender,
        age: form.age!,
        phone: form.telephone,
        diagnosis: form.diagnosis,
        prescription: form.prescription.map((item) => {
          const medicine = chineseMedicines.value.find((med) => med.name === item.medicineName)
          const unitPrice = medicine ? medicine.price : 0
          return {
            medicine_name: item.medicineName,
            unit_price: unitPrice,
            grams: item.grams,
            total_price: unitPrice * item.grams
          }
        }),
        doses: form.doses!,
        single_dose_amount: parseFloat(singleDoseAmount.value),
        total_amount: parseFloat(totalAmount.value),
        doctor_id: form.doctorId,
        doctor_name: doctorName,
        time_of_visit:
          form.timeOfVisit instanceof Date
            ? form.timeOfVisit.toISOString().slice(0, 19).replace('T', ' ')
            : form.timeOfVisit,
        note: form.note || undefined
      }

      console.log('准备发送的处方数据:', prescriptionData)

      // 调用后端API保存处方
      const response = await createChineseMedicinePrescription(prescriptionData)

      if (response.code === 200) {
        ElMessage.success('中药处方生成成功！')
        console.log('保存成功的处方记录:', response.data)

        // 可选：重置表单或跳转到其他页面
        // resetForm()
      } else {
        ElMessage.error(`保存失败: ${response.msg}`)
      }
    } catch (error) {
      console.error('生成中药处方失败:', error)
      ElMessage.error('生成中药处方失败，请重试')
    } finally {
      loading.value = false
    }
  }

  // 生成费用清单
  const generateFeeList = async () => {
    if (loading.value) return

    try {
      loading.value = true
      await formRef.value?.validate()

      // 模拟调用后端API
      console.log('费用清单数据:', {
        totalAmount: totalAmount.value,
        singleDoseAmount: singleDoseAmount.value,
        doses: form.doses,
        items: form.prescription
      })

      ElMessage.success('费用清单生成成功')
    } catch (error) {
      console.error('生成费用清单失败:', error)
      ElMessage.error('生成费用清单失败')
    } finally {
      loading.value = false
    }
  }

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
