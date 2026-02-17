#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..=7).contains(&rank) {
            return None;
        }
        if !(0..=7).contains(&file) {
            return None;
        }
        Some(ChessPosition { rank, file })
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    /// 判断当前皇后是否可以攻击另一个皇后
    ///
    /// 皇后可以攻击：
    /// - 同一行的所有位置（rank 相同）
    /// - 同一列的所有位置（file 相同）
    /// - 同一对角线的所有位置（两个对角线方向）
    pub fn can_attack(&self, other: &Queen) -> bool {
        let self_rank = self.position.rank;
        let self_file = self.position.file;
        let other_rank = other.position.rank;
        let other_file = other.position.file;

        // 同一行
        if self_rank == other_rank {
            return true;
        }

        // 同一列
        if self_file == other_file {
            return true;
        }

        // 同一对角线：主对角线（左上到右下）或副对角线（右上到左下）
        // 主对角线：rank - file 相同
        // 副对角线：rank + file 相同
        (self_rank - self_file) == (other_rank - other_file)
            || (self_rank + self_file) == (other_rank + other_file)
    }
}
