use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Result as SqlResult};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

pub struct Stats {
    conn: Connection,
}

impl Stats {
    fn db_file() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".typehero.db");
        path
    }

    fn legacy_json_file() -> PathBuf {
        let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push(".typehero_stats.json");
        path
    }

    pub fn load() -> Self {
        let path = Self::db_file();
        let conn = Connection::open(&path).expect("Failed to open database");

        // Create tables if they don't exist
        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                duration_secs REAL NOT NULL,
                total_chars INTEGER NOT NULL,
                errors INTEGER NOT NULL,
                raw_wpm REAL NOT NULL,
                accuracy REAL NOT NULL,
                adjusted_wpm REAL NOT NULL
            );

            CREATE INDEX IF NOT EXISTS idx_sessions_timestamp ON sessions(timestamp);

            CREATE TABLE IF NOT EXISTS bigrams (
                pair TEXT PRIMARY KEY,
                total_time_us INTEGER NOT NULL,
                sample_count INTEGER NOT NULL,
                avg_time_ms REAL NOT NULL
            );

            CREATE TABLE IF NOT EXISTS key_errors (
                key TEXT PRIMARY KEY,
                error_count INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
            ",
        )
        .expect("Failed to create tables");

        let mut stats = Self { conn };

        // Migrate from legacy JSON if it exists and DB is empty
        if Self::legacy_json_file().exists() {
            if stats.is_empty() {
                stats.migrate_from_json();
            }
        }

        stats
    }

    fn is_empty(&self) -> bool {
        let count: i64 = self
            .conn
            .query_row("SELECT COUNT(*) FROM sessions", [], |row| row.get(0))
            .unwrap_or(0);
        count == 0
    }

    fn migrate_from_json(&mut self) {
        #[derive(Deserialize)]
        struct LegacyStats {
            sessions: Vec<SessionSummary>,
            total_keys: usize,
            total_errors: usize,
            key_errors: HashMap<char, usize>,
            bigram_times: HashMap<String, Vec<u64>>,
        }

        let json_path = Self::legacy_json_file();
        if let Ok(content) = std::fs::read_to_string(&json_path) {
            if let Ok(legacy) = serde_json::from_str::<LegacyStats>(&content) {
                let session_count = legacy.sessions.len();

                // Import sessions
                for session in legacy.sessions {
                    let _ = self.add_session(session);
                }

                // Import key errors
                for (ch, count) in legacy.key_errors {
                    let _ = self.conn.execute(
                        "INSERT OR REPLACE INTO key_errors (key, error_count) VALUES (?1, ?2)",
                        params![ch.to_string(), count],
                    );
                }

                // Import bigrams (compute averages)
                for (pair, times) in legacy.bigram_times {
                    if !times.is_empty() {
                        let total: u64 = times.iter().sum();
                        let count = times.len();
                        let avg_ms = (total as f64) / (count as f64) / 1000.0;

                        let _ = self.conn.execute(
                            "INSERT OR REPLACE INTO bigrams (pair, total_time_us, sample_count, avg_time_ms)
                             VALUES (?1, ?2, ?3, ?4)",
                            params![pair, total as i64, count as i64, avg_ms],
                        );
                    }
                }

                // Store total counts in metadata
                let _ = self.conn.execute(
                    "INSERT OR REPLACE INTO metadata (key, value) VALUES ('total_keys', ?1)",
                    params![legacy.total_keys.to_string()],
                );
                let _ = self.conn.execute(
                    "INSERT OR REPLACE INTO metadata (key, value) VALUES ('total_errors', ?1)",
                    params![legacy.total_errors.to_string()],
                );

                eprintln!("Migrated {} sessions from JSON to SQLite", session_count);
            }
        }
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // SQLite auto-saves, but we can explicitly sync
        Ok(())
    }

    pub fn add_session(&mut self, summary: SessionSummary) -> SqlResult<()> {
        self.conn.execute(
            "INSERT INTO sessions (timestamp, duration_secs, total_chars, errors, raw_wpm, accuracy, adjusted_wpm)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                summary.timestamp.to_rfc3339(),
                summary.duration_secs,
                summary.total_chars,
                summary.errors,
                summary.raw_wpm,
                summary.accuracy,
                summary.adjusted_wpm,
            ],
        )?;
        Ok(())
    }

    pub fn record_keystroke(&mut self, ch: char, correct: bool) {
        // Increment total_keys
        let total_keys = self.total_keys() + 1;
        let _ = self.conn.execute(
            "INSERT OR REPLACE INTO metadata (key, value) VALUES ('total_keys', ?1)",
            params![total_keys.to_string()],
        );

        if !correct {
            // Increment total_errors
            let total_errors = self.total_errors() + 1;
            let _ = self.conn.execute(
                "INSERT OR REPLACE INTO metadata (key, value) VALUES ('total_errors', ?1)",
                params![total_errors.to_string()],
            );

            // Increment key-specific error count
            let key = ch.to_string();
            self.conn
                .execute(
                    "INSERT INTO key_errors (key, error_count) VALUES (?1, 1)
                     ON CONFLICT(key) DO UPDATE SET error_count = error_count + 1",
                    params![key],
                )
                .ok();
        }
    }

    pub fn record_bigram(&mut self, bigram: (char, char), duration: Duration) {
        let pair = format!("{}{}", bigram.0, bigram.1);
        let time_us = duration.as_micros() as i64;

        // Get current stats
        let (current_total, current_count): (i64, i64) = self
            .conn
            .query_row(
                "SELECT total_time_us, sample_count FROM bigrams WHERE pair = ?1",
                params![pair],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap_or((0, 0));

        let new_total = current_total + time_us;
        let new_count = current_count + 1;
        let new_avg_ms = (new_total as f64) / (new_count as f64) / 1000.0;

        self.conn
            .execute(
                "INSERT OR REPLACE INTO bigrams (pair, total_time_us, sample_count, avg_time_ms)
                 VALUES (?1, ?2, ?3, ?4)",
                params![pair, new_total, new_count, new_avg_ms],
            )
            .ok();
    }

    pub fn slowest_bigrams(&self, limit: usize) -> Vec<(String, f64)> {
        let mut stmt = self
            .conn
            .prepare("SELECT pair, avg_time_ms FROM bigrams WHERE sample_count >= 3 ORDER BY avg_time_ms DESC LIMIT ?1")
            .expect("Failed to prepare query");

        let rows = stmt
            .query_map(params![limit as i64], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
            })
            .expect("Failed to query bigrams");

        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn most_error_prone_keys(&self, limit: usize) -> Vec<(char, f64)> {
        let total_keys = self.total_keys() as f64;
        if total_keys == 0.0 {
            return Vec::new();
        }

        let mut stmt = self
            .conn
            .prepare("SELECT key, error_count FROM key_errors WHERE error_count >= 3 ORDER BY error_count DESC LIMIT ?1")
            .expect("Failed to prepare query");

        let rows = stmt
            .query_map(params![limit as i64], |row| {
                let key: String = row.get(0)?;
                let count: i64 = row.get(1)?;
                let rate = (count as f64 / total_keys) * 100.0;
                Ok((key.chars().next().unwrap_or(' '), rate))
            })
            .expect("Failed to query key errors");

        rows.filter_map(|r| r.ok()).collect()
    }

    pub fn lifetime_accuracy(&self) -> f64 {
        let total_keys = self.total_keys();
        let total_errors = self.total_errors();

        if total_keys == 0 {
            return 100.0;
        }
        ((total_keys - total_errors) as f64 / total_keys as f64) * 100.0
    }

    pub fn average_adjusted_wpm(&self) -> f64 {
        self.conn
            .query_row(
                "SELECT AVG(adjusted_wpm) FROM sessions",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0.0)
    }

    pub fn sessions(&self) -> Vec<SessionSummary> {
        let mut stmt = self
            .conn
            .prepare("SELECT timestamp, duration_secs, total_chars, errors, raw_wpm, accuracy, adjusted_wpm FROM sessions ORDER BY timestamp DESC")
            .expect("Failed to prepare query");

        let rows = stmt
            .query_map([], |row| {
                Ok(SessionSummary {
                    timestamp: DateTime::parse_from_rfc3339(&row.get::<_, String>(0)?)
                        .unwrap()
                        .with_timezone(&Utc),
                    duration_secs: row.get(1)?,
                    total_chars: row.get::<_, i64>(2)? as usize,
                    errors: row.get::<_, i64>(3)? as usize,
                    raw_wpm: row.get(4)?,
                    accuracy: row.get(5)?,
                    adjusted_wpm: row.get(6)?,
                })
            })
            .expect("Failed to query sessions");

        rows.filter_map(|r| r.ok()).collect()
    }

    fn total_keys(&self) -> usize {
        self.conn
            .query_row(
                "SELECT value FROM metadata WHERE key = 'total_keys'",
                [],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }

    fn total_errors(&self) -> usize {
        self.conn
            .query_row(
                "SELECT value FROM metadata WHERE key = 'total_errors'",
                [],
                |row| row.get::<_, String>(0),
            )
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(0)
    }

    pub fn total_keys_public(&self) -> usize {
        self.total_keys()
    }

    pub fn total_errors_public(&self) -> usize {
        self.total_errors()
    }
}

impl Clone for Stats {
    fn clone(&self) -> Self {
        // For cloning, we reconnect to the same database
        Self::load()
    }
}
