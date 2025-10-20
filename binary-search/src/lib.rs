/// 在已排序的数组中查找目标值的索引
/// 使用二分搜索算法，时间复杂度 O(log n)
///
/// # 参数
/// * `array` - 已排序的数组切片
/// * `key` - 要查找的目标值
///
/// # 返回值
/// * `Some(index)` - 找到目标值，返回其索引
/// * `None` - 目标值不存在于数组中
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    // 处理空数组的情况
    if array.is_empty() {
        return None;
    }

    // 使用迭代版本的二分搜索，避免递归栈溢出
    let mut left = 0;
    let mut right = array.len();

    // 当搜索区间不为空时继续搜索
    while left < right {
        // 计算中间位置，避免整数溢出
        let mid = left + (right - left) / 2;

        // 比较中间元素与目标值
        match array[mid].cmp(&key) {
            std::cmp::Ordering::Equal => {
                // 找到目标值，返回索引
                return Some(mid);
            }
            std::cmp::Ordering::Less => {
                // 中间元素小于目标值，搜索右半部分
                left = mid + 1;
            }
            std::cmp::Ordering::Greater => {
                // 中间元素大于目标值，搜索左半部分
                right = mid;
            }
        }
    }

    // 搜索区间为空，目标值不存在
    None
}
