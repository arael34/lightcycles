mod types;

use std::{io::{self, Write}, thread::sleep, time::Duration};
use termion::{clear, color, cursor, cursor::{Hide, Show}};
use types::{Point, Direction};

fn main() -> io::Result<()>{
    // Cleanup when program executes. show cursor, clear screen
    let _handler = ctrlc::set_handler(|| {
        print!("{}{}", clear::All, Show);
        io::stdout().flush().expect("Failed to clean up.");
        std::process::exit(0);
    }).unwrap();

    // Clear screen and hide cursor
    print!("{}{}", clear::All, Hide);
    io::stdout().flush()?;

    // maybe make into arr?
    let mut point_vec: Vec<Point> = vec![];

    point_vec.push(Point::new((2, 2), 0, Direction::Right));
    point_vec.push(Point::new((4, 1), 0, Direction::Down));

    // event loop
    loop {
        // Print and step points
        for point in point_vec.iter_mut() {
            cursor::Goto(point.pos.0, point.pos.1);

            print!("X");
            point.step();
        }

        // Flush the output to the terminal
        io::stdout().flush()?;
        sleep(Duration::from_millis(100));
    }
}

// for reference. cursor::Goto for moving, color::Fg for color
