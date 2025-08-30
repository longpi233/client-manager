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
    //   /** 门诊病历类型 */
    namespace MedicalRecord {
      /** 性别枚举 */
      type Gender = '1' | '2' // 1-男, 2-女

      /** 结算类型枚举 */
      type TradeType = '1' | '2' | '3' // 1-现金结算, 2-省直医保, 3-武汉医保

      /** 治疗项目 */
      interface TreatmentItem {
        /** 治疗项目名称 */
        treatment_item: string
        /** 数量 */
        count: number
        /** 单价（以分为单位） */
        unit_price?: number
        /** 总价（以分为单位） */
        total_price?: number
      }
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
      /** 医师职称 */
      title?: string
      /** 专业科室 */
      department?: string
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

  /** 散客登记类型 */
  namespace Customer {
    /** 性别枚举 */
    type Gender = '1' | '2' // 1-男, 2-女

    /** 结算类型枚举 */
    type TradeType = '1' | '2' | '3' // 1-现金结算, 2-省直医保, 3-武汉医保

    /** 治疗项目 */
    interface TreatmentItem {
      /** 治疗项目名称 */
      treatment_item: string
      /** 数量 */
      count: number
      /** 单价（以分为单位） */
      unit_price?: number
      /** 总价（以分为单位） */
      total_price?: number
    }

    /** 散客信息 */
    interface Info {
      /** ID */
      id?: number
      /** 患者姓名 */
      customer_name: string
      /** 性别 */
      gender: Gender
      /** 年龄 */
      age: number
      /** 联系电话 */
      telephone: string
      /** 就诊时间 */
      time_of_visit: string
      /** 结算类型 */
      trade_type: TradeType
      /** 诊断信息 */
      diagnosis: string
      /** 备注 */
      note?: string
      /** 治疗项目 */
      treatment: TreatmentItem[]
      /** 总金额（以分为单位） */
      total_amount: number
      /** 创建时间 */
      create_time?: string
      /** 更新时间 */
      update_time?: string
    }

    /** 创建散客参数 */
    type CreateParams = Omit<Info, 'id' | 'create_time' | 'update_time'>

    /** 散客列表搜索参数 */
    interface SearchParams extends Common.PaginatingSearchParams {
      /** 患者姓名 */
      customer_name?: string
      /** 联系电话 */
      telephone?: string
      /** 结算类型 */
      trade_type?: TradeType
      /** 开始时间 */
      start_time?: string
      /** 结束时间 */
      end_time?: string
    }

    /** 散客列表响应 */
    interface ListResponse extends Common.PaginatingParams {
      /** 记录列表 */
      records: Info[]
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
      price: number
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

    /** 创建参数 */
    interface CreateParams {
      /** 中药名称 */
      name: string
      /** 中药描述 */
      description?: string
      /** 中药单价（以分为单位） */
      price: number
    }

    /** 更新参数 */
    interface UpdateParams extends CreateParams {
      /** 中药ID */
      id: number
    }

    /** 列表响应 */
    interface ListResponse extends Common.PaginatingParams {
      /** 中药列表 */
      records: Item[]
    }
  }

  /** 门诊病历类型 */
  namespace MedicalRecord {
    /** 性别枚举 */
    type Gender = '1' | '2' // 1-男, 2-女

    /** 结算类型枚举 */
    type TradeType = '1' | '2' | '3' // 1-现金结算, 2-省直医保, 3-武汉医保

    /** 治疗项目 */
    interface TreatmentItem {
      /** 治疗项目名称 */
      treatment_item: string
      /** 数量 */
      count: number
      /** 单价 */
      unit_price?: number
      /** 总价 */
      total_price?: number
    }

    /** 门诊病历信息 */
    interface Info {
      /** ID */
      id?: number
      /** 患者姓名 */
      customer_name: string
      /** 性别 */
      gender: Gender
      /** 年龄 */
      age: number
      /** 联系电话 */
      telephone: string
      /** 就诊时间 */
      time_of_visit: string
      /** 结算类型 */
      trade_type: TradeType
      /** 主诉 */
      chief_complaint: string
      /** 现病史 */
      illness_history: string
      /** 体格检查 */
      physical_examination: string
      /** 既往史 */
      past_history: string
      /** 实验室检查 */
      laboratory_examination: string
      /** 诊断信息 */
      diagnosis: string
      /** 治法治则 */
      treatment_method: string
      /** 备注 */
      note?: string
      /** 治疗项目 */
      treatment: TreatmentItem[]
      /** 总金额（以分为单位） */
      total_amount: number
      /** 医师ID */
      doctor_id: number
      /** 医师姓名 */
      doctor_name?: string
      /** 创建时间 */
      create_time?: string
      /** 更新时间 */
      update_time?: string
    }

    /** 创建门诊病历参数 */
    type CreateParams = Omit<Info, 'id' | 'doctor_name' | 'create_time' | 'update_time'>

    /** 门诊病历列表搜索参数 */
    interface SearchParams extends Common.PaginatingSearchParams {
      /** 患者姓名 */
      customer_name?: string
      /** 联系电话 */
      telephone?: string
      /** 结算类型 */
      trade_type?: TradeType
      /** 医师ID */
      doctor_id?: number
      /** 开始时间 */
      start_time?: string
      /** 结束时间 */
      end_time?: string
    }

    /** 门诊病历列表响应 */
    interface ListResponse extends Common.PaginatingParams {
      /** 记录列表 */
      records: Info[]
    }
  }
}
