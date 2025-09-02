/**
 * namespace: Api
 *
 * 所有接口相关类型定义
 * 在.vue文件使用会报错，需要在 eslint.config.mjs 中配置 globals: { Api: 'readonly' }
 */
declare namespace Api {
  /** 基础类型 */
  namespace Http {
    /** 基础响应 */
    interface BaseResponse<T = any> {
      // 状态码
      code: number
      // 消息
      msg: string
      // 数据
      data: T
    }
  }

  /** 通用类型 */
  namespace Common {
    /** 分页参数 */
    interface PaginatingParams {
      /** 当前页码 */
      current: number
      /** 每页条数 */
      size: number
      /** 总条数 */
      total: number
    }

    /** 通用搜索参数 */
    type PaginatingSearchParams = Pick<PaginatingParams, 'current' | 'size'>
  }

  /** 机构管理类型 */
  namespace Institution {
    /** 医师信息 */
    interface Doctor {
      /** 医师ID */
      id?: number
      /** 医师姓名 */
      name: string
    }

    /** 机构信息 */
    interface Info {
      /** 机构ID */
      id?: number
      /** 机构名称 */
      institution_name: string
      /** 科室设置 */
      department_settings: string
      /** 医师管理 */
      doctor_management: Doctor[]
      /** 创建时间 */
      create_time?: string
      /** 更新时间 */
      update_time?: string
    }

    /** 机构信息更新参数 */
    type UpdateParams = {
      institution_name: string
      department_settings: string
      doctor_management: Doctor[]
    }
  }

  /** 认证类型 */
  namespace Auth {
    /** 登录参数 */
    interface LoginParams {
      userName: string
      password: string
    }

    /** 登录响应 */
    interface LoginResponse {
      token: string
      refreshToken: string
    }
  }

  /** 用户类型 */
  namespace User {
    /** 用户信息 */
    interface UserInfo {
      userId: number
      userName: string
      roles: string[]
      buttons: string[]
      avatar?: string
      email?: string
      phone?: string
    }

    /** 用户列表数据 */
    interface UserListData {
      records: UserListItem[]
      current: number
      size: number
      total: number
    }

    /** 用户列表项 */
    interface UserListItem {
      id: number
      avatar: string
      createBy: string
      createTime: string
      updateBy: string
      updateTime: string
      status: '1' | '2' | '3' | '4' // 1: 在线 2: 离线 3: 异常 4: 注销
      userName: string
      userGender: string
      nickName: string
      userPhone: string
      userEmail: string
      userRoles: string[]
    }
  }

  /** 康复项目类型 */
  namespace Healing {
    /** 康复项目基础信息 */
    interface Item {
      /** 项目ID */
      id?: number
      /** 项目名称 */
      name: string
      /** 项目描述 */
      description?: string
      /** 项目价格（以分为单位） */
      price: number
      /** 创建时间 */
      create_time?: string
      /** 更新时间 */
      update_time?: string
    }

    /** 搜索参数 */
    interface SearchParams extends Common.PaginatingSearchParams {
      /** 项目名称（支持拼音搜索） */
      name?: string
    }

    /** 创建参数 */
    interface CreateParams {
      /** 项目名称 */
      name: string
      /** 项目描述 */
      description?: string
      /** 项目价格（以分为单位） */
      price: number
    }

    /** 更新参数 */
    interface UpdateParams extends CreateParams {
      /** 项目ID */
      id: number
    }

    /** 列表响应 */
    interface ListResponse extends Common.PaginatingParams {
      /** 康复项目列表 */
      records: Item[]
    }
  }

  /** 中药管理类型 */
  namespace ChineseMedicine {
    /** 中药基础信息 */
    interface Item {
      /** 中药ID */
      id?: number
      /** 中药名称 */
      name: string
      /** 中药描述 */
      description?: string
      /** 中药单价（以分为单位） */
      unit_price: number
      /** 创建时间 */
      create_time?: string
      /** 更新时间 */
      update_time?: string
    }

    /** 搜索参数 */
    interface SearchParams extends Common.PaginatingSearchParams {
      /** 中药名称（支持拼音搜索） */
      name?: string
    }

    /** 列表响应 */
    interface ListResponse extends Common.PaginatingParams {
      /** 中药列表 */
      records: Item[]
    }
  }
}
