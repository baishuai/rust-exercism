pub struct ChessPosition {
    x: i32,
    y: i32,
}

impl ChessPosition {
    pub fn new(x: i32, y: i32) -> Result<ChessPosition, ()> {
        if x < 0 || x >= 8 || y < 0 || y >= 8 {
            Err(())
        } else {
            Ok(ChessPosition { x: x, y: y })
        }
    }
}



// #[derive(Debug)]
pub struct Queen {
    pos: ChessPosition,
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen { pos: pos }
    }


    pub fn can_attack(&self, rhs: &Queen) -> bool {
        self.pos.x == rhs.pos.x || self.pos.y == rhs.pos.y ||
        self.pos.x + self.pos.y == rhs.pos.x + rhs.pos.y ||
        self.pos.x - self.pos.y == rhs.pos.x - rhs.pos.y
    }
}