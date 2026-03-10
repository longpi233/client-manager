import { pinyin } from 'pinyin-pro'

const normalize = (value: string): string => value.trim().toLowerCase()

const getFullPinyin = (text: string): string =>
  normalize(
    pinyin(text, {
      toneType: 'none',
      type: 'array'
    }).join('')
  )

const getInitials = (text: string): string =>
  normalize(
    pinyin(text, {
      toneType: 'none',
      pattern: 'first',
      type: 'array'
    }).join('')
  )

export const matchesNameByPinyin = (name: string, keyword: string): boolean => {
  const normalizedKeyword = normalize(keyword)
  if (!normalizedKeyword) return true

  const normalizedName = normalize(name)
  if (normalizedName.includes(normalizedKeyword)) return true

  const fullPinyin = getFullPinyin(name)
  if (fullPinyin.includes(normalizedKeyword)) return true

  const initials = getInitials(name)
  return initials.includes(normalizedKeyword)
}

export const filterByNamePinyin = <T>(
  list: T[],
  getName: (item: T) => string,
  keyword: string
): T[] => list.filter((item) => matchesNameByPinyin(getName(item), keyword))

export const mergeUniqueBy = <T>(first: T[], second: T[], getKey: (item: T) => string): T[] => {
  const map = new Map<string, T>()
  first.forEach((item) => map.set(getKey(item), item))
  second.forEach((item) => map.set(getKey(item), item))
  return Array.from(map.values())
}
