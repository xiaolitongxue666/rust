//! 倍数之和：求 [1, limit) 内所有 factors 倍数的和（去重）
//!
//! 考点：HashSet 去重、factor 为 0 时跳过

use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut sum = 0;
    let mut multiples = HashSet::new();
    for factor in factors {
        sum += sum_of_a_multiple(limit, *factor, &mut multiples);
    }
    sum
}

fn sum_of_a_multiple(limit: u32, factor: u32, hash_set: &mut HashSet<u32>) -> u32 {
    let mut count = 1;
    let mut sum = 0;
    let mut current_factor = factor;
    if factor == 0 {
        return 0;
    }
    while current_factor < limit {
        if !hash_set.contains(&current_factor) {
            hash_set.insert(current_factor);
            sum += current_factor;
        }
        count += 1;
        current_factor = factor * count;
    }
    sum
}
