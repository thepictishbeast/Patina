//! Book annotations — highlights and notes the user attaches to
//! ranges of book content.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user-attached note on a span of book text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotation {
    /// Chapter id (`ch04-01-what-is-ownership`).
    pub chapter: String,
    /// Anchor of the section the annotation lives in. Required —
    /// annotations are always scoped to a section so a chapter
    /// rewrite doesn't strand notes orphaned at "line 47."
    pub anchor: String,
    /// The original text the user highlighted, verbatim. We use
    /// content-addressing instead of byte offsets so annotations
    /// survive cosmetic re-flows of the chapter (whitespace,
    /// line wrapping). If the highlighted text disappears entirely
    /// from the chapter, the annotation surfaces as "orphaned" in
    /// the UI for the user to repair.
    pub highlighted: String,
    /// User's note. Empty string = pure highlight, no comment.
    pub body: String,
    /// When the annotation was created.
    pub created_at: DateTime<Utc>,
    /// Last modified.
    pub modified_at: DateTime<Utc>,
}

/// All annotations the user has attached.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Annotations {
    /// Newest first. UI sorts as needed.
    pub items: Vec<Annotation>,
}

impl Annotations {
    /// Add a new annotation with `Utc::now()` for created/modified.
    pub fn add(
        &mut self,
        chapter: impl Into<String>,
        anchor: impl Into<String>,
        highlighted: impl Into<String>,
        body: impl Into<String>,
    ) {
        let now = Utc::now();
        self.items.push(Annotation {
            chapter: chapter.into(),
            anchor: anchor.into(),
            highlighted: highlighted.into(),
            body: body.into(),
            created_at: now,
            modified_at: now,
        });
    }

    /// Number of annotations attached to a given chapter.
    #[must_use]
    pub fn for_chapter(&self, chapter: &str) -> usize {
        self.items.iter().filter(|a| a.chapter == chapter).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_count() {
        let mut a = Annotations::default();
        a.add("ch04-01", "rules", "highlighted text", "my note");
        a.add("ch04-01", "rules", "another", "");
        a.add("ch04-02", "borrowing", "x", "");
        assert_eq!(a.for_chapter("ch04-01"), 2);
        assert_eq!(a.for_chapter("ch04-02"), 1);
        assert_eq!(a.for_chapter("ch99"), 0);
    }
}
