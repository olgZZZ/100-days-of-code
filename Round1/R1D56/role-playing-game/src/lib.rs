// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
// #![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        if self.health >= 1 {
            None
        } else {
            if self.level >= 10 {
                let player = Player {
                    health: 100,
                    mana: Some(100),
                    level: self.level,
                };
                Some(player)
            } else {
                let player = Player {
                    health: 100,
                    mana: None,
                    level: self.level,
                };
                Some(player)
            }
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if let Some(mana) = self.mana {
            if mana_cost > mana {
                return 0;
            } else {
                self.mana = Some(mana - mana_cost);
                return mana_cost * 2;
            }
        } else {
            if self.health < mana_cost {
                self.health = 0;
                return 0;
            } else {
                self.health -= mana_cost;
                return 0;
            }
        }
    }
}
