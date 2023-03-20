use termion::color::{self, Color};
use std::time::SystemTime;
use crate::TURNCHANCE;

// Generate random num and mod by ceil
fn gen_rand(ceil: u16) -> u16 {
    let mut lcg = Lcg::new();
    let num = lcg.next().unwrap() as u16 % ceil;

    num
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

        _ => panic!("invalid color!")
    }
}

// Pseudorandom number gen
struct Lcg {
    state: u64,
    a: u64,
    c: u64,
}

impl Lcg {
    fn new() -> Self {
        // initialize with "random" state
        let state = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;

        Lcg {
            state,
            a: 1664525,
            c: 1013904223,
        }
    }
}

impl Iterator for Lcg {
    type Item = u64;

    // Generate new "random" number
    fn next(&mut self) -> Option<Self::Item> {
        self.state = self.a.wrapping_mul(self.state).wrapping_add(self.c);
        Some(self.state)
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
            let direction = gen_rand(4) as u8;

            pv.push(Self::new(
                (
                    gen_rand(bounds.0 - 1) + 1,
                    gen_rand(bounds.1 - 1) + 1
                ),
                color(gen_rand(16) as u8),
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
        let gr = gen_rand(TURNCHANCE);
        if gr != 1 { return cont; }

        // if we're turning from horizontal to vertical, increment dir val by 1
        let inc = if self.next_direction.get_u8() % 2 == 0 { 1 } else { 0 };
        let r = gen_rand(2) as u8;
        match r {
            0 => self.next_direction = Direction::from(r + inc),
            1 => self.next_direction = Direction::from(r + inc + 1),
            _ => panic!("invalid mv"),
        }

        cont
    }
}
