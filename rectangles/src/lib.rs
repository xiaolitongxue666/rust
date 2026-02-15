//! # Rectangles - ASCII 图中统计矩形数量
//!
//! 矩形由 + 角、- 横边、| 竖边组成。枚举所有左上-右下角对，检查四边是否完整。
//!
//! ## 考点
//! - 二维网格遍历
//! - 边界检查

pub fn count(lines: &[&str]) -> u32 {
    if lines.is_empty() {
        return 0;
    }
    let grid: Vec<Vec<char>> = lines.iter().map(|s| s.chars().collect()).collect();
    let h = grid.len();
    let w = grid[0].len();
    if w == 0 {
        return 0;
    }

    let mut total = 0u32;
    for y1 in 0..h {
        for x1 in 0..w {
            if grid[y1][x1] != '+' {
                continue;
            }
            for y2 in (y1 + 1)..h {
                for x2 in (x1 + 1)..w {
                    if grid[y1][x2] != '+' || grid[y2][x1] != '+' || grid[y2][x2] != '+' {
                        continue;
                    }
                    let top_ok = (x1 + 1..x2).all(|x| grid[y1][x] == '-' || grid[y1][x] == '+');
                    let bot_ok = (x1 + 1..x2).all(|x| grid[y2][x] == '-' || grid[y2][x] == '+');
                    let left_ok = (y1 + 1..y2).all(|y| grid[y][x1] == '|' || grid[y][x1] == '+');
                    let right_ok = (y1 + 1..y2).all(|y| grid[y][x2] == '|' || grid[y][x2] == '+');
                    if top_ok && bot_ok && left_ok && right_ok {
                        total += 1;
                    }
                }
            }
        }
    }
    total
}
