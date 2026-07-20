//! On-disk state for Tempered Studio (the `rpro` workspace).
//!
//! Everything the user produces (progress, bookmarks, annotations,
//! preferences) lives as a plain JSON or TOML file under
//! `~/.rustlings-pro/`. No database. A user can `git init` their
//! state directory and version-control their notes.
//!
//! The core types here are also the shared model consumed by
//! `rpro-runner` and every surface — `rpro-cli`, `rpro-tui`, and the
//! `rpro-serve` web server — so progress, hints, and the spaced-
//! repetition queue stay identical across them. Write changes through
//! this crate, never by hand-editing the JSON.

#![doc(html_no_source)]

pub mod annotations;
pub mod bookmarks;
pub mod config;
pub mod exercise;
pub mod progress;
pub mod review;
pub mod tutor;
pub mod xp;

pub use annotations::{Annotation, Annotations};
pub use bookmarks::{Bookmark, Bookmarks};
pub use config::Config;
pub use exercise::{BookRef, Difficulty, ExerciseMetadata};
pub use progress::{ExerciseStatus, Progress};
pub use review::ReviewState;
pub use tutor::{GUARDRAILS, HintRung, LoopStep, TutorContext};
