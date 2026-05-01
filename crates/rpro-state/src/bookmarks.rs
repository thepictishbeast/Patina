//! Book bookmarks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A user-saved spot in the Rust Book.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bookmark {
    /// Chapter id (`ch04-01-what-is-ownership`).
    pub chapter: String,
    /// Optional in-chapter anchor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// Free-form short label the user types when bookmarking.
    /// Helps distinguish 5 bookmarks in the same chapter.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// When the bookmark was created.
    pub ts: DateTime<Utc>,
}

/// All bookmarks the user has saved.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bookmarks {
    /// In creation order; UI sorts client-side as needed.
    pub items: Vec<Bookmark>,
}

impl Bookmarks {
    /// Add a new bookmark with `Utc::now()`.
    pub fn add(&mut self, chapter: impl Into<String>, anchor: Option<String>, label: Option<String>) {
        self.items.push(Bookmark {
            chapter: chapter.into(),
            anchor,
            label,
            ts: Utc::now(),
        });
    }

    /// Remove every bookmark on the given chapter+anchor combo.
    /// Returns the number removed.
    pub fn remove_at(&mut self, chapter: &str, anchor: Option<&str>) -> usize {
        let before = self.items.len();
        self.items
            .retain(|b| !(b.chapter == chapter && b.anchor.as_deref() == anchor));
        before - self.items.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_then_remove() {
        let mut b = Bookmarks::default();
        b.add("ch04-01", Some("rules".into()), Some("3 rules".into()));
        b.add("ch04-01", None, None);
        assert_eq!(b.items.len(), 2);
        assert_eq!(b.remove_at("ch04-01", Some("rules")), 1);
        assert_eq!(b.items.len(), 1);
    }
}
