use pinyin::{Pinyin, ToPinyin};

// 拼音搜索辅助函数
pub fn contains_pinyin(text: &str, search: &str) -> bool {
    if search.is_empty() {
        return true;
    }

    let search_lower = search.to_lowercase();

    // 直接文本匹配
    if text.to_lowercase().contains(&search_lower) {
        return true;
    }

    // 拼音匹配
    let pinyin: String = text
        .to_pinyin()
        .map(|p| p.map_or("", Pinyin::plain))
        .collect::<String>()
        .to_lowercase();

    // 首字母匹配
    let initials: String = text
        .to_pinyin()
        .map(|p| p.map_or("", |p| &p.plain()[0..1]))
        .collect::<String>()
        .to_lowercase();

    pinyin.contains(&search_lower) || initials.contains(&search_lower)
}
