import { defineStore } from 'pinia'
import { ref } from 'vue'
import { LanguageEnum } from '@/enums/appEnum'
import { router } from '@/router'
import { AppRouteRecord } from '@/types/router'
import { setPageTitle } from '@/router/utils/utils'

/**
 * 用户状态管理
 * 管理语言设置、搜索历史等基础功能
 */
export const useUserStore = defineStore(
  'userStore',
  () => {
    // 语言设置
    const language = ref(LanguageEnum.ZH)
    // 搜索历史记录
    const searchHistory = ref<AppRouteRecord[]>([])

    /**
     * 设置语言
     * @param lang 语言枚举值
     */
    const setLanguage = (lang: LanguageEnum) => {
      setPageTitle(router.currentRoute.value)
      language.value = lang
    }

    /**
     * 设置搜索历史
     * @param list 搜索历史列表
     */
    const setSearchHistory = (list: AppRouteRecord[]) => {
      searchHistory.value = list
    }

    return {
      language,
      searchHistory,
      setLanguage,
      setSearchHistory
    }
  },
  {
    persist: {
      key: 'user',
      storage: localStorage
    }
  }
)
