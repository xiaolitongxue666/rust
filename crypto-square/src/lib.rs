/// 使用密码方阵加密文本
///
/// # 步骤
/// 1. 规范化：去除空格和标点，转为小写
/// 2. 计算矩形尺寸：找到满足条件的列数 c 和行数 r
/// 3. 按列读取字符
/// 4. 输出：每 c 组，每组 r 个字符，用空格分隔，不足的用空格填充
pub fn encrypt(input: &str) -> String {
    // 规范化：去除空格和标点，转为小写
    let normalized: String = input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_lowercase())
        .collect();

    let length = normalized.len();

    // 空字符串直接返回
    if length == 0 {
        return String::new();
    }

    // 计算矩形尺寸：找到满足条件的 c 和 r
    // 条件：r * c >= length, c >= r, c - r <= 1
    // 从 r = floor(sqrt(length)) 开始，找到满足条件的最小 c
    let sqrt_len = (length as f64).sqrt();
    let mut rows = sqrt_len.floor() as usize;
    let mut cols = ((length as f64) / (rows as f64)).ceil() as usize;

    // 确保满足 c >= r 且 c - r <= 1
    if cols < rows {
        cols = rows;
    }

    // 如果 c - r > 1，尝试增加 r
    while cols - rows > 1 {
        rows += 1;
        cols = ((length as f64) / (rows as f64)).ceil() as usize;
        // 确保 c >= r
        if cols < rows {
            cols = rows;
        }
    }

    // 确保 r * c >= length
    while rows * cols < length {
        cols += 1;
    }

    // 将字符串转为字符向量以提高效率
    let chars: Vec<char> = normalized.chars().collect();

    // 按列读取字符，每列组成一组
    let mut result = String::new();
    for col in 0..cols {
        // 每组之间用空格分隔（除了第一组）
        if col > 0 {
            result.push(' ');
        }

        // 读取当前列的所有行
        for row in 0..rows {
            let index = row * cols + col;
            if index < length {
                result.push(chars[index]);
            } else {
                // 如果超出范围，填充空格
                result.push(' ');
            }
        }
    }

    result
}
