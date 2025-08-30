<!-- 中药管理页面 -->
<template>
  <div class="chinese-medicine-table-demo">
    <!-- 搜索区域 -->
    <ArtSearchBar
      v-model:filter="searchFormState"
      :items="searchItems"
      :is-expand="false"
      :show-expand="true"
      @search="handleSearch"
      @reset="handleReset"
    />

    <!-- 表格区域 -->
    <ElCard class="art-table-card" shadow="never" style="margin-top: 0">
      <template #header>
        <div class="table-header-wrapper">
          <h4>中药数据表格</h4>
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
              新增中药
            </ElButton>

            <!-- 导出导入功能 -->
            <ArtExcelExport
              :data="tableData as any"
              :columns="exportColumns as any"
              filename="中药数据"
              :auto-index="true"
              button-text="导出"
              @export-success="handleExportSuccess"
            />
            <ArtExcelImport
              @import-success="handleImportSuccess"
              @import-error="handleImportError"
            />

            <ElButton @click="handleClearData" plain v-ripple> 清空数据 </ElButton>

            <ElButton @click="handleBatchDelete" :disabled="selectedRows.length === 0" v-ripple>
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
        @row-click="handleRowClick"
        @header-click="handleHeaderClick"
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

    <!-- 编辑中药对话框 -->
    <ElDialog
      v-model="editDialogVisible"
      title="编辑中药"
      width="500px"
      :before-close="handleEditDialogClose"
    >
      <ElForm
        ref="editFormRef"
        :model="editForm"
        :rules="editFormRules"
        label-width="100px"
        @submit.prevent
      >
        <ElFormItem label="中药名称" prop="name">
          <ElInput v-model="editForm.name" placeholder="请输入中药名称" clearable />
        </ElFormItem>
        <ElFormItem label="中药描述" prop="description">
          <ElInput
            v-model="editForm.description"
            type="textarea"
            :rows="3"
            placeholder="请输入中药描述"
            clearable
          />
        </ElFormItem>
        <ElFormItem label="单价（元/克）" prop="price">
          <ElInputNumber
            v-model="editForm.price"
            :min="0"
            :precision="2"
            :step="1"
            style="width: 100%"
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
  import * as chineseMedicineApi from '@/api/chineseMedicineApi'
  import type { SearchFormItem } from '@/types'
  import type { FormInstance, FormRules } from 'element-plus'
  import { convertYuanToCents, convertCentsToYuan } from '@/utils/dataprocess/format'

  defineOptions({ name: 'ChineseMedicineDemo' })

  // 选中的行
  const selectedRows = ref<Api.ChineseMedicine.Item[]>([])

  // 表格实例引用
  const tableRef = ref()

  // 缓存调试状态
  const cacheDebugLogs = ref<string[]>([])

  // 编辑相关状态
  const editDialogVisible = ref(false)
  const editFormLoading = ref(false)
  const editFormRef = ref<FormInstance>()
  const currentEditingItem = ref<Api.ChineseMedicine.Item | null>(null)

  // 编辑表单数据
  const editForm = ref({
    name: '',
    description: '',
    price: 0
  })

  // 表单验证规则
  const editFormRules: FormRules = {
    name: [
      { required: true, message: '请输入中药名称', trigger: 'blur' },
      { min: 1, max: 50, message: '名称长度在 1 到 50 个字符', trigger: 'blur' }
    ],
    price: [
      { required: true, message: '请输入单价', trigger: 'blur' },
      { type: 'number', min: 0, message: '单价不能小于0', trigger: 'blur' }
    ]
  }

  // 表格配置演示
  const tableConfig = ref({
    height: '100%',
    fixedHeight: false
  })

  // 计算实际的表格高度
  const computedTableHeight = computed(() => {
    return tableConfig.value.fixedHeight ? '500px' : ''
  })

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
      label: '中药名称',
      type: 'input',
      config: {
        placeholder: '请输入中药名称',
        clearable: true
      }
    }
  ]

  // 导出列配置
  const exportColumns = computed(() => ({
    name: { title: '中药名称', width: 30 },
    price: { title: '单价（元）', width: 20 }
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
  } = useTable<Api.ChineseMedicine.Item>({
    // 核心配置
    core: {
      apiFn: async (params: Api.ChineseMedicine.SearchParams) => {
        console.log('🚀 API 请求参数:', params)
        addCacheLog(`🚀 API 请求: current=${params.current}, size=${params.size}`)

        try {
          const response = await chineseMedicineApi.getChineseMedicineList(params)
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
        { prop: 'name', label: '中药名称', sortable: true, minWidth: 200 },
        {
          prop: 'price',
          label: '单价（元）',
          sortable: true,
          width: 120,
          formatter: (row) => `¥${convertCentsToYuan(row.price).toFixed(2)}` // 将分转换为元显示
        },
        {
          prop: 'operation',
          label: '操作',
          width: 120,
          useSlot: true,
          fixed: 'right'
        }
      ]
    },

    // 数据处理
    transform: {
      dataTransformer: (records: any) => {
        if (!Array.isArray(records)) return []
        return records.map((item: any) => ({
          ...item
        }))
      }
    },

    // 性能优化
    performance: {
      enableCache: true,
      cacheTime: 5 * 60 * 1000, // 5分钟
      debounceTime: 300,
      maxCacheSize: 100
    },

    // 生命周期钩子
    hooks: {
      onSuccess: (data) => {
        console.log('✅ 数据加载成功:', data.length, '条')
        addCacheLog(`✅ 网络请求成功: ${data.length} 条数据`)
      },
      onError: (error) => {
        console.error('❌ 数据加载失败:', error)
        addCacheLog(`❌ 请求失败: ${error.message}`)
        ElMessage.error(error.message)
      },
      onCacheHit: (data) => {
        console.log('🎯 缓存命中:', data.length, '条')
        addCacheLog(`🎯 缓存命中: ${data.length} 条数据`)
        ElMessage.info('数据来自缓存')
      },
      resetFormCallback: () => {
        console.log('🔄 表单已重置')
        addCacheLog('🔄 表单已重置')
      }
    },

    // 调试配置
    debug: {
      enableLog: true,
      logLevel: 'info'
    }
  })

  // 事件处理函数
  const handleSelectionChange = (selection: Api.ChineseMedicine.Item[]) => {
    selectedRows.value = selection
    console.log('选择变更:', selection)
  }

  const handleRowClick = (row: Api.ChineseMedicine.Item) => {
    console.log('行点击:', row)
  }

  const handleHeaderClick = (column: any) => {
    console.log('表头点击:', column)
  }

  const handleSortChange = (sortInfo: any) => {
    console.log('排序事件:', sortInfo)
  }

  // 搜索与重置
  const handleSearch = () => {
    console.log('搜索参数:', searchFormState.value)

    Object.assign(searchState, {
      name: searchFormState.value.name || '',
      current: 1,
      size: paginationState.size
    })

    searchData()
  }

  const handleReset = () => {
    addCacheLog('🔄 重置搜索')
    searchFormState.value = { ...defaultFilter.value }
    resetSearch()
  }

  const handleRefresh = () => {
    addCacheLog('🔄 手动刷新')
    refreshAll()
  }

  // CRUD 操作
  const handleAdd = async () => {
    try {
      // 检查是否已有"新增中药"项目
      const existingNewItem = tableData.value.find((item) => item.name === '新增中药')
      if (existingNewItem) {
        ElMessage.warning('已存在"新增中药"项目，请先编辑或删除现有项目')
        return
      }

      const newItem: Api.ChineseMedicine.CreateParams = {
        name: '新增中药',
        description: '',
        price: convertYuanToCents(10) // 默认10元，转换为分
      }

      const response = await chineseMedicineApi.createChineseMedicineItem(newItem)
      ElMessage.success(response.msg || '新增中药成功')

      setTimeout(() => {
        refreshAfterCreate()
      }, 1000)
    } catch (error: any) {
      ElMessage.error(error.message || '新增中药失败')
      console.error('新增失败:', error)
    }
  }

  const handleEdit = (row: Api.ChineseMedicine.Item) => {
    currentEditingItem.value = row
    editForm.value = {
      name: row.name,
      description: row.description || '',
      price: convertCentsToYuan(row.price) // 将分转换为元显示
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

      if (!currentEditingItem.value?.id) {
        ElMessage.error('无效的中药ID')
        return
      }

      editFormLoading.value = true

      const updateParams: Api.ChineseMedicine.UpdateParams = {
        id: currentEditingItem.value.id,
        name: editForm.value.name,
        description: editForm.value.description,
        price: convertYuanToCents(editForm.value.price) // 将元转换为分
      }

      const response = await chineseMedicineApi.updateChineseMedicineItem(updateParams)
      ElMessage.success(response.msg || '更新成功')

      handleEditDialogClose()

      setTimeout(() => {
        refreshAfterCreate() // 刷新表格数据
      }, 1000)
    } catch (error: any) {
      ElMessage.error(error.message || '更新失败')
      console.error('更新失败:', error)
    } finally {
      editFormLoading.value = false
    }
  }

  const handleDelete = async (row: Api.ChineseMedicine.Item) => {
    try {
      await ElMessageBox.confirm(`确定要删除中药 ${row.name} 吗？`, '警告', {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        type: 'warning'
      })

      if (!row.id) {
        ElMessage.error('无效的中药ID')
        return
      }

      const response = await chineseMedicineApi.deleteChineseMedicineItem(row.id)
      ElMessage.success(response.msg || '删除成功')

      setTimeout(() => {
        refreshAfterRemove()
      }, 1000)
    } catch (error: any) {
      if (error === 'cancel') {
        ElMessage.info('已取消删除')
      } else {
        ElMessage.error('删除失败')
        console.error('删除失败:', error)
      }
    }
  }

  const handleBatchDelete = async () => {
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedRows.value.length} 个中药吗？`,
        '警告',
        {
          confirmButtonText: '确定',
          cancelButtonText: '取消',
          type: 'warning'
        }
      )

      const ids = selectedRows.value.filter((row) => row.id).map((row) => row.id!)

      if (ids.length === 0) {
        ElMessage.error('没有有效的中药可删除')
        return
      }

      const response = await chineseMedicineApi.batchDeleteChineseMedicineItems(ids)
      ElMessage.success(response.msg || `批量删除 ${ids.length} 个中药成功`)
      selectedRows.value = []

      setTimeout(() => {
        refreshAfterRemove()
      }, 1000)
    } catch (error: any) {
      if (error === 'cancel') {
        ElMessage.info('已取消删除')
      } else {
        ElMessage.error('批量删除失败')
        console.error('批量删除失败:', error)
      }
    }
  }

  // 导入导出
  const handleExportSuccess = (filename: string, count: number) => {
    ElMessage.success(`导出 ${count} 条数据成功`)
  }

  const handleImportSuccess = (data: any[]) => {
    ElMessage.success(`导入 ${data.length} 条数据成功`)
    refreshAfterCreate()
  }

  const handleImportError = (error: Error) => {
    ElMessage.error(`导入失败：${error.message}`)
  }

  const handleClearData = () => {
    ElMessage.info('清空数据功能暂未实现')
  }

  // 添加缓存调试日志
  const addCacheLog = (message: string) => {
    const timestamp = new Date().toLocaleTimeString()
    cacheDebugLogs.value.unshift(`[${timestamp}] ${message}`)
    if (cacheDebugLogs.value.length > 20) {
      cacheDebugLogs.value = cacheDebugLogs.value.slice(0, 20)
    }
  }

  // 监听搜索表单变化，实现实时搜索
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
    },
    { deep: true }
  )
</script>

<style lang="scss" scoped>
  .chinese-medicine-table-demo {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 20px;

    .art-table-card {
      flex: 1;

      .table-header-wrapper {
        display: flex;
        align-items: center;
        justify-content: space-between;

        h4 {
          margin: 0;
        }

        .table-info {
          display: flex;
          gap: 8px;
        }
      }

      .toolbar-left {
        display: flex;
        gap: 8px;
        align-items: center;
      }

      .operation-buttons {
        display: flex;
      }
    }
  }

  // 响应式设计
  @media (width <= 768px) {
    .chinese-medicine-table-demo {
      .art-table-card .toolbar-left {
        flex-direction: column;
        gap: 8px;
        align-items: flex-start;
      }
    }
  }
</style>
