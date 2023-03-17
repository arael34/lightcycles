use termion::color::{Color, self};
use std::time::SystemTime;

// Pseudorandom num gen
fn gen_rand(ceil: u64) -> u16 {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    return ((time / 1000) as u64 % ceil) as u16;
}

pub enum Direction {
    Left, Right,
    Up, Down
}

impl Direction {
    pub fn new(n: u8) -> Self {
        match n {
            0 => Self::Left,
            1 => Self::Up,
            2 => Self::Right,
            3 => Self::Down,
            _ => panic!(),
        }
    }
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
    ) -> Self {
        Point { pos, color, direction,bounds }
    }

    pub fn rand_init(c: u8, bounds: &'a (u16, u16)) -> Vec<Point<'a>> {
        let mut pv: Vec<Point> = vec![];

        for _ in 0..c {
            pv.push(Self::new(
                (gen_rand(bounds.0 as u64), gen_rand(bounds.1 as u64)),
                Box::new(color::Red),
                Direction::new(gen_rand(4) as u8),
                bounds
            ));
        }

        pv
    }

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
            self.direction = Direction::new((self.direction.int() + 1) % 4);
        }
    }
}
