//! Exercise discovery + runner.
//!
//! v0 scope: walk the exercise directory, pair `.rs` files with
//! their `.toml` metadata, validate. The actual `cargo run`
//! integration (and parsing of compiler output back into a typed
//! `Outcome`) lands in v0.1; the structure here is the seam.

#![doc(html_no_source)]

use rpro_state::ExerciseMetadata;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Errors during exercise discovery.
#[derive(Debug, thiserror::Error)]
pub enum DiscoveryError {
    /// I/O.
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
    /// Metadata validation rejected the exercise.
    #[error("{path}: {reason}")]
    Invalid {
        /// Path that failed.
        path: String,
        /// Reason.
        reason: String,
    },
    /// `.rs` file present without a sibling `.toml`.
    #[error("{path}: missing metadata sibling (every exercise needs a .toml)")]
    MissingMeta {
        /// `.rs` path with no metadata.
        path: String,
    },
}

/// One discovered exercise — file path + parsed metadata.
#[derive(Debug, Clone)]
pub struct Exercise {
    /// Path to the `.rs` file.
    pub source: PathBuf,
    /// Parsed metadata from the sibling `.toml`.
    pub meta: ExerciseMetadata,
}

/// Discover every exercise under `root`. Sorted by id.
///
/// # Errors
/// First per-file failure: missing metadata, invalid TOML,
/// failed validation.
pub fn discover(root: &Path) -> Result<Vec<Exercise>, DiscoveryError> {
    let mut exercises = Vec::new();
    for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
        let p = entry.path();
        if p.extension().and_then(|s| s.to_str()) != Some("rs") {
            continue;
        }
        let toml_path = p.with_extension("toml");
        if !toml_path.exists() {
            return Err(DiscoveryError::MissingMeta {
                path: p.display().to_string(),
            });
        }
        let raw = std::fs::read_to_string(&toml_path).map_err(|e| DiscoveryError::Io {
            path: toml_path.display().to_string(),
            source: e,
        })?;
        let meta: ExerciseMetadata = toml::from_str(&raw).map_err(|e| DiscoveryError::Toml {
            path: toml_path.display().to_string(),
            source: e,
        })?;
        meta.validate().map_err(|reason| DiscoveryError::Invalid {
            path: toml_path.display().to_string(),
            reason,
        })?;
        exercises.push(Exercise {
            source: p.to_path_buf(),
            meta,
        });
    }
    exercises.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
    Ok(exercises)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn write(path: &Path, contents: &str) {
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        let mut f = std::fs::File::create(path).unwrap();
        f.write_all(contents.as_bytes()).unwrap();
    }

    #[test]
    fn discovers_a_well_formed_exercise() {
        let dir = tempfile::tempdir().unwrap();
        let rs = dir.path().join("ownership/01_move.rs");
        let toml = dir.path().join("ownership/01_move.toml");
        write(&rs, "// rustlings exercise body\nfn main() {}\n");
        write(
            &toml,
            r#"
id = "ownership/01_move"
title = "Move semantics"
difficulty = "beginner"
estimated_minutes = 8
concept = "move-semantics"

[[book_refs]]
chapter = "ch04-01-what-is-ownership"
why = "Ownership rules."
"#,
        );
        let exs = discover(dir.path()).unwrap();
        assert_eq!(exs.len(), 1);
        assert_eq!(exs[0].meta.id, "ownership/01_move");
    }

    #[test]
    fn missing_toml_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let rs = dir.path().join("orphan.rs");
        write(&rs, "fn main() {}");
        let err = discover(dir.path()).unwrap_err();
        assert!(matches!(err, DiscoveryError::MissingMeta { .. }));
    }

    #[test]
    fn invalid_toml_rejected() {
        let dir = tempfile::tempdir().unwrap();
        write(&dir.path().join("bad.rs"), "fn main() {}");
        write(&dir.path().join("bad.toml"), "not [valid toml :::");
        let err = discover(dir.path()).unwrap_err();
        assert!(matches!(err, DiscoveryError::Toml { .. }));
    }

    #[test]
    fn missing_book_refs_rejected() {
        let dir = tempfile::tempdir().unwrap();
        write(&dir.path().join("noref.rs"), "fn main() {}");
        write(
            &dir.path().join("noref.toml"),
            r#"
id = "x/no_refs"
title = "No refs"
difficulty = "beginner"
estimated_minutes = 1
concept = "x"
book_refs = []
"#,
        );
        let err = discover(dir.path()).unwrap_err();
        match err {
            DiscoveryError::Invalid { reason, .. } => assert!(reason.contains("book_refs")),
            other => panic!("wrong: {other:?}"),
        }
    }
}
