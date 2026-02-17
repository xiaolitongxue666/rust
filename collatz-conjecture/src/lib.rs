//! 考拉兹猜想：偶数除 2，奇数 3n+1，统计到 1 的步数
//!
//! 考点：Option、checked_mul/checked_add 防溢出、is_multiple_of

pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut count = 0;
    let mut current = n;

    while current != 1 {
        count += 1;
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = current.checked_mul(3)?.checked_add(1)?;
        }
    }

    Some(count)
}
