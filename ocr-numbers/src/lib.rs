//! # OCR Numbers - 3×4 ASCII 网格数字识别
//!
//! 每个数字占 3 列×4 行，用 | _ 空格 表示。多行数字用逗号分隔。
//!
//! ## 考点
//! - 模式匹配、chunks
//! - 错误类型 InvalidRowCount/InvalidColumnCount

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    for line in &lines {
        if line.chars().count() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.chars().count()));
        }
    }

    let mut result = Vec::new();
    for chunk in lines.chunks(4) {
        let digit_count = chunk[0].chars().count() / 3;
        let mut digits = String::new();
        for i in 0..digit_count {
            let mut chars = Vec::new();
            for row in chunk {
                let row_chars: Vec<char> = row.chars().collect();
                for c in row_chars.chunks(3).nth(i).unwrap_or(&[]) {
                    chars.push(*c);
                }
            }
            digits.push(match_digit(&chars));
        }
        result.push(digits);
    }
    Ok(result.join(","))
}

fn match_digit(chars: &[char]) -> char {
    const D0: [char; 12] = [' ', '_', ' ', '|', ' ', '|', '|', '_', '|', ' ', ' ', ' '];
    const D1: [char; 12] = [' ', ' ', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '];
    const D2: [char; 12] = [' ', '_', ' ', ' ', '_', '|', '|', '_', ' ', ' ', ' ', ' '];
    const D3: [char; 12] = [' ', '_', ' ', ' ', '_', '|', ' ', '_', '|', ' ', ' ', ' '];
    const D4: [char; 12] = [' ', ' ', ' ', '|', '_', '|', ' ', ' ', '|', ' ', ' ', ' '];
    const D5: [char; 12] = [' ', '_', ' ', '|', '_', ' ', ' ', '_', '|', ' ', ' ', ' '];
    const D6: [char; 12] = [' ', '_', ' ', '|', '_', ' ', '|', '_', '|', ' ', ' ', ' '];
    const D7: [char; 12] = [' ', '_', ' ', ' ', ' ', '|', ' ', ' ', '|', ' ', ' ', ' '];
    const D8: [char; 12] = [' ', '_', ' ', '|', '_', '|', '|', '_', '|', ' ', ' ', ' '];
    const D9: [char; 12] = [' ', '_', ' ', '|', '_', '|', ' ', '_', '|', ' ', ' ', ' '];

    if chars.len() == 12 {
        if chars == D0 {
            return '0';
        }
        if chars == D1 {
            return '1';
        }
        if chars == D2 {
            return '2';
        }
        if chars == D3 {
            return '3';
        }
        if chars == D4 {
            return '4';
        }
        if chars == D5 {
            return '5';
        }
        if chars == D6 {
            return '6';
        }
        if chars == D7 {
            return '7';
        }
        if chars == D8 {
            return '8';
        }
        if chars == D9 {
            return '9';
        }
    }
    '?'
}
