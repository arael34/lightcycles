use termion::color::Color;

pub enum Direction {
    Left, Right,
    Up, Down
}

// Simple Point datatype to store a position on the screen
// and direction. 
pub struct Point {
    pub pos: (u16, u16),
    pub color: u8,
    direction: Direction,
}

impl Point {
    pub fn new( pos: (u16, u16), color: u8, direction: Direction ) -> Self {
        Point { pos, color, direction }
    }
    pub fn step(&mut self) {
        match &self.direction {
            Direction::Left => self.pos.0 -= 1,
            Direction::Right => self.pos.0 += 1,
            Direction::Up => self.pos.1 -= 1,
            Direction::Down => self.pos.1 += 1,
        }

        // randomly change direction
        todo!()
    }
}
