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

    /// Render the chapter's raw mdBook **source** into clean display markdown
    /// suitable for a plain renderer (the TUI line styler or the web `mdToHtml`),
    /// without an mdBook preprocessing pass. `chapter_url` is the live online
    /// page for this chapter, supplied by the caller's language adapter (this
    /// crate is language-neutral and holds no Rust-specific URLs).
    ///
    /// What it fixes — and an honest limitation:
    ///
    /// * **`{{#rustdoc_include}}` / `{{#include}}` directives** pull example code
    ///   from `../listings/` files that are **not** bundled with these chapters.
    ///   We cannot show code we do not have, so a lone-include code block is
    ///   replaced with a link to the live chapter (where the runnable listing
    ///   lives) rather than a confusing raw directive path. **The bundled book is
    ///   therefore prose + links-to-code, not a self-contained code textbook** —
    ///   closing that gap means vendoring ~200 listing files (a separate call).
    /// * **mdBook hidden lines** (`# …` inside a code block) are dropped and
    ///   `##`-escapes unindented to `#`, exactly as mdBook would render them.
    /// * **Fence info strings** (e.g. annotated rust fences) are normalized to the
    ///   bare language so annotations don't leak into the page.
    #[must_use]
    pub fn display_markdown(&self, chapter_url: &str) -> String {
        clean_mdbook_source(&self.markdown, chapter_url)
    }

    /// The chapter's title: the first ATX heading's text, or the id if the
    /// chapter has no heading. Used by the TOC and search result lists.
    #[must_use]
    pub fn title(&self) -> String {
        self.markdown
            .lines()
            .find_map(|l| {
                let t = l.trim_start();
                if !t.starts_with('#') {
                    return None;
                }
                let title = t.trim_start_matches('#').trim();
                (!title.is_empty()).then(|| title.to_string())
            })
            .unwrap_or_else(|| self.id.clone())
    }
}

/// Pure transform behind [`Chapter::display_markdown`].
///
/// `url` is the live-chapter link substituted for un-bundled code listings. Kept
/// free-standing (and URL-agnostic) so it is unit-testable without constructing a
/// [`Chapter`].
#[must_use]
pub fn clean_mdbook_source(source: &str, url: &str) -> String {
    let callout = format!("📖 Read this code listing in the Rust Book: {url}");
    let mut out = String::new();
    // Suppress a run of identical link callouts (adjacent includes collapse to one).
    let push_callout = |out: &mut String| {
        if !out.trim_end().ends_with(&callout) {
            out.push_str(&callout);
            out.push('\n');
        }
    };
    let mut lines = source.lines();
    while let Some(line) = lines.next() {
        let trimmed = line.trim_start();
        // A fenced code block: collect its body up to the closing fence, then
        // decide how to emit it (a lone-include block becomes a link).
        if let Some(info) = trimmed.strip_prefix("```") {
            let lang = info.split(',').next().unwrap_or("").trim();
            let is_rust = lang.is_empty() || lang == "rust";
            let mut body: Vec<&str> = Vec::new();
            for l in lines.by_ref() {
                if l.trim_start().starts_with("```") {
                    break;
                }
                body.push(l);
            }
            let is_directive = |l: &&str| l.trim_start().starts_with("{{#");
            let only_includes = body.iter().any(is_directive)
                && body.iter().all(|l| l.trim().is_empty() || is_directive(l));
            if only_includes {
                // No code to show — link to the live listing instead of an empty fence.
                push_callout(&mut out);
                continue;
            }
            out.push_str("```");
            out.push_str(lang);
            out.push('\n');
            for l in body {
                let t = l.trim_start();
                if t.starts_with("{{#") {
                    out.push_str("// (listing omitted — read it in the Rust Book)\n");
                } else if is_rust && (t == "#" || t.starts_with("# ")) {
                    // mdBook hidden line — not shown to the reader.
                } else if is_rust && t.starts_with("##") {
                    // mdBook `##`-escape renders as a literal single `#`.
                    out.push_str(&t[1..]);
                    out.push('\n');
                } else {
                    out.push_str(l);
                    out.push('\n');
                }
            }
            out.push_str("```\n");
            continue;
        }
        // A stray include outside any fence (e.g. an `output.txt` include).
        if trimmed.starts_with("{{#") {
            push_callout(&mut out);
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
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

    /// Case-insensitive full-text search across all chapters. Returns one
    /// [`SearchHit`] per matching chapter, sorted by descending match count
    /// (ties keep reading order). An empty/blank term returns no hits.
    ///
    /// The matching is the same substring test the CLI `book search` has always
    /// used; lifting it here lets the web surface share one implementation.
    #[must_use]
    pub fn search(&self, term: &str) -> Vec<SearchHit> {
        let needle = term.trim().to_lowercase();
        if needle.is_empty() {
            return Vec::new();
        }
        let mut hits: Vec<SearchHit> = self
            .chapters
            .values()
            .filter_map(|c| {
                let count = c.markdown.to_lowercase().matches(&needle).count();
                (count > 0).then(|| SearchHit {
                    chapter: c.id.clone(),
                    title: c.title(),
                    count,
                    snippet: first_match_snippet(&c.markdown, &needle),
                })
            })
            .collect();
        // Stable sort keeps reading order within equal match counts (descending
        // by count, so the strongest matches lead).
        hits.sort_by_key(|h| std::cmp::Reverse(h.count));
        hits
    }
}

/// One chapter's full-text search result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchHit {
    /// Chapter id (the map key — what `Book::get` takes).
    pub chapter: String,
    /// Chapter title (first heading, see [`Chapter::title`]).
    pub title: String,
    /// Number of case-insensitive occurrences of the term in the chapter.
    pub count: usize,
    /// The first source line containing the term, trimmed of heading/markdown
    /// markers and truncated — a plain-text preview for the results list.
    pub snippet: String,
}

/// First source line containing `needle_lower` (already lowercased), trimmed of
/// leading `#`/`>`/`-`/`*` markers and truncated to ~140 chars on a char
/// boundary. Line-based (not byte-sliced) so it is panic-safe on UTF-8 and keeps
/// the original casing.
fn first_match_snippet(markdown: &str, needle_lower: &str) -> String {
    markdown
        .lines()
        .find(|l| l.to_lowercase().contains(needle_lower))
        .map(|l| {
            let t = l
                .trim()
                .trim_start_matches(['#', '>', '-', '*', ' '])
                .trim();
            if t.chars().count() > 140 {
                let head: String = t.chars().take(140).collect();
                format!("{head}…")
            } else {
                t.to_string()
            }
        })
        .unwrap_or_default()
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

    // A neutral stand-in for the live-chapter URL (this crate is language-neutral;
    // the real URL is supplied by the caller's language adapter at runtime).
    const URL: &str = "https://book.example/ch04.html";

    #[test]
    fn lone_include_block_becomes_a_link() {
        let src =
            "Intro.\n\n```rust\n{{#rustdoc_include ../listings/x/src/main.rs:here}}\n```\n\nMore.";
        let out = clean_mdbook_source(src, URL);
        assert!(!out.contains("{{#"), "raw directive must be gone: {out}");
        assert!(!out.contains("```"), "empty code fence dropped: {out}");
        assert!(out.contains(URL), "links to the live listing: {out}");
        assert!(
            out.contains("Intro.") && out.contains("More."),
            "prose preserved"
        );
    }

    #[test]
    fn adjacent_includes_collapse_to_one_link() {
        let src = "```rust\n{{#rustdoc_include a:here}}\n```\n```rust\n{{#rustdoc_include b:here}}\n```\n";
        let out = clean_mdbook_source(src, URL);
        assert_eq!(
            out.matches(URL).count(),
            1,
            "collapsed to a single callout: {out}"
        );
    }

    #[test]
    fn hidden_lines_dropped_and_escapes_unindented() {
        // `# ` hidden lines vanish; `##` escapes render as a literal `#`; real
        // code and attributes (`#[...]`) survive.
        let src = "```rust\n# use std::fmt;\n#[derive(Debug)]\nstruct S;\n## not hidden\n```\n";
        let out = clean_mdbook_source(src, URL);
        assert!(!out.contains("use std::fmt"), "hidden line removed: {out}");
        assert!(out.contains("#[derive(Debug)]"), "attribute kept: {out}");
        assert!(out.contains("struct S;"), "real code kept");
        assert!(out.contains("# not hidden"), "## unescaped to #: {out}");
    }

    #[test]
    fn fence_info_string_normalized_and_real_code_kept() {
        let src = "```rust,ignore,does_not_compile\nlet x = 5;\n```\n";
        let out = clean_mdbook_source(src, URL);
        assert!(
            out.contains("```rust\n"),
            "info string normalized to bare lang: {out}"
        );
        assert!(!out.contains("does_not_compile"), "annotations stripped");
        assert!(out.contains("let x = 5;"), "code body kept");
    }

    #[test]
    fn non_rust_block_keeps_hash_lines() {
        // A console block's `#` lines are NOT mdBook hidden lines — keep them.
        let src = "```console\n$ run-it\n# a comment in output\n```\n";
        let out = clean_mdbook_source(src, URL);
        assert!(
            out.contains("# a comment in output"),
            "non-rust hash kept: {out}"
        );
    }

    fn chap(id: &str, md: &str) -> Chapter {
        Chapter {
            id: id.into(),
            path: PathBuf::from(format!("{id}.md")),
            markdown: md.into(),
        }
    }
    fn book(chs: &[(&str, &str)]) -> Book {
        Book {
            chapters: chs
                .iter()
                .map(|(id, md)| ((*id).to_string(), chap(id, md)))
                .collect(),
        }
    }

    #[test]
    fn title_is_first_heading_or_id() {
        assert_eq!(
            chap("ch04-01", "## What Is Ownership?\n\nbody").title(),
            "What Is Ownership?"
        );
        assert_eq!(
            chap("ch00", "no heading here").title(),
            "ch00",
            "falls back to id"
        );
    }

    #[test]
    fn search_finds_counts_and_orders_by_frequency() {
        let b = book(&[
            (
                "ch04-01-what-is-ownership",
                "# Ownership\n\nownership ownership ownership",
            ),
            ("ch01-intro", "# Intro\n\none mention of ownership here"),
            ("ch99-empty", "# Misc\n\nnothing relevant"),
        ]);
        let hits = b.search("ownership");
        assert_eq!(hits.len(), 2, "only matching chapters: {hits:?}");
        // ch04 has more occurrences → it sorts first.
        assert_eq!(hits[0].chapter, "ch04-01-what-is-ownership");
        assert_eq!(hits[0].count, 4);
        assert_eq!(hits[0].title, "Ownership");
        assert_eq!(hits[1].chapter, "ch01-intro");
        assert!(
            hits[1].snippet.contains("one mention of ownership"),
            "snippet: {:?}",
            hits[1].snippet
        );
    }

    #[test]
    fn search_is_case_insensitive_and_blank_term_is_empty() {
        let b = book(&[("ch", "# H\n\nThe Borrow Checker enforces rules")]);
        assert_eq!(
            b.search("borrow checker").len(),
            1,
            "case-insensitive match"
        );
        assert!(b.search("   ").is_empty(), "blank term → no hits");
        assert!(b.search("absent-term").is_empty(), "no match → no hits");
    }

    #[test]
    fn snippet_strips_markers_and_is_truncated() {
        let long = format!("# T\n\n> {}", "ownership ".repeat(40));
        let hits = book(&[("ch", &long)]).search("ownership");
        assert!(
            !hits[0].snippet.starts_with('>'),
            "blockquote marker trimmed: {:?}",
            hits[0].snippet
        );
        assert!(
            hits[0].snippet.chars().count() <= 141,
            "truncated (≤140 + ellipsis)"
        );
        assert!(hits[0].snippet.ends_with('…'), "ellipsis on truncation");
    }
}
