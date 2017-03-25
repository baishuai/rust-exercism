// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(PartialEq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    pos: (isize, isize),
    dir: Direction,
}

impl Robot {
    #[allow(unused_variables)]
    pub fn new(x: isize, y: isize, d: Direction) -> Self {
        Robot {
            pos: (x, y),
            dir: d,
        }
    }

    pub fn turn_right(self) -> Self {
        let new_dir = match self.dir {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Robot {
            pos: self.pos,
            dir: new_dir,
        }
    }

    pub fn turn_left(self) -> Self {
        let new_dir = match self.dir {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Robot {
            pos: self.pos,
            dir: new_dir,
        }
    }

    pub fn advance(self) -> Self {
        let (mut nx, mut ny) = self.pos;
        match self.dir {
            Direction::North => ny += 1,
            Direction::East => nx += 1,
            Direction::South => ny -= 1,
            Direction::West => nx -= 1,
        };
        Robot {
            pos: (nx, ny),
            dir: self.dir,
        }
    }

    #[allow(unused_variables)]
    pub fn instructions(self, instructions: &str) -> Self {
        instructions.chars()
            .fold(self, |robot, c| match c {
                'A' => robot.advance(),
                'L' => robot.turn_left(),
                'R' => robot.turn_right(),
                _ => robot,
            })
    }

    pub fn position(&self) -> (isize, isize) {
        self.pos
    }

    pub fn direction(&self) -> &Direction {
        &self.dir
    }
}
