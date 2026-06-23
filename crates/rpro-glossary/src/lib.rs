//! Built-in glossary loader.
//!
//! Loads plain-language Rust term definitions from a flat `glossary.toml` and
//! looks them up by name or alias (case- and separator-insensitive). It is pure
//! data + a map — no markdown parsing, no search index (unlike [`rpro_book`]).
//! Shared by every surface (web/CLI/TUI) so one term map is the single source of
//! truth a learner can consult offline, with no LLM.

#![doc(html_no_source)]

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::Path;

/// One glossary entry: a term, its plain-language definition, and where to read
/// the full treatment.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Term {
    /// Canonical term name (e.g. `"ownership"`).
    pub name: String,
    /// Alternate spellings + the exercise `concept` tags that map here.
    #[serde(default)]
    pub aliases: Vec<String>,
    /// Plain-language, **conceptual** definition — never an exercise's fix.
    pub definition: String,
    /// Bundled Book chapter id for "read more" (resolves in the Book reader).
    pub book_chapter: String,
    /// Attribution: which textbook + section the definition was synthesized from.
    pub source: String,
}

/// The loaded glossary: terms in stable (alphabetical) order plus a normalized
/// lookup index over every name and alias.
#[derive(Debug, Clone, Default)]
pub struct Glossary {
    terms: Vec<Term>,
    /// normalized key → index into `terms`.
    index: BTreeMap<String, usize>,
}

/// Errors loading the glossary.
#[derive(Debug, thiserror::Error)]
pub enum GlossaryError {
    /// I/O reading the file.
    #[error("{path}: {source}")]
    Io {
        /// Path that failed.
        path: String,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },
    /// TOML parse.
    #[error("{path}: {source}")]
    Toml {
        /// Path that failed.
        path: String,
        /// Underlying error.
        #[source]
        source: toml::de::Error,
    },
}

#[derive(Deserialize)]
struct Raw {
    #[serde(default, rename = "term")]
    terms: Vec<Term>,
}

/// Normalize a lookup key: trim, lowercase, and treat `_`/`-`/whitespace alike,
/// so `"Move Semantics"`, `"move-semantics"` and `"move_semantics"` all match.
fn norm(s: &str) -> String {
    s.trim()
        .to_lowercase()
        .chars()
        .map(|c| if c == '_' || c == '-' { ' ' } else { c })
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

impl Glossary {
    /// Load from a `glossary.toml` file.
    ///
    /// # Errors
    /// I/O failure reading the file, or invalid TOML.
    pub fn load(path: &Path) -> Result<Self, GlossaryError> {
        let raw = std::fs::read_to_string(path).map_err(|e| GlossaryError::Io {
            path: path.display().to_string(),
            source: e,
        })?;
        Self::parse(&raw).map_err(|e| GlossaryError::Toml {
            path: path.display().to_string(),
            source: e,
        })
    }

    /// Parse from a TOML string — pure, so it is unit-testable without the
    /// filesystem. The first definition of a key wins if two terms collide.
    ///
    /// # Errors
    /// Invalid TOML.
    pub fn parse(s: &str) -> Result<Self, toml::de::Error> {
        let raw: Raw = toml::from_str(s)?;
        let mut terms = raw.terms;
        terms.sort_by(|a, b| a.name.cmp(&b.name));
        let mut index = BTreeMap::new();
        for (i, t) in terms.iter().enumerate() {
            index.entry(norm(&t.name)).or_insert(i);
            for a in &t.aliases {
                index.entry(norm(a)).or_insert(i);
            }
        }
        Ok(Self { terms, index })
    }

    /// Look up a term by name or alias (case- and separator-insensitive).
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Term> {
        self.index.get(&norm(key)).map(|&i| &self.terms[i])
    }

    /// All terms, in stable alphabetical order.
    #[must_use]
    pub fn all(&self) -> &[Term] {
        &self.terms
    }

    /// True when no terms loaded.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.terms.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
[[term]]
name = "move"
aliases = ["move-semantics", "moved"]
definition = "ownership transfers; the original binding can't be used after."
book_chapter = "ch04-01-what-is-ownership"
source = "The Rust Book, ch.4.1"

[[term]]
name = "ownership"
definition = "each value has one owner."
book_chapter = "ch04-01-what-is-ownership"
source = "The Rust Book, ch.4.1"
"#;

    #[test]
    fn looks_up_by_name_alias_and_normalized_key() {
        let g = Glossary::parse(SAMPLE).unwrap();
        assert_eq!(g.all().len(), 2);
        assert_eq!(g.get("ownership").unwrap().name, "ownership");
        // alias + case + separator normalization all resolve to the same term
        assert_eq!(g.get("Move-Semantics").unwrap().name, "move");
        assert_eq!(g.get("move_semantics").unwrap().name, "move");
        assert_eq!(g.get("  MOVED ").unwrap().name, "move");
        assert!(g.get("borrow").is_none(), "unknown key misses cleanly");
    }

    #[test]
    fn all_is_stable_alphabetical() {
        let g = Glossary::parse(SAMPLE).unwrap();
        let names: Vec<_> = g.all().iter().map(|t| t.name.as_str()).collect();
        assert_eq!(names, vec!["move", "ownership"]);
    }

    /// Editorial guard on the SHIPPED glossary: it must load, every term must
    /// resolve by its own name, and — Hard Rule #1 — the `mutability` entry must
    /// not hand over exercise 01's literal fix (`let mut` / a bare `` `mut` ``).
    #[test]
    fn shipped_glossary_loads_and_never_leaks_the_mut_fix() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../glossary/glossary.toml");
        let g = Glossary::load(&path).expect("the shipped glossary must load");
        assert!(!g.is_empty(), "shipped glossary has terms");
        for t in g.all() {
            assert!(
                g.get(&t.name).is_some(),
                "term '{}' resolves by its own name",
                t.name
            );
            assert!(
                !t.definition.trim().is_empty(),
                "'{}' has a definition",
                t.name
            );
        }
        let m = g.get("mutability").expect("mutability is defined");
        let d = m.definition.to_lowercase();
        assert!(
            !d.contains("let mut") && !d.contains("`mut`"),
            "the mutability definition must stay conceptual, not hand over the `let mut` fix"
        );
    }
}
