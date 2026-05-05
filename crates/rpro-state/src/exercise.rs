//! Exercise metadata — including the typed book references.
//!
//! This is the schema that makes Rustlings-Pro's "book references on
//! every exercise" promise concrete. Every exercise file (`*.rs`)
//! has a sibling `*.toml` that deserializes into `ExerciseMetadata`.

use serde::{Deserialize, Serialize};

/// A pointer from an exercise to a specific spot in The Rust Book.
///
/// Each exercise can carry multiple of these so the hint UI can
/// surface several relevant sections (one for the high-level
/// concept, another for the specific syntax detail). The `why`
/// field is short — one sentence — and tells the learner *why this
/// section helps with this exercise*, not just "go read it."
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookRef {
    /// Chapter id matching the book's filename (without `.md`),
    /// e.g. `"ch04-01-what-is-ownership"`. The TUI book reader uses
    /// this to load and render the chapter.
    pub chapter: String,

    /// Optional in-page anchor — the kebab-case heading slug, e.g.
    /// `"the-stack-and-the-heap"`. The reader scrolls to the
    /// matching `<h2>` / `<h3>` when set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,

    /// One-sentence justification: why this specific section helps
    /// with this specific exercise. Surfaces in the hint UI as
    /// `Read this for: <why>`.
    pub why: String,
}

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

    /// Compiler error code the exercise is designed around (e.g.
    /// `"E0382"`). The runner cross-checks against this; an
    /// unexpected error gets a "you may have changed more than
    /// the exercise asked" hint.
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

    /// Public URL for a [`BookRef`]'s chapter on doc.rust-lang.org.
    /// Used by the hint UI to offer "open in browser" alongside the
    /// in-app reader.
    #[must_use]
    pub fn book_ref_url(r: &BookRef) -> String {
        r.anchor.as_ref().map_or_else(
            || format!("https://doc.rust-lang.org/book/{}.html", r.chapter),
            |a| format!("https://doc.rust-lang.org/book/{}.html#{}", r.chapter, a),
        )
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
            expected_error_code: Some("E0382".into()),
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
    fn book_ref_url_with_anchor() {
        let r = &fx().book_refs[0];
        assert_eq!(
            ExerciseMetadata::book_ref_url(r),
            "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules"
        );
    }

    #[test]
    fn book_ref_url_without_anchor() {
        let mut r = fx().book_refs[0].clone();
        r.anchor = None;
        assert_eq!(
            ExerciseMetadata::book_ref_url(&r),
            "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html"
        );
    }

    #[test]
    fn book_ref_round_trips_toml() {
        let m = fx();
        let s = toml::to_string_pretty(&m).unwrap();
        let back: ExerciseMetadata = toml::from_str(&s).unwrap();
        assert_eq!(m, back);
    }
}
