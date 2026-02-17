//! 字符串反转
//!
//! 考点：chars()、DoubleEndedIterator::rev、collect

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
