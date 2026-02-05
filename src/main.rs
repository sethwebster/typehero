mod code_challenge;
mod display;
mod engine;
mod exercises;
mod guided_v2;
mod input;
mod stats;

use code_challenge::CodeChallenge;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use display::Display;
use engine::{show_summary, TypingSession};
use exercises::{generate_exercise, load_random_code_sample, ExerciseMode};
use input::{read_key, InputEvent};
use stats::Stats;
use std::io::stdout;
use std::time::Duration;

const MENU_OPTIONS: &[&str] = &[
    "1. Guided Practice (learn proper finger placement)",
    "2. Random Words (50 words)",
    "3. Code Patterns (50 patterns)",
    "4. CodeJam (real programming problems)",
    "5. Targeted Practice (your problem areas)",
    "6. Quick Drill (20 words)",
    "7. View Statistics",
    "8. Quit",
];

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    if let Err(e) = enable_raw_mode() {
        eprintln!("Failed to enable raw mode: {}", e);
        eprintln!("Make sure you're running this in a terminal (not piped or redirected)");
        std::process::exit(1);
    }

    if let Err(e) = execute!(stdout(), EnterAlternateScreen) {
        eprintln!("Failed to enter alternate screen: {}", e);
        let _ = disable_raw_mode();
        std::process::exit(1);
    }

    let result = run_app();

    // Cleanup terminal
    let _ = disable_raw_mode();
    let _ = execute!(stdout(), LeaveAlternateScreen);

    result
}

fn run_app() -> Result<(), Box<dyn std::error::Error>> {
    let display = Display::new()?;
    let mut stats = Stats::load();
    let mut selected = 0;

    loop {
        display.render_menu(selected, MENU_OPTIONS)?;

        match read_key(Duration::from_millis(50))? {
            InputEvent::Up => {
                if selected > 0 {
                    selected -= 1;
                }
            }
            InputEvent::Down => {
                if selected < MENU_OPTIONS.len() - 1 {
                    selected += 1;
                }
            }
            InputEvent::Enter | InputEvent::Char(' ') => {
                match selected {
                    0 => {
                        // Guided Practice
                        let mut guided = guided_v2::GuidedPractice::new();
                        let _ = guided.run(&display);
                    }
                    1 => {
                        // Random Words
                        let text = generate_exercise(&ExerciseMode::RandomWords, 50);
                        let mut session = TypingSession::new(text, stats.clone());
                        let summary = session.run(&display)?;
                        stats = session.stats().clone();
                        show_summary(&display, &summary, &stats)?;
                    }
                    2 => {
                        // Code Patterns
                        let text = generate_exercise(&ExerciseMode::Code, 50);
                        let mut session = TypingSession::new(text, stats.clone());
                        let summary = session.run(&display)?;
                        stats = session.stats().clone();
                        show_summary(&display, &summary, &stats)?;
                    }
                    3 => {
                        // CodeJam
                        if let Some(sample) = load_random_code_sample() {
                            let mut challenge = CodeChallenge::new(sample.code, sample.language);
                            let _ = challenge.run(&display, &stats);
                        } else {
                            // Fallback to code patterns if no samples found
                            let text = generate_exercise(&ExerciseMode::Code, 50);
                            let mut session = TypingSession::new(text, stats.clone());
                            let summary = session.run(&display)?;
                            stats = session.stats().clone();
                            show_summary(&display, &summary, &stats)?;
                        }
                    }
                    4 => {
                        // Targeted Practice
                        let slowest = stats.slowest_bigrams(10);
                        if slowest.is_empty() {
                            // Fall back to random words if no data yet
                            let text = generate_exercise(&ExerciseMode::RandomWords, 50);
                            let mut session = TypingSession::new(text, stats.clone());
                            let summary = session.run(&display)?;
                            stats = session.stats().clone();
                            show_summary(&display, &summary, &stats)?;
                        } else {
                            let bigrams: Vec<String> =
                                slowest.iter().map(|(b, _)| b.clone()).collect();
                            let text = generate_exercise(&ExerciseMode::Targeted(bigrams), 50);
                            let mut session = TypingSession::new(text, stats.clone());
                            let summary = session.run(&display)?;
                            stats = session.stats().clone();
                            show_summary(&display, &summary, &stats)?;
                        }
                    }
                    5 => {
                        // Quick Drill
                        let text = generate_exercise(&ExerciseMode::RandomWords, 20);
                        let mut session = TypingSession::new(text, stats.clone());
                        let summary = session.run(&display)?;
                        stats = session.stats().clone();
                        show_summary(&display, &summary, &stats)?;
                    }
                    6 => {
                        // View Statistics
                        show_stats(&display, &stats)?;
                    }
                    7 => {
                        // Quit
                        break;
                    }
                    _ => {}
                }
            }
            InputEvent::Escape => break,
            _ => {}
        }
    }

    Ok(())
}

fn show_stats(display: &Display, stats: &Stats) -> Result<(), Box<dyn std::error::Error>> {
    display.clear()?;

    use crossterm::{cursor, style::Print, QueueableCommand};
    use std::io::{stdout, Write};

    let mut out = stdout();
    let mut row = 0;

    // Title
    out.queue(cursor::MoveTo(2, row))?
        .queue(Print("═══ Lifetime Statistics ═══"))?;
    row += 2;

    // Overall stats
    let sessions = stats.sessions();
    out.queue(cursor::MoveTo(2, row))?
        .queue(Print(format!("Total sessions: {}", sessions.len())))?;
    row += 1;
    out.queue(cursor::MoveTo(2, row))?
        .queue(Print(format!("Total keystrokes: {}", stats.total_keys_public())))?;
    row += 1;
    out.queue(cursor::MoveTo(2, row))?
        .queue(Print(format!("Total errors: {}", stats.total_errors_public())))?;
    row += 1;
    out.queue(cursor::MoveTo(2, row))?
        .queue(Print(format!(
            "Lifetime accuracy: {:.1}%",
            stats.lifetime_accuracy()
        )))?;
    row += 1;
    out.queue(cursor::MoveTo(2, row))?
        .queue(Print(format!(
            "Average adjusted WPM: {:.1}",
            stats.average_adjusted_wpm()
        )))?;
    row += 2;

    // Recent sessions
    if !sessions.is_empty() {
        out.queue(cursor::MoveTo(2, row))?
            .queue(Print("Recent sessions (last 5):"))?;
        row += 1;

        let recent = sessions.iter().take(5);
        for session in recent {
            out.queue(cursor::MoveTo(4, row))?.queue(Print(format!(
                "{} - {:.1} WPM ({:.1}% acc) → {:.1} adj WPM",
                session.timestamp.format("%Y-%m-%d %H:%M"),
                session.raw_wpm,
                session.accuracy,
                session.adjusted_wpm
            )))?;
            row += 1;
        }
        row += 1;
    }

    // Problem areas
    let slowest = stats.slowest_bigrams(5);
    if !slowest.is_empty() {
        out.queue(cursor::MoveTo(2, row))?
            .queue(Print("Slowest bigrams:"))?;
        row += 1;
        for (bigram, ms) in slowest {
            out.queue(cursor::MoveTo(4, row))?
                .queue(Print(format!("  '{}' - {:.0}ms", bigram, ms)))?;
            row += 1;
        }
        row += 1;
    }

    let error_prone = stats.most_error_prone_keys(5);
    if !error_prone.is_empty() {
        out.queue(cursor::MoveTo(2, row))?
            .queue(Print("Most error-prone keys:"))?;
        row += 1;
        for (key, rate) in error_prone {
            out.queue(cursor::MoveTo(4, row))?
                .queue(Print(format!("  '{}' - {:.1}% error rate", key, rate)))?;
            row += 1;
        }
    }

    row += 2;
    out.queue(cursor::MoveTo(2, row))?
        .queue(Print("Press any key to return..."))?;

    out.flush()?;

    // Wait for any key
    loop {
        if let InputEvent::Char(_) | InputEvent::Enter | InputEvent::Escape =
            read_key(Duration::from_millis(100))?
        {
            break;
        }
    }

    Ok(())
}
