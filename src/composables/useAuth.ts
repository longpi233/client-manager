/**
 * 权限管理（已简化，无需登录验证）
 * 用法：
 * const { hasAuth } = useAuth()
 * hasAuth('add') // 始终返回 true（已禁用权限验证）
 */
export const useAuth = () => {
  /**
   * 检查是否拥有某权限标识（已禁用，始终返回 true）
   * @param _auth 权限标识（未使用）
   * @returns 始终返回 true
   */
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const hasAuth = (_auth: string): boolean => {
    // 权限验证已禁用，所有操作都允许
    return true
  }

  return {
    hasAuth
  }
}
