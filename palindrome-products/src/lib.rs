use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value: u64,
    factors: HashSet<(u64, u64)>,
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

// 检查一个数字是否是回文数
fn is_palindrome(n: u64) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    // 检查输入有效性
    if min > max {
        return None;
    }

    // 存储所有回文数及其因子对
    // key: 回文数值, value: 因子对集合
    let mut palindrome_map: HashMap<u64, HashSet<(u64, u64)>> = HashMap::new();

    // 生成所有可能的乘积并检查回文数
    for i in min..=max {
        for j in i..=max {
            let product = i * j;
            if is_palindrome(product) {
                palindrome_map
                    .entry(product)
                    .or_insert_with(HashSet::new)
                    .insert((i, j));
            }
        }
    }

    // 如果没有找到回文数，返回None
    if palindrome_map.is_empty() {
        return None;
    }

    // 找到最小和最大回文数
    let min_value = *palindrome_map.keys().min()?;
    let max_value = *palindrome_map.keys().max()?;

    // 创建Palindrome结构体
    let min_palindrome = Palindrome {
        value: min_value,
        factors: palindrome_map.remove(&min_value).unwrap(),
    };

    let max_palindrome = Palindrome {
        value: max_value,
        factors: palindrome_map.remove(&max_value).unwrap(),
    };

    Some((min_palindrome, max_palindrome))
}
