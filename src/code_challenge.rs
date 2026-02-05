use crate::display::Display;
use crate::input::{read_key, InputEvent};
use crate::stats::Stats;
use crossterm::{
    cursor,
    style::{Color, Print, SetForegroundColor},
    QueueableCommand,
};
use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use syntect::easy::HighlightLines;
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

pub struct CodeChallenge {
    code: String,
    language: String,
    typed: Vec<char>,
    errors: usize,
    start_time: Option<Instant>,
    last_typed_len: usize,
}

impl CodeChallenge {
    pub fn new(code: String, language: String) -> Self {
        Self {
            code,
            language,
            typed: Vec::new(),
            errors: 0,
            start_time: None,
            last_typed_len: 0,
        }
    }

    pub fn run(&mut self, display: &Display, stats: &Stats) -> Result<(), Box<dyn std::error::Error>> {
        let ps = SyntaxSet::load_defaults_newlines();
        let ts = ThemeSet::load_defaults();

        let syntax = ps.find_syntax_by_extension(&self.language)
            .unwrap_or_else(|| ps.find_syntax_plain_text());
        let theme = &ts.themes["base16-ocean.dark"];

        // Initial full render
        self.render_full(syntax, theme, &ps, display)?;

        loop {
            match read_key(Duration::from_millis(50))? {
                InputEvent::Char(ch) => {
                    if self.start_time.is_none() {
                        self.start_time = Some(Instant::now());
                    }

                    let code_chars: Vec<char> = self.code.chars().collect();
                    if self.typed.len() < code_chars.len() {
                        self.typed.push(ch);

                        if ch != code_chars[self.typed.len() - 1] {
                            self.errors += 1;
                        }

                        // Only update changed parts
                        self.update_incremental(syntax, theme, &ps)?;

                        // Check completion
                        if self.typed.len() == code_chars.len() {
                            self.show_summary(display, stats)?;
                            break;
                        }
                    }
                }
                InputEvent::Enter => {
                    if self.start_time.is_none() {
                        self.start_time = Some(Instant::now());
                    }

                    let code_chars: Vec<char> = self.code.chars().collect();
                    if self.typed.len() < code_chars.len() {
                        self.typed.push('\n');

                        if '\n' != code_chars[self.typed.len() - 1] {
                            self.errors += 1;
                        }

                        // Only update changed parts
                        self.update_incremental(syntax, theme, &ps)?;

                        // Check completion
                        if self.typed.len() == code_chars.len() {
                            self.show_summary(display, stats)?;
                            break;
                        }
                    }
                }
                InputEvent::Tab => {
                    if self.start_time.is_none() {
                        self.start_time = Some(Instant::now());
                    }

                    let code_chars: Vec<char> = self.code.chars().collect();
                    if self.typed.len() < code_chars.len() {
                        self.typed.push('\t');

                        if '\t' != code_chars[self.typed.len() - 1] {
                            self.errors += 1;
                        }

                        // Only update changed parts
                        self.update_incremental(syntax, theme, &ps)?;

                        // Check completion
                        if self.typed.len() == code_chars.len() {
                            self.show_summary(display, stats)?;
                            break;
                        }
                    }
                }
                InputEvent::Escape => break,
                _ => {}
            }
        }

        Ok(())
    }

    fn render_full(
        &self,
        syntax: &syntect::parsing::SyntaxReference,
        theme: &syntect::highlighting::Theme,
        ps: &SyntaxSet,
        display: &Display,
    ) -> Result<(), Box<dyn std::error::Error>> {
        display.clear()?;
        let mut out = stdout();

        // Calculate split position
        let term_width = crossterm::terminal::size()?.0 as usize;
        let split_pos = term_width / 2;

        // Title
        out.queue(cursor::MoveTo(0, 0))?
            .queue(Print("TypeHero - CodeJam"))?;

        // Stats line
        let elapsed = self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0);
        let accuracy = if self.typed.is_empty() {
            100.0
        } else {
            ((self.typed.len() - self.errors) as f64 / self.typed.len() as f64) * 100.0
        };

        out.queue(cursor::MoveTo(0, 1))?
            .queue(Print(format!("Time: {}s | Accuracy: {:.1}% | Errors: {}", elapsed, accuracy, self.errors)))?;

        out.queue(cursor::MoveTo(0, 2))?
            .queue(Print("─".repeat(term_width)))?;

        // Left pane: Target code with syntax highlighting
        out.queue(cursor::MoveTo(0, 3))?
            .queue(Print("TARGET:"))?;

        let mut highlighter = HighlightLines::new(syntax, theme);
        let mut row = 4;

        for line in LinesWithEndings::from(&self.code) {
            if row >= crossterm::terminal::size()?.1 - 2 {
                break;
            }

            out.queue(cursor::MoveTo(2, row))?;

            let ranges = highlighter.highlight_line(line, ps)?;
            for (style, text) in ranges {
                let color = style_to_crossterm_color(style);
                out.queue(SetForegroundColor(color))?
                    .queue(Print(text))?;
            }

            row += 1;
        }

        // Right pane: What they're typing
        out.queue(SetForegroundColor(Color::Reset))?;
        out.queue(cursor::MoveTo(split_pos as u16, 3))?
            .queue(Print("YOUR CODE:"))?;

        let typed_str: String = self.typed.iter().collect();
        let code_chars: Vec<char> = self.code.chars().collect();

        row = 4;
        let col = split_pos + 2;
        let mut highlighter_typed = HighlightLines::new(syntax, theme);

        for (i, line) in LinesWithEndings::from(&typed_str).enumerate() {
            if row >= crossterm::terminal::size()?.1 - 2 {
                break;
            }

            out.queue(cursor::MoveTo(col as u16, row))?;

            let ranges = highlighter_typed.highlight_line(line, ps)?;
            for (style, text) in ranges {
                for ch in text.chars() {
                    let idx = i;
                    let correct = self.typed.get(idx) == code_chars.get(idx);

                    let color = if correct {
                        style_to_crossterm_color(style)
                    } else {
                        Color::Red
                    };

                    out.queue(SetForegroundColor(color))?
                        .queue(Print(ch))?;
                }
            }

            row += 1;
        }

        // Cursor position
        out.queue(cursor::MoveTo(col as u16 + (self.typed.len() % 80) as u16, row))?;

        out.queue(SetForegroundColor(Color::Reset))?;
        out.queue(cursor::MoveTo(0, crossterm::terminal::size()?.1 - 1))?
            .queue(Print("Press ESC to quit"))?;

        out.flush()?;
        Ok(())
    }

    fn update_incremental(
        &mut self,
        _syntax: &syntect::parsing::SyntaxReference,
        _theme: &syntect::highlighting::Theme,
        _ps: &SyntaxSet,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut out = stdout();
        let term_width = crossterm::terminal::size()?.0 as usize;

        // Update stats line only
        let elapsed = self.start_time.map(|t| t.elapsed().as_secs()).unwrap_or(0);
        let accuracy = if self.typed.is_empty() {
            100.0
        } else {
            ((self.typed.len() - self.errors) as f64 / self.typed.len() as f64) * 100.0
        };

        out.queue(cursor::MoveTo(0, 1))?
            .queue(Print(format!(
                "Time: {}s | Accuracy: {:.1}% | Errors: {}{}",
                elapsed,
                accuracy,
                self.errors,
                " ".repeat(term_width.saturating_sub(50))
            )))?;

        // Update only the new character in right pane
        let split_pos = term_width / 2;
        let code_chars: Vec<char> = self.code.chars().collect();

        if !self.typed.is_empty() {
            let idx = self.typed.len() - 1;
            let ch = self.typed[idx];
            let correct = self.typed.get(idx) == code_chars.get(idx);

            // Calculate position for new character
            let typed_str: String = self.typed.iter().take(idx + 1).collect();
            let lines: Vec<&str> = typed_str.lines().collect();
            let row = 4 + lines.len().saturating_sub(1);
            let col = split_pos + 2 + lines.last().map(|l| l.len()).unwrap_or(0).saturating_sub(1);

            let color = if correct {
                Color::Green
            } else {
                Color::Red
            };

            out.queue(cursor::MoveTo(col as u16, row as u16))?
                .queue(SetForegroundColor(color))?
                .queue(Print(ch))?;

            self.last_typed_len = self.typed.len();
        }

        out.queue(SetForegroundColor(Color::Reset))?;
        out.flush()?;
        Ok(())
    }

    fn show_summary(&self, _display: &Display, _stats: &Stats) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Show completion summary
        Ok(())
    }
}

fn style_to_crossterm_color(style: Style) -> Color {
    Color::Rgb {
        r: style.foreground.r,
        g: style.foreground.g,
        b: style.foreground.b,
    }
}
