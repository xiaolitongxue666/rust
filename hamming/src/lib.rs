//! 汉明距离：等长字符串中不同字符的个数
//!
//! 考点：Option、chars().zip()、filter、count；注意 chars 按 Unicode 标量，len() 按字节

/// 返回两字符串的汉明距离，长度不等则 None
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    } else {
        return Some(
            s1.chars()
                .zip(s2.chars())
                .filter(|(c1, c2)| *c1 != *c2)
                .count(),
        );
    }
}
