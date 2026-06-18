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
use rpro_core::Core;
use rpro_lang::{ExerciseId, ExerciseSource, RunOp};
use rpro_lang_rust::RustLanguage;
use rpro_state::ExerciseStatus;
use rpro_storage_fs::Store;
use rpro_toolchain_local::LocalProcess;

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
    /// Probe the local toolchain (version + components) and list the
    /// learning tools available. Safe to run before `init`.
    Detect,
    /// Compile-and-run the current exercise (or the one named): shows the
    /// exact toolchain line, then the raw output verbatim, then diagnostics.
    Run {
        /// Exercise id (defaults to the current one).
        exercise: Option<String>,
    },
    /// Type-check the current exercise — fast feedback, no binary produced.
    Check {
        /// Exercise id (defaults to the current one).
        exercise: Option<String>,
    },
    /// Run the current exercise's tests.
    Test {
        /// Exercise id (defaults to the current one).
        exercise: Option<String>,
    },
    /// Explain a diagnostic code in full (e.g. `rpro explain E0382`).
    Explain {
        /// The diagnostic code to explain.
        code: String,
    },
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
        Some(Cmd::Detect) => cmd_detect(),
        Some(Cmd::Run { exercise }) => cmd_exec(exercise.as_deref(), &RunOp::Run),
        Some(Cmd::Check { exercise }) => cmd_exec(exercise.as_deref(), &RunOp::Check),
        Some(Cmd::Test { exercise }) => cmd_exec(exercise.as_deref(), &RunOp::Test),
        Some(Cmd::Explain { code }) => cmd_explain(&code),
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

    // Seed the bundled exercises (the workspace `exercises/` tree) if the store
    // has none yet, then mark the first one current — so a fresh `rpro` lands on
    // a real, runnable exercise instead of an empty screen. This is parity with
    // the web server's auto-seed.
    let exercises_dir = store.root().join("exercises");
    let bundled = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../exercises");
    let have_any = rpro_runner::discover(&exercises_dir).map(|v| !v.is_empty()).unwrap_or(false);
    if !have_any && bundled.is_dir() {
        copy_tree(&bundled, &exercises_dir)
            .with_context(|| format!("seeding exercises from {}", bundled.display()))?;
        let n = rpro_runner::discover(&exercises_dir).map(|v| v.len()).unwrap_or(0);
        println!("  + seeded {} bundled exercise(s)", n);
    }
    // Ensure a Current exercise so the first run isn't an empty screen.
    let mut progress = store.load_progress().unwrap_or_default();
    let has_current = progress.entries.values().any(|e| e.status == ExerciseStatus::Current);
    if !has_current {
        if let Ok(mut exs) = rpro_runner::discover(&exercises_dir) {
            exs.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
            if let Some(first) = exs.first() {
                progress.set_current(&first.meta.id);
                store.save_progress(&progress)?;
                println!("  + current exercise: {}", style(&first.meta.id).cyan());
            }
        }
    }

    // Drop a README in each dir so the layout is discoverable. The book content
    // ships in a later pass (see docs/BACKLOG.md §E).
    write_if_missing(
        &store.root().join("exercises/README.md"),
        "Exercises live here (seeded from the bundled set on `rpro init`). Each \
         exercise is a `name.rs` (the failing starter) plus a sibling `name.toml` \
         with metadata (id, title, difficulty, concept, expected_error_code, \
         book_refs). Drop your own alongside them.\n",
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
            style(rpro_lang::Language::book_ref_url(&rpro_lang_rust::RustLanguage, r))
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
    rpro_tui::run_book_reader(&store, &book, None)?;
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

fn cmd_detect() -> Result<()> {
    use rpro_lang::Language as _;

    let store = Store::user().context("locating ~/.rustlings-pro/")?;
    let lang = RustLanguage;
    let probe = lang.detect_plan();
    let tools = lang.tools();

    // Route the probe through the Core so this command exercises the exact
    // plan → execute → interpret path every surface will use.
    let core = Core::new(Box::new(lang), Box::new(LocalProcess), Box::new(store));

    println!("{}", style("Toolchain check").bold().cyan());
    println!("  {} {}", style("$").dim(), style(&probe.display).dim());
    println!();

    match pollster::block_on(core.detect()) {
        Ok(status) => {
            if status.present {
                println!("  {} toolchain detected", style("✓").green().bold());
            } else {
                println!(
                    "  {} no toolchain detected on PATH",
                    style("✗").yellow().bold()
                );
            }
            if let Some(v) = &status.version {
                println!("  {:<12} {}", "version:", style(v).bold());
            }
            let comps = if status.components.is_empty() {
                style("(none reported)").dim().to_string()
            } else {
                status.components.join(", ")
            };
            println!("  {:<12} {comps}", "components:");
        }
        Err(e) => {
            println!("  {} could not run the probe", style("✗").red().bold());
            println!("  {e}");
            println!();
            println!(
                "  Install the toolchain from {} and re-run.",
                style("https://rustup.rs").underlined()
            );
        }
    }

    println!();
    println!("{}", style("Learning tools").bold().cyan());
    for t in &tools {
        println!(
            "  {:<14} {:<10} {}",
            style(&t.name).bold(),
            format!("{:?}", t.kind).to_lowercase(),
            style(&t.plan.display).dim()
        );
    }
    Ok(())
}

fn cmd_exec(id: Option<&str>, op: &RunOp) -> Result<()> {
    let store = Store::user()?;
    let exercises =
        rpro_runner::discover(&store.root().join("exercises")).context("discovering exercises")?;
    if exercises.is_empty() {
        println!(
            "{}",
            style("No exercises yet. Run `rpro init` (v0.1) to fetch the set, or pass an id.")
                .yellow()
        );
        return Ok(());
    }
    let ex = resolve_exercise(&store, &exercises, id)?;
    // `ex.source` is a path discovered by walking the *local* exercises dir and
    // validated during discovery — not external/untrusted input. The
    // web-oriented (Actix) path-traversal rule is a false positive for this
    // local-CLI read of a file the tool itself found on disk.
    let code = std::fs::read_to_string(&ex.source) // nosemgrep
        .with_context(|| format!("reading {}", ex.source.display()))?;

    // Prepare a scratch project under ~/.rustlings-pro/run/<id>/ (HOME is
    // exec-capable, unlike /tmp) and drop the learner's code in as the entry.
    // `slug` strips every path separator and `.`, so the result is a single
    // safe component that cannot escape `run_base` (no traversal). We still
    // reject an empty slug and assert containment as defense-in-depth.
    let run_base = store.root().join("run");
    let name = rpro_runner::slug(&ex.meta.id);
    if name.is_empty() {
        return Err(anyhow!("exercise id '{}' has no usable characters", ex.meta.id));
    }
    let run_dir = run_base.join(&name);
    debug_assert_eq!(run_dir.parent(), Some(run_base.as_path()));
    prepare_scratch_project(&run_dir, &code)?;

    let core = Core::new(Box::new(RustLanguage), Box::new(LocalProcess), Box::new(Store::user()?));
    let src = ExerciseSource {
        id: ExerciseId(ex.meta.id.clone()),
        dir: run_dir.display().to_string(),
        entry: "src/main.rs".into(),
    };

    // CLI-first contract: show the exact line, then the raw output verbatim.
    println!("{} {}", style(&ex.meta.id).bold().cyan(), style(&ex.meta.title).dim());
    println!("{} {}", style("$").dim(), style(&core.plan(&src, op).display).dim());
    println!();
    let outcome = pollster::block_on(core.run(&src, op)).context("running the toolchain")?;
    if !outcome.raw_stdout.is_empty() {
        print!("{}", outcome.raw_stdout);
    }
    if !outcome.raw_stderr.is_empty() {
        eprint!("{}", outcome.raw_stderr);
    }

    // Additive verdict + a by-hand diagnostic summary (raw is always above).
    println!();
    if outcome.status == Some(0) {
        println!("{} in {}ms", style("✓ passed").green().bold(), outcome.duration_ms);
    } else {
        println!(
            "{} · {} diagnostic(s) · {}ms",
            style("✗ failed").red().bold(),
            outcome.diagnostics.len(),
            outcome.duration_ms
        );
        for d in &outcome.diagnostics {
            let code = d.code.clone().unwrap_or_default();
            println!("  {} {}  {}", style("•").red(), style(code).bold(), d.message);
        }
    }
    Ok(())
}

/// Explain a diagnostic code in full, via the language's explain plan
/// (e.g. `rustc --explain E0382`), routed through the Core.
fn cmd_explain(code: &str) -> Result<()> {
    let core = Core::new(Box::new(RustLanguage), Box::new(LocalProcess), Box::new(Store::user()?));
    // `Explain` ignores the exercise source; a placeholder satisfies the API.
    let ex = ExerciseSource { id: ExerciseId(String::new()), dir: ".".into(), entry: String::new() };
    let op = RunOp::Explain(code.to_string());
    println!("{} {}", style("$").dim(), style(&core.plan(&ex, &op).display).dim());
    println!();
    let outcome = pollster::block_on(core.run(&ex, &op)).context("running the explainer")?;
    if !outcome.raw_stdout.is_empty() {
        print!("{}", outcome.raw_stdout);
    }
    if !outcome.raw_stderr.is_empty() {
        eprint!("{}", outcome.raw_stderr);
    }
    Ok(())
}

/// Resolve which exercise to act on: an explicit id, else the `Current` one,
/// else the first discovered.
fn resolve_exercise<'a>(
    store: &Store,
    exercises: &'a [rpro_runner::Exercise],
    id: Option<&str>,
) -> Result<&'a rpro_runner::Exercise> {
    if let Some(id) = id {
        return exercises
            .iter()
            .find(|e| e.meta.id == id)
            .ok_or_else(|| anyhow!("no exercise with id '{id}'"));
    }
    let progress = store.load_progress().unwrap_or_default();
    let current = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone());
    if let Some(cur) = current {
        if let Some(e) = exercises.iter().find(|e| e.meta.id == cur) {
            return Ok(e);
        }
    }
    exercises.first().ok_or_else(|| anyhow!("no exercises found"))
}

/// Create a minimal scratch project at `dir` from the language's scaffold, so
/// the toolchain can build and run the learner's code. The project layout comes
/// from `RustLanguage::scaffold` (keeps the Rust-specifics behind the seam).
fn prepare_scratch_project(dir: &std::path::Path, main_rs: &str) -> Result<()> {
    for (rel, contents) in RustLanguage::scaffold(main_rs) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("mkdir {}", parent.display()))?;
        }
        std::fs::write(&path, contents).with_context(|| format!("write {}", path.display()))?;
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

/// Recursively copy `from` into `to` (files + subdirs). Used only to seed the
/// bundled, read-only exercise tree into the user's writable store — both paths
/// are program-controlled (a compile-time workspace dir and the `~/.rustlings-pro`
/// store), never derived from external input.
// nosemgrep
fn copy_tree(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    // `from`/`to` are program-controlled only (a compile-time workspace dir and
    // the fixed ~/.rustlings-pro store); no external/user input reaches this path.
    // nosemgrep
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&src, &dst)?;
        } else {
            std::fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}
