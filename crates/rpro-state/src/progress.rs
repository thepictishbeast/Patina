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
    /// Spaced-repetition state over diagnostic codes (the RECALL beat).
    /// `#[serde(default)]` so progress files written before this field
    /// existed still deserialize.
    #[serde(default)]
    pub reviews: crate::review::ReviewState,
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
        let entry = self
            .entries
            .entry(id.to_string())
            .or_insert_with(|| ProgressEntry {
                status: ExerciseStatus::Current,
                started_at: Some(Utc::now()),
                completed_at: None,
                attempts: 0,
            });
        entry.attempts = entry.attempts.saturating_add(1);
    }

    /// Mark `id` as Skipped — the learner chose to move on. Counts as not-done
    /// for progress; clears any completion time.
    pub fn set_skipped(&mut self, id: &str) {
        let now = Utc::now();
        let entry = self.entries.entry(id.to_string()).or_insert(ProgressEntry {
            status: ExerciseStatus::Skipped,
            started_at: Some(now),
            completed_at: None,
            attempts: 0,
        });
        entry.status = ExerciseStatus::Skipped;
        entry.completed_at = None;
    }

    /// Reset `id` back to a fresh Current: clears Done/Skipped, the completion
    /// time, and the attempt count so the learner can re-attempt from scratch.
    /// Keeps `started_at` (their first touch) when it was already recorded.
    pub fn reset(&mut self, id: &str) {
        let now = Utc::now();
        let entry = self.entries.entry(id.to_string()).or_insert(ProgressEntry {
            status: ExerciseStatus::Current,
            started_at: Some(now),
            completed_at: None,
            attempts: 0,
        });
        entry.status = ExerciseStatus::Current;
        entry.completed_at = None;
        entry.attempts = 0;
    }

    /// Jump the learner to `id`, keeping the **single-Current** invariant: any
    /// *other* exercise that was Current is demoted to Locked (it was set aside,
    /// not completed — Locked is the neutral inactive state; `attempts`/
    /// `started_at` are untouched), then `id` is made Current.
    ///
    /// This deliberately never changes a `Done` entry, so the done-count/gauge
    /// cannot regress. Callers select only *not-yet-Done* exercises (revisiting a
    /// completed one is [`reset`](Self::reset)'s job); `select` is the pure state
    /// transition and assumes the caller has validated `id`.
    pub fn select(&mut self, id: &str) {
        for (eid, entry) in &mut self.entries {
            if eid != id && entry.status == ExerciseStatus::Current {
                entry.status = ExerciseStatus::Locked;
            }
        }
        self.set_current(id);
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
        assert_eq!(
            e.started_at, started,
            "started_at preserved across set_done"
        );
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
    fn select_keeps_single_current_and_preserves_done() {
        let mut p = Progress::default();
        p.set_current("a/1");
        p.set_done("a/1"); // a/1 Done
        p.set_current("a/2"); // a/2 Current
        p.record_attempt("a/2"); // a/2 has history
        // Jump to a never-touched exercise.
        p.select("a/3");
        // Exactly one Current, and it's the selected one.
        let currents: Vec<_> = p
            .entries
            .iter()
            .filter(|(_, e)| e.status == ExerciseStatus::Current)
            .map(|(id, _)| id.clone())
            .collect();
        assert_eq!(currents, vec!["a/3".to_string()], "single Current = selection");
        // The previous Current was demoted to Locked, NOT lost (attempts kept).
        assert_eq!(p.entries["a/2"].status, ExerciseStatus::Locked);
        assert_eq!(p.entries["a/2"].attempts, 1, "demote preserves attempts");
        // The Done entry is untouched → gauge cannot regress.
        assert_eq!(p.entries["a/1"].status, ExerciseStatus::Done);
        assert_eq!(p.done_count(), 1);
    }

    #[test]
    fn select_back_to_a_demoted_exercise_restores_it_current() {
        let mut p = Progress::default();
        p.set_current("a/1");
        p.select("a/2"); // a/1 -> Locked, a/2 Current
        p.select("a/1"); // back to a/1
        assert_eq!(p.entries["a/1"].status, ExerciseStatus::Current);
        assert_eq!(p.entries["a/2"].status, ExerciseStatus::Locked);
    }

    #[test]
    fn done_count() {
        let mut p = Progress::default();
        p.set_done("a/1");
        p.set_done("a/2");
        p.set_current("a/3");
        assert_eq!(p.done_count(), 2);
    }

    #[test]
    fn set_skipped_marks_and_excludes_from_done() {
        let mut p = Progress::default();
        p.set_current("a/1");
        p.set_skipped("a/1");
        assert_eq!(p.entries["a/1"].status, ExerciseStatus::Skipped);
        assert!(p.entries["a/1"].completed_at.is_none());
        assert_eq!(p.done_count(), 0, "skipped never counts as done");
    }

    #[test]
    fn reset_clears_done_completion_and_attempts() {
        let mut p = Progress::default();
        p.record_attempt("a/1");
        p.record_attempt("a/1");
        p.set_done("a/1");
        let started = p.entries["a/1"].started_at;
        p.reset("a/1");
        let e = &p.entries["a/1"];
        assert_eq!(e.status, ExerciseStatus::Current);
        assert_eq!(e.attempts, 0);
        assert!(e.completed_at.is_none());
        assert_eq!(e.started_at, started, "first-touch time is preserved across reset");
    }
}
