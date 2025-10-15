pub fn annotate(garden: &[&str]) -> Vec<String> {
    let rows = garden.len();
    if rows == 0 {
        return vec![];
    }
    let cols = garden[0].chars().count();

    garden
        .iter()
        .enumerate()
        .map(|(row, &line)| {
            let mut new_row = String::with_capacity(cols);
            for col in 0..cols {
                let ch = line.chars().nth(col).unwrap_or(' ');
                if ch == '*' {
                    new_row.push('*');
                } else {
                    // 计算周围花的数量
                    let mut count = 0;
                    // 检查 8 个方向：上、下、左、右、四个对角线
                    for dr in -1..=1 {
                        for dc in -1..=1 {
                            if dr == 0 && dc == 0 {
                                continue;
                            }
                            let new_row = row as i32 + dr;
                            let new_col = col as i32 + dc;
                            // 确保不越界
                            if new_row >= 0
                                && new_row < rows as i32
                                && new_col >= 0
                                && new_col < cols as i32
                            {
                                let neighbor = garden[new_row as usize]
                                    .chars()
                                    .nth(new_col as usize)
                                    .unwrap_or(' ');
                                if neighbor == '*' {
                                    count += 1;
                                }
                            }
                        }
                    }
                    new_row.push(if count == 0 { ' ' } else { (count as u8 + b'0') as char });
                }
            }
            new_row
        })
        .collect()
}