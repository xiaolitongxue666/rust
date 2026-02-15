//! # Pythagorean Triplet (勾股三元组)
//!
//! 找出所有满足 a² + b² = c² 且 a < b < c、a + b + c = sum 的自然数三元组。
//!
//! ## 考点
//! - 数学：枚举 a、b，由 a+b+c=sum 得 c，验证 a²+b²=c²
//! - `HashSet` 去重存储

use std::collections::HashSet;

/// 找出所有和为 sum 的勾股三元组 [a, b, c]，其中 a < b < c
///
/// # 算法
/// 枚举 a ∈ [1, sum/3)，b ∈ (a, (sum-a)/2]，c = sum - a - b，
/// 若 a² + b² = c² 则加入结果
pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut result = HashSet::new();

    for a in 1..sum / 3 {
        for b in (a + 1)..=(sum.saturating_sub(a).saturating_sub(1)) / 2 {
            let c = sum - a - b;
            if c <= b {
                break;
            }
            if a * a + b * b == c * c {
                result.insert([a, b, c]);
            }
        }
    }

    result
}
