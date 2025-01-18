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
    fn turn_right(&self) -> Self {
        let v: (i8, i8) = self.into();
        (v.1, -v.0).try_into().unwrap()       
    }

    fn turn_left(&self) -> Self {
        let v: (i8, i8) = self.into();
        (-v.1, v.0).try_into().unwrap()
    }
}

impl Into<(i8, i8)> for &Direction {
    fn into(self) -> (i8, i8) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (1, 0),
            Direction::South => (0, -1),
            Direction::West => (-1, 0),
        }
    }
}

impl TryFrom<(i8, i8)> for Direction {
    type Error=String;
    fn try_from(value: (i8, i8)) -> Result<Self, Self::Error> {
        match value {
            (0, 1) => Ok(Direction::North),
            (1, 0) => Ok(Direction::East),
            (0, -1) => Ok(Direction::South),
            (-1, 0) => Ok(Direction::West),
            _ => Err("Invalid direction".to_string()),
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
    pub fn turn_right(self) -> Self {
        Robot { x: self.x, y: self.y, direction: self.direction.turn_right() }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        Robot { x: self.x, y: self.y, direction: self.direction.turn_left() }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let dir_v: (i8, i8) = (&self.direction).into();
        Robot { x: self.x + dir_v.0 as i32, y: self.y + dir_v.1 as i32, direction: self.direction }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut r = self;
        for instruction in instructions.chars() {
            r = match instruction {
                'A' => r.advance(),
                'L' => r.turn_left(),
                'R' => r.turn_right(),
                _ => r,
            }
        }
        r
    }


    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}
