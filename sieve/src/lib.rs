/// 使用埃拉托斯特尼筛法找出所有小于等于 upper_bound 的素数
/// 
/// # 算法步骤
/// 1. 创建一个布尔数组，标记每个数是否为素数
/// 2. 从 2 开始，将每个素数的倍数标记为非素数
/// 3. 收集所有标记为素数的数
pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    // 如果上限小于 2，没有素数
    if upper_bound < 2 {
        return Vec::new();
    }

    // 创建布尔数组，is_prime[i] 表示 i 是否为素数
    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    is_prime[0] = false; // 0 不是素数
    is_prime[1] = false; // 1 不是素数

    // 从 2 开始，标记所有非素数
    for p in 2..=upper_bound {
        if is_prime[p as usize] {
            // 如果 p 是素数，标记它的所有倍数为非素数
            // 从 p * p 开始，因为更小的倍数已经被之前的素数标记过了
            let mut multiple = p * p;
            while multiple <= upper_bound {
                is_prime[multiple as usize] = false;
                multiple += p;
            }
        }
    }

    // 收集所有素数
    is_prime
        .iter()                                    // 创建布尔数组的迭代器，遍历每个元素
        .enumerate()                               // 为每个元素添加索引，返回 (索引, 值) 元组
        .filter(|&(_, &prime)| prime)              // 过滤出值为 true 的元素（即素数），保留 (索引, true) 的元组
        .map(|(num, _)| num as u64)                // 将索引转换为 u64 类型，提取出素数本身
        .collect()                                 // 收集所有素数到 Vec<u64> 向量中
}
