use termion::color::{self, Color};

fn color(n: u8) -> Box<dyn Color> {
    match n {
        0 => return Box::new(color::Black),
        1 => return Box::new(color::Red),
        2 => return Box::new(color::Green),
        3 => return Box::new(color::Yellow),

        4 => return Box::new(color::Blue),
        5 => return Box::new(color::Magenta),
        6 => return Box::new(color::Cyan),
        7 => return Box::new(color::White),

        8 => return Box::new(color::LightBlack),
        9 => return Box::new(color::LightRed),
        10 => return Box::new(color::LightGreen),
        11 => return Box::new(color::LightYellow),

        12 => return Box::new(color::LightBlue),
        13 => return Box::new(color::LightMagenta),
        14 => return Box::new(color::LightCyan),
        15 => return Box::new(color::LightWhite),

        _ => panic!("invalid color!")
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    Left, Right,
    Up, Down,
}

impl Direction {
    pub fn get_u8(&self) -> u8 {
        match self {
            Direction::Left => 0,
            Direction::Up => 1,
            Direction::Right => 2,
            Direction::Down => 3,
        }
    }
}

// u8 to direction
 impl From<u8> for Direction {
    fn from(n: u8) -> Self {
        match n {
            0 => Direction::Left,
            1 => Direction::Up,
            2 => Direction::Right,
            3 => Direction::Down,
            _ => panic!("invalid direction!"),
        }
    }
 }

// Simple Point datatype to store a position on the screen
// and direction. 
pub struct Point {
    pub pos: (u16, u16),
    pub color: Box<dyn Color>,
    pub direction: (Direction, Direction)
}

impl Point {
    // random point initalization
    pub fn rand_init(n: u8, bounds: &(u16, u16)) -> Vec<Point> {
        let mut pv: Vec<Point> = vec![];

        for _ in 0..n {
            let direction = Direction::from(fastrand::u8(0..4));
            let pos: (u16, u16);
            match direction {
                Direction::Left => pos = (bounds.0, fastrand::u16(1..bounds.1)),
                Direction::Up => pos = (fastrand::u16(1..bounds.0), bounds.1),
                Direction::Right => pos = (1, fastrand::u16(1..bounds.1)),
                Direction::Down => pos = (fastrand::u16(1..bounds.0), 1),
            }

            pv.push(Self {
                pos,
                color: color(fastrand::u8(0..16)),
                direction: (direction, direction)
            });
        }

        pv
    }

    // step a point, with bounds checking
    // return: whether the point has hit the edge or not
    pub fn step(&mut self, bounds: &(u16, u16), turnchance: u8) -> bool {
        self.direction.0 = Direction::from(self.direction.1.get_u8());

        // randomly change direction
        let gr = fastrand::u8(0..turnchance);
        if gr == 1 {
            // if we're turning from horizontal to vertical, increment dir val by 1
            let inc = if self.direction.1.get_u8() % 2 == 0 { 1 } else { 0 };
            let r = fastrand::bool();
            match r {
                false => self.direction.1 = Direction::from(inc),
                true => self.direction.1 = Direction::from(inc + 2),
            }
        }

        // move function
        // if point is out of bounds, pass thru
        // this is a little unclean
        match self.direction.0 {
            Direction::Left => {
                self.pos.0 -= 1;
                if self.pos.0 < 1 {
                    self.pos.0 = bounds.0;
                    return true;
                }
            },
            Direction::Right => {
                self.pos.0 += 1;
                if self.pos.0 > bounds.0 {
                    self.pos.0 = 1;
                    return true;
                }
            },
            Direction::Up => {
                self.pos.1 -= 1;
                if self.pos.1 < 1 {
                    self.pos.1 = bounds.1;
                    return true;
                }
            },
            Direction::Down => {
                self.pos.1 += 1;
                if self.pos.1 > bounds.1 {
                    self.pos.1 = 1;
                    return true;
                }
            },
        }

        false
    }
}
