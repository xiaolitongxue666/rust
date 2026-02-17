// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        // unimplemented!("Revive this player")
        if self.health > 0 {
            None
        } else {
            if self.level >= 10 {
                Some(Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                })
            } else {
                Some(Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                })
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        // unimplemented!("Cast a spell of cost {}", mana_cost)
        match self.mana {
            Some(mana) => {
                if mana < mana_cost {
                    0
                } else {
                    let temp_u32 = self.mana.clone().unwrap() - mana_cost;
                    self.mana = Some(temp_u32);
                    mana_cost * 2
                }
            }
            None => {
                if mana_cost > self.health {
                    self.health = 0;
                } else {
                    self.health -= mana_cost;
                }
                0
            }
        }
    }
}
