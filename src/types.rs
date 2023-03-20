use termion::color::{self, Color};
use crate::TURNCHANCE;

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
    pub next_direction: Direction,
    pub direction: Direction,
}

impl Point {
    fn new(
        pos: (u16, u16),
        color: Box<dyn Color>,
        direction: Direction,
        next_direction: Direction ) -> Self 
    { Point { pos, color, direction, next_direction } }

    // random point initalization
    pub fn rand_init(c: u8, bounds: &(u16, u16)) -> Vec<Point> {
        let mut pv: Vec<Point> = vec![];

        for _ in 0..c {
            let direction = fastrand::u8(0..4);

            pv.push(Self::new(
                (
                    fastrand::u16(1..bounds.0 - 1),
                    fastrand::u16(1..bounds.1 - 1)
                ),
                color(fastrand::u8(0..16)),
                Direction::from(direction),
                Direction::from(direction),
            ));
        }

        pv
    }

    // step a point, with bounds checking
    pub fn step(&mut self, bounds: &(u16, u16)) -> bool {
        self.direction = Direction::from(self.next_direction.get_u8());

        let mut cont = false;

        // move function
        // if point is out of bounds, pass thru
        // this is so unclean
        match self.direction {
            Direction::Left => {
                self.pos.0 -= 1;
                if self.pos.0 < 1 {
                    self.pos.0 = bounds.0;
                    cont = true;
                }
            },
            Direction::Right => {
                self.pos.0 += 1;
                if self.pos.0 > bounds.0 {
                    self.pos.0 = 1;
                    cont = true;
                }
            },
            Direction::Up => {
                self.pos.1 -= 1;
                if self.pos.1 < 1 {
                    self.pos.1 = bounds.1;
                    cont = true;
                }
            },
            Direction::Down => {
                self.pos.1 += 1;
                if self.pos.1 > bounds.1 {
                    self.pos.1 = 1;
                    cont = true;
                }
            },
        }

        // randomly change direction
        let gr = fastrand::u8(0..TURNCHANCE);
        if gr != 1 { return cont; }

        // if we're turning from horizontal to vertical, increment dir val by 1
        let inc = if self.next_direction.get_u8() % 2 == 0 { 1 } else { 0 };
        let r = fastrand::bool();
        match r {
            false => self.next_direction = Direction::from(inc),
            true => self.next_direction = Direction::from(inc + 2),
        }

        cont
    }
}
