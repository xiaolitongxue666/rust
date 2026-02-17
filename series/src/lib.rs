//! 连续子串：从数字字符串中提取所有长度为 len 的连续子串
//!
//! 考点：字符串切片、范围遍历

pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }
    let mut result = Vec::new();
    for i in 0..=digits.len() - len {
        result.push(digits[i..i + len].to_string());
    }
    result
}
