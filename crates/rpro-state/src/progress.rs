//! Per-exercise progress state.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Per-exercise status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExerciseStatus {
    /// Not yet reached.
    Locked,
    /// User is currently on this exercise (passing this exercise
    /// unlocks the next).
    Current,
    /// Exercise compiled and ran successfully.
    Done,
    /// Skipped explicitly by the user (`rpro exercise skip`).
    /// Counts as not-done for progress percentage.
    Skipped,
}

/// User progress across all exercises.
///
/// Stored as JSON at `~/.rustlings-pro/progress.json`. Map key is
/// the exercise id (`area/name`); BTreeMap so the JSON output is
/// stable for diff-friendliness.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Progress {
    /// Per-exercise entries.
    pub entries: BTreeMap<String, ProgressEntry>,
}

/// One row in the progress map.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProgressEntry {
    /// Current state.
    pub status: ExerciseStatus,
    /// First time the user opened or attempted this exercise.
    /// `None` until they touch it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub started_at: Option<DateTime<Utc>>,
    /// When the exercise was last marked Done. `None` for
    /// non-Done states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<DateTime<Utc>>,
    /// Total compile-attempts for this exercise (for "you tried 7
    /// times, here's a hint" kind of nudges).
    #[serde(default)]
    pub attempts: u32,
}

impl Progress {
    /// Mark `id` as the current exercise. Records `started_at` if
    /// this is the first time it's been touched.
    pub fn set_current(&mut self, id: &str) {
        let now = Utc::now();
        let entry = self.entries.entry(id.to_string()).or_insert(ProgressEntry {
            status: ExerciseStatus::Current,
            started_at: Some(now),
            completed_at: None,
            attempts: 0,
        });
        if entry.started_at.is_none() {
            entry.started_at = Some(now);
        }
        entry.status = ExerciseStatus::Current;
    }

    /// Mark `id` as Done. Records `completed_at`.
    pub fn set_done(&mut self, id: &str) {
        let now = Utc::now();
        let entry = self.entries.entry(id.to_string()).or_insert(ProgressEntry {
            status: ExerciseStatus::Done,
            started_at: Some(now),
            completed_at: Some(now),
            attempts: 0,
        });
        entry.status = ExerciseStatus::Done;
        entry.completed_at = Some(now);
    }

    /// Increment the attempt counter for `id`.
    pub fn record_attempt(&mut self, id: &str) {
        let entry = self.entries.entry(id.to_string()).or_insert(ProgressEntry {
            status: ExerciseStatus::Current,
            started_at: Some(Utc::now()),
            completed_at: None,
            attempts: 0,
        });
        entry.attempts = entry.attempts.saturating_add(1);
    }

    /// Count of Done exercises.
    #[must_use]
    pub fn done_count(&self) -> usize {
        self.entries
            .values()
            .filter(|e| e.status == ExerciseStatus::Done)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_current_creates_entry() {
        let mut p = Progress::default();
        p.set_current("ownership/01_move");
        let e = p.entries.get("ownership/01_move").unwrap();
        assert_eq!(e.status, ExerciseStatus::Current);
        assert!(e.started_at.is_some());
    }

    #[test]
    fn set_done_after_current() {
        let mut p = Progress::default();
        p.set_current("a/b");
        let started = p.entries["a/b"].started_at;
        p.set_done("a/b");
        let e = &p.entries["a/b"];
        assert_eq!(e.status, ExerciseStatus::Done);
        assert_eq!(e.started_at, started, "started_at preserved across set_done");
        assert!(e.completed_at.is_some());
    }

    #[test]
    fn record_attempt_increments() {
        let mut p = Progress::default();
        p.record_attempt("a/b");
        p.record_attempt("a/b");
        p.record_attempt("a/b");
        assert_eq!(p.entries["a/b"].attempts, 3);
    }

    #[test]
    fn done_count() {
        let mut p = Progress::default();
        p.set_done("a/1");
        p.set_done("a/2");
        p.set_current("a/3");
        assert_eq!(p.done_count(), 2);
    }
}
