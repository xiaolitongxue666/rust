//! 游程编码：连续相同字符压缩为 "数量+字符"（数量为 1 时省略）
//!
//! 考点：chars 迭代、状态机（前字符+计数）、decode 需解析数字前缀

pub fn encode(source: &str) -> String {
    let mut return_string: String = String::new();

    if source.is_empty() {
        return return_string;
    } else {
        let mut iter = source.chars();
        let mut pre_char: char = '0';
        let mut count: u64 = 0;

        loop {
            if let Some(iterm) = iter.next() {
                if pre_char != iterm {
                    if count != 0 {
                        if count > 1 {
                            return_string.push_str(&count.to_string());
                        }
                        return_string.push(pre_char);
                    }
                    count = 1;
                    pre_char = iterm;
                } else {
                    count += 1;
                }
            } else {
                if count != 0 {
                    if count > 1 {
                        return_string.push_str(&count.to_string());
                    }
                    return_string.push(pre_char);
                }
                break;
            }
        }

        return_string
    }
}

pub fn decode(source: &str) -> String {
    let mut result = String::new();
    let mut chars = source.chars().peekable();
    let mut count_str = String::new();

    while let Some(c) = chars.next() {
        if c.is_ascii_digit() {
            count_str.push(c);
        } else {
            let n: usize = count_str.parse().unwrap_or(1);
            for _ in 0..n {
                result.push(c);
            }
            count_str.clear();
        }
    }
    result
}
