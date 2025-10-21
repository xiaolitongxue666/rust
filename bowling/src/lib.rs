#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug)]
pub struct BowlingGame {
    rolls: Vec<u16>,
    current_frame: usize,
    current_roll_in_frame: usize,
    game_complete: bool,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            rolls: Vec::new(),
            current_frame: 0,
            current_roll_in_frame: 0,
            game_complete: false,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // 检查游戏是否已完成
        if self.game_complete {
            return Err(Error::GameComplete);
        }

        // 检查单次投掷的分数是否有效
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // 检查当前帧的分数是否有效（除了第10帧）
        if self.current_frame < 9 {
            if self.current_roll_in_frame == 0 {
                // 第一次投掷
                if pins == 10 {
                    // 全中，跳到下一帧
                    self.rolls.push(pins);
                    self.current_frame += 1;
                    self.current_roll_in_frame = 0;
                } else {
                    // 非全中，继续当前帧
                    self.rolls.push(pins);
                    self.current_roll_in_frame = 1;
                }
            } else {
                // 第二次投掷
                let first_roll = self.rolls[self.rolls.len() - 1];
                if first_roll + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }
                self.rolls.push(pins);
                self.current_frame += 1;
                self.current_roll_in_frame = 0;
            }
        } else {
            // 第10帧特殊处理
            self.rolls.push(pins);

            if self.current_roll_in_frame == 0 {
                // 第10帧第一次投掷
                if pins == 10 {
                    // 全中，需要额外两次投掷
                    self.current_roll_in_frame = 1;
                } else {
                    self.current_roll_in_frame = 1;
                }
            } else if self.current_roll_in_frame == 1 {
                // 第10帧第二次投掷
                let first_roll = self.rolls[self.rolls.len() - 2];
                if first_roll == 10 {
                    // 第一次是全中，第二次投掷
                    self.current_roll_in_frame = 2;
                } else {
                    // 第一次不是全中
                    if first_roll + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    if first_roll + pins == 10 {
                        // 补中，需要额外一次投掷
                        self.current_roll_in_frame = 2;
                    } else {
                        // 开放帧，游戏结束
                        self.game_complete = true;
                    }
                }
            } else {
                // 第10帧第三次投掷（额外投掷）
                let first_roll = self.rolls[self.rolls.len() - 3];
                let second_roll = self.rolls[self.rolls.len() - 2];

                if first_roll == 10 {
                    // 第一次全中
                    if second_roll == 10 {
                        // 第二次也是全中，第三次可以是任何分数
                        self.game_complete = true;
                    } else {
                        // 第二次不是全中
                        if second_roll + pins > 10 {
                            return Err(Error::NotEnoughPinsLeft);
                        }
                        self.game_complete = true;
                    }
                } else {
                    // 第一次不是全中，第二次是补中
                    self.game_complete = true;
                }
            }
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if !self.game_complete {
            return None;
        }

        let mut total_score = 0;
        let mut roll_index = 0;

        // 处理前9帧
        for _frame in 0..9 {
            if roll_index >= self.rolls.len() {
                return None;
            }

            let first_roll = self.rolls[roll_index];

            if first_roll == 10 {
                // 全中：10 + 接下来两次投掷的分数
                if roll_index + 2 >= self.rolls.len() {
                    return None;
                }
                total_score += 10 + self.rolls[roll_index + 1] + self.rolls[roll_index + 2];
                roll_index += 1;
            } else {
                // 非全中
                if roll_index + 1 >= self.rolls.len() {
                    return None;
                }
                let second_roll = self.rolls[roll_index + 1];

                if first_roll + second_roll == 10 {
                    // 补中：10 + 下一次投掷的分数
                    if roll_index + 2 >= self.rolls.len() {
                        return None;
                    }
                    total_score += 10 + self.rolls[roll_index + 2];
                } else {
                    // 开放帧
                    total_score += first_roll + second_roll;
                }
                roll_index += 2;
            }
        }

        // 处理第10帧
        if roll_index < self.rolls.len() {
            // 第10帧的分数就是所有剩余投掷的总和
            for i in roll_index..self.rolls.len() {
                total_score += self.rolls[i];
            }
        }

        Some(total_score)
    }
}
