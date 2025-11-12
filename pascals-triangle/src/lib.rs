/// 杨辉三角结构体
pub struct PascalsTriangle {
    /// 行数
    // row_count: u32,
    /// 存储所有行的数据
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    /// 创建一个新的杨辉三角
    /// 
    /// # 参数
    /// * `row_count` - 要生成的行数
    /// 
    /// # 返回
    /// 返回包含指定行数的杨辉三角实例
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::new();
        // 生成每一行
        for i in 0..row_count {
            let mut row = Vec::new();
            // 生成当前行的每个元素
            for j in 0..=i {
                // 每行的首尾元素都是1
                if j == 0 || j == i {
                    row.push(1);
                } else {
                    // 中间元素等于上一行对应位置和前一个位置的和
                    let prev_row: &Vec<u32> = &rows[(i - 1) as usize];
                    row.push(prev_row[(j - 1) as usize] + prev_row[j as usize]);
                }
            }
            rows.push(row);
        }
        Self {
            // row_count,
            rows,
        }
    }

    /// 获取所有行的数据
    /// 
    /// # 返回
    /// 返回包含所有行的向量的副本
    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
