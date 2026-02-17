//! Luhn 校验算法：为多种类型实现同一 trait
//!
//! 考点：trait 定义、blanket impl（impl<T: ToString + Sized> Luhn for T）、
//! 具体类型 impl（impl Luhn for str）、Luhn 算法（从右往左偶数位乘 2，>9 则减 9）

pub trait Luhn {
    fn valid_luhn(&self) -> bool;
}

/// Blanket impl：所有 ToString 类型先转字符串再校验，实现一次覆盖全部
/// for the &str type. Naturally, you can implement this trait
/// by hand for every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?

impl<T: ToString + Sized> Luhn for T {
    fn valid_luhn(&self) -> bool {
        self.to_string().as_str().valid_luhn()
    }
}

/// 核心实现：str 的 Luhn 校验。考点：chars、filter、map、enumerate、rev、sum
impl Luhn for str {
    fn valid_luhn(&self) -> bool {
        let digits: Vec<char> = self.chars().filter(|c| c.is_ascii_digit()).collect();
        if digits.len() <= 1 {
            return false;
        }
        let numbers: Vec<u32> = digits.iter().map(|c| c.to_digit(10).unwrap()).collect();
        let sum: u32 = numbers
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &digit)| {
                if i % 2 == 1 {
                    let doubled = digit * 2;
                    if doubled > 9 { doubled - 9 } else { doubled }
                } else {
                    digit
                }
            })
            .sum();
        sum % 10 == 0
    }
}
