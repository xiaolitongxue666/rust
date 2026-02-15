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

/// Combines all items in all nested iterators inside into one flattened iterator
pub fn concat<I>(nested_iter: I) -> impl Iterator<Item = <I::Item as Iterator>::Item>
where
    I: Iterator,
    I::Item: Iterator,
{
    let mut outer = nested_iter;
    let mut cur: Option<I::Item> = None;

    std::iter::from_fn(move || loop {
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
    })
}

/// Returns an iterator of all items in iter for which `predicate(item)` is true
pub fn filter<I, F>(mut _iter: I, _predicate: F) -> impl Iterator<Item = I::Item>
where
    I: Iterator,
    F: Fn(&I::Item) -> bool,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || _iter.find(|item| _predicate(item)))
}

pub fn length<I: Iterator>(mut _iter: I) -> usize {
    _iter.count()
}

/// Returns an iterator of the results of applying `function(item)` on all iter items
pub fn map<I, F, U>(mut mut_iter: I, _function: F) -> impl Iterator<Item = U>
where
    I: Iterator,
    F: Fn(I::Item) -> U,
{
    // this empty iterator silences a compiler complaint that
    // () doesn't implement Iterator
    std::iter::from_fn(move || mut_iter.next().map(&_function))
}

pub fn foldl<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: Iterator,
    F: Fn(U, I::Item) -> U,
{
    iter.fold(initial, function)
}

pub fn foldr<I, F, U>(iter: I, initial: U, function: F) -> U
where
    I: DoubleEndedIterator,
    F: Fn(U, I::Item) -> U,
{
    iter.rev().fold(initial, function)
}

/// Returns an iterator with all the original items, but in reverse order
pub fn reverse<I: DoubleEndedIterator>(iter: I) -> impl Iterator<Item = I::Item> {
    iter.rev()
}
