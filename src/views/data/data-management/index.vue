<template>
  <div class="data-management-container">
    <!-- 页面标题和操作区 -->
    <ElCard class="header-card" shadow="never">
      <div class="page-header">
        <div class="header-left">
          <h2>数据管理</h2>
          <p class="subtitle">数据库备份、恢复、压缩和数据导入导出</p>
        </div>
        <div class="header-right">
          <ElButton type="primary" :icon="Plus" @click="showCreateBackupDialog = true">
            创建备份
          </ElButton>
          <ElButton type="success" :icon="Download" @click="showExportDialog = true">
            导出数据
          </ElButton>
          <ElButton type="warning" :icon="Upload" @click="showImportDialog = true">
            导入数据
          </ElButton>
        </div>
      </div>
    </ElCard>

    <!-- 数据库状态信息 -->
    <ElRow :gutter="20" class="status-cards" style="margin-top: 16px">
      <ElCol :span="6">
        <ElCard class="status-card">
          <div class="status-item">
            <div class="status-icon database">
              <i class="iconfont-sys">&#xe6f4;</i>
            </div>
            <div class="status-content">
              <div class="status-title">数据库大小</div>
              <div class="status-value">{{ formatFileSize(databaseStatus.size) }}</div>
            </div>
          </div>
        </ElCard>
      </ElCol>
      <ElCol :span="6">
        <ElCard class="status-card">
          <div class="status-item">
            <div class="status-icon records">
              <i class="iconfont-sys">&#xe6f5;</i>
            </div>
            <div class="status-content">
              <div class="status-title">总记录数</div>
              <div class="status-value">{{ databaseStatus.record_count.toLocaleString() }}</div>
            </div>
          </div>
        </ElCard>
      </ElCol>
      <ElCol :span="6">
        <ElCard class="status-card">
          <div class="status-item">
            <div class="status-icon backup">
              <i class="iconfont-sys">&#xe6f6;</i>
            </div>
            <div class="status-content">
              <div class="status-title">最后备份</div>
              <div class="status-value">{{
                databaseStatus.last_backup ? formatDateTime(databaseStatus.last_backup) : '暂无备份'
              }}</div>
            </div>
          </div>
        </ElCard>
      </ElCol>
      <ElCol :span="6">
        <ElCard class="status-card">
          <div class="status-item">
            <div class="status-icon compact">
              <i class="iconfont-sys">&#xe6f7;</i>
            </div>
            <div class="status-content">
              <div class="status-title">文件操作</div>
              <div class="operation-buttons">
                <ElButton
                  type="info"
                  size="small"
                  :icon="FolderOpened"
                  @click="handleOpenDatabaseFile"
                  title="在文件管理器中打开数据库文件"
                >
                  打开数据库
                </ElButton>
                <ElButton
                  type="warning"
                  size="small"
                  :icon="FolderOpened"
                  @click="handleOpenBackupDirectory"
                  title="在文件管理器中打开备份目录"
                >
                  打开备份目录
                </ElButton>
                <ElButton
                  type="primary"
                  size="small"
                  :icon="Operation"
                  :loading="compacting"
                  @click="handleCompactDatabase"
                >
                  压缩数据库
                </ElButton>
              </div>
            </div>
          </div>
        </ElCard>
      </ElCol>
    </ElRow>

    <!-- 数据表统计 -->
    <ElCard class="table-stats-card" shadow="never">
      <template #header>
        <div class="card-header">
          <span>数据表统计</span>
          <ElButton type="text" :icon="Refresh" @click="loadDatabaseStatus">刷新</ElButton>
        </div>
      </template>
      <ElRow :gutter="20">
        <ElCol :span="6" v-for="table in databaseStatus.tables" :key="table.name">
          <div class="table-stat-item">
            <div class="table-name">{{ table.name }}</div>
            <div class="table-count">{{ table.count.toLocaleString() }} 条记录</div>
          </div>
        </ElCol>
      </ElRow>
    </ElCard>

    <!-- 备份列表 -->
    <ElCard class="backup-list-card" shadow="never">
      <template #header>
        <div class="card-header">
          <span>备份列表</span>
          <ElButton type="text" :icon="Refresh" @click="loadBackupList">刷新</ElButton>
        </div>
      </template>

      <ElTable
        :data="backupList"
        style="width: 100%"
        v-loading="loading"
        @selection-change="handleSelectionChange"
      >
        <ElTableColumn type="selection" width="55" />
        <ElTableColumn prop="filename" label="备份文件名" min-width="200">
          <template #default="{ row }">
            <div class="filename-cell">
              <i class="iconfont-sys file-icon">&#xe6f8;</i>
              <span>{{ row.filename }}</span>
            </div>
          </template>
        </ElTableColumn>
        <ElTableColumn prop="description" label="备注" min-width="150">
          <template #default="{ row }">
            <span>{{ row.description || '-' }}</span>
          </template>
        </ElTableColumn>
        <ElTableColumn prop="size" label="文件大小" width="120">
          <template #default="{ row }">
            <span>{{ formatFileSize(row.size) }}</span>
          </template>
        </ElTableColumn>
        <ElTableColumn prop="created_at" label="创建时间" width="180">
          <template #default="{ row }">
            <span>{{ formatDateTime(row.created_at) }}</span>
          </template>
        </ElTableColumn>
        <ElTableColumn label="操作" width="280" fixed="right">
          <template #default="{ row }">
            <ElButton
              type="primary"
              size="small"
              :icon="FolderOpened"
              @click="handleOpenBackupFile(row)"
              title="在文件管理器中打开"
            >
              打开
            </ElButton>
            <ElButton
              type="success"
              size="small"
              :icon="Download"
              @click="handleRestoreBackup(row)"
            >
              恢复
            </ElButton>
            <ElButton type="danger" size="small" :icon="Delete" @click="handleDeleteBackup(row)">
              删除
            </ElButton>
          </template>
        </ElTableColumn>
      </ElTable>

      <!-- 批量操作 -->
      <div class="batch-operations" v-if="selectedBackups.length > 0">
        <div class="selected-info"> 已选择 {{ selectedBackups.length }} 个备份 </div>
        <ElButton type="danger" size="small" :icon="Delete" @click="handleBatchDelete">
          批量删除
        </ElButton>
      </div>
    </ElCard>

    <!-- 创建备份对话框 -->
    <ElDialog
      v-model="showCreateBackupDialog"
      title="创建数据库备份"
      width="500px"
      :close-on-click-modal="false"
    >
      <ElForm :model="backupForm" :rules="backupRules" ref="backupFormRef" label-width="80px">
        <ElFormItem label="备注" prop="description">
          <ElInput
            v-model="backupForm.description"
            placeholder="请输入备份备注信息"
            maxlength="100"
            show-word-limit
          />
        </ElFormItem>
      </ElForm>
      <template #footer>
        <ElButton @click="showCreateBackupDialog = false">取消</ElButton>
        <ElButton type="primary" :loading="creating" @click="handleCreateBackup">
          创建备份
        </ElButton>
      </template>
    </ElDialog>

    <!-- 导出数据对话框 -->
    <ElDialog
      v-model="showExportDialog"
      title="导出数据"
      width="600px"
      :close-on-click-modal="false"
    >
      <ElForm :model="exportForm" ref="exportFormRef" label-width="100px">
        <ElFormItem label="选择数据表">
          <ElCheckboxGroup v-model="exportForm.tables">
            <ElCheckbox
              v-for="table in databaseStatus.tables"
              :key="table.name"
              :label="table.name"
            >
              {{ table.name }} ({{ table.count }} 条记录)
            </ElCheckbox>
          </ElCheckboxGroup>
        </ElFormItem>
      </ElForm>
      <template #footer>
        <ElButton @click="showExportDialog = false">取消</ElButton>
        <ElButton type="primary" :loading="exporting" @click="handleExportData">
          导出数据
        </ElButton>
      </template>
    </ElDialog>

    <!-- 导入数据对话框 -->
    <ElDialog
      v-model="showImportDialog"
      title="导入数据"
      width="500px"
      :close-on-click-modal="false"
    >
      <div class="import-area">
        <ElUpload
          ref="uploadRef"
          :file-list="importFileList"
          :auto-upload="false"
          :limit="1"
          accept=".json"
          @change="handleFileChange"
          @remove="handleFileRemove"
        >
          <ElButton type="primary" :icon="Upload">选择JSON文件</ElButton>
          <template #tip>
            <div class="el-upload__tip"> 只能上传JSON文件，文件大小不超过100MB </div>
          </template>
        </ElUpload>
      </div>
      <template #footer>
        <ElButton @click="showImportDialog = false">取消</ElButton>
        <ElButton
          type="primary"
          :loading="importing"
          :disabled="importFileList.length === 0"
          @click="handleImportData"
        >
          导入数据
        </ElButton>
      </template>
    </ElDialog>
  </div>
</template>

<script setup lang="ts">
  import { ref, onMounted } from 'vue'
  import { ElMessage, ElMessageBox, ElNotification } from 'element-plus'
  import {
    Plus,
    Download,
    Upload,
    Delete,
    Refresh,
    FolderOpened,
    Operation
  } from '@element-plus/icons-vue'
  import { DataManagementService, type DatabaseBackup } from '@/api/dataManagementApi'

  defineOptions({ name: 'DataManagement' })

  // 响应式数据
  const loading = ref(false)
  const creating = ref(false)
  const compacting = ref(false)
  const exporting = ref(false)
  const importing = ref(false)

  // 备份列表和选中状态
  const backupList = ref<DatabaseBackup[]>([])
  const selectedBackups = ref<DatabaseBackup[]>([])

  // 数据库状态
  const databaseStatus = ref({
    size: 0,
    last_backup: null as string | null,
    record_count: 0,
    tables: [] as Array<{ name: string; count: number }>
  })

  // 对话框显示状态
  const showCreateBackupDialog = ref(false)
  const showExportDialog = ref(false)
  const showImportDialog = ref(false)

  // 表单数据
  const backupForm = ref({
    description: ''
  })

  const exportForm = ref({
    tables: [] as string[]
  })

  const importFileList = ref<any[]>([])

  // 表单引用
  const backupFormRef = ref()
  const exportFormRef = ref()
  const uploadRef = ref()

  // 表单验证规则
  const backupRules = {
    description: [
      { required: true, message: '请输入备份备注信息', trigger: 'blur' },
      { max: 100, message: '备注信息不能超过100个字符', trigger: 'blur' }
    ]
  }

  // 加载备份列表
  const loadBackupList = async () => {
    try {
      loading.value = true
      backupList.value = await DataManagementService.getBackupList()
    } catch (error) {
      ElMessage.error('加载备份列表失败')
      console.error(error)
    } finally {
      loading.value = false
    }
  }

  // 加载数据库状态
  const loadDatabaseStatus = async () => {
    try {
      databaseStatus.value = await DataManagementService.getDatabaseStatus()
    } catch (error) {
      ElMessage.error('加载数据库状态失败')
      console.error(error)
    }
  }

  // 创建备份
  const handleCreateBackup = async () => {
    try {
      await backupFormRef.value?.validate()
      creating.value = true

      const backup = await DataManagementService.createBackup({
        description: backupForm.value.description
      })

      ElNotification.success({
        title: '备份成功',
        message: `备份文件 ${backup.filename} 创建成功`
      })

      showCreateBackupDialog.value = false
      backupForm.value.description = ''

      // 刷新列表和状态
      await Promise.all([loadBackupList(), loadDatabaseStatus()])
    } catch (error) {
      ElMessage.error('创建备份失败')
      console.error(error)
    } finally {
      creating.value = false
    }
  }

  // 恢复备份
  const handleRestoreBackup = async (backup: DatabaseBackup) => {
    try {
      await ElMessageBox.confirm(
        `确定要恢复备份 "${backup.filename}" 吗？\n此操作将覆盖当前数据库中的所有数据，请谨慎操作！`,
        '确认恢复',
        {
          confirmButtonText: '确认恢复',
          cancelButtonText: '取消',
          type: 'warning',
          dangerouslyUseHTMLString: true
        }
      )

      const result = await DataManagementService.restoreBackup({
        backup_id: backup.id
      })

      if (result) {
        ElNotification.success({
          title: '恢复成功',
          message: `数据库已从备份 ${backup.filename} 恢复`
        })

        // 刷新状态
        await loadDatabaseStatus()
      }
    } catch (error) {
      if (error !== 'cancel') {
        ElMessage.error('恢复备份失败')
        console.error(error)
      }
    }
  }

  // 打开数据库文件
  const handleOpenDatabaseFile = async () => {
    try {
      await DataManagementService.openDatabaseFile()
      ElMessage.success('已在文件管理器中打开数据库文件')
    } catch (error) {
      ElMessage.error('打开数据库文件失败')
      console.error(error)
    }
  }

  // 打开备份目录
  const handleOpenBackupDirectory = async () => {
    try {
      await DataManagementService.openBackupDirectory()
      ElMessage.success('已在文件管理器中打开备份目录')
    } catch (error) {
      ElMessage.error('打开备份目录失败')
      console.error(error)
    }
  }

  // 打开备份文件
  const handleOpenBackupFile = async (backup: DatabaseBackup) => {
    try {
      await DataManagementService.openBackupFile(backup.id)
      ElMessage.success('已在文件管理器中打开备份文件')
    } catch (error) {
      ElMessage.error('打开备份文件失败')
      console.error(error)
    }
  }

  // 删除备份
  const handleDeleteBackup = async (backup: DatabaseBackup) => {
    try {
      await ElMessageBox.confirm(`确定要删除备份 "${backup.filename}" 吗？`, '确认删除', {
        confirmButtonText: '确认删除',
        cancelButtonText: '取消',
        type: 'warning'
      })

      await DataManagementService.deleteBackup(backup.id)

      ElMessage.success('备份删除成功')
      await loadBackupList()
    } catch (error) {
      if (error !== 'cancel') {
        ElMessage.error('删除备份失败')
        console.error(error)
      }
    }
  }

  // 批量删除备份
  const handleBatchDelete = async () => {
    try {
      await ElMessageBox.confirm(
        `确定要删除选中的 ${selectedBackups.value.length} 个备份吗？`,
        '确认批量删除',
        {
          confirmButtonText: '确认删除',
          cancelButtonText: '取消',
          type: 'warning'
        }
      )

      const deletePromises = selectedBackups.value.map((backup) =>
        DataManagementService.deleteBackup(backup.id)
      )

      await Promise.all(deletePromises)

      ElMessage.success(`成功删除 ${selectedBackups.value.length} 个备份`)
      selectedBackups.value = []
      await loadBackupList()
    } catch (error) {
      if (error !== 'cancel') {
        ElMessage.error('批量删除失败')
        console.error(error)
      }
    }
  }

  // 压缩数据库
  const handleCompactDatabase = async () => {
    try {
      await ElMessageBox.confirm(
        '确定要压缩数据库吗？这将优化数据库存储空间，可能需要一些时间。',
        '确认压缩',
        {
          confirmButtonText: '确认压缩',
          cancelButtonText: '取消',
          type: 'info'
        }
      )

      compacting.value = true
      const result = await DataManagementService.compactDatabase()

      ElNotification.success({
        title: '压缩完成',
        message: `数据库压缩完成，节省空间：${formatFileSize(result.saved_space)}`
      })

      await loadDatabaseStatus()
    } catch (error) {
      if (error !== 'cancel') {
        ElMessage.error('压缩数据库失败')
        console.error(error)
      }
    } finally {
      compacting.value = false
    }
  }

  // 导出数据
  const handleExportData = async () => {
    try {
      if (exportForm.value.tables.length === 0) {
        ElMessage.warning('请至少选择一个数据表')
        return
      }

      exporting.value = true
      const jsonData = await DataManagementService.exportDataAsJson(exportForm.value.tables)

      // 创建下载链接
      const blob = new Blob([jsonData], { type: 'application/json' })
      const url = URL.createObjectURL(blob)
      const link = document.createElement('a')
      link.href = url
      link.download = `数据导出_${new Date().toISOString().slice(0, 19).replace(/:/g, '-')}.json`
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
      URL.revokeObjectURL(url)

      ElMessage.success('数据导出成功')
      showExportDialog.value = false
      exportForm.value.tables = []
    } catch (error) {
      ElMessage.error('导出数据失败')
      console.error(error)
    } finally {
      exporting.value = false
    }
  }

  // 导入数据
  const handleImportData = async () => {
    try {
      if (importFileList.value.length === 0) {
        ElMessage.warning('请选择要导入的JSON文件')
        return
      }

      const file = importFileList.value[0].raw
      const reader = new FileReader()

      reader.onload = async (e) => {
        try {
          importing.value = true
          const jsonData = e.target?.result as string

          const result = await DataManagementService.importDataFromJson(jsonData)

          if (result.errors.length > 0) {
            ElNotification.warning({
              title: '导入完成（有警告）',
              message: `成功导入 ${result.imported_records} 条记录，${result.errors.length} 个错误`
            })
          } else {
            ElNotification.success({
              title: '导入成功',
              message: `成功导入 ${result.imported_records} 条记录`
            })
          }

          showImportDialog.value = false
          importFileList.value = []

          // 刷新状态
          await loadDatabaseStatus()
        } catch (error) {
          ElMessage.error('导入数据失败')
          console.error(error)
        } finally {
          importing.value = false
        }
      }

      reader.readAsText(file)
    } catch (error) {
      ElMessage.error('读取文件失败')
      console.error(error)
    }
  }

  // 处理文件选择
  const handleFileChange = (file: any) => {
    importFileList.value = [file]
  }

  // 处理文件移除
  const handleFileRemove = () => {
    importFileList.value = []
  }

  // 处理表格选择变化
  const handleSelectionChange = (selection: DatabaseBackup[]) => {
    selectedBackups.value = selection
  }

  // 格式化文件大小
  const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B'
    const k = 1024
    const sizes = ['B', 'KB', 'MB', 'GB']
    const i = Math.floor(Math.log(bytes) / Math.log(k))
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
  }

  // 格式化日期时间
  const formatDateTime = (dateString: string): string => {
    return new Date(dateString).toLocaleString('zh-CN')
  }

  // 页面加载时初始化数据
  onMounted(async () => {
    await Promise.all([loadBackupList(), loadDatabaseStatus()])
  })
</script>

<style lang="scss" scoped>
  .data-management-container {
    min-height: 100vh;
    padding: 20px;
    background-color: var(--el-bg-color-page);

    .header-card {
      margin-bottom: 20px;
      border: none;

      .page-header {
        display: flex;
        align-items: center;
        justify-content: space-between;

        .header-left {
          h2 {
            margin: 0 0 8px;
            font-size: 24px;
            font-weight: 600;
            color: var(--el-text-color-primary);
          }

          .subtitle {
            margin: 0;
            font-size: 14px;
            color: var(--el-text-color-regular);
          }
        }

        .header-right {
          display: flex;
          gap: 12px;
        }
      }
    }

    .status-cards {
      display: flex;
      margin-bottom: 20px;

      :deep(.el-row) {
        display: flex;
        width: 100%;
      }

      :deep(.el-col) {
        display: flex;
      }

      .status-card {
        width: 100%;
        height: 100%;
        border: none;

        .status-item {
          display: flex;
          gap: 16px;
          align-items: center;
          width: 100%;
          //   min-height: 120px;

          .status-icon {
            display: flex;
            flex-shrink: 0;
            align-items: center;
            justify-content: center;
            width: 48px;
            height: 48px;
            font-size: 24px;
            color: white;
            border-radius: 8px;

            &.database {
              background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            }

            &.records {
              background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
            }

            &.backup {
              background: linear-gradient(135deg, #4facfe 0%, #00f2fe 100%);
            }

            &.compact {
              background: linear-gradient(135deg, #43e97b 0%, #38f9d7 100%);
            }
          }

          .status-content {
            display: flex;
            flex: 1;
            flex-direction: column;
            min-height: 120px;

            .status-title {
              margin-bottom: 8px;
              font-size: 14px;
              font-weight: 500;
              color: var(--el-text-color-regular);
            }

            .status-value {
              display: flex;
              flex: 1;
              align-items: center;
              font-size: 18px;
              font-weight: 600;
              line-height: 1.4;
              color: var(--el-text-color-primary);
            }

            .operation-buttons {
              display: flex;
              flex-direction: column;
              gap: 8px;
              align-items: center;
              padding-top: 8px;
              margin-top: auto;

              .el-button {
                box-sizing: border-box;
                display: flex !important;
                align-items: center !important;
                justify-content: center !important;
                width: 100% !important;
                height: 32px;
                margin-left: 2px !important;
                font-size: 13px;
                text-align: center !important;

                :deep(.el-button__content) {
                  display: flex !important;
                  gap: 4px;
                  align-items: center !important;
                  justify-content: center !important;
                  width: 100%;
                }

                :deep(.el-icon) {
                  margin-right: 0 !important;
                  margin-left: 0 !important;
                }

                :deep(span) {
                  display: flex;
                  align-items: center;
                  justify-content: center;
                }

                &:hover {
                  transition: all 0.2s ease;
                  transform: translateY(-1px);
                }
              }
            }
          }
        }
      }
    }

    .table-stats-card,
    .backup-list-card {
      margin-bottom: 20px;
      border: none;

      .card-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        font-weight: 600;
      }
    }

    .table-stat-item {
      padding: 16px;
      text-align: center;
      background: var(--el-fill-color-lighter);
      border-radius: 8px;

      .table-name {
        margin-bottom: 8px;
        font-size: 16px;
        font-weight: 600;
        color: var(--el-text-color-primary);
      }

      .table-count {
        font-size: 14px;
        color: var(--el-text-color-regular);
      }
    }

    .filename-cell {
      display: flex;
      gap: 8px;
      align-items: center;

      .file-icon {
        font-size: 16px;
        color: var(--el-color-primary);
      }
    }

    .batch-operations {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 12px;
      margin-top: 16px;
      background: var(--el-fill-color-light);
      border-radius: 8px;

      .selected-info {
        font-size: 14px;
        color: var(--el-text-color-regular);
      }
    }

    .import-area {
      padding: 20px;
      text-align: center;
      background: var(--el-fill-color-lighter);
      border: 2px dashed var(--el-border-color);
      border-radius: 8px;
    }
  }

  :deep(.el-upload__tip) {
    margin-top: 8px;
    font-size: 12px;
    color: var(--el-text-color-regular);
  }
</style>
