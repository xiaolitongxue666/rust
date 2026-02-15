//! # Secret Handshake
//!
//! 将 1-31 的数字转换为秘密握手的动作序列。根据数字的二进制表示的最右五位确定动作：
//! 00001=wink, 00010=double blink, 00100=close your eyes, 01000=jump, 10000=反转顺序
//!
//! ## 考点
//! - 位运算：`&` 检查特定位、`>>` 移位
//! - `Vec::reverse()` 反转顺序

const ACTIONS: [&str; 4] = ["wink", "double blink", "close your eyes", "jump"];

/// 将数字转换为秘密握手动作序列
///
/// # 参数
/// * `n` - 输入数字 (0-31)，取低 5 位
///
/// # 算法
/// 从右到左检查 bit 0-3，对应 4 种动作；若 bit 4 为 1 则反转结果
pub fn actions(n: u8) -> Vec<&'static str> {
    let mut result = Vec::new();

    // 考点：位运算检查 bit 0-3
    for (i, &action) in ACTIONS.iter().enumerate() {
        if (n >> i) & 1 == 1 {
            result.push(action);
        }
    }

    // bit 4 (16)：反转顺序
    if (n >> 4) & 1 == 1 {
        result.reverse();
    }

    result
}
