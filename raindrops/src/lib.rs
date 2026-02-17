//! 雨滴数：3/5/7 的倍数对应 Pling/Plang/Plong，否则返回数字字符串
//!
//! 考点：取模、字符串拼接、to_string

pub fn raindrops(n: u32) -> String {
    let mut result_string = String::new();
    if n % 3 == 0 || n % 5 == 0 || n % 7 == 0 {
        if n % 3 == 0 {
            result_string.push_str("Pling");
        }
        if n % 5 == 0 {
            result_string.push_str("Plang");
        }
        if n % 7 == 0 {
            result_string.push_str("Plong");
        }
    } else {
        result_string = n.to_string();
    }

    result_string
}
