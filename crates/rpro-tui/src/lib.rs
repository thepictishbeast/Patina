//! TUI shell for Rustlings Pro.
//!
//! v0 stub: the public surface that the CLI calls into. Actual
//! ratatui screens (dashboard, exercise view, book reader) land
//! incrementally; the shape is here so the CLI dispatch wires up
//! today and the screen implementations don't reshuffle the API.

#![doc(html_no_source)]

pub mod status;
pub mod theme;

use anyhow::Result;
use rpro_book::Book;
use rpro_storage_fs::Store;

/// Open the dashboard — entry screen, shows progress + the current
/// exercise's title + book references.
///
/// # Errors
/// I/O setting up the terminal or loading state.
pub fn run_dashboard(_store: &Store, _book: &Book) -> Result<()> {
    println!("(rpro-tui v0 stub: dashboard screen lands in v0.1)");
    Ok(())
}

/// Open the book reader. `start_chapter` is optional — if Some,
/// the reader scrolls to that chapter id; if None, it opens at
/// the first chapter.
///
/// # Errors
/// I/O setting up the terminal.
pub fn run_book_reader(_book: &Book, _start_chapter: Option<&str>) -> Result<()> {
    println!("(rpro-tui v0 stub: book reader screen lands in v0.1)");
    Ok(())
}
