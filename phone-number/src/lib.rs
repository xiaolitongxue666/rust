//! 北美电话号码解析
//!
//! 考点：字符串解析、Option、切片、matches!、边界条件（11 位需以 1 开头，区号/交换码不能以 0/1 开头）

const ALLOWED_NON_DIGITS: &[char] = &[' ', '(', ')', '-', '.', '+'];

/// 从用户输入提取 10 位数字，非法字符返回 None；11 位且以 1 开头则去掉国家码
pub fn number(user_number: &str) -> Option<String> {
    let mut digits = String::new();
    for c in user_number.chars() {
        if c.is_ascii_digit() {
            digits.push(c);
        } else if !ALLOWED_NON_DIGITS.contains(&c) {
            return None;
        }
    }

    let digits: Vec<char> = digits.chars().collect();
    let len = digits.len();

    let ten_digits = if len == 11 && digits[0] == '1' {
        &digits[1..]
    } else if len == 10 {
        digits.as_slice()
    } else {
        return None;
    };

    let area_first = ten_digits[0];
    let exchange_first = ten_digits[3];

    if matches!(area_first, '0' | '1') || matches!(exchange_first, '0' | '1') {
        return None;
    }

    Some(ten_digits.iter().collect())
}
