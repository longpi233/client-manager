/**
 * 数据管理相关API
 * 包含数据库备份、恢复、时间打标等功能
 */

import { invoke } from '@tauri-apps/api/core'

export interface DatabaseBackup {
  id: string
  filename: string
  timestamp: string
  size: number
  description?: string
  created_at: string
}

export interface BackupCreateRequest {
  description?: string
}

export interface BackupRestoreRequest {
  backup_id: string
}

/**
 * 数据管理服务类
 */
export class DataManagementService {
  /**
   * 获取数据库备份列表
   */
  static async getBackupList(): Promise<DatabaseBackup[]> {
    try {
      const result = await invoke<DatabaseBackup[]>('get_database_backups')
      return result
    } catch (error) {
      console.error('获取备份列表失败:', error)
      throw error
    }
  }

  /**
   * 创建数据库备份
   */
  static async createBackup(request: BackupCreateRequest): Promise<DatabaseBackup> {
    try {
      const result = await invoke<DatabaseBackup>('create_database_backup', {
        description: request.description || `备份_${new Date().toLocaleString()}`
      })
      return result
    } catch (error) {
      console.error('创建备份失败:', error)
      throw error
    }
  }

  /**
   * 恢复数据库备份
   */
  static async restoreBackup(request: BackupRestoreRequest): Promise<boolean> {
    try {
      const result = await invoke<boolean>('restore_database_backup', {
        backupId: request.backup_id
      })
      return result
    } catch (error) {
      console.error('恢复备份失败:', error)
      throw error
    }
  }

  /**
   * 删除数据库备份
   */
  static async deleteBackup(backupId: string): Promise<boolean> {
    try {
      const result = await invoke<boolean>('delete_database_backup', {
        backupId
      })
      return result
    } catch (error) {
      console.error('删除备份失败:', error)
      throw error
    }
  }

  /**
   * 获取数据库状态信息
   */
  static async getDatabaseStatus(): Promise<{
    size: number
    last_backup: string | null
    record_count: number
    tables: Array<{ name: string; count: number }>
  }> {
    try {
      const result = await invoke('get_database_status')
      return result as any
    } catch (error) {
      console.error('获取数据库状态失败:', error)
      throw error
    }
  }

  /**
   * 压缩数据库
   */
  static async compactDatabase(): Promise<{
    original_size: number
    new_size: number
    saved_space: number
  }> {
    try {
      const result = await invoke('compact_database')
      return result as any
    } catch (error) {
      console.error('压缩数据库失败:', error)
      throw error
    }
  }

  /**
   * 导出数据为JSON格式
   */
  static async exportDataAsJson(tables: string[]): Promise<string> {
    try {
      const result = await invoke<string>('export_data_as_json', {
        tables
      })
      return result
    } catch (error) {
      console.error('导出数据失败:', error)
      throw error
    }
  }

  /**
   * 从JSON导入数据
   */
  static async importDataFromJson(jsonData: string): Promise<{
    imported_records: number
    errors: string[]
  }> {
    try {
      const result = await invoke('import_data_from_json', {
        jsonData
      })
      return result as any
    } catch (error) {
      console.error('导入数据失败:', error)
      throw error
    }
  }

  /**
   * 在系统文件管理器中打开数据库文件
   */
  static async openDatabaseFile(): Promise<boolean> {
    try {
      const result = await invoke<boolean>('open_database_file')
      return result
    } catch (error) {
      console.error('打开数据库文件失败:', error)
      throw error
    }
  }

  /**
   * 在系统文件管理器中打开备份文件
   */
  static async openBackupFile(backupId: string): Promise<boolean> {
    try {
      const result = await invoke<boolean>('open_backup_file', {
        backupId
      })
      return result
    } catch (error) {
      console.error('打开备份文件失败:', error)
      throw error
    }
  }

  /**
   * 在系统文件管理器中打开备份目录
   */
  static async openBackupDirectory(): Promise<boolean> {
    try {
      const result = await invoke<boolean>('open_backup_directory')
      return result
    } catch (error) {
      console.error('打开备份目录失败:', error)
      throw error
    }
  }

  /**
   * 获取数据库文件完整路径
   */
  static async getDatabaseFilePath(): Promise<string> {
    try {
      const result = await invoke<string>('get_database_file_path')
      return result
    } catch (error) {
      console.error('获取数据库文件路径失败:', error)
      throw error
    }
  }

  /**
   * 获取备份文件完整路径
   */
  static async getBackupFilePath(backupId: string): Promise<string> {
    try {
      const result = await invoke<string>('get_backup_file_path', {
        backupId
      })
      return result
    } catch (error) {
      console.error('获取备份文件路径失败:', error)
      throw error
    }
  }
}

export default DataManagementService
