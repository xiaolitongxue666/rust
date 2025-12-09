#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

use std::collections::{HashSet, VecDeque};

/// Solve the bucket problem
/// 
/// 使用 BFS（广度优先搜索）算法找到最少步数达到目标水量
/// 
/// # 算法思路
/// 1. 从初始状态开始（根据 start_bucket 装满对应的桶）
/// 2. 使用 BFS 搜索所有可能的状态
/// 3. 每次操作：装满、倒空、从一个桶倒到另一个桶
/// 4. 找到目标水量后返回结果
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // 边界检查：目标大于两个桶的容量
    if goal > capacity_1 && goal > capacity_2 {
        return None;
    }
    
    // 初始化状态：根据 start_bucket 装满对应的桶
    let initial_state = match start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };
    
    // 如果初始状态就已经达到目标
    if initial_state.0 == goal {
        return Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::One,
            other_bucket: initial_state.1,
        });
    }
    if initial_state.1 == goal {
        return Some(BucketStats {
            moves: 1,
            goal_bucket: Bucket::Two,
            other_bucket: initial_state.0,
        });
    }
    
    // BFS 队列：存储 (bucket1_amount, bucket2_amount, moves)
    let mut queue = VecDeque::new();
    queue.push_back((initial_state.0, initial_state.1, 1));
    
    // 记录已访问的状态，避免重复搜索
    let mut visited = HashSet::new();
    visited.insert(initial_state);
    
    // 记录找到的所有解：目标在起始桶中的解，和目标在非起始桶中的解
    let mut solution_in_start_bucket: Option<BucketStats> = None;
    let mut solution_in_other_bucket: Option<BucketStats> = None;
    
    // 确定起始桶是哪个
    let start_is_one = matches!(start_bucket, Bucket::One);
    
    while let Some((b1, b2, moves)) = queue.pop_front() {
        // 生成所有可能的下一个状态
        let next_states = vec![
            // 操作1：装满桶1
            (capacity_1, b2),
            // 操作2：装满桶2
            (b1, capacity_2),
            // 操作3：倒空桶1
            (0, b2),
            // 操作4：倒空桶2
            (b1, 0),
            // 操作5：从桶1倒到桶2
            {
                let pour = b1.min(capacity_2 - b2);
                (b1 - pour, b2 + pour)
            },
            // 操作6：从桶2倒到桶1
            {
                let pour = b2.min(capacity_1 - b1);
                (b1 + pour, b2 - pour)
            },
        ];
        
        // 处理每个新状态
        for (new_b1, new_b2) in next_states {
            // 规则：不能到达起始桶为空且另一个桶满的状态
            let is_invalid_state = match start_bucket {
                Bucket::One => new_b1 == 0 && new_b2 == capacity_2,
                Bucket::Two => new_b2 == 0 && new_b1 == capacity_1,
            };
            if is_invalid_state {
                continue; // 跳过这个无效状态
            }
            
            // 检查是否达到目标
            if new_b1 == goal {
                // 目标在桶1中
                if start_is_one {
                    // 目标在起始桶中，保存解（只保存第一个）
                    if solution_in_start_bucket.is_none() {
                        solution_in_start_bucket = Some(BucketStats {
                            moves: moves + 1,
                            goal_bucket: Bucket::One,
                            other_bucket: new_b2,
                        });
                    }
                } else {
                    // 目标在非起始桶中，保存为备选解（只保存第一个）
                    if solution_in_other_bucket.is_none() {
                        solution_in_other_bucket = Some(BucketStats {
                            moves: moves + 1,
                            goal_bucket: Bucket::One,
                            other_bucket: new_b2,
                        });
                    }
                }
            }
            
            if new_b2 == goal {
                // 目标在桶2中
                if !start_is_one {
                    // 目标在起始桶中，保存解（只保存第一个）
                    if solution_in_start_bucket.is_none() {
                        solution_in_start_bucket = Some(BucketStats {
                            moves: moves + 1,
                            goal_bucket: Bucket::Two,
                            other_bucket: new_b1,
                        });
                    }
                } else {
                    // 目标在非起始桶中，保存为备选解（只保存第一个）
                    if solution_in_other_bucket.is_none() {
                        solution_in_other_bucket = Some(BucketStats {
                            moves: moves + 1,
                            goal_bucket: Bucket::Two,
                            other_bucket: new_b1,
                        });
                    }
                }
            }
            
            // 如果状态未访问过，加入队列
            let new_state = (new_b1, new_b2);
            if !visited.contains(&new_state) {
                visited.insert(new_state);
                queue.push_back((new_b1, new_b2, moves + 1));
            }
        }
    }
    
    // 优先返回目标在起始桶中的解，如果找不到，返回目标在另一个桶中的解
    solution_in_start_bucket.or(solution_in_other_bucket)
}
