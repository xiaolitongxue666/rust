//! # Word Count
//!
//! 统计文本中每个词的出现次数。词由标点或空白分隔，但缩写中的撇号（如 don't）不分隔。
//! 数字视为词，比较时大小写不敏感。
//!
//! ## 考点
//! - `HashMap` 与 `Entry` API：`entry().or_insert()` 或 `and_modify()`
//! - `str::to_lowercase()` 规范化大小写
//! - 自定义分词逻辑：按非字母数字且非撇号字符分割

use std::collections::HashMap;

/// 统计输入文本中每个词的出现次数
///
/// # 规则
/// - 词由标点或空白分隔，撇号（'）在缩写中不分隔（如 don't 为一个词）
/// - 数字视为词
/// - 大小写不敏感，输出键为小写
///
/// # 考点：HashMap::entry() 简化计数逻辑
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let mut counts: HashMap<String, u32> = HashMap::new();

    // 按非词字符分割：词可包含字母、数字、撇号
    // 使用 split 配合闭包：当 c 不是字母数字且不是撇号时分割
    for word in words.split(|c: char| !c.is_alphanumeric() && c != '\'') {
        let word = word.trim_matches('\''); // 去除首尾引号，如 'large' -> large
        if !word.is_empty() {
            let normalized = word.to_lowercase();
            // Entry API：若不存在则插入 0，然后 +1
            counts
                .entry(normalized)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    counts
}
