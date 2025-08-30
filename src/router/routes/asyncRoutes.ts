import { RoutesAlias } from '../routesAlias'
import { AppRouteRecord } from '@/types/router'

/**
 * 菜单列表、异步路由
 *
 * 支持两种模式:
 * 前端静态配置 - 直接使用本文件中定义的路由配置
 * 后端动态配置 - 后端返回菜单数据，前端解析生成路由
 *
 * 菜单标题（title）:
 * 可以是 i18n 的 key，也可以是字符串，比如：'用户列表'
 *
 * RoutesAlias.Layout 指向的是布局组件，后端返回的菜单数据中，component 字段需要指向 /index/index
 * 路由元数据（meta）：异步路由在 asyncRoutes 中配置，静态路由在 staticRoutes 中配置
 */
export const asyncRoutes: AppRouteRecord[] = [
  {
    name: 'Dashboard',
    path: '/dashboard',
    component: RoutesAlias.Layout,
    meta: {
      title: 'menus.dashboard.title',
      icon: '&#xe6cc;',
      roles: ['R_SUPER', 'R_ADMIN']
    },
    children: [
      {
        name: 'Information',
        path: 'information',
        component: RoutesAlias.Information,
        meta: {
          title: 'menus.dashboard.information',
          keepAlive: false,
          fixedTab: true
        }
      }
    ]
  },
  {
    name: 'Healing',
    path: '/healing',
    component: RoutesAlias.Layout,
    meta: {
      title: 'menus.healing.title',
      icon: '&#xe8b0;'
    },
    children: [
      {
        name: 'CustomerRegister',
        path: 'customer-register',
        component: RoutesAlias.CustomerRegister,
        meta: {
          title: 'menus.healing.customerRegister',
          keepAlive: true
        }
      },
      {
        name: 'MedicalRecord',
        path: 'medical-record',
        component: RoutesAlias.MedicalRecord,
        meta: {
          title: 'menus.healing.medicalRecord',
          keepAlive: true
        }
      },
      {
        name: 'ChineseMedicinePrescription',
        path: 'chinese-medicine-prescription',
        component: RoutesAlias.ChineseMedicinePrescription,
        meta: {
          title: 'menus.healing.chineseMedicinePrescription',
          keepAlive: true
        }
      }
    ]
  },
  {
    name: 'Management',
    path: '/management',
    component: RoutesAlias.Layout,
    meta: {
      title: 'menus.management.title',
      icon: '&#xe7ba;'
    },
    children: [
      {
        name: 'HealingItem',
        path: 'healing-item',
        component: RoutesAlias.HealingItem,
        meta: {
          title: 'menus.management.healingItem',
          keepAlive: true
        }
      },
      {
        name: 'ChineseMedicineItem',
        path: 'chinese-medicine-item',
        component: RoutesAlias.ChineseMedicineItem,
        meta: {
          title: 'menus.management.chineseMedicineItem',
          keepAlive: true
        }
      }
    ]
  },
  {
    name: 'Data',
    path: '/data',
    component: RoutesAlias.Layout,
    meta: {
      title: 'menus.data.title',
      icon: '&#xe7d9;'
    },
    children: [
      {
        name: 'DataInquiry',
        path: 'data-inquiry',
        component: RoutesAlias.DataInquiry,
        meta: {
          title: 'menus.data.dataInquiry',
          keepAlive: true
        }
      },
      {
        name: 'OutpatientLog',
        path: 'outpatient-log',
        component: RoutesAlias.OutpatientLog,
        meta: {
          title: 'menus.data.outpatientLog',
          keepAlive: true
        }
      }
    ]
  }
]
