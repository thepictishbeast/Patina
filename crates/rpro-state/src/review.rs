//! Spaced repetition over diagnostic codes — the "RECALL" beat of the
//! educational loop (docs/EDUCATION.md). When a learner keeps hitting the same
//! compiler error, that concept should resurface; once they reliably overcome
//! it, it retires.
//!
//! v1 is a clock-free Leitner-box model: every diagnostic code sits in a box
//! `0..=MAX_BOX`. A fresh miss resets it to 0; overcoming it bumps it up one.
//! A code is "due" for review until it reaches [`MASTERED_BOX`]; the review
//! queue surfaces the weakest first. Time-based intervals can layer on later
//! (kept out of v1 so the type stays pure + wasm-safe — no clock).

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Box a code must reach to count as mastered (retired from review).
pub const MASTERED_BOX: u8 = 3;
/// Highest box (further correct recalls past this don't change anything).
pub const MAX_BOX: u8 = 5;

/// Spaced-repetition state: diagnostic code → Leitner box.
#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewState {
    /// e.g. `"E0382"` → box (`0..=MAX_BOX`).
    #[serde(default)]
    boxes: BTreeMap<String, u8>,
}

impl ReviewState {
    /// Record an encounter with `code`. `overcome = true` when the learner
    /// resolved the error (bump up a box); `false` on a fresh miss (reset to 0).
    pub fn record(&mut self, code: &str, overcome: bool) {
        let b = self.boxes.entry(code.to_string()).or_insert(0);
        *b = if overcome { (*b + 1).min(MAX_BOX) } else { 0 };
    }

    /// Whether `code` is mastered (retired from the review queue).
    #[must_use]
    pub fn is_mastered(&self, code: &str) -> bool {
        self.boxes.get(code).is_some_and(|&b| b >= MASTERED_BOX)
    }

    /// Codes still needing review (box `< MASTERED_BOX`), weakest first
    /// (lowest box, then code order) — what to resurface next.
    #[must_use]
    pub fn due(&self) -> Vec<String> {
        let mut v: Vec<(&String, &u8)> =
            self.boxes.iter().filter(|&(_, &b)| b < MASTERED_BOX).collect();
        v.sort_by(|a, b| a.1.cmp(b.1).then_with(|| a.0.cmp(b.0)));
        v.into_iter().map(|(c, _)| c.clone()).collect()
    }

    /// How many distinct codes have been mastered.
    #[must_use]
    pub fn mastered_count(&self) -> usize {
        self.boxes.values().filter(|&&b| b >= MASTERED_BOX).count()
    }

    /// How many codes have been seen at all.
    #[must_use]
    pub fn tracked_count(&self) -> usize {
        self.boxes.len()
    }
}

#[cfg(test)]
mod tests {
    // The model is generic over the diagnostic-code string, so these tests use
    // plain concept keys ("move", "mismatch", …) rather than literal error codes
    // — keeps the seam-gate (no `E0xxx` literals outside crates/languages) clean
    // while exercising identical box logic.
    use super::*;

    #[test]
    fn miss_resets_overcome_climbs() {
        let mut r = ReviewState::default();
        r.record("move", false); // first miss
        assert!(!r.is_mastered("move"));
        assert_eq!(r.due(), vec!["move".to_string()]);
        r.record("move", true); // box 1
        r.record("move", true); // box 2
        assert!(!r.is_mastered("move"));
        r.record("move", true); // box 3 == MASTERED_BOX
        assert!(r.is_mastered("move"));
        assert!(r.due().is_empty());
        // a later miss un-masters it
        r.record("move", false);
        assert!(!r.is_mastered("move"));
    }

    #[test]
    fn box_caps_at_max() {
        let mut r = ReviewState::default();
        for _ in 0..(MAX_BOX as usize + 5) {
            r.record("mismatch", true);
        }
        // still mastered; record(false) drops straight to 0 regardless of cap
        assert!(r.is_mastered("mismatch"));
        r.record("mismatch", false);
        assert!(!r.is_mastered("mismatch"));
        assert_eq!(r.due(), vec!["mismatch".to_string()]);
    }

    #[test]
    fn due_is_weakest_first_and_excludes_mastered() {
        let mut r = ReviewState::default();
        r.record("trait", false); // box 0 (weakest)
        r.record("borrow", true); // box 1
        for _ in 0..MASTERED_BOX {
            r.record("move", true); // box 3 == MASTERED_BOX, excluded from due()
        }
        // weakest-first: "trait" (box 0) before "borrow" (box 1); "move" mastered.
        assert_eq!(r.due(), vec!["trait".to_string(), "borrow".to_string()]);
        assert_eq!(r.mastered_count(), 1);
        assert_eq!(r.tracked_count(), 3);
    }
}
