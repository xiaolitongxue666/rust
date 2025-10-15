/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    // 1. 去除空格并过滤出数字字符，同时检查是否有无效字符
    let mut has_invalid_chars = false;
    let digits: Vec<char> = code.chars()
        .filter(|c| {
            if c.is_ascii_digit() {
                true
            } else if c.is_whitespace() {
                false  // 跳过空格
            } else {
                has_invalid_chars = true;  // 标记有无效字符
                false
            }
        })
        .collect();
    
    // 2. 检查长度和无效字符
    if digits.len() <= 1 || has_invalid_chars {
        return false;
    }
    
    // 4. 转换为 Vec<u32>
    let numbers: Vec<u32> = digits.iter()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    // 5. 执行 Luhn 算法：从右到左，每隔一位翻倍
    let sum: u32 = numbers.iter()
        .rev()  // 从右到左
        .enumerate()
        .map(|(i, &digit)| {
            if i % 2 == 1 {  // 每隔一位（从右边数第二位开始）
                let doubled = digit * 2;
                if doubled > 9 {
                    doubled - 9
                } else {
                    doubled
                }
            } else {
                digit
            }
        })
        .sum();
    
    // 6. 检查是否能被10整除
    sum % 10 == 0
}
