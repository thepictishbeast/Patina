//! Rust Book content loader.
//!
//! Loads chapter markdown from a directory and exposes a
//! `Book::chapters()` iterator and a `Book::get(chapter_id)`
//! lookup. Rendering (to ratatui spans for the TUI, to HTML for the
//! web GUI) is handled by the consuming crate; this one stops at
//! parsing the markdown into events.

#![doc(html_no_source)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Errors loading the book.
#[derive(Debug, thiserror::Error)]
pub enum BookError {
    /// I/O reading content.
    #[error("{path}: {source}")]
    Io {
        /// Path that failed.
        path: String,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },
}

/// One chapter — markdown source + filename id.
#[derive(Debug, Clone)]
pub struct Chapter {
    /// Filename without extension (`ch04-01-what-is-ownership`).
    pub id: String,
    /// Path to the markdown file.
    pub path: PathBuf,
    /// Raw markdown source.
    pub markdown: String,
}

impl Chapter {
    /// Parsed markdown events. Borrows the chapter's source.
    #[must_use]
    pub fn parse(&self) -> pulldown_cmark::Parser<'_> {
        pulldown_cmark::Parser::new_ext(&self.markdown, pulldown_cmark::Options::all())
    }
}

/// All loaded chapters indexed by id.
#[derive(Debug, Clone, Default)]
pub struct Book {
    /// Chapter id → chapter. BTreeMap so iteration order matches
    /// the file-system sort, which is the expected reading order.
    pub chapters: BTreeMap<String, Chapter>,
}

impl Book {
    /// Load every `*.md` file under `root` as a chapter.
    ///
    /// # Errors
    /// First I/O failure encountered.
    pub fn load(root: &Path) -> Result<Self, BookError> {
        let mut chapters = BTreeMap::new();
        if !root.exists() {
            return Ok(Self::default());
        }
        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(Result::ok)
        {
            let p = entry.path();
            if p.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }
            let id = match p.file_stem().and_then(|s| s.to_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };
            let markdown = std::fs::read_to_string(p).map_err(|e| BookError::Io {
                path: p.display().to_string(),
                source: e,
            })?;
            chapters.insert(
                id.clone(),
                Chapter {
                    id,
                    path: p.to_path_buf(),
                    markdown,
                },
            );
        }
        Ok(Self { chapters })
    }

    /// Look up by chapter id.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Chapter> {
        self.chapters.get(id)
    }

    /// Number of chapters loaded.
    #[must_use]
    pub fn len(&self) -> usize {
        self.chapters.len()
    }

    /// True if no chapters loaded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.chapters.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_markdown_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("ch01-00-getting-started.md"),
            "# Getting Started\n\nbody",
        )
        .unwrap();
        std::fs::write(
            dir.path().join("ch04-01-what-is-ownership.md"),
            "# What Is Ownership?\n",
        )
        .unwrap();
        std::fs::write(dir.path().join("ignore.txt"), "not markdown").unwrap();
        let book = Book::load(dir.path()).unwrap();
        assert_eq!(book.len(), 2);
        assert!(book.get("ch01-00-getting-started").is_some());
        assert!(book.get("ignore").is_none());
    }

    #[test]
    fn parse_yields_events() {
        let c = Chapter {
            id: "test".into(),
            path: PathBuf::from("test.md"),
            markdown: "# Hi\n\nbody".into(),
        };
        assert!(c.parse().next().is_some());
    }

    #[test]
    fn missing_root_returns_empty() {
        let book = Book::load(Path::new("/this/does/not/exist")).unwrap();
        assert!(book.is_empty());
    }
}
