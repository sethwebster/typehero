use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Classic terminal spinner frames
pub struct Spinner {
    pub frames: Vec<&'static str>,
    pub interval: Duration,
}

impl Spinner {
    pub const DOTS: Spinner = Spinner {
        frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
        interval: Duration::from_millis(80),
    };

    pub const LINE: Spinner = Spinner {
        frames: vec!["-", "\\", "|", "/"],
        interval: Duration::from_millis(100),
    };

    pub const DOTS2: Spinner = Spinner {
        frames: vec!["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
        interval: Duration::from_millis(80),
    };

    pub const CIRCLE: Spinner = Spinner {
        frames: vec!["◐", "◓", "◑", "◒"],
        interval: Duration::from_millis(120),
    };

    pub const ARROW: Spinner = Spinner {
        frames: vec!["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
        interval: Duration::from_millis(100),
    };

    pub const BOUNCE: Spinner = Spinner {
        frames: vec!["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"],
        interval: Duration::from_millis(100),
    };

    pub const SHARK: Spinner = Spinner {
        frames: vec!["▐⠂       ▌", "▐⠈       ▌", "▐ ⠂      ▌", "▐ ⠠      ▌", "▐  ⡀     ▌", "▐  ⠠     ▌", "▐   ⠂    ▌", "▐   ⠈    ▌", "▐    ⠂   ▌", "▐    ⠠   ▌", "▐     ⡀  ▌", "▐     ⠠  ▌", "▐      ⠂ ▌", "▐      ⠈ ▌", "▐       ⠂▌", "▐       ⠠▌", "▐       ⡀▌", "▐      ⠠ ▌", "▐      ⠂ ▌", "▐     ⠈  ▌", "▐     ⠂  ▌", "▐    ⠠   ▌", "▐    ⡀   ▌", "▐   ⠠    ▌", "▐   ⠂    ▌", "▐  ⠈     ▌", "▐  ⠂     ▌", "▐ ⠠      ▌", "▐ ⡀      ▌", "▐⠠       ▌"],
        interval: Duration::from_millis(120),
    };

    pub const PULSE: Spinner = Spinner {
        frames: vec!["⚫", "⚫", "⚫", "⚫", "⚫", "⚪", "⚪", "⚪", "⚪", "⚪"],
        interval: Duration::from_millis(100),
    };
}

fn demo_spinner(name: &str, spinner: &Spinner, iterations: usize) {
    print!("\n{}: ", name);
    io::stdout().flush().unwrap();

    for i in 0..(iterations * spinner.frames.len()) {
        let frame = spinner.frames[i % spinner.frames.len()];
        print!("\r{}: {} ", name, frame);
        io::stdout().flush().unwrap();
        thread::sleep(spinner.interval);
    }
    println!();
}

fn main() {
    println!("Terminal Spinner Collection\n");
    println!("Press Ctrl+C to exit\n");

    // Hide cursor
    print!("\x1B[?25l");
    io::stdout().flush().unwrap();

    demo_spinner("Dots (Classic)", &Spinner::DOTS, 3);
    demo_spinner("Dots 2", &Spinner::DOTS2, 3);
    demo_spinner("Circle", &Spinner::CIRCLE, 3);
    demo_spinner("Line", &Spinner::LINE, 3);
    demo_spinner("Arrow", &Spinner::ARROW, 2);
    demo_spinner("Bounce", &Spinner::BOUNCE, 3);
    demo_spinner("Pulse", &Spinner::PULSE, 2);
    demo_spinner("Shark (Worm)", &Spinner::SHARK, 1);

    // Show cursor
    print!("\x1B[?25h");
    io::stdout().flush().unwrap();

    println!("\n✓ Demo complete!");
}
