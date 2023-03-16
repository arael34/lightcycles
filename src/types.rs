pub enum Direction {
    Left, Right,
    Up, Down
}

// Simple Point datatype to store a position on the screen
// and direction. 
pub struct Point {
    pub pos: (u16, u16),
    direction: Direction,
}

impl Point {
    pub fn new( x: u16, y: u16, direction: Direction ) -> Self {
        Point { pos: (x, y), direction }
    }
}
