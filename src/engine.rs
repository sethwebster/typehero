use crate::display::Display;
use crate::input::{read_key, InputEvent};
use crate::stats::{SessionSummary, Stats};
use chrono::Utc;
use std::time::{Duration, Instant};

pub struct TypingSession {
    text: String,
    typed: Vec<(char, bool)>, // (char, was_correct)
    start_time: Instant,
    last_char_time: Instant,
    current_pos: usize,
    errors: usize,
    stats: Stats,
    quit_warning_shown: bool,
}

impl TypingSession {
    pub fn new(text: String, stats: Stats) -> Self {
        Self {
            text,
            typed: Vec::new(),
            start_time: Instant::now(),
            last_char_time: Instant::now(),
            current_pos: 0,
            errors: 0,
            stats,
            quit_warning_shown: false,
        }
    }

    fn chars(&self) -> Vec<char> {
        self.text.chars().collect()
    }

    fn elapsed_secs(&self) -> f64 {
        self.start_time.elapsed().as_secs_f64()
    }

    fn raw_wpm(&self) -> f64 {
        let chars = self.typed.len();
        let minutes = self.elapsed_secs() / 60.0;
        if minutes == 0.0 {
            return 0.0;
        }
        (chars as f64 / 5.0) / minutes
    }

    fn accuracy(&self) -> f64 {
        if self.typed.is_empty() {
            return 100.0;
        }
        let correct = self.typed.iter().filter(|(_, c)| *c).count();
        (correct as f64 / self.typed.len() as f64) * 100.0
    }

    fn adjusted_wpm(&self) -> f64 {
        let acc = self.accuracy() / 100.0;
        self.raw_wpm() * acc * acc
    }

    pub fn run(&mut self, display: &Display) -> Result<SessionSummary, Box<dyn std::error::Error>> {
        let chars = self.chars();

        loop {
            // Render current state
            display.render_test(
                &self.text,
                &self.typed,
                self.current_pos,
                self.elapsed_secs(),
                self.raw_wpm(),
                self.accuracy(),
                self.adjusted_wpm(),
                self.quit_warning_shown,
            )?;

            // Check for input
            match read_key(Duration::from_millis(50))? {
                InputEvent::Char(ch) => {
                    if self.current_pos >= chars.len() {
                        continue; // Ignore extra input after completion
                    }

                    let expected = chars[self.current_pos];
                    let correct = ch == expected;

                    // Record the keystroke
                    self.typed.push((ch, correct));
                    if !correct {
                        self.errors += 1;
                    }

                    // Track in stats
                    self.stats.record_keystroke(expected, correct);

                    // Track bigram timing
                    if self.current_pos > 0 {
                        let prev = chars[self.current_pos - 1];
                        let duration = self.last_char_time.elapsed();
                        self.stats.record_bigram((prev, expected), duration);
                    }

                    self.last_char_time = Instant::now();
                    self.current_pos += 1;

                    // Check if test is complete
                    if self.current_pos >= chars.len() {
                        break;
                    }
                }
                InputEvent::Escape => {
                    // Check if quitting too early
                    let completion_pct = (self.current_pos as f64 / chars.len() as f64) * 100.0;

                    if completion_pct < 50.0 && !self.quit_warning_shown {
                        self.quit_warning_shown = true;
                        // Warning will be shown on next render
                    } else {
                        break;
                    }
                }
                _ => {}
            }
        }

        // Create session summary
        let summary = SessionSummary {
            timestamp: Utc::now(),
            duration_secs: self.elapsed_secs(),
            total_chars: self.typed.len(),
            errors: self.errors,
            raw_wpm: self.raw_wpm(),
            accuracy: self.accuracy(),
            adjusted_wpm: self.adjusted_wpm(),
        };

        // Add to stats and save
        self.stats.add_session(summary.clone());
        self.stats.save()?;

        Ok(summary)
    }

    pub fn stats(&self) -> &Stats {
        &self.stats
    }
}

pub fn show_summary(
    display: &Display,
    summary: &SessionSummary,
    stats: &Stats,
) -> Result<(), Box<dyn std::error::Error>> {
    let slowest = stats.slowest_bigrams(5);
    let error_prone = stats.most_error_prone_keys(5);

    display.render_summary(
        summary.duration_secs,
        summary.total_chars,
        summary.errors,
        summary.raw_wpm,
        summary.accuracy,
        summary.adjusted_wpm,
        &slowest,
        &error_prone,
    )?;

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
