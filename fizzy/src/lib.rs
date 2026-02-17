//! # Fizzy (泛型 FizzBuzz)
//!
//! 可自定义规则的 FizzBuzz，支持泛型类型（含 f64、自定义类型）。
//! 每个 Matcher 包含判断函数和替换字符串；Fizzy 按顺序应用所有 Matcher。
//!
//! ## 考点
//! - 泛型与 trait bounds：`T: Copy + ToString`
//! - `impl Iterator<Item = String>` 返回迭代器
//! - `Box<dyn Fn(T) -> bool>` 存储闭包
//! - 多 trait 实现支持不同数值类型

use std::ops::Rem;

type MatchFn<T> = Box<dyn Fn(T) -> bool>;

pub struct Matcher<T> {
    matcher: MatchFn<T>,
    subs: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: AsRef<str>,
    {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.as_ref().to_string(),
        }
    }
}

pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T>
where
    T: Copy + ToString,
{
    pub fn new() -> Self {
        Fizzy(Vec::new())
    }

    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    fn apply_to(&self, item: T) -> String {
        let mut out = String::new();
        for m in &self.0 {
            if (m.matcher)(item) {
                out.push_str(&m.subs);
            }
        }
        if out.is_empty() {
            out = item.to_string();
        }
        out
    }

    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |item| self.apply_to(item))
    }
}

impl<T: Copy + ToString> Default for Fizzy<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// 支持 FizzBuzz 的类型：需 Rem、PartialEq、零值、3 和 5
pub trait FizzBuzzNum:
    Copy + ToString + Rem<Output = Self> + PartialEq + 'static
{
    fn zero() -> Self;
    fn three() -> Self;
    fn five() -> Self;
}

/// 对满足 From<u8>+Default 的类型做 blanket impl（i32, u8, u64, f64, Fizzable 等）
impl<T> FizzBuzzNum for T
where
    T: Copy + ToString + Rem<Output = T> + PartialEq + From<u8> + Default + 'static,
{
    fn zero() -> Self {
        T::default()
    }
    fn three() -> Self {
        3u8.into()
    }
    fn five() -> Self {
        5u8.into()
    }
}

pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: FizzBuzzNum,
{
    let three = T::three();
    let five = T::five();
    let zero = T::zero();

    Fizzy(vec![
        Matcher::new(move |n: T| n % three == zero, "fizz"),
        Matcher::new(move |n: T| n % five == zero, "buzz"),
    ])
}
