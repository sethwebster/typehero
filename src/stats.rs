use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
    pub timestamp: DateTime<Utc>,
    pub duration_secs: f64,
    pub total_chars: usize,
    pub errors: usize,
    pub raw_wpm: f64,
    pub accuracy: f64,
    pub adjusted_wpm: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub sessions: Vec<SessionSummary>,
    pub total_keys: usize,
    pub total_errors: usize,
    pub key_errors: HashMap<char, usize>,
    pub bigram_times: HashMap<String, Vec<u64>>, // Store as micros for serialization
}

impl Default for Stats {
    fn default() -> Self {
        Self {
            sessions: Vec::new(),
            total_keys: 0,
            total_errors: 0,
            key_errors: HashMap::new(),
            bigram_times: HashMap::new(),
        }
    }
}

impl Stats {
    fn stats_file() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".typefix_stats.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::stats_file();
        if path.exists() {
            fs::read_to_string(&path)
                .ok()
                .and_then(|s| serde_json::from_str(&s).ok())
                .unwrap_or_default()
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::stats_file();
        let json = serde_json::to_string_pretty(self)?;
        fs::write(&path, json)?;
        Ok(())
    }

    pub fn add_session(&mut self, summary: SessionSummary) {
        self.sessions.push(summary);
    }

    pub fn record_keystroke(&mut self, ch: char, correct: bool) {
        self.total_keys += 1;
        if !correct {
            self.total_errors += 1;
            *self.key_errors.entry(ch).or_insert(0) += 1;
        }
    }

    pub fn record_bigram(&mut self, bigram: (char, char), duration: Duration) {
        let key = format!("{}{}", bigram.0, bigram.1);
        self.bigram_times
            .entry(key)
            .or_insert_with(Vec::new)
            .push(duration.as_micros() as u64);
    }

    pub fn slowest_bigrams(&self, limit: usize) -> Vec<(String, f64)> {
        let mut avg_times: Vec<(String, f64)> = self
            .bigram_times
            .iter()
            .filter(|(_, times)| times.len() >= 3) // Need at least 3 samples
            .map(|(bigram, times)| {
                let avg = times.iter().sum::<u64>() as f64 / times.len() as f64;
                (bigram.clone(), avg / 1000.0) // Convert to ms
            })
            .collect();

        avg_times.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        avg_times.into_iter().take(limit).collect()
    }

    pub fn most_error_prone_keys(&self, limit: usize) -> Vec<(char, f64)> {
        let mut error_rates: Vec<(char, f64)> = self
            .key_errors
            .iter()
            .filter(|(_, &count)| count >= 3) // Need at least 3 errors
            .map(|(&ch, &errors)| {
                let rate = (errors as f64 / self.total_keys as f64) * 100.0;
                (ch, rate)
            })
            .collect();

        error_rates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        error_rates.into_iter().take(limit).collect()
    }

    pub fn lifetime_accuracy(&self) -> f64 {
        if self.total_keys == 0 {
            return 100.0;
        }
        ((self.total_keys - self.total_errors) as f64 / self.total_keys as f64) * 100.0
    }

    pub fn average_adjusted_wpm(&self) -> f64 {
        if self.sessions.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.sessions.iter().map(|s| s.adjusted_wpm).sum();
        sum / self.sessions.len() as f64
    }
}
