//! # Rotational Cipher (Caesar Cipher)
//!
//! 旋转密码是一种简单的移位密码，将字母表中的每个字母按密钥值进行移位。
//! 密钥 0 或 26 时输出与输入相同（模运算）。
//!
//! ## 考点
//! - `char` 与 `u8` 的转换：`c as u8`、`char::from()`
//! - `char::is_ascii_alphabetic()` 判断字母
//! - `char::is_ascii_lowercase()` / `char::is_ascii_uppercase()` 区分大小写
//! - 迭代器 `map` 与 `collect` 构建新字符串

/// 对输入文本应用旋转密码（凯撒密码）变换
///
/// # 参数
/// * `input` - 待加密的明文
/// * `key` - 移位密钥 (0-26)，0 和 26 等价
///
/// # 返回
/// 加密后的密文。仅字母被移位，空格、数字、标点保持不变。
///
/// # 算法
/// 对每个字母：`(char - base + key) % 26 + base`
/// 其中 base 为 'a'(97) 或 'A'(65)
pub fn rotate(input: &str, key: u8) -> String {
    // 考点：key 对 26 取模，避免 key=26 时的边界问题
    let key = key % 26;

    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                // 小写字母：base = 'a' = 97
                // (c as u8 - 97 + key) % 26 + 97
                let shifted = ((c as u8 - b'a') + key) % 26;
                char::from(b'a' + shifted)
            } else if c.is_ascii_uppercase() {
                // 大写字母：base = 'A' = 65
                let shifted = ((c as u8 - b'A') + key) % 26;
                char::from(b'A' + shifted)
            } else {
                // 非字母（空格、数字、标点）原样保留
                c
            }
        })
        .collect()
}
