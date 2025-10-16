pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0;
    let mut n = display_value;

    // Brian Kernighan 算法：n & (n-1) 会移除最右边的 1
    while n != 0 {
        count += 1;
        n &= n - 1;
    }

    count
}
