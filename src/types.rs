use termion::color::{self, Color};
use std::time::SystemTime;

// Pseudorandom num gen
fn gen_rand(ceil: u64) -> u16 {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    return ((time / 1000) as u64 % ceil) as u16;
}

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

        _ => panic!()
    }
}

pub enum Direction {
    Left, Right,
    Up, Down
}

impl Direction {
    // u8 to direction
    pub fn direction(n: u8) -> Self {
        match n {
            0 => Self::Left,
            1 => Self::Up,
            2 => Self::Right,
            3 => Self::Down,
            _ => panic!(),
        }
    }
    // direction to u8
    pub fn int(&self) -> u8 {
        match self {
            Self::Left => 0,
            Self::Up => 1,
            Self::Right => 2,
            Self::Down => 3,
        }
    }
 }

// Simple Point datatype to store a position on the screen
// and direction. 
pub struct Point<'a> {
    pub pos: (u16, u16),
    pub color: Box<dyn Color>,
    direction: Direction,
    bounds: &'a (u16, u16)
}

impl<'a> Point<'a> {
    fn new(
        pos: (u16, u16),
        color: Box<dyn Color>,
        direction: Direction,
        bounds: &'a (u16, u16),
    ) -> Self 
    { Point { pos, color, direction,bounds } }

    // random point initalization
    pub fn rand_init(c: u8, bounds: &'a (u16, u16)) -> Vec<Point<'a>> {
        let mut pv: Vec<Point> = vec![];

        for _ in 0..c {
            pv.push(Self::new(
                (
                    gen_rand((bounds.0 - 1) as u64) + 1,
                    gen_rand((bounds.1 - 1) as u64) + 1
                ),
                color(gen_rand(16) as u8),
                Direction::direction(gen_rand(4) as u8),
                bounds
            ));
        }

        pv
    }

    // step a point, with bounds checking
    pub fn step(&mut self) -> () {
        match &self.direction {
            Direction::Left => {
                if self.pos.0 > 1 { self.pos.0 -= 1; }
                else { self.direction = Direction::Right; }
            },
            Direction::Right => {
                if self.pos.0 < self.bounds.0 - 1 { self.pos.0 += 1; }
                else { self.direction = Direction::Left }
            },
            Direction::Up => {
                if self.pos.1 > 1 { self.pos.1 -= 1; }
                else { self.direction = Direction::Down; }
            },
            Direction::Down => {
                if self.pos.1 <= self.bounds.1 { self.pos.1 += 1; }
                else { self.direction = Direction::Up; }
            },
        }
        
        // randomly change direction
        let gr = gen_rand(50);
        if gr == 0 {
            self.direction = Direction::direction((self.direction.int() + 1) % 4);
        }
    }
}
