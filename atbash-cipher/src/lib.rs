//! Atbash 密码：字母表反转 a↔z，对称加密
//!
//! 考点：char 与 u8 转换、b'z'-index 反转、每 5 字符加空格

/// 将明文字符转换为密文字符：a->z, b->y, ..., z->a
fn atbash_transform(ch: char) -> Option<char> {
    if ch.is_ascii_alphabetic() {
        // 转换为小写并计算索引（0-25）
        let index = ch.to_ascii_lowercase() as u8 - b'a';
        // Atbash 转换：'z' - index
        let cipher_char = (b'z' - index) as char;
        Some(cipher_char)
    } else {
        None
    }
}

/// 使用 Atbash 密码加密明文
///
/// # 规则
/// - 字母转换为小写并应用 Atbash 转换
/// - 数字保持不变
/// - 忽略标点符号和空格
/// - 输出每 5 个字符一组，用空格分隔
pub fn encode(plain: &str) -> String {
    let mut result = String::new();
    let mut count = 0; // 用于每 5 个字符添加空格

    for ch in plain.chars() {
        if ch.is_ascii_alphabetic() {
            // 每 5 个字符添加一个空格
            if count > 0 && count % 5 == 0 {
                result.push(' ');
            }
            if let Some(cipher_char) = atbash_transform(ch) {
                result.push(cipher_char);
                count += 1;
            }
        } else if ch.is_ascii_digit() {
            // 数字保持不变
            if count > 0 && count % 5 == 0 {
                result.push(' ');
            }
            result.push(ch);
            count += 1;
        }
        // 忽略空格和标点符号
    }

    result
}

/// 使用 Atbash 密码解密密文
///
/// # 规则
/// - Atbash 是对称密码，加密和解密使用相同的转换
/// - 忽略空格
/// - 数字保持不变
pub fn decode(cipher: &str) -> String {
    let mut result = String::new();

    for ch in cipher.chars() {
        if ch.is_ascii_alphabetic() {
            // Atbash 是对称的，使用相同的转换函数
            if let Some(plain_char) = atbash_transform(ch) {
                result.push(plain_char);
            }
        } else if ch.is_ascii_digit() {
            // 数字保持不变
            result.push(ch);
        }
        // 忽略空格和标点符号
    }

    result
}
