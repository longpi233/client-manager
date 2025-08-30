<template>
  <div class="institution-card-list">
    <div class="form-wrap">
      <ElForm :model="form" size="large" :rules="formRules" ref="formRef">
        <ElFormItem label="机构名称" prop="institutionName">
          <ElInput
            placeholder="请输入机构名称"
            v-model="form.institutionName"
            :disabled="!modifyForm"
            class="input-style"
          />
        </ElFormItem>
        <ElFormItem label="科室设置" prop="departmentSettings">
          <ElInput
            placeholder="请输入科室设置"
            v-model="form.departmentSettings"
            :disabled="!modifyForm"
            class="input-style"
          />
        </ElFormItem>
        <ElFormItem label="医师管理" prop="doctorManagement">
          <div class="input-style">
            <div v-for="(doctor, idx) in form.doctorManagement" :key="idx" class="doctor-item">
              <ElInput
                v-model="doctor.name"
                :disabled="!modifyForm"
                placeholder="请输入医师名称"
                class="doctor-input"
              />
              <ElButton
                v-if="modifyForm"
                type="danger"
                @click="removeDoctor(idx)"
                :disabled="form.doctorManagement.length === 1"
                class="delete-button"
              >
                删除
              </ElButton>
            </div>
          </div>
        </ElFormItem>
        <ElFormItem v-if="!modifyForm">
          <ElButton type="primary" @click="modifyForm = true">修改</ElButton>
        </ElFormItem>
        <ElFormItem v-if="modifyForm">
          <ElButton type="primary" @click="submitForm" :loading="loading">保存</ElButton>
          <ElButton @click="resetForm">重置</ElButton>
          <ElButton type="warning" @click="addDoctor">添加医师</ElButton>
        </ElFormItem>
      </ElForm>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ElFormItem, ElInput, ElMessage, FormInstance } from 'element-plus'
  import { ref, onMounted } from 'vue'
  import { getInstitutionInfo, updateInstitutionInfo } from '@/api/institutionApi'

  // 前端表单数据接口（使用驼峰命名）
  interface InstitutionForm {
    institutionName: string
    departmentSettings: string
    doctorManagement: { name: string }[]
  }

  // 表单引用
  const formRef = ref<FormInstance>()

  // 状态管理
  const loading = ref(false)
  const modifyForm = ref(false)

  // 表单数据
  const form = ref<InstitutionForm>({
    institutionName: '',
    departmentSettings: '',
    doctorManagement: [{ name: '' }]
  })

  // 保存的表单数据（用于重置）
  const savedForm = ref<InstitutionForm>({
    institutionName: '',
    departmentSettings: '',
    doctorManagement: [{ name: '' }]
  })

  // 表单验证规则
  const formRules = {
    institutionName: [
      { required: true, message: '请输入机构名称', trigger: 'blur' },
      { max: 50, message: '机构名称长度不能超过50个字符', trigger: 'blur' }
    ],
    departmentSettings: [
      { required: true, message: '请输入科室设置', trigger: 'blur' },
      { max: 100, message: '科室设置长度不能超过100个字符', trigger: 'blur' }
    ],
    doctorManagement: [
      { required: true, message: '请输入医师信息', trigger: 'blur' },
      {
        validator: (rule: any, value: { name: string }[], callback: any) => {
          if (!value || value.length === 0) {
            callback(new Error('请至少添加一个医师'))
            return
          }
          for (const doctor of value) {
            if (!doctor.name || doctor.name.trim() === '') {
              callback(new Error('医师名称不能为空'))
              return
            }
            if (doctor.name.length > 20) {
              callback(new Error('医师名称长度不能超过20个字符'))
              return
            }
          }
          callback()
        },
        trigger: 'blur'
      }
    ]
  }

  // 获取机构信息
  const fetchInstitutionInfo = async () => {
    try {
      loading.value = true
      const res = await getInstitutionInfo()
      if (res.code === 200) {
        // 转换后端数据格式到前端格式
        form.value = {
          institutionName: res.data.institution_name || '',
          departmentSettings: res.data.department_settings || '',
          doctorManagement: res.data.doctor_management || [{ name: '' }]
        }
        savedForm.value = JSON.parse(JSON.stringify(form.value))
      } else {
        ElMessage.error(res.msg || '获取机构信息失败')
      }
    } catch (error) {
      console.error('获取机构信息失败:', error)
      ElMessage.error('获取机构信息失败')
    } finally {
      loading.value = false
    }
  }

  // 重置表单
  const resetForm = () => {
    form.value = JSON.parse(JSON.stringify(savedForm.value))
    modifyForm.value = false
  }

  // 提交表单
  const submitForm = async () => {
    if (!formRef.value) return

    const isValid = await formRef.value.validate().catch(() => false)
    if (!isValid) {
      ElMessage.warning('请检查表单填写是否正确')
      return
    }

    try {
      loading.value = true
      const res = await updateInstitutionInfo({
        institution_name: form.value.institutionName,
        department_settings: form.value.departmentSettings,
        doctor_management: form.value.doctorManagement.filter((doctor) => doctor.name.trim() !== '')
      })

      if (res.code === 200) {
        ElMessage.success('机构信息保存成功')
        modifyForm.value = false
        savedForm.value = JSON.parse(JSON.stringify(form.value))
      } else {
        ElMessage.error(res.msg || '保存失败')
      }
    } catch (error) {
      console.error('保存机构信息失败:', error)
      ElMessage.error('保存失败')
    } finally {
      loading.value = false
    }
  }

  // 添加医师
  const addDoctor = () => {
    if (form.value.doctorManagement.length >= 10) {
      ElMessage.warning('最多只能添加10个医师')
      return
    }

    // 检查最后一个医师是否已填写
    const lastDoctor = form.value.doctorManagement[form.value.doctorManagement.length - 1]
    if (lastDoctor && lastDoctor.name.trim() !== '') {
      form.value.doctorManagement.push({ name: '' })
    } else {
      ElMessage.warning('请先完善当前医师信息')
    }
  }

  // 删除医师
  const removeDoctor = (index: number) => {
    if (form.value.doctorManagement.length === 1) {
      ElMessage.warning('至少需要保留一个医师')
      return
    }
    form.value.doctorManagement.splice(index, 1)
  }

  // 组件挂载时获取数据
  onMounted(() => {
    fetchInstitutionInfo()
  })
</script>

<style lang="scss" scoped>
  .institution-card-list {
    max-width: 1000px;
    margin: 20px auto;
    border: 1px solid var(--art-border-color);
    border-radius: calc(var(--custom-radius) / 2 + 2px) !important;

    .form-wrap {
      padding: 20px;
      margin-top: 20px;
      // background-color: var(--art-main-bg-color);

      .input-style {
        width: 100%;
        max-width: 1000px;
      }

      .doctor-item {
        display: flex;
        align-items: center;
        margin-bottom: 20px;
      }

      .doctor-input {
        flex: 1;
      }

      .delete-button {
        margin-left: 8px;
      }

      h2 {
        margin-bottom: 20px;
        font-size: 20px;
        font-weight: 500;
      }
    }
  }
</style>
