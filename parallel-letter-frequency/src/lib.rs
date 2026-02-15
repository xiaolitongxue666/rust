//! # Parallel Letter Frequency
//!
//! 使用并行计算统计多段文本中字母出现频率。仅统计字母（含 Unicode），大小写不敏感。
//!
//! ## 考点
//! - `rayon` 并行迭代器 `par_iter`
//! - `char::is_alphabetic()` 判断字母
//! - `rayon::ThreadPoolBuilder` 限制 worker 数量

use std::collections::HashMap;
use rayon::prelude::*;

/// 统计单段文本中的字母频率（小写）
fn count_letters(text: &str) -> HashMap<char, usize> {
    let mut counts = HashMap::new();
    for c in text.chars().filter(|c| c.is_alphabetic()) {
        *counts.entry(c.to_lowercase().next().unwrap()).or_insert(0) += 1;
    }
    counts
}

/// 合并两个 HashMap
fn merge_counts(
    mut a: HashMap<char, usize>,
    b: HashMap<char, usize>,
) -> HashMap<char, usize> {
    for (k, v) in b {
        *a.entry(k).or_insert(0) += v;
    }
    a
}

/// 使用 worker_count 个 worker 并行统计字母频率
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(worker_count.max(1))
        .build()
        .unwrap();

    pool.install(|| {
        let results: Vec<HashMap<char, usize>> = input
            .par_iter()
            .map(|text| count_letters(text))
            .collect();

        results
            .into_iter()
            .reduce(merge_counts)
            .unwrap_or_default()
    })
}
