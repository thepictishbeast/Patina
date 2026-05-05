//! `rpro` — Rustlings Pro CLI.
//!
//! Top-level command dispatch. Subcommands:
//!
//! ```text
//! rpro init                   — one-time setup
//! rpro                        — open the TUI dashboard (default)
//! rpro exercise list          — every exercise with status
//! rpro exercise next          — jump to next unfinished
//! rpro exercise hint          — book references for current
//! rpro book                   — open the TUI book reader
//! rpro book search <term>     — text search across chapters
//! rpro progress               — completion summary
//! ```
//!
//! v0 wires `init` end-to-end and stubs the rest.

#![doc(html_no_source)]
#![allow(clippy::doc_markdown)]

use anyhow::{Context, Result, anyhow};
use clap::{Parser, Subcommand};
use console::style;
use rpro_state::{ExerciseStatus, Store};

#[derive(Parser, Debug)]
#[command(name = "rpro", version, about = "Rustlings Pro — Rust learning + the Rust Book", long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// One-time setup: clone exercises, fetch the book, write
    /// state files at ~/.rustlings-pro/.
    Init,
    /// Exercise commands.
    Exercise {
        #[command(subcommand)]
        sub: ExerciseCmd,
    },
    /// Book commands.
    Book {
        #[command(subcommand)]
        sub: Option<BookCmd>,
    },
    /// Print a one-screen progress summary.
    Progress,
}

#[derive(Subcommand, Debug)]
enum ExerciseCmd {
    /// List every exercise with status + difficulty + concept.
    List,
    /// Jump to the next unfinished exercise (sets it to Current).
    Next,
    /// Book references for the current exercise — what to read
    /// when stuck.
    Hint {
        /// Also surface the (one-line) solution outline.
        #[arg(long)]
        solution: bool,
    },
}

#[derive(Subcommand, Debug)]
enum BookCmd {
    /// Full-text search across loaded chapters.
    Search {
        /// Search term.
        term: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        None => cmd_default(),
        Some(Cmd::Init) => cmd_init(),
        Some(Cmd::Exercise { sub }) => match sub {
            ExerciseCmd::List => cmd_exercise_list(),
            ExerciseCmd::Next => cmd_exercise_next(),
            ExerciseCmd::Hint { solution } => cmd_exercise_hint(solution),
        },
        Some(Cmd::Book { sub }) => match sub {
            None => cmd_book_open(),
            Some(BookCmd::Search { term }) => cmd_book_search(&term),
        },
        Some(Cmd::Progress) => cmd_progress(),
    }
}

// ---------------------------------------------------------------------------
// Commands
// ---------------------------------------------------------------------------

fn cmd_init() -> Result<()> {
    println!(
        "{}",
        style("Rustlings Pro — first-time setup").bold().cyan()
    );
    let store = Store::user().context("locating ~/.rustlings-pro/")?;
    println!("  state directory: {}", style(store.root().display()).dim());

    // Create the standard subdirs.
    for sub in ["exercises", "book"] {
        let p = store.root().join(sub);
        std::fs::create_dir_all(&p).with_context(|| format!("mkdir {}", p.display()))?;
        println!("  + {}", style(p.display()).dim());
    }

    // Write a default config + empty progress/bookmarks/annotations
    // so subsequent commands can always load something.
    if !store.root().join("config.toml").exists() {
        store.save_config(&rpro_state::Config::default())?;
        println!("  + {}", style("config.toml").dim());
    }
    if !store.root().join("progress.json").exists() {
        store.save_progress(&rpro_state::Progress::default())?;
        println!("  + {}", style("progress.json").dim());
    }
    if !store.root().join("bookmarks.json").exists() {
        store.save_bookmarks(&rpro_state::Bookmarks::default())?;
        println!("  + {}", style("bookmarks.json").dim());
    }
    if !store.root().join("annotations.json").exists() {
        store.save_annotations(&rpro_state::Annotations::default())?;
        println!("  + {}", style("annotations.json").dim());
    }

    // The exercise + book downloads land in v0.1 — for now, drop a
    // README in each empty dir explaining what goes there. Keeps
    // the layout discoverable without faking content.
    write_if_missing(
        &store.root().join("exercises/README.md"),
        "Exercises live here. Each exercise is a `name.rs` plus a sibling \
         `name.toml` with metadata (id, title, difficulty, concept, book_refs). \
         Run `rpro init --refresh-exercises` (v0.1) to download the rustlings \
         set + Rustlings-Pro originals. For now, drop your own here.\n",
    )?;
    write_if_missing(
        &store.root().join("book/README.md"),
        "The Rust Book chapters live here as one markdown file per chapter, \
         filename = chapter id (e.g. `ch04-01-what-is-ownership.md`). Run \
         `rpro init --refresh-book` (v0.1) to fetch the official content.\n",
    )?;

    println!();
    println!("{}", style("✓ setup complete").green().bold());
    println!();
    println!("Next:");
    println!(
        "  - {} (when v0.1 ships) — fetch exercises + book content",
        style("rpro init --refresh-all").yellow()
    );
    println!(
        "  - {} — list what's been imported so far",
        style("rpro exercise list").yellow()
    );
    println!(
        "  - {} — open the book reader (in-app)",
        style("rpro book").yellow()
    );
    println!();
    Ok(())
}

fn cmd_default() -> Result<()> {
    let store = Store::user()?;
    let book = rpro_book::Book::load(&store.root().join("book"))?;
    rpro_tui::run_dashboard(&store, &book)?;
    Ok(())
}

fn cmd_exercise_list() -> Result<()> {
    let store = Store::user()?;
    let progress = store.load_progress()?;
    let exercises = rpro_runner::discover(&store.root().join("exercises"))?;
    if exercises.is_empty() {
        println!(
            "{}",
            style("No exercises yet. Run `rpro init --refresh-exercises` (v0.1) to fetch the set.")
                .yellow()
        );
        return Ok(());
    }
    println!(
        "{:<30} {:<14} {:<14} {:<6} title",
        "id", "status", "difficulty", "min"
    );
    println!("{}", "-".repeat(80));
    for ex in &exercises {
        let status = progress
            .entries
            .get(&ex.meta.id)
            .map_or(ExerciseStatus::Locked, |e| e.status);
        let status_str = match status {
            ExerciseStatus::Locked => style("locked").dim(),
            ExerciseStatus::Current => style("current").yellow(),
            ExerciseStatus::Done => style("done").green(),
            ExerciseStatus::Skipped => style("skipped").red(),
        };
        println!(
            "{:<30} {:<14} {:?} {:<6} {}",
            ex.meta.id, status_str, ex.meta.difficulty, ex.meta.estimated_minutes, ex.meta.title
        );
    }
    Ok(())
}

fn cmd_exercise_next() -> Result<()> {
    let store = Store::user()?;
    let mut progress = store.load_progress()?;
    let exercises = rpro_runner::discover(&store.root().join("exercises"))?;
    let current = exercises.iter().find(|ex| {
        let s = progress
            .entries
            .get(&ex.meta.id)
            .map_or(ExerciseStatus::Locked, |e| e.status);
        !matches!(s, ExerciseStatus::Done | ExerciseStatus::Skipped)
    });
    let Some(ex) = current else {
        println!(
            "{}",
            style("All exercises done. Time to ship something.")
                .green()
                .bold()
        );
        return Ok(());
    };
    progress.set_current(&ex.meta.id);
    store.save_progress(&progress)?;
    println!("{} {}", style("→").bold().cyan(), style(&ex.meta.id).bold());
    println!("  {}", ex.meta.title);
    println!("  file: {}", style(ex.source.display()).dim());
    println!();
    println!(
        "  {} `{}` to see the book sections that explain this exercise.",
        style("Tip:").yellow(),
        style("rpro exercise hint").bold()
    );
    Ok(())
}

fn cmd_exercise_hint(show_solution: bool) -> Result<()> {
    let store = Store::user()?;
    let progress = store.load_progress()?;
    let exercises = rpro_runner::discover(&store.root().join("exercises"))?;
    let current_id = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone())
        .ok_or_else(|| anyhow!("no current exercise — run `rpro exercise next` first"))?;
    let ex = exercises
        .iter()
        .find(|e| e.meta.id == current_id)
        .ok_or_else(|| anyhow!("current exercise '{current_id}' not found on disk"))?;

    println!(
        "{}",
        style(format!("📖 Book references for {}", ex.meta.id))
            .bold()
            .cyan()
    );
    println!("   {}", style(&ex.meta.title).dim());
    println!();
    for (i, r) in ex.meta.book_refs.iter().enumerate() {
        println!(
            "  {}. {} {}",
            i + 1,
            style(&r.chapter).bold(),
            r.anchor
                .as_deref()
                .map_or(String::new(), |a| format!("# {a}"))
        );
        println!("     {}", r.why);
        println!(
            "     {}",
            style(rpro_state::ExerciseMetadata::book_ref_url(r))
                .dim()
                .underlined()
        );
        println!();
    }
    if show_solution {
        if let Some(s) = &ex.meta.solution_outline {
            println!("{}", style("Solution outline (one line):").yellow().bold());
            println!("  {s}");
        } else {
            println!(
                "{}",
                style("(no solution outline shipped with this exercise)").dim()
            );
        }
    } else {
        println!(
            "  {} `{}` to also reveal the one-line solution outline.",
            style("Tip:").yellow(),
            style("rpro exercise hint --solution").bold()
        );
    }
    Ok(())
}

fn cmd_book_open() -> Result<()> {
    let store = Store::user()?;
    let book = rpro_book::Book::load(&store.root().join("book"))?;
    rpro_tui::run_book_reader(&book, None)?;
    Ok(())
}

fn cmd_book_search(term: &str) -> Result<()> {
    let store = Store::user()?;
    let book = rpro_book::Book::load(&store.root().join("book"))?;
    if book.is_empty() {
        println!(
            "{}",
            style("No book content yet. Run `rpro init --refresh-book` (v0.1).").yellow()
        );
        return Ok(());
    }
    let needle = term.to_lowercase();
    let mut hits = 0;
    for chap in book.chapters.values() {
        let lower = chap.markdown.to_lowercase();
        if lower.contains(&needle) {
            hits += 1;
            println!(
                "{}: {} matches",
                style(&chap.id).cyan().bold(),
                lower.matches(&needle).count()
            );
        }
    }
    if hits == 0 {
        println!("{}", style(format!("no chapters match `{term}`")).dim());
    }
    Ok(())
}

fn cmd_progress() -> Result<()> {
    let store = Store::user()?;
    let progress = store.load_progress()?;
    let exercises = rpro_runner::discover(&store.root().join("exercises"))?;
    let total = exercises.len();
    let done = progress.done_count();
    let pct = (done * 100).checked_div(total).unwrap_or(0);
    println!("{}", style("Rustlings Pro — progress").bold().cyan());
    println!("  {done} / {total} done ({pct}%)");
    let remaining: u32 = exercises
        .iter()
        .filter(|ex| {
            progress
                .entries
                .get(&ex.meta.id)
                .is_none_or(|e| e.status != ExerciseStatus::Done)
        })
        .map(|ex| ex.meta.estimated_minutes)
        .sum();
    if remaining > 0 {
        println!("  estimated remaining: {remaining} min");
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn write_if_missing(path: &std::path::Path, contents: &str) -> Result<()> {
    if path.exists() {
        return Ok(());
    }
    std::fs::write(path, contents).with_context(|| format!("write {}", path.display()))?;
    Ok(())
}
