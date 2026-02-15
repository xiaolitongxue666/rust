//! # Book Store - 购书折扣
//!
//! 不同数量组享受不同折扣，求最优分组使总价最低。DP + 递归枚举分组。
//!
//! ## 考点
//! - 动态规划、组合优化

use std::collections::HashMap;

const PRICE: u32 = 800;

fn group_price(n: usize) -> u32 {
    let discount = match n {
        1 => 100,
        2 => 95,
        3 => 90,
        4 => 80,
        5 => 75,
        _ => 100,
    };
    (n as u32) * PRICE * discount / 100
}

fn min_cost(counts: &mut [u32; 5], memo: &mut HashMap<[u32; 5], u32>) -> u32 {
    if let Some(&c) = memo.get(counts) {
        return c;
    }
    let mut result = u32::MAX;
    let mut any = false;
    for group_size in 1..=5 {
        let indices: Vec<usize> = (0..5).filter(|&i| counts[i] > 0).collect();
        if indices.len() < group_size {
            continue;
        }
        for combo in combinations(&indices, group_size) {
            any = true;
            for &i in &combo {
                counts[i] -= 1;
            }
            let cost = group_price(group_size) + min_cost(counts, memo);
            for &i in &combo {
                counts[i] += 1;
            }
            result = result.min(cost);
        }
    }
    if !any {
        result = 0;
    }
    memo.insert(*counts, result);
    result
}

fn combinations(indices: &[usize], k: usize) -> Vec<Vec<usize>> {
    let mut out = Vec::new();
    let mut combo = Vec::with_capacity(k);
    fn go(
        indices: &[usize],
        k: usize,
        start: usize,
        combo: &mut Vec<usize>,
        out: &mut Vec<Vec<usize>>,
    ) {
        if combo.len() == k {
            out.push(combo.clone());
            return;
        }
        for i in start..indices.len() {
            combo.push(indices[i]);
            go(indices, k, i + 1, combo, out);
            combo.pop();
        }
    }
    go(indices, k, 0, &mut combo, &mut out);
    out
}

pub fn lowest_price(books: &[u32]) -> u32 {
    let mut counts = [0u32; 5];
    for &b in books {
        if (1..=5).contains(&b) {
            counts[(b - 1) as usize] += 1;
        }
    }
    let mut memo = HashMap::new();
    min_cost(&mut counts, &mut memo)
}
