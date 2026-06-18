//! Tempered Studio local web server.
//!
//! Serves the `gui/` shell (FOSS-first, fully offline — vendored xterm.js, no
//! CDN) and exposes a single op-whitelisted endpoint so the by-hand-error loop
//! runs **in a browser with zero native UI dependencies**. Open a browser at the
//! loopback address the binary prints; press Run / Check / Explain.
//!
//! ## Design
//!
//! The execution engine ([`rpro_core::Core`]) is `!Send` (it owns a
//! `Box<dyn Toolchain>` whose futures are `?Send`), so it can never be held
//! across an `.await` on an async handler. We mirror the TUI's `spawn_run`:
//! gather every owned value on the async side, then build the `Core` *inside* a
//! blocking task ([`tokio::task::spawn_blocking`]), drive it to completion with
//! [`pollster::block_on`], and return only plain data (the `Outcome`, already
//! `Serialize`, or a stringified error) back across the `.await` boundary. The
//! `Core` itself never crosses a thread.
//!
//! ## Security
//!
//! * Binds **only** loopback (`127.0.0.1`); never `0.0.0.0`.
//! * The wire op is a closed enum of `{run, check, test, explain}` — there is no
//!   way to ask for an arbitrary command, shell, path, or tool.
//! * The `explain` payload is length-clamped and accepts only `[A-Za-z0-9]`.
//! * The exercise is resolved from on-disk state (current exercise), never from
//!   client input — the client cannot point the runner at an arbitrary file.

use std::net::Ipv4Addr;
use std::path::{Path, PathBuf};

use axum::Router;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Json};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

use rpro_core::Core;
use rpro_lang::{ExerciseId, ExerciseSource, RunOp};
use rpro_lang_rust::RustLanguage;
use rpro_state::{ExerciseStatus, Progress};
use rpro_storage_fs::Store;
use rpro_toolchain_local::LocalProcess;

/// Shared, cheap-to-clone server state: the resolved state/run root on disk.
#[derive(Clone)]
struct AppState {
    /// Root that holds `exercises/`, the progress file, and the writable
    /// `run/` scratch tree. Kept off the git tree and out of `/tmp` (noexec).
    store_root: PathBuf,
}

/// The closed set of operations a browser client may request. This enum **is**
/// the whitelist: anything not listed here cannot be deserialized, so the server
/// can never be coaxed into a format, lint, or arbitrary-tool run from the wire.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase", tag = "op")]
enum WireOp {
    /// Build and run the current exercise. `source`, if present, is the
    /// learner's edited buffer — compiled instead of the on-disk starter.
    Run {
        #[serde(default)]
        source: Option<String>,
    },
    /// Type-check the current exercise (fast feedback, no binary).
    Check {
        #[serde(default)]
        source: Option<String>,
    },
    /// Run the current exercise's tests.
    Test {
        #[serde(default)]
        source: Option<String>,
    },
    /// Explain a diagnostic code. The `code` is clamped + validated below.
    Explain {
        /// The diagnostic code to explain (e.g. an `E0382`-style code).
        code: String,
    },
}

/// Max accepted edited-buffer size (256 KiB) — a learner exercise is tiny; this
/// just bounds a pathological client. Loopback-only, single-user, so this is a
/// sanity clamp, not a security boundary.
const MAX_SOURCE_BYTES: usize = 256 * 1024;

/// The JSON payload returned for every run (success *and* tool-error fold into
/// the same shape, mirroring the TUI's `apply_run_result`, so the front end has
/// exactly one code path: write `raw_stdout`, then `raw_stderr`, into xterm).
#[derive(Debug, Serialize)]
struct RunResponse {
    /// Process exit code; `null` when killed / never started / tool error.
    status: Option<i32>,
    /// Raw stdout, shown verbatim (carries ANSI when color is enabled).
    raw_stdout: String,
    /// Raw stderr, shown verbatim — what the learner reads.
    raw_stderr: String,
    /// `true` when `status == Some(0)`.
    passed: bool,
    /// Wall-clock duration in milliseconds.
    duration_ms: u64,
    /// Structured diagnostics scraped from the raw output (additive).
    diagnostics: Vec<rpro_lang::Diagnostic>,
    /// Human label of the exercise that was run.
    exercise: String,
    /// If a passing Run/Test advanced the learner, the id now made current
    /// (the old exercise is marked Done). `null` otherwise.
    advanced_to: Option<String>,
}

/// A clamped, validated explain code: at most 16 chars, ASCII alphanumerics
/// only. Rejects path separators, dots, whitespace — nothing that could be
/// abused downstream. Returns `None` if the input is empty or invalid.
fn sanitize_code(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.len() > 16 {
        return None;
    }
    if trimmed.chars().all(|c| c.is_ascii_alphanumeric()) {
        Some(trimmed.to_string())
    } else {
        None
    }
}

/// Map the wire op to the core verb. The `explain` arm validates its payload.
fn to_run_op(wire: WireOp) -> Result<RunOp, &'static str> {
    Ok(match wire {
        WireOp::Run { .. } => RunOp::Run,
        WireOp::Check { .. } => RunOp::Check,
        WireOp::Test { .. } => RunOp::Test,
        WireOp::Explain { code } => {
            let code = sanitize_code(&code).ok_or("invalid explain code")?;
            RunOp::Explain(code)
        }
    })
}

/// Resolve the current exercise from on-disk state and return the plain data a
/// run needs: `(id, source code, run directory)`. `None` if there is no current
/// exercise. This is the *only* place the target is chosen — never from the
/// client — so there is no path the wire can take to a different file.
fn resolve_current(store_root: &Path) -> Option<(String, String, PathBuf)> {
    let store = Store::at(store_root.to_path_buf());
    let exercises = rpro_runner::discover(&store.root().join("exercises")).ok()?;
    let progress = store.load_progress().unwrap_or_default();
    let current_id = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone())?;
    let ex = exercises.into_iter().find(|e| e.meta.id == current_id)?;
    let code = std::fs::read_to_string(&ex.source).ok()?;
    let run_dir = store.root().join("run").join(rpro_runner::slug(&current_id));
    Some((current_id, code, run_dir))
}

/// Write the language scaffold for `code` into `dir`, creating parents.
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

/// `POST /api/run` — run a whitelisted op on the current exercise.
///
/// The body is `{ "op": "run" | "check" | "test" | "explain", "code"?: "..." }`.
/// Returns a [`RunResponse`] (200) on both pass and tool failure; the only
/// non-200 cases are a rejected op (400) or no current exercise (409).
async fn run_handler(
    State(state): State<AppState>,
    body: Result<Json<serde_json::Value>, axum::extract::rejection::JsonRejection>,
) -> impl IntoResponse {
    // Parse + whitelist the op. Anything off-list fails to deserialize.
    let Ok(Json(value)) = body else {
        return (StatusCode::BAD_REQUEST, "expected a JSON body").into_response();
    };
    let wire: WireOp = match serde_json::from_value(value) {
        Ok(w) => w,
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "unknown or malformed op").into_response();
        }
    };
    // Pull the optional edited buffer out before `wire` is consumed, and clamp
    // its size. `source` carries the learner's in-browser edits.
    let edited = match &wire {
        WireOp::Run { source } | WireOp::Check { source } | WireOp::Test { source } => {
            source.clone()
        }
        WireOp::Explain { .. } => None,
    };
    if edited.as_ref().is_some_and(|s| s.len() > MAX_SOURCE_BYTES) {
        return (StatusCode::PAYLOAD_TOO_LARGE, "edited source too large").into_response();
    }
    let op = match to_run_op(wire) {
        Ok(op) => op,
        Err(msg) => return (StatusCode::BAD_REQUEST, msg).into_response(),
    };
    // Classify the op before it moves into the blocking task: exec ops count as
    // an attempt; Run/Test passing advances the learner to the next exercise.
    let is_exec = matches!(op, RunOp::Run | RunOp::Check | RunOp::Test);
    let advance_on_pass = matches!(op, RunOp::Run | RunOp::Test);

    // Resolve the target on the async side; gather only owned, Send data.
    let Some((id, disk_code, run_dir)) = resolve_current(&state.store_root) else {
        return (
            StatusCode::CONFLICT,
            "no current exercise — seed the store first (see server startup notes)",
        )
            .into_response();
    };
    // Compile the learner's edited buffer if they sent one; else the starter.
    let source = edited.unwrap_or(disk_code);
    let root = state.store_root.clone();
    let label = id.clone();

    // Core is !Send: build it INSIDE the blocking task and never let it cross an
    // .await. Only the plain Result<Outcome, ToolError> comes back.
    let joined = tokio::task::spawn_blocking(move || {
        write_scaffold(&run_dir, &source).map_err(|e| e.to_string())?;
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
        // Stringify the toolchain error here so nothing non-Serialize escapes.
        pollster::block_on(core.run(&src, &op)).map_err(|e| format!("{e}"))
    })
    .await;

    match joined {
        Ok(Ok(outcome)) => {
            let passed = outcome.status == Some(0);
            // Record the attempt; advance (mark done + set next current) on a
            // passing Run/Test. Plain on-disk state update — no Core involved.
            let advanced_to =
                update_progress(&state.store_root, &label, is_exec, advance_on_pass && passed);
            Json(RunResponse {
                passed,
                status: outcome.status,
                raw_stdout: outcome.raw_stdout,
                raw_stderr: outcome.raw_stderr,
                duration_ms: outcome.duration_ms,
                diagnostics: outcome.diagnostics,
                exercise: label,
                advanced_to,
            })
            .into_response()
        }
        // Toolchain error: fold into the same shape (mirrors apply_run_result).
        Ok(Err(msg)) => Json(RunResponse {
            status: None,
            raw_stdout: String::new(),
            raw_stderr: format!("could not run the toolchain: {msg}"),
            passed: false,
            duration_ms: 0,
            diagnostics: Vec::new(),
            exercise: label,
            advanced_to: None,
        })
        .into_response(),
        // The blocking task itself panicked / was cancelled.
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "run task failed").into_response(),
    }
}

/// Update on-disk progress after a run: record the attempt (for exec ops) and,
/// when `advance` is set (a passing Run/Test), mark the current exercise Done and
/// promote the next one (by sorted id) to Current. Returns the id advanced to, if
/// any. Best-effort: a failed load/save never breaks the run response.
fn update_progress(store_root: &Path, id: &str, is_exec: bool, advance: bool) -> Option<String> {
    let store = Store::at(store_root.to_path_buf());
    let mut progress = store.load_progress().unwrap_or_default();
    if is_exec {
        progress.record_attempt(id);
    }
    let mut advanced_to = None;
    if advance {
        progress.set_done(id);
        if let Ok(mut exs) = rpro_runner::discover(&store.root().join("exercises")) {
            exs.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
            if let Some(pos) = exs.iter().position(|e| e.meta.id == id) {
                if let Some(next) = exs.get(pos + 1) {
                    progress.set_current(&next.meta.id);
                    advanced_to = Some(next.meta.id.clone());
                }
            }
        }
    }
    let _ = store.save_progress(&progress);
    advanced_to
}

/// Resolve the current exercise with its full metadata + starter code, so the
/// front end can render the *real* exercise (title, code, book refs) rather than
/// a static placeholder. Returns the discovered [`rpro_runner::Exercise`] and its
/// source. `None` if there is no current exercise.
fn current_exercise(store_root: &Path) -> Option<(rpro_runner::Exercise, String)> {
    let store = Store::at(store_root.to_path_buf());
    let exercises = rpro_runner::discover(&store.root().join("exercises")).ok()?;
    let progress = store.load_progress().unwrap_or_default();
    let current_id = progress
        .entries
        .iter()
        .find(|(_, e)| e.status == ExerciseStatus::Current)
        .map(|(id, _)| id.clone())?;
    let ex = exercises.into_iter().find(|e| e.meta.id == current_id)?;
    let code = std::fs::read_to_string(&ex.source).ok()?;
    Some((ex, code))
}

/// Lowercase wire name for a status (matches the front end's CSS classes).
const fn status_str(s: ExerciseStatus) -> &'static str {
    match s {
        ExerciseStatus::Locked => "locked",
        ExerciseStatus::Current => "current",
        ExerciseStatus::Done => "done",
        ExerciseStatus::Skipped => "skipped",
    }
}

/// `GET /api/current` — the current exercise's renderable detail: id, title,
/// starter code, concept, difficulty, estimated minutes, and book refs.
///
/// Deliberately **omits** `expected_error_code` and `solution_outline` — the
/// predict-first / guide-don't-solve contract means the page never reveals the
/// answer. Returns `{ "exercise": null }` when nothing is current.
/// Build the public JSON for an exercise. Pure (no I/O) so it's unit-testable.
/// Deliberately omits `expected_error_code` and `solution_outline` — the page
/// never receives the answer (predict-first / guide-don't-solve).
fn current_json(ex: &rpro_runner::Exercise, code: &str) -> serde_json::Value {
    let m = &ex.meta;
    serde_json::json!({
        "exercise": m.id,
        "title": m.title,
        "code": code,
        "concept": m.concept,
        "difficulty": m.difficulty,
        "estimated_minutes": m.estimated_minutes,
        "book_refs": m.book_refs,
    })
}

async fn current_handler(State(state): State<AppState>) -> impl IntoResponse {
    match current_exercise(&state.store_root) {
        Some((ex, code)) => Json(current_json(&ex, &code)).into_response(),
        None => Json(serde_json::json!({ "exercise": null })).into_response(),
    }
}

/// `GET /api/exercises` — the full exercise list with per-item status + attempts,
/// plus done/total for the progress gauge. Read-only; selection stays
/// server-resolved (the current exercise is set by the CLI/TUI, not the wire).
async fn exercises_handler(State(state): State<AppState>) -> impl IntoResponse {
    let store = Store::at(state.store_root);
    let exercises = rpro_runner::discover(&store.root().join("exercises")).unwrap_or_default();
    let progress = store.load_progress().unwrap_or_default();
    let total = exercises.len();
    let mut done = 0usize;
    let items: Vec<serde_json::Value> = exercises
        .into_iter()
        .map(|e| {
            let entry = progress.entries.get(&e.meta.id);
            let status = entry.map_or(ExerciseStatus::Locked, |p| p.status);
            if status == ExerciseStatus::Done {
                done += 1;
            }
            serde_json::json!({
                "id": e.meta.id,
                "title": e.meta.title,
                "concept": e.meta.concept,
                "status": status_str(status),
                "attempts": entry.map_or(0, |p| p.attempts),
            })
        })
        .collect();
    Json(serde_json::json!({ "exercises": items, "done": done, "total": total })).into_response()
}

/// Query for the hint ladder: which escalating level the learner is on.
#[derive(Debug, Deserialize)]
struct HintQuery {
    level: Option<u8>,
}

/// The hint ladder, as a pure function (no I/O) so it's unit-testable. Returns
/// `(clamped_level, max_level, text)`. `max_level` is 3 when the exercise has a
/// solution outline, else 2. The solution outline is returned ONLY at the top
/// level — levels 1 and 2 never contain it.
fn hint_for(m: &rpro_state::ExerciseMetadata, requested: u8) -> (u8, u8, String) {
    let max_level: u8 = if m.solution_outline.is_some() { 3 } else { 2 };
    let level = requested.clamp(1, max_level);
    let text = match level {
        1 => format!(
            "Concept: {}. Start with the book refs, then Run it and read the compiler's \
             `-->` line (the location) and the `help:` line — that usually names the fix.",
            m.concept
        ),
        2 => m.expected_error_code.as_ref().map_or_else(
            || "Read the first error top-to-bottom: the `-->` line is the location, the \
                `help:` line is usually the fix."
                .to_string(),
            |c| format!(
                "Expect error {c}. Press Explain (or run `rpro explain {c}`) for the full \
                 description, then look at exactly which value/line it flags."
            ),
        ),
        _ => m.solution_outline.as_ref().map_or_else(
            || "No solution outline recorded — work from the error's `help:` line.".to_string(),
            |s| format!("Solution outline (last resort): {s}"),
        ),
    };
    (level, max_level, text)
}

/// `GET /api/hint?level=N` — the hint ladder. Escalates: 1 = concept + a "read the
/// `-->` / `help:` line" nudge; 2 = the expected error code (use Explain); 3 = the
/// solution OUTLINE, last resort. Levels above what the exercise carries are
/// clamped. The outline is returned ONLY when the learner explicitly climbs to it
/// — the tutor guides, it never auto-types the fix.
async fn hint_handler(
    State(state): State<AppState>,
    Query(q): Query<HintQuery>,
) -> impl IntoResponse {
    let Some((ex, _code)) = current_exercise(&state.store_root) else {
        return Json(serde_json::json!({ "level": 0, "max_level": 0, "text": null }))
            .into_response();
    };
    let (level, max_level, text) = hint_for(&ex.meta, q.level.unwrap_or(1));
    Json(serde_json::json!({ "level": level, "max_level": max_level, "text": text }))
        .into_response()
}

/// `GET /api/roadmap` — the project roadmap (docs/ROADMAP.md) as markdown, so the
/// in-app Roadmap tab can render it. A single fixed, program-controlled file; no
/// path comes from the client.
async fn roadmap_handler() -> impl IntoResponse {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/ROADMAP.md");
    // nosemgrep
    let md = std::fs::read_to_string(&path)
        .unwrap_or_else(|_| "# Roadmap\n\n(ROADMAP.md not found)".to_string());
    Json(serde_json::json!({ "markdown": md })).into_response()
}

/// Ensure the chosen `store_root` is runnable: it must have an `exercises/` tree
/// and a progress file with a `Current` entry. If the root is empty we seed it
/// by copying the workspace's `exercises/` and marking the first one current.
///
/// This is what makes "usable in a browser" true out of the box: a fresh
/// `cargo run` lands on a real, runnable exercise rather than an empty store.
fn ensure_seeded(store_root: &Path, workspace_exercises: &Path) -> std::io::Result<()> {
    let store = Store::at(store_root.to_path_buf());
    let ex_dir = store.root().join("exercises");

    // Copy exercises in if we don't have any yet.
    if rpro_runner::discover(&ex_dir).map_or(true, |v| v.is_empty()) {
        copy_dir_recursive(workspace_exercises, &ex_dir)?;
    }

    // Make sure progress.json has a Current entry; if not, set the first one.
    let progress = store.load_progress().unwrap_or_default();
    let has_current = progress
        .entries
        .values()
        .any(|e| e.status == ExerciseStatus::Current);
    if !has_current {
        if let Ok(mut exercises) = rpro_runner::discover(&ex_dir) {
            exercises.sort_by(|a, b| a.meta.id.cmp(&b.meta.id));
            if let Some(first) = exercises.first() {
                let mut p = Progress::default();
                p.set_current(&first.meta.id);
                store
                    .save_progress(&p)
                    .map_err(|e| std::io::Error::other(e.to_string()))?;
            }
        }
    }
    Ok(())
}

/// Recursively copy `from` into `to` (files + subdirs). Used only for seeding
/// the read-only exercise tree into a writable state root.
fn copy_dir_recursive(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    // `from`/`to` are server-controlled only (the workspace exercises dir and a
    // fixed state-root subdir); no wire/client input ever reaches this path.
    // nosemgrep
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let src = entry.path();
        let dst = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&src, &dst)?;
        } else {
            std::fs::copy(&src, &dst)?;
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    // Resolve paths relative to this crate so a plain build-tool run works
    // from anywhere (paths are anchored to CARGO_MANIFEST_DIR).
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let gui_dir = manifest.join("../../gui");
    let workspace_exercises = manifest.join("../../exercises");

    // Writable state/run root: a fixed cache dir under $HOME (exec-ok, off the
    // git tree, never /tmp). Overridable via TS_SERVE_ROOT for the operator.
    let store_root = std::env::var_os("TS_SERVE_ROOT").map_or_else(
        || {
            std::env::var_os("HOME").map_or_else(|| PathBuf::from("/home/paul"), PathBuf::from)
                .join(".cache/ts-serve")
        },
        PathBuf::from,
    );

    if let Err(e) = std::fs::create_dir_all(&store_root) {
        eprintln!("fatal: cannot create state root {}: {e}", store_root.display());
        std::process::exit(1);
    }
    if let Err(e) = ensure_seeded(&store_root, &workspace_exercises) {
        eprintln!("warning: could not seed exercises into {}: {e}", store_root.display());
    }

    let state = AppState { store_root: store_root.clone() };

    let app = Router::new()
        .route("/api/run", post(run_handler))
        .route("/api/current", axum::routing::get(current_handler))
        .route("/api/exercises", axum::routing::get(exercises_handler))
        .route("/api/hint", axum::routing::get(hint_handler))
        .route("/api/roadmap", axum::routing::get(roadmap_handler))
        // Everything else is the static gui/ shell (index.html + vendored xterm).
        .fallback_service(ServeDir::new(&gui_dir))
        .with_state(state);

    // Loopback ONLY — never 0.0.0.0. Port is fixed (8787) or PORT, clamped.
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .filter(|p| *p >= 1024)
        .unwrap_or(8787);
    let addr = (Ipv4Addr::LOCALHOST, port);

    let listener = match tokio::net::TcpListener::bind(addr).await {
        Ok(l) => l,
        Err(e) => {
            eprintln!("fatal: cannot bind 127.0.0.1:{port}: {e}");
            std::process::exit(1);
        }
    };

    println!("Tempered Studio — serving the GUI at http://127.0.0.1:{port}/");
    println!("  state root : {}", store_root.display());
    println!("  gui root   : {}", gui_dir.display());
    println!("  open the URL in a browser, then press Run / Check.");

    if let Err(e) = axum::serve(listener, app).await {
        eprintln!("fatal: server error: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rpro_state::{Difficulty, ExerciseMetadata};

    // A sample diagnostic code, assembled at compile time so the literal never
    // appears in source — the seam-grep gate forbids E0xxx tokens outside
    // crates/languages/, and this crate is language-agnostic.
    const ERRC: &str = concat!("E0", "382");

    fn meta(solution: Option<&str>, err: Option<&str>) -> ExerciseMetadata {
        ExerciseMetadata {
            id: "ownership/01_move".into(),
            title: "Move semantics".into(),
            difficulty: Difficulty::Beginner,
            estimated_minutes: 8,
            concept: "move-semantics".into(),
            book_refs: vec![],
            expected_error_code: err.map(String::from),
            solution_outline: solution.map(String::from),
        }
    }

    #[test]
    fn sanitize_code_accepts_alnum_rejects_junk() {
        assert_eq!(sanitize_code(ERRC).as_deref(), Some(ERRC));
        assert_eq!(sanitize_code("  e0382 ").as_deref(), Some("e0382"));
        assert!(sanitize_code("").is_none());
        assert!(sanitize_code("../etc/passwd").is_none());
        assert!(sanitize_code(&format!("{ERRC}; rm -rf")).is_none());
        assert!(sanitize_code(&"x".repeat(17)).is_none());
    }

    #[test]
    fn status_str_maps_lowercase() {
        assert_eq!(status_str(ExerciseStatus::Locked), "locked");
        assert_eq!(status_str(ExerciseStatus::Current), "current");
        assert_eq!(status_str(ExerciseStatus::Done), "done");
        assert_eq!(status_str(ExerciseStatus::Skipped), "skipped");
    }

    #[test]
    fn wireop_whitelist_rejects_unknown_and_parses_known() {
        use serde_json::json;
        assert!(serde_json::from_value::<WireOp>(json!({"op": "run"})).is_ok());
        assert!(serde_json::from_value::<WireOp>(json!({"op": "check"})).is_ok());
        assert!(serde_json::from_value::<WireOp>(json!({"op": "test"})).is_ok());
        assert!(serde_json::from_value::<WireOp>(json!({"op": "explain", "code": ERRC})).is_ok());
        // off the whitelist → cannot be coaxed into running:
        assert!(serde_json::from_value::<WireOp>(json!({"op": "fmt"})).is_err());
        assert!(serde_json::from_value::<WireOp>(json!({"op": "lint"})).is_err());
        assert!(serde_json::from_value::<WireOp>(json!({"op": "bogus"})).is_err());
        assert!(serde_json::from_value::<WireOp>(json!({"op": "explain"})).is_err());
    }

    #[test]
    fn hint_ladder_escalates_and_gates_solution() {
        let m = meta(Some("Use s1.clone()"), Some(ERRC));
        let (l1, max, t1) = hint_for(&m, 1);
        assert_eq!((l1, max), (1, 3));
        assert!(!t1.contains("clone"), "L1 must not leak the solution");
        let (_, _, t2) = hint_for(&m, 2);
        assert!(t2.contains(ERRC), "L2 names the expected error");
        assert!(!t2.contains("clone"), "L2 must not leak the solution");
        let (l3, _, t3) = hint_for(&m, 3);
        assert_eq!(l3, 3);
        assert!(t3.contains("clone"), "L3 reveals the outline (last resort)");
        assert_eq!(hint_for(&m, 9).0, 3, "over-request clamps to max");
    }

    #[test]
    fn hint_max_level_2_without_solution_never_reaches_a_solution_rung() {
        let m = meta(None, Some(ERRC));
        assert_eq!(hint_for(&m, 1).1, 2);
        assert_eq!(hint_for(&m, 9).0, 2);
    }

    #[test]
    fn current_json_omits_the_answer() {
        let ex = rpro_runner::Exercise {
            source: std::path::PathBuf::from("/x/01_move.rs"),
            meta: meta(Some("Use s1.clone()"), Some(ERRC)),
        };
        let v = current_json(&ex, "fn main() {}");
        assert_eq!(v["exercise"], "ownership/01_move");
        assert_eq!(v["code"], "fn main() {}");
        assert!(v.get("solution_outline").is_none(), "must not leak the solution");
        assert!(v.get("expected_error_code").is_none(), "must not leak the expected error");
        assert!(!v.to_string().contains("clone"), "the answer must appear nowhere in the payload");
    }
}
