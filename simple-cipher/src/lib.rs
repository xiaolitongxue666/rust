//! # Simple Cipher (Vigenère Cipher)
//!
//! 维吉尼亚密码是一种多表替换密码。密钥由小写字母组成，每个明文字母按对应密钥字母的偏移量进行移位。
//! 密钥 'a' 表示偏移 0，'b' 表示偏移 1，以此类推。
//!
//! ## 考点
//! - `char::is_ascii_lowercase()` 验证密钥格式
//! - `Option` 用于无效密钥的返回
//! - `rand` crate 生成随机密钥
//! - 迭代器 `cycle()` 实现密钥循环使用

use rand::Rng;

/// 验证密钥是否有效：非空且全部为小写字母
fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.chars().all(|c| c.is_ascii_lowercase())
}

/// 使用维吉尼亚密码编码
///
/// # 参数
/// * `key` - 密钥，必须非空且全部为小写字母
/// * `s` - 待编码的明文（仅小写字母会被编码）
///
/// # 返回
/// `Some(密文)` 若密钥有效，否则 `None`
///
/// # 算法
/// 对每个字母：`(plain + key_offset) % 26`
pub fn encode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }

    let key_bytes: Vec<u8> = key.bytes().map(|b| b - b'a').collect();

    Some(
        s.chars()
            .zip(key_bytes.iter().cycle())
            .map(|(c, &shift)| {
                if c.is_ascii_lowercase() {
                    let shifted = ((c as u8 - b'a') + shift) % 26;
                    char::from(b'a' + shifted)
                } else {
                    c
                }
            })
            .collect(),
    )
}

/// 使用维吉尼亚密码解码
///
/// # 算法
/// 对每个字母：`(cipher - key_offset + 26) % 26` 确保非负
pub fn decode(key: &str, s: &str) -> Option<String> {
    if !is_valid_key(key) {
        return None;
    }

    let key_bytes: Vec<u8> = key.bytes().map(|b| b - b'a').collect();

    Some(
        s.chars()
            .zip(key_bytes.iter().cycle())
            .map(|(c, &shift)| {
                if c.is_ascii_lowercase() {
                    let decoded = (c as u8 - b'a' + 26 - shift) % 26;
                    char::from(b'a' + decoded)
                } else {
                    c
                }
            })
            .collect(),
    )
}

/// 生成随机密钥并编码
///
/// 密钥至少 100 个随机小写字母，保证每次调用密钥不同
pub fn encode_random(s: &str) -> (String, String) {
    let mut rng = rand::rng();
    let key: String = (0..100)
        .map(|_| rng.random_range(b'a'..=b'z') as char)
        .collect();
    let encoded = encode(&key, s).unwrap();
    (key, encoded)
}
