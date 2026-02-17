//! 高分榜：最新分、最高分、前三名
//!
//! 考点：生命周期 'a、切片引用、last/max、sort_by 降序、truncate

#[derive(Debug)]
pub struct HighScores<'a> {
    high_scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores {
            high_scores: scores,
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.high_scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.high_scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.high_scores.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut scores = self.high_scores.to_vec();
        scores.sort_by(|a, b| b.cmp(a));
        scores.truncate(3);
        scores
    }
}
