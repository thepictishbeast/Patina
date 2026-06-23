//! TUI shell for Rustlings Pro / Tempered Studio.
//!
//! The public surface the CLI calls into ([`run_dashboard`],
//! [`run_book_reader`]); screens are built from the pure [`app::App`] model and
//! the [`render`] functions (which are `TestBackend`-tested). This same TUI is
//! what runs inside the GUI's embedded terminal pane (see `gui/`, `docs/UX.md`).

#![doc(html_no_source)]

pub mod app;
pub mod render;
pub mod status;
pub mod theme;

use anyhow::Result;
use app::{App, Tab};
use rpro_book::Book;
use rpro_core::Core;
use rpro_lang::{EditorAssists, ExerciseId, ExerciseSource, Outcome, RunOp, ToolError};
use rpro_lang_rust::RustLanguage;
use rpro_state::ExerciseStatus;
use rpro_storage_fs::Store;
use rpro_toolchain_local::LocalProcess;
use std::path::Path;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

/// What a background run sends back to the UI thread.
type RunResult = Result<Outcome, ToolError>;

/// The project roadmap, baked into the binary so the Roadmap tab always has it.
const ROADMAP_MD: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../docs/ROADMAP.md"
));

/// Open the dashboard — progress, the current exercise, and what's up next.
///
/// # Errors
/// I/O setting up the terminal or driving the render loop.
pub fn run_dashboard(store: &Store, book: &Book) -> Result<()> {
    run_tui(store, book, Tab::Dashboard, 0)
}

/// Open the TUI on the book reader. `start_chapter`, if `Some`, selects that
/// chapter id; otherwise the first chapter.
///
/// # Errors
/// I/O setting up the terminal or driving the render loop.
pub fn run_book_reader(store: &Store, book: &Book, start_chapter: Option<&str>) -> Result<()> {
    let start = start_chapter
        .and_then(|c| book.chapters.keys().position(|k| k == c))
        .unwrap_or(0);
    run_tui(store, book, Tab::Book, start)
}

/// The one tabbed TUI, opened on `start_tab` with row `start_selected`
/// pre-selected. Every screen shares the tab bar and one event loop. On the
/// Exercise tab, `r`/`c` run/check the current exercise on a **background
/// thread** (so a compile never freezes the UI); the result streams back over a
/// channel and fills the raw-output pane — the live by-hand-error loop.
fn run_tui(store: &Store, book: &Book, start_tab: Tab, start_selected: usize) -> Result<()> {
    let theme_name = store
        .load_config()
        .map_or_else(|_| "dark".to_string(), |c| c.theme);
    let mut app = App::new(theme::Theme::from_env(&theme_name), status::ascii_only());
    app.tab = start_tab;
    app.selected = start_selected;
    let mut dash = dashboard_data(store);
    let mut ex = exercise_view_data(store);
    let store_root = store.root().to_path_buf();

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;
    let mut run_rx: Option<Receiver<RunResult>> = None;
    // Whether the in-flight run advances the learner on pass (Run/Test, not Check).
    let mut run_advances = false;

    let loop_result = (|| -> Result<()> {
        loop {
            let list_len = match app.tab {
                Tab::Dashboard => dash.up_next.len(),
                Tab::Book => book.chapters.len(),
                Tab::Exercise | Tab::Roadmap => 0,
            };
            app.set_list_len(list_len);
            // Collect a finished background run, if one landed.
            if let Some(rx) = &run_rx {
                if let Ok(result) = rx.try_recv() {
                    apply_run_result(&mut ex, result);
                    app.running = false;
                    run_rx = None;
                    // Record into shared progress (attempt + spaced repetition +
                    // advance) via the one helper the web surface also uses, then
                    // refresh the dashboard so the gauge + Recall reflect it.
                    let passed = ex.verdict.is_some_and(|(p, _)| p);
                    let advanced = rpro_runner::record_run(
                        store,
                        &ex.id,
                        &ex.diagnostics,
                        passed,
                        run_advances && passed,
                    );
                    dash = dashboard_data(store);
                    if advanced.is_some() {
                        // Promote the next exercise into the view so r/c target it,
                        // and reset the hint ladder (it belongs to the old exercise).
                        ex = exercise_view_data(store);
                        app.scroll = 0;
                        app.hint_level = 0;
                    }
                }
            }
            terminal.draw(|f| match app.tab {
                Tab::Dashboard => render::render_dashboard(f, &app, &dash),
                Tab::Exercise => render::render_exercise(f, &app, &ex),
                Tab::Book => render::render_book(f, &app, book),
                Tab::Roadmap => render::render_roadmap(f, &app, ROADMAP_MD),
            })?;
            if event::poll(Duration::from_millis(80))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                            KeyCode::Tab => app.next_tab(),
                            KeyCode::BackTab => app.prev_tab(),
                            KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                            KeyCode::Up | KeyCode::Char('k') => app.select_prev(),
                            KeyCode::PageDown | KeyCode::Char(' ') => app.scroll_down(),
                            KeyCode::PageUp => app.scroll_up(),
                            KeyCode::Char('r' | 'c')
                                if app.tab == Tab::Exercise && run_rx.is_none() =>
                            {
                                let op = if key.code == KeyCode::Char('c') {
                                    RunOp::Check
                                } else {
                                    RunOp::Run
                                };
                                let advances = matches!(op, RunOp::Run | RunOp::Test);
                                if let Some(rx) = spawn_run(&store_root, op) {
                                    app.running = true;
                                    ex.running = true;
                                    ex.verdict = None;
                                    run_rx = Some(rx);
                                    run_advances = advances;
                                }
                            }
                            // Climb the hint ladder one rung (concept → expected
                            // error → book/source review, last resort; never the
                            // solution). Shared with the web via ExerciseMetadata::hint.
                            KeyCode::Char('h') if app.tab == Tab::Exercise => {
                                if let Some(m) = &ex.meta {
                                    let (level, max, text) = m.hint(app.hint_level + 1);
                                    app.hint_level = level;
                                    ex.hint = Some((level, max, text));
                                }
                            }
                            _ => {}
                        }
                    }
                }
            } else {
                app.tick();
            }
            if app.should_quit {
                return Ok(());
            }
        }
    })();

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    loop_result
}

/// Build the dashboard's display data from on-disk state. Defensive: an
/// un-initialized store (no progress / no exercises) yields an empty dashboard
/// rather than an error.
fn dashboard_data(store: &Store) -> render::DashboardData {
    let progress = store.load_progress().unwrap_or_default();
    let exercises = rpro_runner::discover(&store.root().join("exercises")).unwrap_or_default();

    let status_of = |id: &str| {
        progress
            .entries
            .get(id)
            .map_or(ExerciseStatus::Locked, |e| e.status)
    };

    let current_id = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone());
    let current_title = current_id.as_ref().and_then(|id| {
        exercises
            .iter()
            .find(|e| &e.meta.id == id)
            .map(|e| e.meta.title.clone())
    });

    let up_next = exercises
        .iter()
        .filter(|e| {
            !matches!(
                status_of(&e.meta.id),
                ExerciseStatus::Done | ExerciseStatus::Current
            )
        })
        .map(|e| e.meta.id.clone())
        .take(8)
        .collect();

    render::DashboardData {
        done: progress.done_count(),
        total: exercises.len(),
        current_id,
        current_title,
        up_next,
        // Spaced-repetition queue from shared progress (web + TUI write the same
        // `Progress.reviews`); weakest-first, retired codes excluded.
        due: progress.reviews.due(),
        mastered: progress.reviews.mastered_count(),
        tracked: progress.reviews.tracked_count(),
    }
}

/// Build the exercise view's data from the current exercise on disk. No run has
/// happened yet (that's the `rpro run` keystone), so output/diagnostics are
/// empty — the screen shows the ready-to-run scaffold + book refs.
fn exercise_view_data(store: &Store) -> render::ExerciseViewData {
    let progress = store.load_progress().unwrap_or_default();
    let exercises = rpro_runner::discover(&store.root().join("exercises")).unwrap_or_default();
    let current = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone())
        .and_then(|id| exercises.into_iter().find(|e| e.meta.id == id));
    current.map_or_else(
        || render::ExerciseViewData {
            id: "(no current exercise — run `rpro exercise next`)".into(),
            assists: EditorAssists::default(),
            ..Default::default()
        },
        |e| render::ExerciseViewData {
            id: e.meta.id.clone(),
            title: e.meta.title.clone(),
            book_refs: e
                .meta
                .book_refs
                .iter()
                .map(|r| (r.chapter.clone(), r.why.clone()))
                .collect(),
            assists: EditorAssists::default(),
            meta: Some(e.meta.clone()),
            ..Default::default()
        },
    )
}

/// Write the language scaffold for `code` into `dir` (creating parents).
fn write_scaffold(dir: &Path, code: &str) -> std::io::Result<()> {
    for (rel, contents) in RustLanguage::scaffold(code) {
        let path = dir.join(rel);
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&path, contents)?;
    }
    Ok(())
}

/// Resolve the current exercise, scaffold it, and run `op` on a **background
/// thread**; returns the receiver the UI polls (or `None` if there's no current
/// exercise). The `Core` is built *inside* the thread because it isn't `Send`;
/// only plain data (paths, code, the `Outcome`) crosses the boundary.
fn spawn_run(store_root: &Path, op: RunOp) -> Option<Receiver<RunResult>> {
    let store = Store::at(store_root.to_path_buf());
    let exercises = rpro_runner::discover(&store.root().join("exercises")).ok()?;
    let progress = store.load_progress().unwrap_or_default();
    let current_id = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone());
    let ex = current_id.and_then(|id| exercises.into_iter().find(|e| e.meta.id == id))?;
    let code = std::fs::read_to_string(&ex.source).ok()?;
    let id = ex.meta.id;
    let run_dir = store.root().join("run").join(rpro_runner::slug(&id));
    let root = store_root.to_path_buf();

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        if write_scaffold(&run_dir, &code).is_err() {
            let _ = tx.send(Err(ToolError::Spawn(
                "could not write scratch project".into(),
            )));
            return;
        }
        // Core is !Send → construct it here, never move it across the boundary.
        let core = Core::new(
            Box::new(RustLanguage),
            Box::new(LocalProcess),
            Box::new(Store::at(root)),
        );
        let src = ExerciseSource {
            id: ExerciseId(id),
            dir: run_dir.display().to_string(),
            entry: "src/main.rs".into(),
        };
        let _ = tx.send(pollster::block_on(core.run(&src, &op)));
    });
    Some(rx)
}

/// Fold a finished run's result into the exercise view's data.
fn apply_run_result(ex: &mut render::ExerciseViewData, result: RunResult) {
    ex.running = false;
    match result {
        Ok(outcome) => {
            ex.verdict = Some((outcome.status == Some(0), outcome.duration_ms));
            ex.raw_stderr = outcome.raw_stderr;
            ex.raw_stdout = outcome.raw_stdout;
            ex.diagnostics = outcome.diagnostics;
        }
        Err(e) => {
            ex.verdict = Some((false, 0));
            ex.raw_stderr = format!("could not run the toolchain: {e}");
            ex.raw_stdout = String::new();
            ex.diagnostics = Vec::new();
        }
    }
}
