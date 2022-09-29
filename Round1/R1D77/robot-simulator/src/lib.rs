// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn right(&self) -> Direction {
        match self {
            &Direction::North => Direction::East,
            &Direction::East => Direction::South,
            &Direction::South => Direction::West,
            &Direction::West => Direction::North,
        }
    }

    pub fn left(&self) -> Direction {
        match self {
            &Direction::North => Direction::West,
            &Direction::East => Direction::North,
            &Direction::South => Direction::East,
            &Direction::West => Direction::South,
        }
    }
}
pub struct Robot {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Robot { x, y, direction: d }
    }

    #[must_use]
    pub fn turn_right(mut self) -> Self {
        self.direction = self.direction.right();
        self
    }

    #[must_use]
    pub fn turn_left(mut self) -> Self {
        self.direction = self.direction.left();
        self
    }

    #[must_use]
    pub fn advance(mut self) -> Self {
        match self.direction {
            Direction::North => self.y += 1,
            Direction::East => self.x += 1,
            Direction::South => self.y -= 1,
            Direction::West => self.x -= 1,
        };

        self
    }

    #[must_use]
    pub fn instructions(mut self, instructions: &str) -> Self {
        for c in instructions.chars() {
            self = match c {
                'A' => self.advance(),
                'L' => self.turn_left(),
                'R' => self.turn_right(),
                _ => self,
            };
        }

        self
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
