use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, ClearType},
    QueueableCommand,
};
use std::io::{stdout, Write};

pub struct Display {
    width: u16,
    #[allow(dead_code)]
    height: u16,
}

impl Display {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (width, height) = terminal::size()?;
        Ok(Self { width, height })
    }

    pub fn clear(&self) -> Result<(), Box<dyn std::error::Error>> {
        stdout()
            .queue(terminal::Clear(ClearType::All))?
            .queue(cursor::MoveTo(0, 0))?
            .flush()?;
        Ok(())
    }

    pub fn render_test(
        &self,
        text: &str,
        typed: &[(char, bool)],
        current_pos: usize,
        elapsed_secs: f64,
        wpm: f64,
        accuracy: f64,
        adjusted_wpm: f64,
        quit_warning_shown: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = stdout();
        self.clear()?;

        // Title
        out.queue(cursor::MoveTo(0, 0))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print("═".repeat(self.width as usize)))?
            .queue(cursor::MoveTo(0, 1))?
            .queue(Print("  TypeHero - Accuracy-First Typing Trainer"))?
            .queue(cursor::MoveTo(0, 2))?
            .queue(Print("═".repeat(self.width as usize)))?
            .queue(ResetColor)?;

        // Stats panel
        out.queue(cursor::MoveTo(0, 4))?
            .queue(SetForegroundColor(Color::Yellow))?
            .queue(Print(format!("Time: {:.1}s", elapsed_secs)))?
            .queue(cursor::MoveTo(20, 4))?
            .queue(Print(format!("Raw WPM: {:.1}", wpm)))?
            .queue(cursor::MoveTo(40, 4))?
            .queue(SetForegroundColor(if accuracy >= 95.0 {
                Color::Green
            } else if accuracy >= 85.0 {
                Color::Yellow
            } else {
                Color::Red
            }))?
            .queue(Print(format!("Accuracy: {:.1}%", accuracy)))?
            .queue(cursor::MoveTo(60, 4))?
            .queue(SetForegroundColor(Color::Green))?
            .queue(Print(format!("Adj WPM: {:.1}", adjusted_wpm)))?
            .queue(ResetColor)?;

        // Text display area
        let start_row = 7;
        let _max_width = (self.width - 4) as usize;

        // Render the original text with current character highlighted
        out.queue(cursor::MoveTo(2, start_row))?
            .queue(SetForegroundColor(Color::DarkGrey))?;

        for (i, ch) in text.chars().enumerate() {
            if i == current_pos {
                out.queue(SetBackgroundColor(Color::DarkBlue))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(ch))?
                    .queue(ResetColor)?
                    .queue(SetForegroundColor(Color::DarkGrey))?;
            } else {
                out.queue(Print(ch))?;
            }
        }
        out.queue(ResetColor)?;

        // Render typed text with mistakes highlighted
        out.queue(cursor::MoveTo(2, start_row + 5))?;

        for (ch, correct) in typed {
            if *correct {
                out.queue(SetForegroundColor(Color::Green))?
                    .queue(Print(ch))?;
            } else {
                out.queue(SetForegroundColor(Color::Red))?
                    .queue(SetBackgroundColor(Color::DarkRed))?
                    .queue(Print(ch))?
                    .queue(ResetColor)?;
            }
        }
        out.queue(ResetColor)?;

        // Instructions or warning
        let instr_row = start_row + 9;
        if quit_warning_shown {
            out.queue(cursor::MoveTo(2, instr_row))?
                .queue(SetForegroundColor(Color::Yellow))?
                .queue(Print("The point isn't perfection on every run."))?;
            out.queue(cursor::MoveTo(2, instr_row + 1))?
                .queue(Print("Keep going through the discomfort - that's where growth happens."))?;
            out.queue(cursor::MoveTo(2, instr_row + 2))?
                .queue(SetForegroundColor(Color::DarkGrey))?
                .queue(Print("Press ESC again to quit anyway"))?
                .queue(ResetColor)?;
        } else {
            out.queue(cursor::MoveTo(2, instr_row))?
                .queue(SetForegroundColor(Color::DarkGrey))?
                .queue(Print("Press ESC to quit anytime"))?
                .queue(ResetColor)?;
        }

        out.flush()?;
        Ok(())
    }

    pub fn render_summary(
        &self,
        elapsed_secs: f64,
        total_chars: usize,
        errors: usize,
        raw_wpm: f64,
        accuracy: f64,
        adjusted_wpm: f64,
        slowest_bigrams: &[(String, f64)],
        error_prone_keys: &[(char, f64)],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = stdout();
        self.clear()?;

        let mut row = 0;

        // Title
        out.queue(cursor::MoveTo(0, row))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print("═".repeat(self.width as usize)))?;
        row += 1;
        out.queue(cursor::MoveTo(0, row))?
            .queue(Print("  Session Summary"))?;
        row += 1;
        out.queue(cursor::MoveTo(0, row))?
            .queue(Print("═".repeat(self.width as usize)))?
            .queue(ResetColor)?;
        row += 2;

        // Overall stats
        out.queue(cursor::MoveTo(2, row))?
            .queue(Print(format!("Duration: {:.1}s", elapsed_secs)))?;
        row += 1;
        out.queue(cursor::MoveTo(2, row))?
            .queue(Print(format!("Characters: {}", total_chars)))?;
        row += 1;
        out.queue(cursor::MoveTo(2, row))?
            .queue(Print(format!("Errors: {}", errors)))?;
        row += 1;
        out.queue(cursor::MoveTo(2, row))?
            .queue(Print(format!("Raw WPM: {:.1}", raw_wpm)))?;
        row += 1;
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(if accuracy >= 95.0 {
                Color::Green
            } else if accuracy >= 85.0 {
                Color::Yellow
            } else {
                Color::Red
            }))?
            .queue(Print(format!("Accuracy: {:.1}%", accuracy)))?
            .queue(ResetColor)?;
        row += 1;
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(Color::Green))?
            .queue(Print(format!("Adjusted WPM: {:.1}", adjusted_wpm)))?
            .queue(ResetColor)?;
        row += 2;

        // Verdict
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(if accuracy >= 95.0 {
                Color::Green
            } else {
                Color::Red
            }))?
            .queue(Print(if accuracy >= 95.0 {
                "✓ PASSED - Great accuracy!"
            } else {
                "✗ FAILED - Need 95%+ accuracy"
            }))?
            .queue(ResetColor)?;
        row += 2;

        // Slowest bigrams
        if !slowest_bigrams.is_empty() {
            out.queue(cursor::MoveTo(2, row))?
                .queue(SetForegroundColor(Color::Yellow))?
                .queue(Print("Slowest bigrams (need practice):"))?
                .queue(ResetColor)?;
            row += 1;

            for (bigram, ms) in slowest_bigrams {
                out.queue(cursor::MoveTo(4, row))?
                    .queue(Print(format!("  '{}' - {:.0}ms", bigram, ms)))?;
                row += 1;
            }
            row += 1;
        }

        // Error-prone keys
        if !error_prone_keys.is_empty() {
            out.queue(cursor::MoveTo(2, row))?
                .queue(SetForegroundColor(Color::Red))?
                .queue(Print("Most error-prone keys:"))?
                .queue(ResetColor)?;
            row += 1;

            for (key, rate) in error_prone_keys {
                out.queue(cursor::MoveTo(4, row))?
                    .queue(Print(format!("  '{}' - {:.1}% error rate", key, rate)))?;
                row += 1;
            }
        }

        row += 2;
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print("Press any key to continue..."))?
            .queue(ResetColor)?;

        out.flush()?;
        Ok(())
    }

    pub fn render_menu(
        &self,
        selected: usize,
        options: &[&str],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = stdout();
        self.clear()?;

        let mut row = 0;

        // Title
        out.queue(cursor::MoveTo(0, row))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print("═".repeat(self.width as usize)))?;
        row += 1;
        out.queue(cursor::MoveTo(0, row))?
            .queue(Print("  TypeHero - Select Practice Mode"))?;
        row += 1;
        out.queue(cursor::MoveTo(0, row))?
            .queue(Print("═".repeat(self.width as usize)))?
            .queue(ResetColor)?;
        row += 2;

        // Options
        for (i, option) in options.iter().enumerate() {
            out.queue(cursor::MoveTo(2, row))?;

            if i == selected {
                out.queue(SetBackgroundColor(Color::DarkBlue))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(format!("  → {}  ", option)))?
                    .queue(ResetColor)?;
            } else {
                out.queue(SetForegroundColor(Color::White))?
                    .queue(Print(format!("    {}  ", option)))?
                    .queue(ResetColor)?;
            }
            row += 1;
        }

        row += 2;
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print("↑/↓ to navigate, Enter to select, ESC to quit"))?
            .queue(ResetColor)?;

        out.flush()?;
        Ok(())
    }

    pub fn render_guided_lesson(
        &self,
        lesson_name: &str,
        active_fingers: &[crate::guided_v2::Finger],
        text: &str,
        typed: &[(char, bool)],
        current_pos: usize,
        current_char: Option<char>,
        lesson_idx: usize,
        total_lessons: usize,
        attempts: &[crate::guided_v2::AttemptStats],
        mastery_status: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = stdout();
        self.clear()?;

        let mut row = 0;

        // Title
        out.queue(cursor::MoveTo(0, row))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print("═".repeat(self.width as usize)))?;
        row += 1;
        out.queue(cursor::MoveTo(0, row))?
            .queue(Print(format!(
                "  Guided Practice - Lesson {}/{}",
                lesson_idx + 1,
                total_lessons
            )))?;
        row += 1;
        out.queue(cursor::MoveTo(0, row))?
            .queue(Print("═".repeat(self.width as usize)))?
            .queue(ResetColor)?;
        row += 2;

        // Lesson name
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(Color::Yellow))?
            .queue(Print(format!("Lesson: {}", lesson_name)))?
            .queue(ResetColor)?;
        row += 1;

        // Active fingers (color-coded)
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print("Active fingers: "))?
            .queue(ResetColor)?;

        for (i, finger) in active_fingers.iter().enumerate() {
            if i > 0 {
                out.queue(Print(", "))?;
            }
            out.queue(SetForegroundColor(finger.color()))?
                .queue(Print(finger.name()))?
                .queue(ResetColor)?;
        }
        row += 2;

        // Keyboard map showing active fingers
        row = self.render_keyboard_map(&mut out, row, active_fingers)?;
        row += 1;

        // Mastery status
        out.queue(cursor::MoveTo(2, row))?;
        if mastery_status.contains("MASTERED") {
            out.queue(SetForegroundColor(Color::Green))?;
        } else if mastery_status.contains("Progress") {
            out.queue(SetForegroundColor(Color::Yellow))?;
        } else if mastery_status.contains("violation") {
            out.queue(SetForegroundColor(Color::Red))?;
        } else {
            out.queue(SetForegroundColor(Color::DarkGrey))?;
        }
        out.queue(Print(format!("Status: {}", mastery_status)))?
            .queue(ResetColor)?;
        row += 2;

        // Current key instruction
        if let Some(ch) = current_char {
            out.queue(cursor::MoveTo(2, row))?
                .queue(SetForegroundColor(Color::Cyan))?
                .queue(Print(format!("Next key: '{}'", ch)))?
                .queue(ResetColor)?;
        }
        row += 2;

        // Target text
        out.queue(cursor::MoveTo(4, row))?
            .queue(SetForegroundColor(Color::DarkGrey))?;

        for (i, ch) in text.chars().enumerate() {
            if i == current_pos {
                out.queue(SetBackgroundColor(Color::DarkBlue))?
                    .queue(SetForegroundColor(Color::White))?
                    .queue(Print(format!(" {} ", ch)))?
                    .queue(ResetColor)?
                    .queue(SetForegroundColor(Color::DarkGrey))?;
            } else {
                out.queue(Print(format!(" {} ", ch)))?;
            }
        }
        out.queue(ResetColor)?;
        row += 3;

        // Typed text
        out.queue(cursor::MoveTo(4, row))?;
        for (ch, correct) in typed {
            if *correct {
                out.queue(SetForegroundColor(Color::Green))?
                    .queue(Print(format!(" {} ", ch)))?;
            } else {
                out.queue(SetForegroundColor(Color::Red))?
                    .queue(SetBackgroundColor(Color::DarkRed))?
                    .queue(Print(format!(" {} ", ch)))?
                    .queue(ResetColor)?;
            }
        }
        out.queue(ResetColor)?;
        row += 3;

        // Constraint notice
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(Color::Yellow))?
            .queue(Print("⚠ Only keys from active fingers are accepted"))?
            .queue(ResetColor)?;
        row += 2;

        // Attempt history
        if !attempts.is_empty() {
            out.queue(cursor::MoveTo(2, row))?
                .queue(SetForegroundColor(Color::DarkGrey))?
                .queue(Print("Recent attempts:"))?
                .queue(ResetColor)?;
            row += 1;

            for (i, attempt) in attempts.iter().rev().take(5).enumerate() {
                out.queue(cursor::MoveTo(4, row))?
                    .queue(SetForegroundColor(if attempt.accuracy >= 95.0
                        && attempt.illegal_keys == 0
                    {
                        Color::Green
                    } else if attempt.illegal_keys > 0 {
                        Color::Red
                    } else if attempt.accuracy >= 85.0 {
                        Color::Yellow
                    } else {
                        Color::Red
                    }))?
                    .queue(Print(format!(
                        "#{}: {}ms - {:.0}% acc - {} errors - {} illegal",
                        attempts.len() - i,
                        attempt.duration_ms,
                        attempt.accuracy,
                        attempt.errors,
                        attempt.illegal_keys
                    )))?
                    .queue(ResetColor)?;
                row += 1;
            }
            row += 1;
        }

        // Instructions
        row += 1;
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(Color::DarkGrey))?;

        if typed.is_empty() && attempts.is_empty() {
            // First time in lesson
            out.queue(Print(
                "Press Enter to start, Ctrl-N/Ctrl-P to navigate, ESC to quit",
            ))?;
        } else if typed.is_empty() && !attempts.is_empty() {
            // Between attempts - auto-countdown
            out.queue(Print(
                "Auto-starting in 0.5s... (Ctrl-N/P to navigate, ESC to quit)",
            ))?;
        } else {
            // During practice
            out.queue(Print(
                "Type using ONLY the active fingers - other keys will be rejected",
            ))?;
        }
        out.queue(ResetColor)?;

        out.flush()?;
        Ok(())
    }

    pub fn render_countdown(&self, count: u8) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = stdout();
        self.clear()?;

        let mid_col = self.width / 2;
        let mid_row = 10;

        out.queue(cursor::MoveTo(mid_col, mid_row))?
            .queue(SetForegroundColor(Color::Yellow))?
            .queue(Print(format!("{}", count)))?
            .queue(ResetColor)?;

        out.flush()?;
        Ok(())
    }

    pub fn render_keyboard_map(
        &self,
        out: &mut std::io::Stdout,
        row: u16,
        active_fingers: &[crate::guided_v2::Finger],
    ) -> Result<u16, Box<dyn std::error::Error>> {
        use crossterm::style::Color;
        use crate::guided_v2::Finger;

        let is_active = |finger: &Finger| active_fingers.contains(finger);

        // Number row
        out.queue(cursor::MoveTo(2, row))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftPinky) { Finger::LeftPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("` "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftRing) { Finger::LeftRing.color() } else { Color::DarkGrey }))?
            .queue(Print("2 "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftMiddle) { Finger::LeftMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print("3 "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftIndex) { Finger::LeftIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("4 5 "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightIndex) { Finger::RightIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("6 7 "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightMiddle) { Finger::RightMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print("8 "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightRing) { Finger::RightRing.color() } else { Color::DarkGrey }))?
            .queue(Print("9 "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightPinky) { Finger::RightPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("0 - = "))?
            .queue(ResetColor)?;

        // Top row
        out.queue(cursor::MoveTo(2, row + 1))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftPinky) { Finger::LeftPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("q "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftRing) { Finger::LeftRing.color() } else { Color::DarkGrey }))?
            .queue(Print("w "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftMiddle) { Finger::LeftMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print("e "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftIndex) { Finger::LeftIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("r t "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightIndex) { Finger::RightIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("y u "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightMiddle) { Finger::RightMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print("i "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightRing) { Finger::RightRing.color() } else { Color::DarkGrey }))?
            .queue(Print("o "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightPinky) { Finger::RightPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("p [ ] "))?
            .queue(ResetColor)?;

        // Home row
        out.queue(cursor::MoveTo(2, row + 2))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftPinky) { Finger::LeftPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("a "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftRing) { Finger::LeftRing.color() } else { Color::DarkGrey }))?
            .queue(Print("s "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftMiddle) { Finger::LeftMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print("d "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftIndex) { Finger::LeftIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("f g "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightIndex) { Finger::RightIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("h j "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightMiddle) { Finger::RightMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print("k "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightRing) { Finger::RightRing.color() } else { Color::DarkGrey }))?
            .queue(Print("l "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightPinky) { Finger::RightPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("; ' "))?
            .queue(ResetColor)?;

        // Bottom row
        out.queue(cursor::MoveTo(2, row + 3))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftPinky) { Finger::LeftPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("z "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftRing) { Finger::LeftRing.color() } else { Color::DarkGrey }))?
            .queue(Print("x "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftMiddle) { Finger::LeftMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print("c "))?
            .queue(SetForegroundColor(if is_active(&Finger::LeftIndex) { Finger::LeftIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("v b "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightIndex) { Finger::RightIndex.color() } else { Color::DarkGrey }))?
            .queue(Print("n m "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightMiddle) { Finger::RightMiddle.color() } else { Color::DarkGrey }))?
            .queue(Print(", "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightRing) { Finger::RightRing.color() } else { Color::DarkGrey }))?
            .queue(Print(". "))?
            .queue(SetForegroundColor(if is_active(&Finger::RightPinky) { Finger::RightPinky.color() } else { Color::DarkGrey }))?
            .queue(Print("/ "))?
            .queue(ResetColor)?;

        // Space bar
        out.queue(cursor::MoveTo(2, row + 4))?
            .queue(SetForegroundColor(if is_active(&Finger::Thumbs) { Finger::Thumbs.color() } else { Color::DarkGrey }))?
            .queue(Print("        [space]        "))?
            .queue(ResetColor)?;

        Ok(row + 6)
    }
}
