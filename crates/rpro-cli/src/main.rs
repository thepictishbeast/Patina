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
    /// Book references + a laddered hint for the current exercise — what
    /// to read (and think) when stuck.
    Hint {
        /// Climb the shared hint ladder to this rung: 1 = concept nudge,
        /// 2 = the expected error code, 3 = the solution outline.
        #[arg(long, default_value_t = 1)]
        level: u8,
        /// Shortcut for the top rung — surface the one-line solution outline.
        #[arg(long)]
        solution: bool,
    },
    /// Skip the current exercise (mark it skipped) and move to the next.
    Skip,
    /// Reset the current exercise to a fresh attempt (clears done/skipped +
    /// attempt count so you can try it again from scratch).
    Reset,
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
            ExerciseCmd::Hint { level, solution } => cmd_exercise_hint(level, solution),
            ExerciseCmd::Skip => cmd_exercise_skip(),
            ExerciseCmd::Reset => cmd_exercise_reset(),
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
    let have_any = rpro_runner::discover(&exercises_dir).is_ok_and(|v| !v.is_empty());
    if !have_any && bundled.is_dir() {
        copy_tree(&bundled, &exercises_dir)
            .with_context(|| format!("seeding exercises from {}", bundled.display()))?;
        let n = rpro_runner::discover(&exercises_dir).map_or(0, |v| v.len());
        println!("  + seeded {n} bundled exercise(s)");
    }
    // Seed the bundled Rust Book chapters the same way, so the Book reader has
    // real content offline (parity with the exercises seed).
    let book_dir = store.root().join("book");
    let bundled_book = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../book");
    let have_book = rpro_book::Book::load(&book_dir).is_ok_and(|b| !b.chapters.is_empty());
    if !have_book && bundled_book.is_dir() {
        copy_tree(&bundled_book, &book_dir)
            .with_context(|| format!("seeding book from {}", bundled_book.display()))?;
        if let Ok(b) = rpro_book::Book::load(&book_dir) {
            println!("  + seeded {} Rust Book chapter(s)", b.chapters.len());
        }
    }
    // Ensure a Current exercise so the first run isn't an empty screen.
    let mut progress = store.load_progress().unwrap_or_default();
    let has_current = progress.entries.values().any(|e| e.status == ExerciseStatus::Current);
    if !has_current {
        if let Ok(mut exs) = rpro_runner::discover(&exercises_dir) {
            exs.sort_by(|a, b| a.source.cmp(&b.source)); // path order = learning order
            if let Some(first) = exs.first() {
                progress.set_current(&first.meta.id);
                store.save_progress(&progress)?;
                println!("  + current exercise: {}", style(&first.meta.id).cyan());
            }
        }
    }

    // Drop a README in the exercises dir so the layout is discoverable. (The book
    // dir intentionally gets no README: it's seeded with the real chapters, and a
    // `.md` there would be picked up by the book loader as a bogus chapter.)
    write_if_missing(
        &store.root().join("exercises/README.md"),
        "Exercises live here (seeded from the bundled set on `rpro init`). Each \
         exercise is a `name.rs` (the failing starter) plus a sibling `name.toml` \
         with metadata (id, title, difficulty, concept, expected_error_code, \
         book_refs). Drop your own alongside them.\n",
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

/// Resolve the current exercise id from on-disk progress, if any.
fn current_exercise_id(progress: &rpro_state::Progress) -> Option<String> {
    progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone())
}

fn cmd_exercise_skip() -> Result<()> {
    let store = Store::user()?;
    let mut progress = store.load_progress()?;
    let exercises = rpro_runner::discover(&store.root().join("exercises"))?;
    let Some(current_id) = current_exercise_id(&progress) else {
        println!(
            "{}",
            style("No current exercise to skip — run `rpro exercise next` first.").dim()
        );
        return Ok(());
    };
    progress.set_skipped(&current_id);
    // Advance to the next unfinished exercise (discovery is in learning order).
    let next = exercises.iter().find(|ex| {
        let s = progress.entries.get(&ex.meta.id).map_or(ExerciseStatus::Locked, |e| e.status);
        !matches!(s, ExerciseStatus::Done | ExerciseStatus::Skipped)
    });
    if let Some(ex) = next {
        progress.set_current(&ex.meta.id);
    }
    store.save_progress(&progress)?;
    println!("{} skipped {}", style("»").red().bold(), style(&current_id).bold());
    if let Some(ex) = next {
        println!("{} {}", style("→").cyan().bold(), style(&ex.meta.id).bold());
        println!("  {}", ex.meta.title);
    } else {
        println!("  {}", style("nothing left unfinished — you've reached the end.").green());
    }
    Ok(())
}

fn cmd_exercise_reset() -> Result<()> {
    let store = Store::user()?;
    let mut progress = store.load_progress()?;
    let Some(current_id) = current_exercise_id(&progress) else {
        println!(
            "{}",
            style("No current exercise to reset — run `rpro exercise next` first.").dim()
        );
        return Ok(());
    };
    progress.reset(&current_id);
    store.save_progress(&progress)?;
    println!(
        "{} reset {} — fresh attempt (done/attempt count cleared).",
        style("↺").yellow().bold(),
        style(&current_id).bold()
    );
    Ok(())
}

fn cmd_exercise_hint(level: u8, show_solution: bool) -> Result<()> {
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
    // Laddered hint text — the SAME shared ladder the web + TUI show
    // (`ExerciseMetadata::hint`), so guidance never drifts across surfaces.
    // `--solution` jumps to the top rung; otherwise climb to `--level`.
    let requested = if show_solution { u8::MAX } else { level.max(1) };
    let (lvl, max, text) = ex.meta.hint(requested);
    let last = lvl >= max;
    println!(
        "{}",
        style(format!(
            "Hint {lvl}/{max}{}:",
            if last { " · last resort" } else { "" }
        ))
        .yellow()
        .bold()
    );
    println!("  {text}");
    if !last {
        println!();
        let mut tip = format!("rpro exercise hint --level {}", lvl + 1);
        if max >= 3 {
            tip.push_str("  (or --solution for the outline)");
        }
        println!("  {} {}", style("Tip:").yellow(), style(tip).bold());
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
            style("No book content yet. Run `rpro init` to seed the bundled chapters.").yellow()
        );
        return Ok(());
    }
    // Shared with the web surface (rpro-serve `/api/book?q=`) via rpro-book.
    let hits = book.search(term);
    if hits.is_empty() {
        println!("{}", style(format!("no chapters match `{term}`")).dim());
        return Ok(());
    }
    for h in &hits {
        let n = if h.count == 1 { "match" } else { "matches" };
        // Human title first, then the chapter id (the canonical ref) + count;
        // the snippet gives one line of context — all from the shared SearchHit.
        println!(
            "{}  {}  {}",
            style(&h.title).cyan().bold(),
            style(&h.chapter).dim(),
            style(format!("({} {n})", h.count)).dim(),
        );
        if !h.snippet.is_empty() {
            println!("    {}", style(&h.snippet).dim());
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use rpro_state::{Difficulty, ExerciseMetadata, Progress};
    use std::path::PathBuf;

    fn ex(id: &str) -> rpro_runner::Exercise {
        rpro_runner::Exercise {
            source: PathBuf::from(format!("/x/{id}.rs")),
            meta: ExerciseMetadata {
                id: id.to_string(),
                title: id.to_string(),
                difficulty: Difficulty::Beginner,
                estimated_minutes: 5,
                concept: "concept".into(),
                book_refs: vec![],
                expected_error_code: None,
                solution_outline: None,
            },
        }
    }

    #[test]
    fn current_exercise_id_finds_only_the_current() {
        let mut p = Progress::default();
        assert_eq!(current_exercise_id(&p), None, "empty progress has no current");
        p.set_done("a/1");
        assert_eq!(current_exercise_id(&p), None, "a Done entry is not current");
        p.set_current("a/2");
        assert_eq!(current_exercise_id(&p).as_deref(), Some("a/2"));
    }

    #[test]
    fn resolve_exercise_prefers_id_then_current_then_first() {
        let dir = tempfile::tempdir().unwrap();
        let store = Store::at(dir.path().to_path_buf());
        let exs = vec![ex("a/1"), ex("a/2"), ex("a/3")];
        // An explicit id wins.
        assert_eq!(resolve_exercise(&store, &exs, Some("a/2")).unwrap().meta.id, "a/2");
        // An unknown explicit id is an error (never silently falls back).
        assert!(resolve_exercise(&store, &exs, Some("missing")).is_err());
        // No id + no progress → the first exercise (learning order).
        assert_eq!(resolve_exercise(&store, &exs, None).unwrap().meta.id, "a/1");
        // No id + a saved Current → that one.
        let mut p = Progress::default();
        p.set_current("a/3");
        store.save_progress(&p).unwrap();
        assert_eq!(resolve_exercise(&store, &exs, None).unwrap().meta.id, "a/3");
    }

    #[test]
    fn write_if_missing_never_overwrites() {
        let dir = tempfile::tempdir().unwrap();
        let f = dir.path().join("f.txt");
        write_if_missing(&f, "first").unwrap();
        write_if_missing(&f, "second").unwrap();
        assert_eq!(std::fs::read_to_string(&f).unwrap(), "first", "existing file is kept");
    }

    #[test]
    fn copy_tree_copies_files_and_nested_subdirs() {
        let src = tempfile::tempdir().unwrap();
        std::fs::write(src.path().join("top.txt"), "t").unwrap();
        std::fs::create_dir(src.path().join("sub")).unwrap();
        std::fs::write(src.path().join("sub").join("inner.txt"), "i").unwrap();
        let dst = tempfile::tempdir().unwrap();
        let to = dst.path().join("out");
        copy_tree(src.path(), &to).unwrap();
        assert_eq!(std::fs::read_to_string(to.join("top.txt")).unwrap(), "t");
        assert_eq!(std::fs::read_to_string(to.join("sub").join("inner.txt")).unwrap(), "i");
    }

    /// Data invariant behind the in-app Book jump links: every exercise's
    /// `book_refs` must resolve to a *bundled* chapter. A broken ref would
    /// otherwise only surface when a learner clicked it (chapter-not-found).
    /// Reads the real workspace `exercises/` + `book/` (pure data — no toolchain).
    #[test]
    fn every_book_ref_resolves_to_a_bundled_chapter() {
        let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
        let exercises = rpro_runner::discover(&manifest.join("../../exercises"))
            .expect("discover workspace exercises");
        let book = rpro_book::Book::load(&manifest.join("../../book")).expect("load bundled book");
        assert!(!exercises.is_empty() && !book.is_empty(), "fixtures present");
        let mut broken = Vec::new();
        for ex in &exercises {
            for r in &ex.meta.book_refs {
                if book.get(&r.chapter).is_none() {
                    broken.push(format!("{} -> {}", ex.meta.id, r.chapter));
                }
            }
        }
        assert!(
            broken.is_empty(),
            "book_refs with no bundled chapter:\n{}",
            broken.join("\n")
        );
    }
}
