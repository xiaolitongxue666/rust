//! # hashmap! 宏
//!
//! 类似 vec!，提供 HashMap 的字面量构造语法。
//! 用法：hashmap!(k => v, ...) 或 hashmap!()
//!
//! ## 考点
//! - macro_rules! 声明宏
//! - 重复模式 $(...)* 与 $(...)+
//! - 使用 ::std::collections::HashMap 避免用户覆盖

#[macro_export]
macro_rules! hashmap {
    // 空 map
    () => {
        ::std::collections::HashMap::new()
    };
    // 带尾逗号：递归展开去掉逗号
    ($($key:expr => $value:expr,)+) => {
        $crate::hashmap!($($key => $value),+)
    };
    // 一个或多个 k=>v
    ($($key:expr => $value:expr),*) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $value);
            )*
            _map
        }
    };
}

/// 编译失败测试的占位模块
pub mod compile_fail_tests;
