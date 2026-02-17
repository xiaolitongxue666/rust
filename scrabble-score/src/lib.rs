//! 拼字游戏得分：按字母表查分并求和
//!
//! 考点：filter_map、match 多模式、Iterator::sum

/// 计算单词的 Scrabble 得分
///
/// 计算单词的 Scrabble 得分。每个字母根据其价值得分：
/// - 1 分：A, E, I, O, U, L, N, R, S, T
/// - 2 分：D, G
/// - 3 分：B, C, M, P
/// - 4 分：F, H, V, W, Y
/// - 5 分：K
/// - 8 分：J, X
/// - 10 分：Q, Z
///
/// 非英文字母不得分（0 分）
pub fn score(word: &str) -> u64 {
    word.chars()
        .filter_map(|c| {
            // 只处理 ASCII 字母（A-Z, a-z）
            if c.is_ascii_alphabetic() {
                let upper = c.to_ascii_uppercase();
                Some(match upper {
                    'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
                    'D' | 'G' => 2,
                    'B' | 'C' | 'M' | 'P' => 3,
                    'F' | 'H' | 'V' | 'W' | 'Y' => 4,
                    'K' => 5,
                    'J' | 'X' => 8,
                    'Q' | 'Z' => 10,
                    _ => 0,
                })
            } else {
                None // 非 ASCII 字母不参与计分
            }
        })
        .sum()
}
