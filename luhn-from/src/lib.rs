//! Luhn 校验：通过 From trait 支持多种输入类型
//!
//! 考点：newtype 模式、From<T> blanket impl、闭包中修改外部变量（需 Cell/RefCell 或重构）

pub struct Luhn(String);

// 实现 From<T> 后，Rust 会自动为 T 实现 Into<目标类型>
// 因此可以使用 into() 方法进行转换 :
//   let luhn: Luhn = "123".into();  // 自动实现

impl Luhn {
    pub fn is_valid(&self) -> bool {
        // 1. 去除空格并过滤出数字字符，同时检查是否有无效字符
        let mut has_invalid_chars = false;
        let digits: Vec<char> = self
            .0
            .chars()
            .filter(|c| {
                if c.is_ascii_digit() {
                    true
                } else if c.is_whitespace() {
                    false // 跳过空格
                } else {
                    has_invalid_chars = true; // 标记有无效字符
                    false
                }
            })
            .collect();

        // 2. 检查长度和无效字符
        if digits.len() <= 1 || has_invalid_chars {
            return false;
        }

        // 3. 转换为 Vec<u32>
        let numbers: Vec<u32> = digits.iter().map(|c| c.to_digit(10).unwrap()).collect();

        // 4. 执行 Luhn 算法：从右到左，每隔一位翻倍
        let sum: u32 = numbers
            .iter()
            .rev() // 从右到左
            .enumerate()
            .map(|(i, &digit)| {
                if i % 2 == 1 {
                    // 每隔一位（从右边数第二位开始）
                    let doubled = digit * 2;
                    if doubled > 9 { doubled - 9 } else { doubled }
                } else {
                    digit
                }
            })
            .sum();

        // 5. 检查是否能被10整除
        sum % 10 == 0
    }
}

/// 为所有实现了ToString的类型实现From trait
/// 这是一个通用的解决方案，可以处理任何可以转换为字符串的类型
impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        Luhn(input.to_string())
    }
}
