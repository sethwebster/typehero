use std::io::{self, Write};
use std::thread;
use std::time::Duration;

fn main() {
    let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let worm_frames = vec![
        "    ⢀⣀ ",
        "   ⢀⣀  ",
        "  ⢀⣀   ",
        " ⢀⣀    ",
        "⢀⣀     ",
        " ⣀     ",
        "  ⣀    ",
        "   ⣀   ",
        "    ⣀  ",
        "     ⣀ ",
        "      ⣀",
        "      ⠀",
    ];

    let spinners = [
        "◐", "◓", "◑", "◒",
    ];

    let dots = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];

    println!("Spinning Worm Demo - Ctrl+C to exit\n");

    // Hide cursor
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();

    let mut i = 0;
    loop {
        // Braille spinner
        print!("\r{} Loading (braille dots)...   ", dots[i % dots.len()]);
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(80));

        i += 1;
        if i == 50 {
            println!("\n");
            i = 0;
        }

        // Simple spinner
        if i % 50 == 0 {
            for _ in 0..50 {
                print!("\r{} Loading (simple)...        ", spinners[i % spinners.len()]);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(80));
                i += 1;
            }
            println!("\n");
        }

        // Moving worm
        if i % 100 == 0 {
            for _ in 0..50 {
                print!("\r{} Loading (worm)...          ", worm_frames[i % worm_frames.len()]);
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(100));
                i += 1;
            }
            println!("\n");
            i = 0;
        }
    }
}
