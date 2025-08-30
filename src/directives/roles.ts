import { App, Directive } from 'vue'

/**
 * 角色权限指令 - 已禁用
 * 由于移除了登录机制，所有用户都有完整权限
 * 此指令保持为空实现以维持兼容性
 * 用法：
 * <el-button v-roles="['R_SUPER', 'R_ADMIN']">按钮</el-button>
 * <el-button v-roles="'R_ADMIN'">按钮</el-button>
 */

function checkRolePermission(): void {
  // 登录机制已移除，所有用户都有完整权限
  // 不再进行角色权限检查，直接允许访问所有内容
  return
}

const rolesDirective: Directive = {
  mounted: checkRolePermission,
  updated: checkRolePermission
}

export function setupRolesDirective(app: App): void {
  app.directive('roles', rolesDirective)
}
