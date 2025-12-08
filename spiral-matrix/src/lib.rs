/// 生成螺旋矩阵
/// 
/// 创建一个 n×n 的矩阵，从左上角开始按顺时针方向螺旋填充数字 1 到 n²
/// 
/// # 示例
/// 
/// size = 3 时返回：
/// - 第一行: [1, 2, 3]
/// - 第二行: [8, 9, 4]
/// - 第三行: [7, 6, 5]
pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    if size == 0 {
        return Vec::new();
    }

    let n = size as usize;
    let mut matrix = vec![vec![0; n]; n];
    
    // 定义四个方向：右、下、左、上
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut dir_index = 0;
    
    let mut row = 0i32;
    let mut col = 0i32;
    let mut num = 1u32;
    
    while num <= size * size {
        matrix[row as usize][col as usize] = num;
        num += 1;
        
        // 计算下一个位置
        let (dr, dc) = directions[dir_index];
        let next_row = row + dr;
        let next_col = col + dc;
        
        // 检查是否需要改变方向（超出边界或已填充）
        if next_row < 0 || next_row >= n as i32 || 
           next_col < 0 || next_col >= n as i32 ||
           matrix[next_row as usize][next_col as usize] != 0 {
            // 改变方向
            dir_index = (dir_index + 1) % 4;
            let (dr, dc) = directions[dir_index];
            row += dr;
            col += dc;
        } else {
            // 继续当前方向
            row = next_row;
            col = next_col;
        }
    }
    
    matrix
}
