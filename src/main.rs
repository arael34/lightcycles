mod types;

use std::{io::{self, stdout, Write}, thread::sleep, time::Duration};
use termion::{
    style,
    clear,
    color,
    terminal_size,
    cursor::{self, Hide, Show},
    raw::IntoRawMode,
};
use types::Point;

// point count
const COUNT: u8 = 4;
// Trail characters
const TRAILS: [[char; 4]; 4] = [
    ['━', '┗', '━', '┏'],
    ['┓', '┃', '┏', '┃'],
    ['━', '┛', '━', '┓'],
    ['┛', '┃', '┗', '┃'],
];
// chance that a point will turn, 1/TURNCHANCE
pub const TURNCHANCE: u16 = 12;

fn main() -> io::Result<()>{
    // let stdout = stdout();
    // let _ = stdout.into_raw_mode().unwrap();

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

    let s: u32 = bounds.0 as u32 * bounds.1 as u32 * 3 / 4;
    let mut n = 0;

    let mut active: usize = 0;

    // event loop
    loop {
        // Print and step point
        let point = &mut pv[active as usize];
        let ch = TRAILS[(&point.direction).get_u8() as usize]
            [(&point.next_direction).get_u8() as usize];
        print!(
            "{}{}{}", // half block
            cursor::Goto(point.pos.0, point.pos.1),
            color::Fg(point.color.as_ref()),
            ch
        );
        if point.step(&bounds) { active += 1; }
        if active as u8 >= COUNT { active = 0; }

        // Flush the output to the terminal
        io::stdout().flush()?;
        sleep(Duration::from_millis(30));
        
        n += 1;
        // reset terminal after a certain number of prints
        if n < s { continue; }
        print!("{}", clear::All);
        n = 0;
    }
}
