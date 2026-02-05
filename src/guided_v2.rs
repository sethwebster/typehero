use crate::display::Display;
use crate::input::{read_key, InputEvent};
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Finger {
    LeftPinky,
    LeftRing,
    LeftMiddle,
    LeftIndex,
    RightIndex,
    RightMiddle,
    RightRing,
    RightPinky,
    Thumbs,
}

impl Finger {
    pub fn name(&self) -> &str {
        match self {
            Finger::LeftPinky => "Left Pinky",
            Finger::LeftRing => "Left Ring",
            Finger::LeftMiddle => "Left Middle",
            Finger::LeftIndex => "Left Index",
            Finger::RightIndex => "Right Index",
            Finger::RightMiddle => "Right Middle",
            Finger::RightRing => "Right Ring",
            Finger::RightPinky => "Right Pinky",
            Finger::Thumbs => "Thumbs",
        }
    }

    pub fn color(&self) -> crossterm::style::Color {
        use crossterm::style::Color;
        match self {
            Finger::LeftPinky | Finger::RightPinky => Color::Magenta,
            Finger::LeftRing | Finger::RightRing => Color::Blue,
            Finger::LeftMiddle | Finger::RightMiddle => Color::Cyan,
            Finger::LeftIndex | Finger::RightIndex => Color::Green,
            Finger::Thumbs => Color::Yellow,
        }
    }

    pub fn keys(&self) -> Vec<char> {
        match self {
            Finger::LeftPinky => vec!['`', 'q', 'a', 'z'],
            Finger::LeftRing => vec!['w', 's', 'x'],
            Finger::LeftMiddle => vec!['e', 'd', 'c'],
            Finger::LeftIndex => vec!['r', 'f', 'v', 't', 'g', 'b'],
            Finger::RightIndex => vec!['y', 'h', 'n', 'u', 'j', 'm'],
            Finger::RightMiddle => vec!['i', 'k', ','],
            Finger::RightRing => vec!['o', 'l', '.'],
            Finger::RightPinky => vec!['p', ';', '/'],
            Finger::Thumbs => vec![' '],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Lesson {
    pub name: String,
    pub active_fingers: Vec<Finger>,
    pub allowed_keys: HashSet<char>,
    pub reps: usize,
}

impl Lesson {
    pub fn new(name: String, fingers: Vec<Finger>, reps: usize) -> Self {
        let mut allowed_keys = HashSet::new();
        for finger in &fingers {
            for key in finger.keys() {
                allowed_keys.insert(key);
            }
        }

        Self {
            name,
            active_fingers: fingers,
            allowed_keys,
            reps,
        }
    }

    pub fn generate_text(&self) -> String {
        let mut rng = rand::thread_rng();
        let keys: Vec<char> = self.allowed_keys.iter().copied().collect();
        let has_space = self.allowed_keys.contains(&' ');

        // Generate sequences based on available keys
        let mut segments = Vec::new();

        // Single key repetition
        for _ in 0..self.reps {
            let key = keys.choose(&mut rng).unwrap();
            if *key == ' ' {
                // Don't create triple spaces
                continue;
            }
            segments.push(format!("{}{}{}", key, key, key));
        }

        // Key pairs
        if keys.len() >= 2 {
            for _ in 0..self.reps {
                let k1 = keys.choose(&mut rng).unwrap();
                let k2 = keys.choose(&mut rng).unwrap();
                segments.push(format!("{}{}", k1, k2));
            }
        }

        // Join with space only if space is allowed, otherwise concatenate directly
        if has_space {
            segments.join(" ")
        } else {
            segments.join("")
        }
    }

    pub fn is_key_allowed(&self, ch: char) -> bool {
        self.allowed_keys.contains(&ch)
    }
}

#[derive(Debug, Clone)]
pub struct AttemptStats {
    pub duration_ms: u64,
    pub errors: usize,
    pub illegal_keys: usize,
    pub accuracy: f64,
}

pub struct GuidedPractice {
    lessons: Vec<Lesson>,
    current_lesson_idx: usize,
    attempts: Vec<AttemptStats>,
    lesson_stats: HashMap<usize, Vec<AttemptStats>>,
}

impl GuidedPractice {
    pub fn new() -> Self {
        let lessons = vec![
            // Phase 1: Index fingers (home row anchors)
            Lesson::new(
                "Left Index - Home Row".to_string(),
                vec![Finger::LeftIndex],
                3,
            ),
            Lesson::new(
                "Right Index - Home Row".to_string(),
                vec![Finger::RightIndex],
                3,
            ),
            Lesson::new(
                "Both Index Fingers".to_string(),
                vec![Finger::LeftIndex, Finger::RightIndex],
                3,
            ),
            // Phase 2: Add middle fingers
            Lesson::new(
                "Left Middle Finger".to_string(),
                vec![Finger::LeftMiddle],
                3,
            ),
            Lesson::new(
                "Right Middle Finger".to_string(),
                vec![Finger::RightMiddle],
                3,
            ),
            Lesson::new(
                "Index + Middle Fingers".to_string(),
                vec![
                    Finger::LeftIndex,
                    Finger::LeftMiddle,
                    Finger::RightIndex,
                    Finger::RightMiddle,
                ],
                3,
            ),
            // Phase 3: Add ring fingers
            Lesson::new(
                "Left Ring Finger".to_string(),
                vec![Finger::LeftRing],
                3,
            ),
            Lesson::new(
                "Right Ring Finger".to_string(),
                vec![Finger::RightRing],
                3,
            ),
            Lesson::new(
                "Index + Middle + Ring".to_string(),
                vec![
                    Finger::LeftIndex,
                    Finger::LeftMiddle,
                    Finger::LeftRing,
                    Finger::RightIndex,
                    Finger::RightMiddle,
                    Finger::RightRing,
                ],
                3,
            ),
            // Phase 4: Add pinkies
            Lesson::new(
                "Left Pinky Finger".to_string(),
                vec![Finger::LeftPinky],
                3,
            ),
            Lesson::new(
                "Right Pinky Finger".to_string(),
                vec![Finger::RightPinky],
                3,
            ),
            Lesson::new(
                "All Fingers (no space)".to_string(),
                vec![
                    Finger::LeftPinky,
                    Finger::LeftRing,
                    Finger::LeftMiddle,
                    Finger::LeftIndex,
                    Finger::RightIndex,
                    Finger::RightMiddle,
                    Finger::RightRing,
                    Finger::RightPinky,
                ],
                3,
            ),
            // Phase 5: Add space
            Lesson::new(
                "Add Space Bar".to_string(),
                vec![
                    Finger::LeftPinky,
                    Finger::LeftRing,
                    Finger::LeftMiddle,
                    Finger::LeftIndex,
                    Finger::RightIndex,
                    Finger::RightMiddle,
                    Finger::RightRing,
                    Finger::RightPinky,
                    Finger::Thumbs,
                ],
                5,
            ),
        ];

        Self {
            lessons,
            current_lesson_idx: 0,
            attempts: Vec::new(),
            lesson_stats: HashMap::new(),
        }
    }

    fn is_lesson_mastered(&self, lesson_idx: usize) -> bool {
        if let Some(attempts) = self.lesson_stats.get(&lesson_idx) {
            if attempts.len() < 3 {
                return false;
            }

            let recent: Vec<&AttemptStats> = attempts.iter().rev().take(3).collect();
            recent.iter().all(|a| a.accuracy >= 95.0 && a.illegal_keys == 0)
        } else {
            false
        }
    }

    fn get_mastery_status(&self, lesson_idx: usize) -> String {
        if self.is_lesson_mastered(lesson_idx) {
            "✓ MASTERED".to_string()
        } else if let Some(attempts) = self.lesson_stats.get(&lesson_idx) {
            if attempts.is_empty() {
                "Not attempted".to_string()
            } else {
                let recent = attempts.last().unwrap();
                if recent.illegal_keys > 0 {
                    format!("Constraint violation - {} illegal keys", recent.illegal_keys)
                } else if recent.accuracy >= 95.0 {
                    let count = attempts
                        .iter()
                        .rev()
                        .take(3)
                        .filter(|a| a.accuracy >= 95.0 && a.illegal_keys == 0)
                        .count();
                    format!("Progress: {}/3 mastery attempts", count)
                } else {
                    "Keep practicing".to_string()
                }
            }
        } else {
            "Not attempted".to_string()
        }
    }

    pub fn run(&mut self, display: &Display) -> Result<(), Box<dyn std::error::Error>> {
        self.load_attempts_for_current_lesson();
        let mut first_attempt_in_lesson = true;

        loop {
            let lesson = &self.lessons[self.current_lesson_idx].clone();
            let mastery_status = self.get_mastery_status(self.current_lesson_idx);
            let text = lesson.generate_text();

            // Only wait for Enter on first attempt in a lesson
            let mut should_continue = false;
            if first_attempt_in_lesson {
                loop {
                    display.render_guided_lesson(
                        &lesson.name,
                        &lesson.active_fingers,
                        &text,
                        &[],
                        0,
                        None,
                        self.current_lesson_idx,
                        self.lessons.len(),
                        &self.attempts,
                        &mastery_status,
                    )?;

                    match read_key(Duration::from_millis(50))? {
                        InputEvent::Enter => break,
                        InputEvent::CtrlN => {
                            if self.current_lesson_idx < self.lessons.len() - 1 {
                                self.current_lesson_idx += 1;
                                self.load_attempts_for_current_lesson();
                                first_attempt_in_lesson = true;
                                should_continue = true;
                                break;
                            }
                        }
                        InputEvent::CtrlP => {
                            if self.current_lesson_idx > 0 {
                                self.current_lesson_idx -= 1;
                                self.load_attempts_for_current_lesson();
                                first_attempt_in_lesson = true;
                                should_continue = true;
                                break;
                            }
                        }
                        InputEvent::Escape => return Ok(()),
                        _ => {}
                    }
                }
                if should_continue {
                    continue;
                }
                first_attempt_in_lesson = false;
            }

            // Countdown before practice
            for countdown in (1..=3).rev() {
                display.render_countdown(countdown)?;
                std::thread::sleep(Duration::from_millis(800));
            }

            let stats = self.practice_lesson(display, lesson, &text)?;
            self.attempts.push(stats.clone());

            self.lesson_stats
                .entry(self.current_lesson_idx)
                .or_insert_with(Vec::new)
                .push(stats);

            if self.is_lesson_mastered(self.current_lesson_idx) {
                std::thread::sleep(Duration::from_millis(1000));

                if self.current_lesson_idx < self.lessons.len() - 1 {
                    self.current_lesson_idx += 1;
                    self.load_attempts_for_current_lesson();
                    first_attempt_in_lesson = true;
                }
            }
        }
    }

    fn load_attempts_for_current_lesson(&mut self) {
        self.attempts = self
            .lesson_stats
            .get(&self.current_lesson_idx)
            .cloned()
            .unwrap_or_default();
    }

    fn practice_lesson(
        &self,
        display: &Display,
        lesson: &Lesson,
        text: &str,
    ) -> Result<AttemptStats, Box<dyn std::error::Error>> {
        let chars: Vec<char> = text.chars().collect();
        let mut typed: Vec<(char, bool)> = Vec::new();
        let mut current_pos = 0;
        let mut errors = 0;
        let mut illegal_keys = 0;
        let start_time = Instant::now();
        let mastery_status = self.get_mastery_status(self.current_lesson_idx);

        loop {
            let current_char = if current_pos < chars.len() {
                Some(chars[current_pos])
            } else {
                None
            };

            display.render_guided_lesson(
                &lesson.name,
                &lesson.active_fingers,
                text,
                &typed,
                current_pos,
                current_char,
                self.current_lesson_idx,
                self.lessons.len(),
                &self.attempts,
                &mastery_status,
            )?;

            match read_key(Duration::from_millis(50))? {
                InputEvent::Char(ch) => {
                    if current_pos >= chars.len() {
                        continue;
                    }

                    // CONSTRAINT ENFORCEMENT: Only accept keys from allowed set
                    if !lesson.is_key_allowed(ch) {
                        illegal_keys += 1;
                        // Don't advance - reject the input
                        continue;
                    }

                    let expected = chars[current_pos];
                    let correct = ch == expected;

                    typed.push((ch, correct));
                    if !correct {
                        errors += 1;
                    }

                    current_pos += 1;

                    if current_pos >= chars.len() {
                        break;
                    }
                }
                InputEvent::Escape => {
                    return Err("Quit early".into());
                }
                _ => {}
            }
        }

        let duration = start_time.elapsed();
        let accuracy = if typed.is_empty() {
            100.0
        } else {
            ((typed.len() - errors) as f64 / typed.len() as f64) * 100.0
        };

        Ok(AttemptStats {
            duration_ms: duration.as_millis() as u64,
            errors,
            illegal_keys,
            accuracy,
        })
    }
}
