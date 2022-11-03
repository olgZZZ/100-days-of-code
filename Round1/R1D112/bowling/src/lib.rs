#[derive(Debug, PartialEq, Eq)]

pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: Vec<(u16, u16, bool)>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { rolls: Vec::new() }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        let length = self.rolls.len();

        if length == 0 {
            self.rolls.push((pins, 0, pins == 10));

            return Ok(());
        }

        match (self.rolls[length - 1], length) {
            ((x, 0, false), _) if x + pins > 10 => return Err(Error::NotEnoughPinsLeft),

            // 10th is neither spare or strike
            ((x, y, true), 10) if x + y < 10 => return Err(Error::GameComplete),

            // 10th strike but 11th isnt
            ((x, _, true), 11) if x != 10 => return Err(Error::GameComplete),

            // 10th is spare
            ((x, y, true), 10) if x + y == 10 && x != 10 => self.rolls.push((pins, 0, true)),

            // new frame
            ((_, _, true), _) => self.rolls.push((pins, 0, pins == 10)),

            // finish frame
            ((x, 0, false), _) => self.rolls[length - 1] = (x, pins, true),

            _ => unreachable!(),
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        let length = self.rolls.len();

        if length < 10 {
            return None;
        }

        // Last strike of spare not finished

        let ten = self.rolls[9];

        if ten == (10, 0, true) && (length < 11 || (length < 12 && self.rolls[10] == (10, 0, true)))
            || ten.0 + ten.1 == 10 && length < 11
        {
            return None;
        }

        // Fill to 12 frames to properly allign windows, since only the first roll is handled

        let mut copy = self.rolls.clone();

        while copy.len() < 12 {
            copy.push((0, 0, true));
        }

        Some(copy.windows(3).fold(0, |acc, roll| {
            acc + match (roll[0], roll[1]) {
                ((10, _, _), (10, _, _)) => 20 + roll[2].0,

                ((10, _, _), (x, y, _)) => 10 + x + y,

                ((x, y, _), (z, _, _)) if x + y == 10 => 10 + z,

                _ => roll[0].0 + roll[0].1,
            }
        }))
    }
}
