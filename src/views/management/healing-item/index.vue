<template>
  <div class="healing-item-management">
    <!-- 搜索区域 -->
    <ArtSearchBar
      v-model:filter="searchFormState"
      :items="searchItems"
      :is-expand="false"
      :show-expand="false"
      @search="handleSearch"
      @reset="handleReset"
    />

    <!-- 表格区域 -->
    <ElCard class="art-table-card" shadow="never" style="margin-top: 0">
      <template #header>
        <div class="table-header-wrapper">
          <h4>治疗项目管理</h4>
          <div class="table-info">
            <ElTag v-if="hasError" type="danger">{{ hasError.message }}</ElTag>
            <ElTag v-else-if="isLoading" type="warning">加载中...</ElTag>
            <ElTag v-else type="success">{{ tableData.length }} 条数据</ElTag>
          </div>
        </div>
      </template>

      <!-- 表格工具栏 -->
      <ArtTableHeader
        v-model:columns="columnChecks"
        @refresh="handleRefresh"
        layout="refresh,size,fullscreen,columns,settings"
        fullClass="art-table-card"
      >
        <template #left>
          <div class="toolbar-left">
            <ElButton type="primary" @click="handleAdd" v-ripple>
              <ElIcon><Plus /></ElIcon>
              新增治疗项目
            </ElButton>

            <!-- 导出导入功能 -->
            <ArtExcelExport
              :data="tableData as any"
              :columns="exportColumns as any"
              filename="康复治疗项目数据"
              :auto-index="true"
              button-text="导出"
              @export-success="handleExportSuccess"
            />
            <ArtExcelImport
              @import-success="handleImportSuccess"
              @import-error="handleImportError"
            />

            <ElButton
              type="danger"
              plain
              @click="handleBatchDelete"
              :disabled="selectedRows.length === 0"
              v-ripple
            >
              <ElIcon><Delete /></ElIcon>
              批量删除 ({{ selectedRows.length }})
            </ElButton>
          </div>
        </template>
      </ArtTableHeader>

      <ArtTable
        ref="tableRef"
        :loading="isLoading"
        :pagination="paginationState"
        :data="tableData"
        :columns="columns"
        :height="computedTableHeight"
        @selection-change="handleSelectionChange"
        @sort-change="handleSortChange"
        @pagination:size-change="onPageSizeChange"
        @pagination:current-change="onCurrentPageChange"
      >
        <!-- 操作列 -->
        <template #operation="{ row }">
          <div class="operation-buttons">
            <ArtButtonTable type="edit" :row="row" @click="handleEdit(row)" />
            <ArtButtonTable type="delete" :row="row" @click="handleDelete(row)" />
          </div>
        </template>
      </ArtTable>
    </ElCard>

    <!-- 新增/编辑诊疗项目对话框 -->
    <ElDialog
      v-model="editDialogVisible"
      :title="isEdit ? '编辑诊疗项目' : '新增诊疗项目'"
      width="520px"
      :before-close="handleEditDialogClose"
    >
      <ElForm
        ref="editFormRef"
        :model="editForm"
        :rules="editFormRules"
        label-width="100px"
        @submit.prevent
      >
        <ElFormItem label="项目名称" prop="name">
          <ElInput v-model="editForm.name" placeholder="请输入项目名称" clearable />
        </ElFormItem>
        <ElFormItem label="单价（元）" prop="price">
          <ElInputNumber
            v-model="editForm.price"
            :min="0"
            :precision="2"
            :step="1"
            style="width: 100%"
          />
        </ElFormItem>
        <ElFormItem label="项目描述" prop="description">
          <ElInput
            v-model="editForm.description"
            type="textarea"
            :rows="3"
            placeholder="请输入项目描述（选填）"
            clearable
          />
        </ElFormItem>
      </ElForm>

      <template #footer>
        <div class="dialog-footer">
          <ElButton @click="handleEditDialogClose">取消</ElButton>
          <ElButton type="primary" @click="handleEditConfirm" :loading="editFormLoading">
            确定
          </ElButton>
        </div>
      </template>
    </ElDialog>
  </div>
</template>

<script setup lang="ts">
  import { ref, computed, watch } from 'vue'
  import { ElMessage, ElMessageBox } from 'element-plus'
  import { Plus, Delete } from '@element-plus/icons-vue'
  import { useTable } from '@/composables/useTable'
  import * as healingApi from '@/api/healingApi'
  import type { SearchFormItem } from '@/types'
  import type { FormInstance, FormRules } from 'element-plus'
  import {
    convertYuanToTenThousandths,
    convertTenThousandthsToYuan
  } from '@/utils/dataprocess/format'

  defineOptions({ name: 'HealingItemManagement' })

  // 选中的行
  const selectedRows = ref<Api.Healing.Item[]>([])

  // 表格实例引用
  const tableRef = ref()

  // 表格配置
  const tableConfig = ref({
    height: '100%',
    fixedHeight: false
  })

  // 计算实际的表格高度
  const computedTableHeight = computed(() => {
    return tableConfig.value.fixedHeight ? '500px' : ''
  })

  // 编辑/新增相关状态
  const editDialogVisible = ref(false)
  const isEdit = ref(false)
  const editFormLoading = ref(false)
  const editFormRef = ref<FormInstance>()
  const currentEditingItem = ref<Api.Healing.Item | null>(null)

  // 编辑表单数据
  const editForm = ref({
    name: '',
    description: '',
    price: 0
  })

  // 表单验证规则
  const editFormRules: FormRules = {
    name: [
      { required: true, message: '请输入项目名称', trigger: 'blur' },
      { min: 1, max: 50, message: '名称长度在 1 到 50 个字符', trigger: 'blur' }
    ],
    price: [
      { required: true, message: '请输入单价', trigger: 'blur' },
      { type: 'number', min: 0, message: '单价不能小于0', trigger: 'blur' }
    ]
  }

  // 表单搜索初始值
  const defaultFilter = ref({
    name: ''
  })

  // 搜索表单状态
  const searchFormState = ref({ ...defaultFilter.value })

  // 搜索表单配置
  const searchItems: SearchFormItem[] = [
    {
      prop: 'name',
      label: '治疗项目',
      type: 'input',
      config: {
        placeholder: '请输入治疗项目名称',
        clearable: true
      }
    }
  ]

  // 导出列配置
  const exportColumns = computed(() => ({
    name: { title: '治疗项目', width: 30 },
    price: { title: '单价（元）', width: 20 },
    description: { title: '项目描述', width: 40 }
  }))

  // 使用 useTable Hook
  const {
    // 数据相关
    tableData,
    isLoading,
    hasError,

    // 分页相关
    paginationState,
    onPageSizeChange,
    onCurrentPageChange,

    // 搜索相关
    searchState,
    resetSearch,

    // 数据操作
    searchData,
    searchDataDebounced,

    // 刷新策略
    refreshAll,
    refreshAfterCreate,
    refreshAfterRemove,

    // 列配置
    columns,
    columnChecks
  } = useTable<Api.Healing.Item>({
    core: {
      apiFn: async (params: Api.Healing.SearchParams) => {
        try {
          const response = await healingApi.getHealingList(params)
          return response.data
        } catch (error) {
          console.error('API调用失败:', error)
          throw error
        }
      },
      apiParams: {
        current: 1,
        size: 20,
        ...defaultFilter.value
      },
      immediate: true,
      columnsFactory: () => [
        { type: 'selection', width: 50 },
        { type: 'globalIndex', width: 60, label: '序号' },
        { prop: 'name', label: '治疗项目', sortable: true, minWidth: 180 },
        {
          prop: 'price',
          label: '单价（元）',
          sortable: true,
          width: 140,
          formatter: (row) => `¥${convertTenThousandthsToYuan(row.price).toFixed(2)}`
        },
        { prop: 'description', label: '项目描述', minWidth: 200 },
        {
          prop: 'operation',
          label: '操作',
          width: 120,
          useSlot: true,
          fixed: 'right'
        }
      ]
    },
    performance: {
      enableCache: true,
      cacheTime: 5 * 60 * 1000,
      debounceTime: 300,
      maxCacheSize: 100
    },
    hooks: {
      onError: (error) => {
        ElMessage.error(error.message)
      }
    }
  })

  // 表格选择事件
  const handleSelectionChange = (selection: Api.Healing.Item[]) => {
    selectedRows.value = selection
  }

  const handleSortChange = () => {
    // 支持后续服务端排序扩展
  }

  // 搜索与重置
  const handleSearch = () => {
    Object.assign(searchState, {
      name: searchFormState.value.name || '',
      current: 1,
      size: paginationState.size
    })
    searchData()
  }

  const handleReset = () => {
    searchFormState.value = { ...defaultFilter.value }
    resetSearch()
  }

  const handleRefresh = () => {
    refreshAll()
  }

  // CRUD 操作
  const handleAdd = () => {
    isEdit.value = false
    currentEditingItem.value = null
    editForm.value = {
      name: '',
      description: '',
      price: 0
    }
    editDialogVisible.value = true
  }

  const handleEdit = (row: Api.Healing.Item) => {
    isEdit.value = true
    currentEditingItem.value = row
    editForm.value = {
      name: row.name,
      description: row.description || '',
      price: convertTenThousandthsToYuan(row.price)
    }
    editDialogVisible.value = true
  }

  const handleEditDialogClose = () => {
    editDialogVisible.value = false
    editForm.value = {
      name: '',
      description: '',
      price: 0
    }
    currentEditingItem.value = null
    editFormRef.value?.clearValidate()
  }

  const handleEditConfirm = async () => {
    if (!editFormRef.value) return

    try {
      const valid = await editFormRef.value.validate()
      if (!valid) return

      editFormLoading.value = true

      if (isEdit.value) {
        if (!currentEditingItem.value?.id) {
          ElMessage.error('无效的项目ID')
          return
        }

        const updateParams: Api.Healing.UpdateParams = {
          id: currentEditingItem.value.id,
          name: editForm.value.name,
          description: editForm.value.description,
          price: convertYuanToTenThousandths(editForm.value.price)
        }

        const response = await healingApi.updateHealingItem(updateParams)
        ElMessage.success(response.msg || '更新成功')
      } else {
        const createParams: Api.Healing.CreateParams = {
          name: editForm.value.name,
          description: editForm.value.description,
          price: convertYuanToTenThousandths(editForm.value.price)
        }

        const response = await healingApi.createHealingItem(createParams)
        ElMessage.success(response.msg || '新增成功')
      }

      handleEditDialogClose()
      refreshAfterCreate()
    } catch (error: any) {
      ElMessage.error(error.message || (isEdit.value ? '更新失败' : '新增失败'))
      console.error('保存失败:', error)
    } finally {
      editFormLoading.value = false
    }
  }

  const handleDelete = async (row: Api.Healing.Item) => {
    try {
      await ElMessageBox.confirm(`确定要删除治疗项目「${row.name}」吗？`, '提示', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      })

      if (!row.id) {
        ElMessage.error('无效的项目ID')
        return
      }

      const response = await healingApi.deleteHealingItem(row.id)
      ElMessage.success(response.msg || '删除成功')
      refreshAfterRemove()
    } catch (error: any) {
      if (error !== 'cancel') {
        ElMessage.error(error.message || '删除失败')
      }
    }
  }

  const handleBatchDelete = async () => {
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedRows.value.length} 个治疗项目吗？`,
        '警告',
        {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning'
        }
      )

      const ids = selectedRows.value.filter((row) => row.id).map((row) => row.id!)
      if (ids.length === 0) {
        ElMessage.error('没有有效的项目可删除')
        return
      }

      const response = await healingApi.batchDeleteHealingItems(ids)
      ElMessage.success(response.msg || '批量删除成功')
      selectedRows.value = []
      refreshAfterRemove()
    } catch (error: any) {
      if (error !== 'cancel') {
        ElMessage.error(error.message || '批量删除失败')
      }
    }
  }

  // 导入导出
  const handleExportSuccess = (_filename: string, count: number) => {
    ElMessage.success(`导出 ${count} 条数据成功`)
  }

  const handleImportSuccess = async (data: any[]) => {
    if (!Array.isArray(data) || data.length === 0) {
      ElMessage.warning('导入文件无有效数据')
      return
    }

    try {
      let count = 0
      for (const row of data) {
        const name = row['治疗项目'] || row.name
        const priceVal = parseFloat(row['单价（元）'] || row.price || 0)
        if (name) {
          await healingApi.createHealingItem({
            name: String(name).trim(),
            description: String(row['项目描述'] || row.description || ''),
            price: convertYuanToTenThousandths(isNaN(priceVal) ? 0 : priceVal)
          })
          count++
        }
      }
      ElMessage.success(`成功导入 ${count} 条治疗项目`)
      refreshAfterCreate()
    } catch (error: any) {
      ElMessage.error(error.message || '导入数据保存失败')
    }
  }

  const handleImportError = (error: Error) => {
    ElMessage.error(`导入失败：${error.message}`)
  }

  // 监听搜索表单变化
  watch(
    () => searchFormState.value.name,
    (newName) => {
      if (newName !== undefined) {
        Object.assign(searchState, {
          name: newName || '',
          current: 1,
          size: paginationState.size
        })
        searchDataDebounced()
      }
    }
  )
</script>

<style lang="scss" scoped>
  .healing-item-management {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 20px;

    .art-table-card {
      flex: 1;

      .table-header-wrapper {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        align-items: center;
        justify-content: space-between;

        h4 {
          margin: 0;
          font-size: 16px;
          font-weight: 600;
        }

        .table-info {
          display: flex;
          gap: 8px;
        }
      }

      .toolbar-left {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
        align-items: center;
      }

      .operation-buttons {
        display: flex;
        gap: 8px;
      }
    }
  }
</style>
