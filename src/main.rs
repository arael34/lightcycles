mod types;

use std::io::{self, Write};
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

    // event loop
    loop {
        // Move the cursor to the top-left corner of the terminal screen
        print!("{}{}", cursor::Goto(1, 1), color::Fg(color::Red));
        print!("X");

        print!("{}{}", cursor::Goto(5, 1), color::Fg(color::Reset));
        println!("O");

        // Flush the output to the terminal
        io::stdout().flush()?;
    }
}
