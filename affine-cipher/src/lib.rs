/// 仿射密码错误类型
/// 当 a 和 26 不互质时返回此错误
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// 计算最大公约数（GCD）
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

/// 检查两个数是否互质
fn is_coprime(a: i32, m: i32) -> bool {
    gcd(a, m) == 1
}

/// 计算模逆元（Modular Multiplicative Inverse）
/// 返回满足 (a * x) mod m == 1 的 x
fn mod_inverse(a: i32, m: i32) -> Option<i32> {
    for x in 1..m {
        if (a * x) % m == 1 {
            return Some(x);
        }
    }
    None
}

/// 使用仿射密码加密明文
/// 
/// # 参数
/// * `plaintext` - 要加密的明文
/// * `a` - 密钥 a（必须与 26 互质）
/// * `b` - 密钥 b
/// 
/// # 返回
/// 加密后的密文，每 5 个字母一组，用空格分隔
/// 
/// # 错误
/// 如果 a 和 26 不互质，返回 `AffineCipherError::NotCoprime(a)`
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    const M: i32 = 26; // 字母表长度
    
    // 检查 a 和 26 是否互质
    if !is_coprime(a, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    
    let mut result = String::new();
    let mut count = 0; // 用于每 5 个字母添加空格
    
    for ch in plaintext.chars() {
        if ch.is_ascii_alphabetic() {
            // 转换为小写并计算索引（0-25）
            let i = (ch.to_ascii_lowercase() as i32 - 'a' as i32) % M;
            // 应用加密公式：E(x) = (a*i + b) mod 26
            let encrypted = (a * i + b).rem_euclid(M);
            let encrypted_char = (encrypted + 'a' as i32) as u8 as char;
            
            // 每 5 个字母添加一个空格
            if count > 0 && count % 5 == 0 {
                result.push(' ');
            }
            result.push(encrypted_char);
            count += 1;
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
    
    Ok(result)
}

/// 使用仿射密码解密密文
/// 
/// # 参数
/// * `ciphertext` - 要解密的密文
/// * `a` - 密钥 a（必须与 26 互质）
/// * `b` - 密钥 b
/// 
/// # 返回
/// 解密后的明文（小写，无空格）
/// 
/// # 错误
/// 如果 a 和 26 不互质，返回 `AffineCipherError::NotCoprime(a)`
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    const M: i32 = 26; // 字母表长度
    
    // 检查 a 和 26 是否互质
    if !is_coprime(a, M) {
        return Err(AffineCipherError::NotCoprime(a));
    }
    
    // 计算 a 的模逆元
    let a_inv = mod_inverse(a, M).ok_or(AffineCipherError::NotCoprime(a))?;
    
    let mut result = String::new();
    
    for ch in ciphertext.chars() {
        if ch.is_ascii_alphabetic() {
            // 转换为小写并计算索引（0-25）
            let y = (ch.to_ascii_lowercase() as i32 - 'a' as i32) % M;
            // 应用解密公式：D(y) = (a^-1 * (y - b)) mod 26
            let decrypted = (a_inv * (y - b)).rem_euclid(M);
            let decrypted_char = (decrypted + 'a' as i32) as u8 as char;
            result.push(decrypted_char);
        } else if ch.is_ascii_digit() {
            // 数字保持不变
            result.push(ch);
        }
        // 忽略空格和标点符号
    }
    
    Ok(result)
}
