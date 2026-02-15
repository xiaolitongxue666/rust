//! # Yacht (快艇骰子)
//!
//! 骰子游戏计分：五个骰子根据所选类别计分。
//! 类别包括：Ones-Sixes（点数×个数）、Full House（三带二）、Four of a Kind、
//! Little/Big Straight（1-5 或 2-6）、Choice（总和）、Yacht（五同=50）。
//!
//! ## 考点
//! - `match` 模式匹配
//! - `HashMap` 或数组统计点数频率
//! - 切片排序与比较

use std::collections::HashMap;

#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

/// 统计各点数出现次数
fn count_dice(dice: &Dice) -> HashMap<u8, u8> {
    let mut counts = HashMap::new();
    for &d in dice {
        *counts.entry(d).or_insert(0) += 1;
    }
    counts
}

/// 检查是否为 1-2-3-4-5
fn is_little_straight(dice: &Dice) -> bool {
    let mut sorted: Vec<u8> = dice.to_vec();
    sorted.sort_unstable();
    sorted == [1, 2, 3, 4, 5]
}

/// 检查是否为 2-3-4-5-6
fn is_big_straight(dice: &Dice) -> bool {
    let mut sorted: Vec<u8> = dice.to_vec();
    sorted.sort_unstable();
    sorted == [2, 3, 4, 5, 6]
}

pub fn score(dice: Dice, category: Category) -> u8 {
    let counts = count_dice(&dice);

    match category {
        Category::Ones => dice.iter().filter(|&&d| d == 1).sum::<u8>(),
        Category::Twos => dice.iter().filter(|&&d| d == 2).sum::<u8>(),
        Category::Threes => dice.iter().filter(|&&d| d == 3).sum::<u8>(),
        Category::Fours => dice.iter().filter(|&&d| d == 4).sum::<u8>(),
        Category::Fives => dice.iter().filter(|&&d| d == 5).sum::<u8>(),
        Category::Sixes => dice.iter().filter(|&&d| d == 6).sum::<u8>(),
        Category::FullHouse => {
            let mut groups: Vec<_> = counts.values().copied().collect();
            groups.sort_by(|a, b| b.cmp(a));
            if groups == [3, 2] {
                dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            let mut result = 0;
            for (&val, &cnt) in &counts {
                if cnt >= 4 {
                    result = val * 4;
                    break;
                }
            }
            result
        }
        Category::LittleStraight => {
            if is_little_straight(&dice) {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if is_big_straight(&dice) {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if counts.len() == 1 {
                50
            } else {
                0
            }
        }
    }
}
