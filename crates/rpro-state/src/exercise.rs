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

    /// Authoring reference only — a one-sentence pointer at the fix shape, kept
    /// server-side so authors can sanity-check exercises. NEVER served to any
    /// surface: [`ExerciseMetadata::hint`] deliberately omits it at every rung,
    /// including the top one (Hard Rule #1: never hand the answer).
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
    /// * **3** — last rung: sends the learner back to the SOURCE (the exercise's
    ///   book sections + concept) and tells them to apply the compiler's `help:`
    ///   line themselves. It NEVER returns `solution_outline` — that stored field
    ///   is an authoring reference only (Hard Rule #1: never hand a solution, not
    ///   even at the top rung, not even if asked directly).
    ///
    /// The tutor guides; it never auto-types or reveals the fix (see
    /// `docs/EDUCATION.md`). Forcing a real attempt before hints unlock, and
    /// escalating the rung as a learner proves they're stuck, is the server's job
    /// (it knows the attempt count); this method just produces each rung's text.
    #[must_use]
    pub fn hint(&self, requested: u8) -> (u8, u8, String) {
        // Three guiding rungs that escalate toward the SOURCE, never the answer.
        // `solution_outline` is an authoring reference and is DELIBERATELY never
        // returned here — not even at the top rung (Hard Rule #1).
        let max_level: u8 = 3;
        let level = requested.clamp(1, max_level);
        let text = match level {
            1 => format!(
                "Concept: {}. Start with the book refs, then run it and read the \
                 compiler's `-->` line (the location) and the `help:` line — that \
                 usually names the fix.",
                self.concept
            ),
            2 => self.expected_error_code.as_ref().map_or_else(
                || {
                    "Read the first error top-to-bottom: the `-->` line is the \
                    location, the `help:` line is usually the fix."
                        .to_string()
                },
                |c| {
                    format!(
                        "Expect error {c}. Ask for its full explanation, then look at \
                         exactly which value or line it flags."
                    )
                },
            ),
            // Last rung: back to the SOURCE, never the code. The compiler's own
            // `help:` line names the change; the learner applies it themselves.
            _ => {
                let refs = self
                    .book_refs
                    .iter()
                    .map(|r| r.chapter.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                if refs.is_empty() {
                    format!(
                        "Still stuck? Go back to the book on {} and re-read it, then \
                         apply the compiler's `help:` line yourself — it names the exact \
                         change. Writing the fix is the lesson.",
                        self.concept
                    )
                } else {
                    format!(
                        "Still stuck? Re-read these book sections on {}: {}. Then apply \
                         the compiler's `help:` line yourself — it names the exact change. \
                         There's no shortcut to the answer; writing the fix is the lesson.",
                        self.concept, refs
                    )
                }
            }
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
    fn hint_ladder_escalates_and_never_leaks_solution() {
        let m = fx(); // has a solution outline ("...clone()...") + code "EXXXX"
        // NO rung may ever contain the stored solution outline.
        for lvl in 1..=3 {
            let (got, max, t) = m.hint(lvl);
            assert_eq!((got, max), (lvl, 3));
            assert!(
                !t.to_lowercase().contains("clone"),
                "level {lvl} must NEVER leak the solution outline"
            );
        }
        assert!(m.hint(2).2.contains("EXXXX"), "L2 names the expected error");
        // Top rung sends them back to the book, not to the code.
        let t3 = m.hint(3).2;
        assert!(
            t3.contains("ch04-01-what-is-ownership"),
            "L3 points at the book section, not the answer"
        );
        assert_eq!(m.hint(9).0, 3, "over-request clamps to max");
    }

    #[test]
    fn hint_top_rung_independent_of_solution_outline() {
        // The top rung is the book/concept review, so it exists (and is safe)
        // whether or not an outline is stored. Removing the outline changes
        // nothing the learner can see.
        let mut m = fx();
        m.solution_outline = None;
        assert_eq!(m.hint(1).1, 3, "max_level stays 3 with no outline");
        let t3 = m.hint(3).2;
        assert!(
            t3.contains("ch04-01-what-is-ownership") || t3.contains("move-semantics"),
            "L3 is the source/concept review regardless of outline"
        );
        assert!(!t3.to_lowercase().contains("clone"), "still no solution");
    }
}
