#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    // 1. 输入验证
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    // 2. 处理空数组
    if number.is_empty() {
        return Ok(vec![0]);
    }

    // 3. 验证数字有效性
    for &digit in number {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
    }

    // 4. 直接转换算法
    let mut result = Vec::new();
    let mut digits = number.to_vec();

    while !digits.is_empty() && !all_zero(&digits) {
        let mut remainder = 0u64;
        let mut new_digits = Vec::new();
        let mut found_non_zero = false;

        // 模拟除法：从高位到低位
        for &digit in &digits {
            let value = remainder * from_base as u64 + digit as u64;
            let quotient = (value / to_base as u64) as u32;
            remainder = value % to_base as u64;

            // 跳过前导零，但保留至少一位
            if found_non_zero || quotient != 0 || !new_digits.is_empty() {
                new_digits.push(quotient);
                found_non_zero = true;
            }
        }

        // 收集余数（目标进制的一位）
        result.push(remainder as u32);
        digits = new_digits;
    }

    // 5. 处理结果
    if result.is_empty() {
        result.push(0);
    } else {
        result.reverse(); // 余数是倒序收集的
    }

    Ok(result)
}

/// 检查数组是否全为零
fn all_zero(digits: &[u32]) -> bool {
    digits.iter().all(|&d| d == 0)
}
