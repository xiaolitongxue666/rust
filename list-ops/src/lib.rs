//! 列表操作：用迭代器实现 append、concat、filter、map、fold、reverse
//!
//! 考点：Iterator、impl Iterator、std::iter::from_fn、闭包 move、Option::or_else、
//! DoubleEndedIterator::rev、fold 左/右结合

/// Yields each item of a and then each item of b
pub fn append<I, J>(mut a: I, mut b: J) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    J: Iterator<Item = I::Item>,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    /*
    std::iter::from_fn 需要的闭包类型是 FnMut() -> Option<T>，也就是：
    参数：0 个
    返回值：Option<T>

    .or_else(|| b.next())
    Option 的方法，当当前值是 None 时，执行闭包并返回其结果：
    若 a.next() 是 Some(x) → 直接返回 Some(x)，不执行闭包
    若 a.next() 是 None → 执行 || b.next()，返回 b.next() 的结果
    */
    std::iter::from_fn(move || a.next().or_else(|| b.next()))
}

/// 展平嵌套迭代器。考点：loop + Option 状态机、ref mut 借用
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut outer = nested_iter;
    let mut cur: Option<I::Item> = None;

    std::iter::from_fn(move || {
        loop {
            if let Some(ref mut inner) = cur {
                if let Some(val) = inner.next() {
                    return Some(val);
                }
                cur = None;
            }
            if let Some(inner) = outer.next() {
                cur = Some(inner);
            } else {
                return None;
            }
        }
    })
}

/// 过滤。考点：Iterator::find 替代手写循环
pub fn filter<I, F>(mut _iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || _iter.find(|item| _predicate(item)))
}

/// 长度。考点：Iterator::count
pub fn length<I: Iterator>(mut _iter: I) -> usize {
    _iter.count()
}

/// 映射。考点：Option::map 配合闭包引用 &_function
pub fn map<I, F, U>(mut mut_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || mut_iter.next().map(&_function))
}

/// 左折叠。考点：Iterator::fold
pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    iter.fold(initial, function)
}

/// 右折叠。考点：DoubleEndedIterator::rev + fold
pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    iter.rev().fold(initial, function)
}

/// 反转。考点：DoubleEndedIterator::rev
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    iter.rev()
}
