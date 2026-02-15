//! # Knapsack (0/1 背包问题)
//!
//! 在容量限制下选择物品使总价值最大。每个物品最多选一次。
//!
//! ## 考点
//! - 动态规划：`dp[w]` 表示容量为 w 时的最大价值
//! - 反向遍历避免同一物品重复使用

#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

/// 计算在容量限制下能获得的最大价值
///
/// # 算法
/// 0/1 背包 DP：dp[w] = max(dp[w], dp[w-weight] + value)
/// 从大到小遍历 w 避免同一物品重复使用
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let max_weight = max_weight as usize;
    let mut dp = vec![0u32; max_weight + 1];

    for item in items {
        let w = item.weight as usize;
        let v = item.value;
        // 考点：反向遍历，确保每个物品只考虑一次
        for capacity in (w..=max_weight).rev() {
            let with_item = dp[capacity - w] + v;
            if with_item > dp[capacity] {
                dp[capacity] = with_item;
            }
        }
    }

    dp[max_weight]
}
