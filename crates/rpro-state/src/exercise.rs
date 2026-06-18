//! Exercise metadata — including the typed book references.
//!
//! This is the schema that makes Rustlings-Pro's "book references on
//! every exercise" promise concrete. Every exercise file (`*.rs`)
//! has a sibling `*.toml` that deserializes into `ExerciseMetadata`.

use serde::{Deserialize, Serialize};

/// A pointer from an exercise to a specific spot in the book.
///
/// Re-exported from [`rpro_lang`]: the neutral, language-agnostic type
/// lives in the seam crate so every surface speaks the same `BookRef`.
/// (Defined here originally; moved during the seam refactor.)
pub use rpro_lang::BookRef;

/// Difficulty hint — drives ordering on `rpro exercise list` and
/// the badge colour in the TUI.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    /// Suitable for a learner on day 1.
    Beginner,
    /// Borrow-checker, lifetimes, basic trait usage.
    Intermediate,
    /// Async, unsafe, macro-rules, advanced trait design.
    Advanced,
}

/// Typed exercise metadata. Lives in the `*.toml` sibling next to
/// each `*.rs` exercise file.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseMetadata {
    /// Stable identifier — `area/short-name`, e.g.
    /// `"ownership/01_move"`. Never changes once the exercise
    /// ships; renaming the file is a breaking change.
    pub id: String,

    /// Human-readable title shown in the dashboard + exercise
    /// header. One short sentence.
    pub title: String,

    /// Beginner / intermediate / advanced.
    pub difficulty: Difficulty,

    /// Rough completion time in minutes. Used for the progress
    /// summary's "estimated remaining time" line.
    pub estimated_minutes: u32,

    /// One-token concept tag (`"move-semantics"`, `"async-await"`,
    /// `"trait-bounds"`). Used for grouping in the dashboard and
    /// for matching exercises to book sections in `rpro hint`.
    pub concept: String,

    /// Book references — what to read when stuck. ≥1 required.
    pub book_refs: Vec<BookRef>,

    /// Error code the exercise is designed around (a language-specific
    /// code). The runner cross-checks against this; an unexpected error
    /// gets a "you may have changed more than the exercise asked" hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_error_code: Option<String>,

    /// Hidden by default. Surfaced only when the user explicitly
    /// asks via `rpro exercise hint --solution`. One sentence
    /// pointing at the fix shape, never the full diff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub solution_outline: Option<String>,
}

impl ExerciseMetadata {
    /// Validate the metadata. Returns the first reason the
    /// exercise should not load.
    ///
    /// # Errors
    /// Reason for rejection.
    pub fn validate(&self) -> Result<(), String> {
        if self.id.trim().is_empty() {
            return Err("exercise id is empty".into());
        }
        if !self.id.contains('/') {
            return Err(format!(
                "exercise id '{}' missing '/' separator (expected `area/name`)",
                self.id
            ));
        }
        if self.title.trim().is_empty() {
            return Err("exercise title is empty".into());
        }
        if self.book_refs.is_empty() {
            return Err(format!(
                "exercise '{}' has no book_refs — every exercise must point at the book",
                self.id
            ));
        }
        for r in &self.book_refs {
            if r.chapter.trim().is_empty() {
                return Err(format!(
                    "exercise '{}' has a book_ref with empty chapter",
                    self.id
                ));
            }
            if r.why.trim().is_empty() {
                return Err(format!(
                    "exercise '{}' has a book_ref to '{}' with empty `why`",
                    self.id, r.chapter
                ));
            }
        }
        Ok(())
    }

    /// The hint ladder for this exercise — pure, so every surface (web, TUI, CLI)
    /// shows the *same* escalating guidance. Returns `(clamped_level, max_level,
    /// text)`.
    ///
    /// * **1** — the concept + a nudge to read the compiler's location (`-->`) and
    ///   `help:` lines (the by-hand-error habit).
    /// * **2** — the expected error code, when known, so the learner can look it
    ///   up — never the fix itself.
    /// * **3** — the solution OUTLINE, last resort, returned ONLY at the top rung.
    ///   `max_level` is 3 when an outline exists, else 2, so levels 1–2 can never
    ///   contain it.
    ///
    /// The tutor guides; it never auto-types the fix (see `docs/EDUCATION.md`).
    #[must_use]
    pub fn hint(&self, requested: u8) -> (u8, u8, String) {
        let max_level: u8 = if self.solution_outline.is_some() { 3 } else { 2 };
        let level = requested.clamp(1, max_level);
        let text = match level {
            1 => format!(
                "Concept: {}. Start with the book refs, then run it and read the \
                 compiler's `-->` line (the location) and the `help:` line — that \
                 usually names the fix.",
                self.concept
            ),
            2 => self.expected_error_code.as_ref().map_or_else(
                || "Read the first error top-to-bottom: the `-->` line is the \
                    location, the `help:` line is usually the fix."
                    .to_string(),
                |c| {
                    format!(
                        "Expect error {c}. Ask for its full explanation, then look at \
                         exactly which value or line it flags."
                    )
                },
            ),
            _ => self.solution_outline.as_ref().map_or_else(
                || "No solution outline recorded — work from the error's `help:` line.".to_string(),
                |s| format!("Solution outline (last resort): {s}"),
            ),
        };
        (level, max_level, text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fx() -> ExerciseMetadata {
        ExerciseMetadata {
            id: "ownership/01_move".into(),
            title: "Move semantics: when assignment transfers ownership".into(),
            difficulty: Difficulty::Beginner,
            estimated_minutes: 8,
            concept: "move-semantics".into(),
            book_refs: vec![BookRef {
                chapter: "ch04-01-what-is-ownership".into(),
                anchor: Some("ownership-rules".into()),
                why: "The three ownership rules.".into(),
            }],
            expected_error_code: Some("EXXXX".into()),
            solution_outline: Some("Use clone() to keep both valid.".into()),
        }
    }

    #[test]
    fn happy_path_validates() {
        fx().validate().unwrap();
    }

    #[test]
    fn empty_book_refs_rejected() {
        let mut e = fx();
        e.book_refs.clear();
        assert!(e.validate().is_err());
    }

    #[test]
    fn id_without_slash_rejected() {
        let mut e = fx();
        e.id = "no-slash".into();
        assert!(e.validate().is_err());
    }

    #[test]
    fn book_ref_round_trips_toml() {
        let m = fx();
        let s = toml::to_string_pretty(&m).unwrap();
        let back: ExerciseMetadata = toml::from_str(&s).unwrap();
        assert_eq!(m, back);
    }

    #[test]
    fn hint_ladder_escalates_and_gates_solution() {
        let m = fx(); // has a solution outline + expected code "EXXXX"
        let (l1, max, t1) = m.hint(1);
        assert_eq!((l1, max), (1, 3));
        assert!(!t1.contains("clone"), "L1 must not leak the solution");
        let (_, _, t2) = m.hint(2);
        assert!(t2.contains("EXXXX"), "L2 names the expected error");
        assert!(!t2.contains("clone"), "L2 must not leak the solution");
        let (l3, _, t3) = m.hint(3);
        assert_eq!(l3, 3);
        assert!(t3.contains("clone"), "L3 reveals the outline (last resort)");
        assert_eq!(m.hint(9).0, 3, "over-request clamps to max");
    }

    #[test]
    fn hint_without_solution_caps_at_level_2() {
        let mut m = fx();
        m.solution_outline = None;
        assert_eq!(m.hint(1).1, 2, "max_level is 2 with no outline");
        assert_eq!(m.hint(9).0, 2, "over-request clamps to 2 (never a solution rung)");
    }
}
