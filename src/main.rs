mod types;

use std::{io::{self, Write}, thread::sleep, time::Duration};
use termion::{
    style,
    clear,
    color,
    terminal_size,
    cursor::{self, Hide, Show},
};
use types::Point;

// point count
const COUNT: u8 = 5;

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

    // terminal bounds and vec of points
    let bounds:(u16, u16) = terminal_size().unwrap();
    let mut pv: Vec<Point> = Point::rand_init(COUNT, &bounds);

    // event loop
    loop {
        // Print and step points
        for point in pv.iter_mut() {
            print!(
                "{}{}x",
                cursor::Goto(point.pos.0, point.pos.1),
                color::Fg(point.color.as_ref())
            );
            point.step();
        }

        // Flush the output to the terminal
        io::stdout().flush()?;
        sleep(Duration::from_millis(50));

        // testing
        print!("{}", clear::All);
    }
}
