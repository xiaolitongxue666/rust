//! # Matrix
//!
//! 解析矩阵字符串，支持按行、按列提取。行和列均为 1-indexed。
//!
//! ## 考点
//! - `str::lines()`、`split_whitespace()` 解析
//! - `parse::<u32>()` 与 `Result` 处理
//! - 迭代器 `map`、`filter_map`、`collect`

pub struct Matrix {
    /// 按行存储：rows[row_idx] = 该行元素
    rows: Vec<Vec<u32>>,
}

impl Matrix {
    /// 从字符串解析矩阵，格式为 "1 2 3\n4 5 6"
    pub fn new(input: &str) -> Self {
        let rows: Vec<Vec<u32>> = input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();
        Matrix { rows }
    }

    /// 获取第 row_no 行（1-indexed）
    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        if row_no == 0 || row_no > self.rows.len() {
            return None;
        }
        Some(self.rows[row_no - 1].clone())
    }

    /// 获取第 col_no 列（1-indexed）
    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        if col_no == 0 || self.rows.is_empty() || col_no > self.rows[0].len() {
            return None;
        }
        Some(
            self.rows
                .iter()
                .map(|row| row[col_no - 1])
                .collect(),
        )
    }
}
