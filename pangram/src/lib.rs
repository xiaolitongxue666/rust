//! 全字母句：判断是否包含 a-z 全部 26 个字母
//!
//! 考点：HashSet、to_lowercase、is_ascii_alphabetic、集合相等

use std::collections::HashSet;

/// 判断是否为全字母句
pub fn is_pangram(sentence: &str) -> bool {
    let english_26_characters: HashSet<char> =
        HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());

    // 提取句子中的所有小写字母并去重
    let sentence_letters: HashSet<char> = sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    // 检查句子的字母集合是否包含所有26个字母
    sentence_letters == english_26_characters
}
