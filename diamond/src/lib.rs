/// 生成钻石形状的字符串向量
/// 
/// # 参数
/// * `c` - 钻石最宽处的字母（如 'C' 会生成从 'A' 到 'C' 的钻石）
/// 
/// # 返回
/// 包含钻石每一行的字符串向量
pub fn get_diamond(c: char) -> Vec<String> {
    let letter = c.to_ascii_uppercase();
    
    // 特殊情况：如果输入是 'A'，直接返回
    if letter == 'A' {
        return vec!["A".to_string()];
    }
    
    // 计算字母索引（A=0, B=1, C=2, ...）
    let letter_index = (letter as u8 - b'A') as usize;
    
    // 总行数 = 2 * (letter_index + 1) - 1 = 2 * letter_index + 1
    let total_rows = 2 * letter_index + 1;
    
    let mut diamond = Vec::new();
    
    for row in 0..total_rows {
        let mut line = String::new();
        
        // 确定当前行应该显示的字母
        let current_letter_index = if row <= letter_index {
            row
        } else {
            2 * letter_index - row
        };
        let current_letter = (b'A' + current_letter_index as u8) as char;
        
        // 计算左侧空格数
        let left_spaces = letter_index - current_letter_index;
        
        // 添加左侧空格
        for _ in 0..left_spaces {
            line.push(' ');
        }
        
        // 添加字母
        line.push(current_letter);
        
        // 如果不是 'A'，需要添加中间空格和右侧字母
        if current_letter_index > 0 {
            // 中间空格数 = 2 * current_letter_index - 1
            let middle_spaces = 2 * current_letter_index - 1;
            for _ in 0..middle_spaces {
                line.push(' ');
            }
            // 添加右侧字母
            line.push(current_letter);
        }
        
        // 添加右侧空格（与左侧对称）
        for _ in 0..left_spaces {
            line.push(' ');
        }
        
        diamond.push(line);
    }
    
    diamond
}
