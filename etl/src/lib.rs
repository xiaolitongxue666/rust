//! ETL：将 (分数 -> 字母列表) 转为 (字母 -> 分数)
//!
//! 考点：BTreeMap、嵌套循环、键值反转

use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result = BTreeMap::new();

    for (key, value) in h {
        for letter in value {
            result.insert(letter.to_ascii_lowercase(), *key);
        }
    }
    result
}
