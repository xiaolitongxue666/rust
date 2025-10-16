pub fn abbreviate(phrase: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = phrase.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        if c.is_alphabetic() {
            let should_add = if i == 0 {
                // 第一个字母总是添加
                true
            } else {
                let prev_char = chars[i - 1];
                // 判断当前字母是否应该添加到缩写中，需要满足以下任一条件：
                // 1. 前一个字符是空白字符（空格、制表符、换行符等）
                //    例如："Hello World" 中的 'W' 应该被添加
                // 2. 前一个字符是连字符 '-'
                //    例如："self-contained" 中的 'c' 应该被添加
                // 3. 前一个字符是下划线 '_'
                //    例如："file_name" 中的 'n' 应该被添加
                // 4. 前一个字符是小写字母且当前字符是大写字母（驼峰命名法）
                //    例如："HyperText" 中的 'T' 应该被添加
                prev_char.is_whitespace()
                    || prev_char == '-'
                    || prev_char == '_'
                    || (prev_char.is_lowercase() && c.is_uppercase())
            };

            if should_add {
                result.push(c.to_uppercase().next().unwrap());
            }
        }
    }

    result
}
