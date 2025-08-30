<!-- 高级表格能力展示 -->
<!-- 实际开发中根据需求选择使用哪些功能，可参考功能示例下面的最小化示例进行开发 -->
<template>
  <div class="advanced-table-demo">
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
          <h4>康复项目数据表格</h4>
          <div class="table-info">
            <ElTag v-if="hasError" type="danger">{{ hasError.message }}</ElTag>
            <ElTag v-else-if="isLoading" type="warning">加载中...</ElTag>
            <ElTag v-else type="success">{{ tableData.length }} 条数据</ElTag>
          </div>
        </div>
      </template>

      <!-- 表格工具栏 -->
      <!-- fullClass 属性用于设置全屏区域，如果需要设置全屏区域，请使用此属性 -->
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
              新增康复项目
            </ElButton>

            <!-- 导出导入功能 -->
            <ArtExcelExport
              :data="tableData as any"
              :columns="exportColumns as any"
              filename="康复项目数据"
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
        <!-- 用户信息列 -->
        <template #avatar="{ row }">
          <div class="user-info">
            <ElAvatar :src="row.avatar" :size="40" />
            <div class="user-details">
              <p class="user-name">{{ row.userName }}</p>
              <p class="user-email">{{ row.userEmail }}</p>
            </div>
          </div>
        </template>

        <!-- 自定义用户信息表头 -->
        <template #avatar-header="{ column }">
          <div style="display: flex; gap: 5px; align-items: center">
            <span>{{ column.label }}</span>
            <ElTooltip content="包含头像、姓名和邮箱" placement="top">
              <ElIcon class="help-icon"><QuestionFilled /></ElIcon>
            </ElTooltip>
          </div>
        </template>

        <!-- 操作列 -->
        <template #operation="{ row }">
          <div class="operation-buttons">
            <ArtButtonTable type="edit" :row="row" @click="handleEdit(row)" />
            <ArtButtonTable type="delete" :row="row" @click="handleDelete(row)" />
          </div>
        </template>

        <!-- 自定义手机号表头 -->
        <template #userPhone-header="{ column }">
          <ElPopover placement="bottom" :width="200" trigger="hover">
            <template #reference>
              <div class="custom-header">
                <span>{{ column.label }}</span>
                <ElIcon><Search /></ElIcon>
              </div>
            </template>
            <ElInput
              v-model="phoneSearch"
              placeholder="搜索手机号"
              size="small"
              @input="handlePhoneSearch"
            >
              <template #prefix>
                <ElIcon><Search /></ElIcon>
              </template>
            </ElInput>
          </ElPopover>
        </template>
      </ArtTable>
    </ElCard>

    <!-- 编辑诊疗项目对话框 -->
    <ElDialog
      v-model="editDialogVisible"
      title="编辑诊疗项目"
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
        <ElFormItem label="项目名称" prop="name">
          <ElInput v-model="editForm.name" placeholder="请输入项目名称" clearable />
        </ElFormItem>
        <ElFormItem label="项目描述" prop="description">
          <ElInput
            v-model="editForm.description"
            type="textarea"
            :rows="3"
            placeholder="请输入项目描述"
            clearable
          />
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
  import { Plus, Delete, Search, QuestionFilled } from '@element-plus/icons-vue'
  import { useTable } from '@/composables/useTable'
  import * as healingApi from '@/api/healingApi'
  import { ACCOUNT_TABLE_DATA } from '@/mock/temp/formData'
  import type { SearchFormItem } from '@/types'
  import type { FormInstance, FormRules } from 'element-plus'
  import { convertYuanToCents, convertCentsToYuan } from '@/utils/dataprocess/format'

  defineOptions({ name: 'AdvancedTableDemo' })

  // 选中的行
  const selectedRows = ref<Api.Healing.Item[]>([])

  // 表格实例引用
  const tableRef = ref()

  // 缓存调试状态
  const cacheDebugLogs = ref<string[]>([])
  const requestParams = ref<any>({
    current: 1,
    size: 20,
    name: '',
    phone: '',
    department: '',
    dateRange: undefined
  })

  // 编辑相关状态
  const editDialogVisible = ref(false)
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

  // 缓存键信息

  // 手机号搜索
  const phoneSearch = ref('')

  // 事件演示相关
  const eventDemoEnabled = ref(false)
  const eventLogs = ref<Array<{ type: string; message: string; time: string }>>([])

  // 表格配置演示
  const tableConfig = ref({
    height: '100%',
    fixedHeight: false // 新增：是否固定高度的开关
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
    searchState, // 搜索参数
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
    // 核心配置
    core: {
      apiFn: async (params: Api.Healing.SearchParams) => {
        // 在API调用前添加调试信息
        const requestKey = JSON.stringify(params)
        console.log('🚀 API 请求参数:', params)
        addCacheLog(`🚀 API 请求: current=${params.current}, size=${params.size}`)
        addCacheLog(`🔑 请求键: ${requestKey.substring(0, 100)}...`)

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
        // { type: 'index', width: 60, label: '序号' }, // 本地序号列
        { type: 'globalIndex', width: 60, label: '序号' }, // 全局序号列
        { prop: 'name', label: '治疗项目', sortable: true, minWidth: 200 },
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

        return records.map((item: any, index: number) => ({
          ...item,
          avatar: ACCOUNT_TABLE_DATA[index % ACCOUNT_TABLE_DATA.length].avatar,
          department: ['技术部', '产品部', '运营部', '市场部', '设计部'][
            Math.floor(Math.random() * 5)
          ],
          score: Math.floor(Math.random() * 5) + 1,
          status: ['1', '2', '3', '4'][Math.floor(Math.random() * 4)]
        }))
      }
      // 自定义响应适配器，处理后端特殊的返回格式
      // responseAdapter: (data: any) => {
      //   const { list, total, pageNum, pageSize } = data
      //   return {
      //     records: list,
      //     total: total,
      //     current: pageNum,
      //     size: pageSize
      //   }
      // }
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
      onSuccess: (data, response) => {
        console.log('✅ 数据加载成功:', data.length, '条')
        console.log('📊 响应详情:', response)
        addCacheLog(`✅ 网络请求成功: ${data.length} 条数据`)
        addCacheLog(
          `📝 响应信息: total=${response.total}, current=${response.current}, size=${response.size}`
        )
        // ElMessage.success(`加载 ${data.length} 条数据成功`)
      },
      onError: (error) => {
        console.error('❌ 数据加载失败:', error)
        addCacheLog(`❌ 请求失败: ${error.message}`)
        ElMessage.error(error.message)
      },
      onCacheHit: (data, response) => {
        console.log('🎯 缓存命中:', data.length, '条')
        console.log('🔑 缓存来源:', response)
        addCacheLog(
          `🎯 缓存命中: ${data.length} 条数据 (current=${response.current}, size=${response.size})`
        )
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
  const handleSelectionChange = (selection: Api.Healing.Item[]) => {
    selectedRows.value = selection
    console.log('选择变更:', selection)
  }

  const handleRowClick = (row: Api.Healing.Item) => {
    console.log('行点击:', row)
    logEvent('行点击', `点击了康复项目: ${row.name}`)
  }

  const handleHeaderClick = (column: any) => {
    console.log('表头点击:', column)
    logEvent('表头点击', `点击了 ${column.label} 列表头`)
  }

  const handleSortChange = (sortInfo: any) => {
    console.log('排序事件:', sortInfo)
    console.log('排序字段:', sortInfo.prop)
    console.log('排序方向:', sortInfo.order)
    logEvent('排序变更', `字段: ${sortInfo.prop}, 方向: ${sortInfo.order}`)
  }

  // 事件日志记录
  const logEvent = (type: string, message: string) => {
    if (!eventDemoEnabled.value) return

    const time = new Date().toLocaleTimeString()
    eventLogs.value.unshift({ type, message, time })

    // 限制日志数量
    if (eventLogs.value.length > 20) {
      eventLogs.value = eventLogs.value.slice(0, 20)
    }
  }

  // 搜索与重置
  const handleSearch = () => {
    console.log('搜索参数:', searchFormState.value)

    // 将搜索表单的参数合并到搜索状态中
    Object.assign(searchState, {
      name: searchFormState.value.name || '',
      current: 1, // 重置到第一页
      size: paginationState.size
    })

    // 触发搜索
    searchData()
  }

  const handleReset = () => {
    addCacheLog('🔄 重置搜索')
    // 重置搜索表单状态
    searchFormState.value = { ...defaultFilter.value }
    resetSearch()
  }

  const handlePhoneSearch = (value: string) => {
    // searchFormState.value.phone = value
    // searchState.phone = value
    // requestParams.value = { ...searchState, phone: value }
    addCacheLog(`📱 手机号搜索: ${value}`)
    searchDataDebounced()
  }

  const handleRefresh = () => {
    addCacheLog('🔄 手动刷新')
    refreshAll()
  }

  // CRUD 操作
  const handleAdd = async () => {
    try {
      // 检查是否已有"新增康复项目"项目
      const existingNewItem = tableData.value.find((item) => item.name === '新增康复项目')
      if (existingNewItem) {
        ElMessage.warning('已存在"新增康复项目"项目，请先编辑或删除现有项目')
        return
      }

      // 创建新的康复项目，只需要名称和价格
      const newItem: Api.Healing.CreateParams = {
        name: '新增康复项目',
        description: '',
        price: convertYuanToCents(1) // 默认1元，转换为分
      }

      const response = await healingApi.createHealingItem(newItem)
      ElMessage.success(response.msg || '新增康复项目成功')

      // 刷新表格数据
      setTimeout(() => {
        refreshAfterCreate()
      }, 1000)
    } catch (error: any) {
      ElMessage.error(error.message || '新增康复项目失败')
      console.error('新增失败:', error)
    }
  }

  const handleEdit = (row: Api.Healing.Item) => {
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
        ElMessage.error('无效的项目ID')
        return
      }

      editFormLoading.value = true

      const updateParams: Api.Healing.UpdateParams = {
        id: currentEditingItem.value.id,
        name: editForm.value.name,
        description: editForm.value.description,
        price: convertYuanToCents(editForm.value.price) // 将元转换为分
      }

      const response = await healingApi.updateHealingItem(updateParams)
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

  const handleDelete = async (row: Api.Healing.Item) => {
    try {
      await ElMessageBox.confirm(`确定要删除康复项目 ${row.name} 吗？`, '警告', {
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

      // 刷新表格数据
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
        `确定要删除选中的 ${selectedRows.value.length} 个康复项目吗？`,
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
      ElMessage.success(response.msg || `批量删除 ${ids.length} 个康复项目成功`)
      selectedRows.value = []

      // 刷新表格数据
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
  // 监听分页和搜索状态变化
  watch(
    () => [paginationState.current, paginationState.size, searchFormState.value],
    ([current, size, search]) => {
      requestParams.value = {
        ...(search as any),
        current,
        size
      }
    },
    { deep: true, immediate: true }
  )

  // 监听搜索表单变化，实现实时搜索
  watch(
    () => searchFormState.value.name,
    (newName) => {
      if (newName !== undefined) {
        // 将搜索参数更新到搜索状态
        Object.assign(searchState, {
          name: newName || '',
          current: 1, // 重置到第一页
          size: paginationState.size
        })

        // 使用防抖搜索
        searchDataDebounced()
      }
    },
    { deep: true }
  )
</script>

<style lang="scss" scoped>
  .advanced-table-demo {
    display: flex;
    flex-direction: column;
    gap: 16px;
    padding-bottom: 20px;

    .intro-card {
      .intro-header {
        display: flex;
        flex-wrap: wrap;
        gap: 12px;
        align-items: center;
        justify-content: space-between;

        h3 {
          margin: 0;
        }

        .intro-badges {
          display: flex;
          flex-wrap: wrap;
          gap: 8px;
        }
      }

      .intro-content {
        .intro-text {
          margin: 0 0 16px;
          line-height: 1.6;
          color: var(--el-text-color-regular);
        }

        .debug-panel {
          margin: 16px 0;

          .debug-info {
            display: flex;
            flex-direction: column;
            gap: 8px;

            .stat-item {
              display: flex;
              align-items: center;
              justify-content: space-between;

              .label {
                font-weight: 500;
                color: var(--el-text-color-regular);
              }

              .value {
                font-weight: 600;
                color: var(--el-color-primary);
              }
            }

            .debug-actions {
              display: flex;
              gap: 8px;
              margin-top: 8px;
            }
          }
        }

        .feature-toggles {
          display: flex;
          flex-wrap: wrap;
          gap: 16px;
          margin-top: 16px;
        }
      }
    }

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

        .table-actions {
          margin-left: 8px;
        }
      }

      .user-info {
        display: flex;
        gap: 12px;
        align-items: center;

        .el-avatar {
          flex-shrink: 0;
          width: 40px !important;
          height: 40px !important;

          img {
            width: 100%;
            height: 100%;
            object-fit: cover;
            object-position: center;
          }
        }

        .user-details {
          flex: 1;
          min-width: 0;

          .user-name {
            margin: 0;
            overflow: hidden;
            font-weight: 500;
            color: var(--el-text-color-primary);
            text-overflow: ellipsis;
            white-space: nowrap;
          }

          .user-email {
            margin: 4px 0 0;
            overflow: hidden;
            font-size: 12px;
            color: var(--el-text-color-regular);
            text-overflow: ellipsis;
            white-space: nowrap;
          }
        }
      }

      .operation-buttons {
        display: flex;
      }

      .custom-header {
        display: inline-block;
        gap: 4px;
        align-items: center;
        color: var(--el-color-primary);
        cursor: pointer;

        &:hover {
          color: var(--el-color-primary-light-3);
        }
      }
    }

    .advanced-features-card {
      .feature-demo-section {
        display: flex;
        flex-direction: column;
        gap: 24px;

        .demo-group {
          padding: 16px;
          background: var(--el-bg-color-page);
          border: 1px solid var(--el-border-color-lighter);
          border-radius: 8px;

          h5 {
            margin: 0 0 16px;
            font-size: 14px;
            font-weight: 600;
            color: var(--el-text-color-primary);
          }

          .demo-buttons {
            display: flex;
            flex-wrap: wrap;
            gap: 8px;
            margin-bottom: 12px;

            &:last-child {
              margin-bottom: 0;
            }
          }

          .config-toggles {
            display: flex;
            flex-wrap: wrap;
            gap: 16px;
            margin-bottom: 12px;

            .el-switch {
              --el-switch-on-color: var(--el-color-primary);
            }
          }

          .event-logs {
            padding: 12px;
            margin-top: 12px;
            background: var(--el-bg-color-page);
            border: 1px solid var(--el-border-color-light);
            border-radius: 6px;

            .log-header {
              display: flex;
              align-items: center;
              justify-content: space-between;
              margin-bottom: 8px;
              font-weight: 500;
              color: var(--el-text-color-regular);
            }

            .log-list {
              display: flex;
              flex-direction: column;
              gap: 4px;
              max-height: 200px;
              overflow-y: auto;

              .log-item {
                display: flex;
                gap: 8px;
                align-items: center;
                padding: 6px 8px;
                font-size: 12px;
                background: var(--el-bg-color);
                border-left: 3px solid var(--el-border-color);
                border-radius: 4px;

                .log-message {
                  flex: 1;
                  color: var(--el-text-color-regular);
                }

                .log-time {
                  font-size: 11px;
                  color: var(--el-text-color-placeholder);
                }
              }
            }
          }

          .performance-info {
            margin-top: 12px;

            .el-alert {
              --el-alert-padding: 12px;
            }
          }
        }
      }
    }

    .refresh-demo-card {
      .refresh-buttons {
        display: flex;
        flex-wrap: wrap;
        gap: 8px;
      }
    }
  }

  // 响应式设计
  @media (width <= 768px) {
    .advanced-table-demo {
      .intro-card .intro-header {
        flex-direction: column;
        align-items: flex-start;

        .intro-badges {
          width: 100%;
        }
      }

      .art-table-card .toolbar-left {
        flex-direction: column;
        gap: 8px;
        align-items: flex-start;

        .table-actions {
          margin-left: 0;
        }
      }

      .refresh-demo-card .refresh-buttons {
        flex-direction: column;
      }
    }
  }

  .request-params {
    display: flex;
    flex-direction: column;
    gap: 8px;

    .params-display {
      max-height: 200px;
      padding: 8px;
      overflow-y: auto;
      font-size: 12px;
      background: var(--el-bg-color-page);
      border: 1px solid var(--el-border-color-light);
      border-radius: 6px;
    }
  }

  .logs-container {
    max-height: 200px;
    overflow-y: auto;

    .empty-logs {
      padding: 20px;
      text-align: center;
    }

    .log-list {
      display: flex;
      flex-direction: column;
      gap: 4px;

      .log-item {
        padding: 6px 8px;
        font-size: 12px;
        line-height: 1.4;
        background: var(--el-bg-color-page);
        border-left: 3px solid var(--el-border-color);
        border-radius: 4px;

        &.log-success {
          background: rgb(103 194 58 / 10%);
          border-left-color: var(--el-color-success);
        }

        &.log-cache {
          background: rgb(64 158 255 / 10%);
          border-left-color: var(--el-color-primary);
        }

        &.log-error {
          background: rgb(245 108 108 / 10%);
          border-left-color: var(--el-color-danger);
        }
      }
    }
  }
</style>
