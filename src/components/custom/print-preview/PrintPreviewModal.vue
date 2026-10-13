<template>
  <ElDialog
    v-model="visible"
    :title="dialogTitle"
    width="920px"
    top="4vh"
    destroy-on-close
    append-to-body
    class="print-preview-dialog"
  >
    <!-- 顶部操作工具栏 -->
    <div class="preview-toolbar">
      <div class="toolbar-left">
        <ElRadioGroup v-model="currentPaperSize" size="small">
          <ElRadioButton value="a4">A4 纸张 (标准)</ElRadioButton>
          <ElRadioButton value="a5">A5 纸张 (处方笺)</ElRadioButton>
        </ElRadioGroup>

        <ElRadioGroup v-model="zoomScale" size="small" style="margin-left: 12px">
          <ElRadioButton :value="0.85">85%</ElRadioButton>
          <ElRadioButton :value="1">100%</ElRadioButton>
          <ElRadioButton :value="1.15">115%</ElRadioButton>
        </ElRadioGroup>
      </div>

      <div class="toolbar-right">
        <ElButton type="primary" :icon="Printer" @click="handlePrint"> 立即打印 </ElButton>
        <ElButton type="success" :icon="Document" @click="handleSavePdf"> 保存为 PDF </ElButton>
        <ElButton @click="visible = false">关闭</ElButton>
      </div>
    </div>

    <!-- 纸质预览舞台区 -->
    <div class="preview-viewport">
      <div
        class="paper-sheet"
        :class="[`paper-${currentPaperSize}`]"
        :style="{ transform: `scale(${zoomScale})`, transformOrigin: 'top center' }"
        v-html="htmlContent"
      ></div>
    </div>

    <template #footer>
      <div class="preview-footer-tip">
        <span
          >💡 提示：如需导出为 PDF，点击“保存为 PDF”后，在系统打印对话框的“目标打印机”中选择“另存为
          PDF”即可。</span
        >
      </div>
    </template>
  </ElDialog>
</template>

<script setup lang="ts">
  import { ref } from 'vue'
  import { ElDialog, ElRadioGroup, ElRadioButton, ElButton, ElMessage } from 'element-plus'
  import { Printer, Document } from '@element-plus/icons-vue'
  import { printDocument } from '@/utils/print/printUtils'

  export interface PrintPreviewOptions {
    title?: string
    htmlContent: string
    filename?: string
    paperSize?: 'a4' | 'a5'
  }

  const visible = ref(false)
  const dialogTitle = ref('打印与导出预览')
  const htmlContent = ref('')
  const exportFilename = ref('医疗文书')
  const currentPaperSize = ref<'a4' | 'a5'>('a4')
  const zoomScale = ref<number>(1)

  /**
   * 打开预览窗口
   */
  const open = (options: PrintPreviewOptions) => {
    dialogTitle.value = options.title ? `${options.title} - 打印与导出预览` : '打印与导出预览'
    htmlContent.value = options.htmlContent || ''
    exportFilename.value = options.filename || options.title || '医疗文书'
    currentPaperSize.value = options.paperSize || 'a4'
    zoomScale.value = 1
    visible.value = true
  }

  /**
   * 立即打印
   */
  const handlePrint = () => {
    try {
      printDocument(htmlContent.value, exportFilename.value, {
        showPdfOption: false,
        filename: exportFilename.value
      })
      ElMessage.success('已发送打印任务')
    } catch (error) {
      console.error('打印执行异常:', error)
      ElMessage.error('打印任务调用失败，请检查打印服务')
    }
  }

  /**
   * 保存为 PDF
   */
  const handleSavePdf = () => {
    try {
      printDocument(htmlContent.value, exportFilename.value, {
        showPdfOption: true,
        filename: exportFilename.value
      })
      ElMessage.info('请在弹出的打印窗口中选择【另存为 PDF】并点击保存')
    } catch (error) {
      console.error('导出 PDF 异常:', error)
      ElMessage.error('导出任务调用失败，请检查环境')
    }
  }

  defineExpose({
    open,
    close: () => {
      visible.value = false
    }
  })
</script>

<style lang="scss" scoped>
  .print-preview-dialog {
    :deep(.el-dialog__body) {
      padding: 12px 20px 20px;
    }
  }

  .preview-toolbar {
    display: flex;
    flex-wrap: wrap;
    gap: 12px;
    align-items: center;
    justify-content: space-between;
    padding: 10px 16px;
    margin-bottom: 16px;
    background-color: var(--el-fill-color-light);
    border: 1px solid var(--el-border-color-lighter);
    border-radius: 6px;
  }

  .toolbar-left,
  .toolbar-right {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
    align-items: center;
  }

  .preview-viewport {
    display: flex;
    justify-content: center;
    height: 68vh;
    padding: 24px;
    overflow: auto;
    background-color: #525659;
    border-radius: 6px;
  }

  .paper-sheet {
    box-sizing: border-box;
    padding: 30px 36px;
    color: #222;
    background: #fff;
    box-shadow: 0 4px 18px rgb(0 0 0 / 35%);
    transition: transform 0.2s ease;

    &.paper-a4 {
      width: 760px;
      min-height: 980px;
    }

    &.paper-a5 {
      width: 580px;
      min-height: 720px;
    }

    :deep(.print-header) {
      padding-bottom: 12px;
      margin-bottom: 24px;
      text-align: center;
      border-bottom: 2px solid #333;

      h1 {
        margin: 0 0 8px;
        font-size: 22px;
        font-weight: 700;
        letter-spacing: 2px;
      }

      h2 {
        margin: 0;
        font-size: 17px;
        color: #444;
        letter-spacing: 4px;
      }
    }

    :deep(.form-content) {
      font-size: 14px;
      line-height: 1.6;
    }

    :deep(.form-row) {
      display: flex;
      align-items: flex-start;
      margin-bottom: 12px;
    }

    :deep(.form-label) {
      font-weight: 600;
      color: #333;
      white-space: nowrap;
    }

    :deep(.form-value) {
      flex: 1;
      min-height: 20px;
      padding: 0 6px;
      margin-right: 16px;
      word-break: break-all;
      border-bottom: 1px dashed #aaa;
    }

    :deep(.prescription-table),
    :deep(.treatment-table) {
      width: 100%;
      margin: 8px 0;
      border-collapse: collapse;

      th,
      td {
        padding: 6px 10px;
        text-align: left;
        border: 1px solid #dcdfe6;
      }

      th {
        font-weight: 600;
        background-color: #f5f7fa;
      }
    }

    :deep(.signature-row) {
      display: flex;
      justify-content: space-between;
      padding-top: 12px;
      margin-top: 24px;
      border-top: 1px solid #333;
    }
  }

  .preview-footer-tip {
    font-size: 12px;
    color: var(--el-text-color-secondary);
    text-align: left;
  }
</style>
