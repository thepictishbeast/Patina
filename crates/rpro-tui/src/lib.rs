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
use rpro_lang::EditorAssists;
use rpro_state::ExerciseStatus;
use rpro_storage_fs::Store;
use std::time::Duration;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::Frame;
use ratatui::Terminal;
use ratatui::backend::CrosstermBackend;

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
/// pre-selected. Every screen shares the tab bar and the one event loop.
fn run_tui(store: &Store, book: &Book, start_tab: Tab, start_selected: usize) -> Result<()> {
    let theme_name = store.load_config().map_or_else(|_| "dark".to_string(), |c| c.theme);
    let mut app = App::new(theme::Theme::from_env(&theme_name), status::ascii_only());
    app.tab = start_tab;
    app.selected = start_selected;
    let dash = dashboard_data(store);
    let ex = exercise_view_data(store);
    // The selectable-list length depends on which screen is showing.
    let relen = |tab: Tab| match tab {
        Tab::Dashboard => dash.up_next.len(),
        Tab::Book => book.chapters.len(),
        Tab::Exercise | Tab::Roadmap => 0,
    };
    run_loop(&mut app, &relen, |f, app| match app.tab {
        Tab::Dashboard => render::render_dashboard(f, app, &dash),
        Tab::Exercise => render::render_exercise(f, app, &ex),
        Tab::Book => render::render_book(f, app, book),
        Tab::Roadmap => render::render_placeholder(f, app, "Roadmap"),
    })
}

/// Build the dashboard's display data from on-disk state. Defensive: an
/// un-initialized store (no progress / no exercises) yields an empty dashboard
/// rather than an error.
fn dashboard_data(store: &Store) -> render::DashboardData {
    let progress = store.load_progress().unwrap_or_default();
    let exercises = rpro_runner::discover(&store.root().join("exercises")).unwrap_or_default();

    let status_of = |id: &str| {
        progress.entries.get(id).map_or(ExerciseStatus::Locked, |e| e.status)
    };

    let current_id = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone());
    let current_title = current_id.as_ref().and_then(|id| {
        exercises.iter().find(|e| &e.meta.id == id).map(|e| e.meta.title.clone())
    });

    let up_next = exercises
        .iter()
        .filter(|e| !matches!(status_of(&e.meta.id), ExerciseStatus::Done | ExerciseStatus::Current))
        .map(|e| e.meta.id.clone())
        .take(8)
        .collect();

    render::DashboardData {
        done: progress.done_count(),
        total: exercises.len(),
        current_id,
        current_title,
        up_next,
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
            book_refs: e.meta.book_refs.iter().map(|r| (r.chapter.clone(), r.why.clone())).collect(),
            assists: EditorAssists::default(),
            ..Default::default()
        },
    )
}

/// The shared event loop: set up the terminal, draw + handle input until quit,
/// then always restore the terminal (even on a draw error).
fn run_loop(
    app: &mut App,
    relen: &dyn Fn(Tab) -> usize,
    mut draw: impl FnMut(&mut Frame, &App),
) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout))?;

    let loop_result = (|| -> Result<()> {
        loop {
            // Keep the selection clamped to whatever list the active screen shows.
            app.set_list_len(relen(app.tab));
            terminal.draw(|f| draw(f, &*app))?;
            // Poll with a timeout so the spinner animates on idle; on input,
            // route keys; on timeout, advance the throbber.
            if event::poll(Duration::from_millis(80))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Char('q') | KeyCode::Esc => app.quit(),
                            KeyCode::Tab => app.next_tab(),
                            KeyCode::BackTab => app.prev_tab(),
                            KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                            KeyCode::Up | KeyCode::Char('k') => app.select_prev(),
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
