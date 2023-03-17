mod types;

use std::{io::{self, Write}, thread::sleep, time::Duration};
use termion::{
    style,
    clear,
    color,
    terminal_size,
    cursor::{self, Hide, Show},
};
use types::{Point, Direction};
use std::time::SystemTime;

// Pseudorandom num gen
pub fn gen_rand(ceil: u64) -> u8{
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    return ((time / 1000) as u64 % ceil) as u8;
}

fn main() -> io::Result<()>{
    // Cleanup when program executes. show cursor, clear screen
    let _handler = ctrlc::set_handler(|| {
        print!("{}{}{}{}", clear::All, style::Reset, Show, cursor::Goto(1, 1));
        io::stdout().flush().expect("Failed to clean up.");
        std::process::exit(0);
    }).unwrap();

    // Clear screen and hide cursor
    print!("{}{}{}", clear::All, style::Bold, Hide);
    io::stdout().flush()?;

    // maybe make into arr?
    let mut point_vec: Vec<Point> = vec![];
    let (width, height) = terminal_size().unwrap();

    // test
    point_vec.push(Point::new((10, 10), color::Rgb(255, 0, 0), Direction::new(gen_rand(4))));

    // event loop
    loop {
        // Print and step points
        for point in point_vec.iter_mut() {
            print!("{}{}x", cursor::Goto(point.pos.0, point.pos.1), color::Fg(point.color));
            point.step(width, height);
        }

        // Flush the output to the terminal
        io::stdout().flush()?;
        sleep(Duration::from_millis(50));
    }
}
