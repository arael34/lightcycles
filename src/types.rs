use termion::color;

pub enum Direction {
    Left, Right,
    Up, Down
}

// Simple Point datatype to store a position on the screen
// and direction. 
pub struct Point {
    pub pos: (u16, u16),
    pub color: color::Rgb,
    direction: Direction,
}

impl Point {
    pub fn new( pos: (u16, u16), color: color::Rgb, direction: Direction ) -> Self {
        Point { pos, color, direction }
    }
    pub fn step(&mut self, width: u16, height: u16) {
        match &self.direction {
            Direction::Left => {
                if self.pos.0 > 1 { self.pos.0 -= 1; }
                else { self.direction = Direction::Right; }
            },
            Direction::Right => {
                if self.pos.0 < width - 1 { self.pos.0 += 1; }
                else { self.direction = Direction::Left }
            },
            Direction::Up => {
                if self.pos.1 > 1 { self.pos.1 -= 1; }
                else { self.direction = Direction::Down; }
            },
            Direction::Down => {
                if self.pos.1 <= height { self.pos.1 += 1; }
                else { self.direction = Direction::Up; }
            },
        }

        // randomly change direction
        // todo!()
    }
}
