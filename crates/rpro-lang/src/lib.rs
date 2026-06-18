//! `rpro-lang` — the **language seam** for Tempered Studio.
//!
//! PURE and wasm-safe. This crate holds the trait *definitions* and the neutral,
//! serde-able types that cross every boundary (backend JSON response, wasm client
//! value, on-disk state). It contains **zero** language-specific knowledge — no
//! `cargo`, no `rustc`, no error-code literals. The one Rust implementation lives
//! in `crates/languages/rust` (`rpro-lang-rust`); the seam-grep CI gate enforces
//! that any such token appears only there.
//!
//! See `MASTERPLAN.md` §2.2–2.4. Boundary types all derive `Serialize + Deserialize`
//! so the *same* type is the backend's wire format and the wasm client's value.

use serde::{Deserialize, Serialize};

// ───────────────────────── neutral execution types ─────────────────────────

/// The neutral verb set the core speaks; a [`Language`] turns each into a [`CommandPlan`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunOp {
    /// Type-check only (fast feedback, no binary).
    Check,
    /// Build and run the exercise.
    Run,
    /// Run the exercise's tests.
    Test,
    /// Format the source.
    Fmt,
    /// Lint the source.
    Lint,
    /// Explain a diagnostic code (the payload is the code, e.g. `E0382`).
    Explain(String),
    /// Invoke a named tool from [`Language::tools`].
    Tool(String),
}

/// A concrete, executor-agnostic command. The verb is already baked in.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandPlan {
    /// Program to execute (e.g. `cargo`).
    pub program: String,
    /// Arguments, in order.
    pub args: Vec<String>,
    /// Working directory, if the command needs one.
    pub cwd: Option<String>,
    /// Extra environment variables as `(key, value)` pairs.
    pub env: Vec<(String, String)>,
    /// Exactly what the learner sees typed in the terminal (the CLI-first contract).
    pub display: String,
}

/// The result of running a [`CommandPlan`].
///
/// The **raw** output is always present and always shown; the parsed
/// [`diagnostics`](Outcome::diagnostics) are strictly additive. Because this
/// guarantee lives on the shared type, no surface can produce a verdict without
/// `raw_stderr` — that is what keeps the by-hand-error, CLI-first thesis true
/// even on the IDE surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Outcome {
    /// Process exit code; `None` if killed by a signal or never started.
    pub status: Option<i32>,
    /// Raw stdout — shown verbatim.
    pub raw_stdout: String,
    /// Raw stderr — shown verbatim (what the learner reads).
    pub raw_stderr: String,
    /// Structured diagnostics scraped from the raw output (additive).
    pub diagnostics: Vec<Diagnostic>,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u64,
}

/// Severity of a [`Diagnostic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagLevel {
    /// A hard error.
    Error,
    /// A warning.
    Warning,
    /// A note or help line.
    Note,
}

/// A source span (1-based line / column).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    /// File the span points into.
    pub file: String,
    /// 1-based line number.
    pub line: u32,
    /// 1-based column number.
    pub col: u32,
}

/// One parsed diagnostic. `code` is the language's own code (e.g. `E0382`), if any.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Diagnostic {
    /// Language-specific code, if the diagnostic has one.
    pub code: Option<String>,
    /// Severity.
    pub level: DiagLevel,
    /// Human-readable message.
    pub message: String,
    /// Primary span, if known.
    pub span: Option<Span>,
}

/// How to launch the language server for the Free / soft-Learning editor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LspSpec {
    /// Server executable (e.g. `rust-analyzer`).
    pub server: String,
    /// Launch arguments.
    pub args: Vec<String>,
    /// Optional initialization options, as a JSON blob.
    pub init_options: Option<String>,
    /// Glob of files the server owns (e.g. `**/*.rs`).
    pub file_glob: String,
}

/// Per-exercise editor assists (the two-mode contract). All-on is Free mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorAssists {
    /// Syntax highlighting on?
    pub syntax_highlight: bool,
    /// Autocomplete on?
    pub autocomplete: bool,
    /// Inline (in-editor) diagnostics on?
    pub inline_diagnostics: bool,
    /// Format-on-save on?
    pub format_on_save: bool,
}

impl Default for EditorAssists {
    /// Free mode — every assist on. (Absent `[editor]` block ⇒ this.)
    fn default() -> Self {
        Self { syntax_highlight: true, autocomplete: true, inline_diagnostics: true, format_on_save: true }
    }
}

/// The starter file the course hands the learner for a concept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseTemplate {
    /// File name to create (e.g. `main.rs`).
    pub filename: String,
    /// Initial contents (the scaffold the learner fixes — never the solution).
    pub contents: String,
}

/// What a [`LangTool`] is for (drives the Free/Learning tool choreography).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToolKind {
    /// A linter (e.g. clippy).
    Lint,
    /// A formatter (e.g. rustfmt).
    Format,
    /// A REPL (e.g. evcxr).
    Repl,
    /// A background watcher (e.g. bacon).
    Watch,
    /// A visualizer (e.g. aquascope).
    Visualize,
    /// A macro/codegen expander (e.g. cargo-expand).
    Expand,
    /// An undefined-behavior / sanitizer tool (e.g. miri).
    Sanitize,
    /// Anything else.
    Other,
}

/// A learning tool the language offers (clippy, rustfmt, bacon, evcxr, miri, aquascope, …).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LangTool {
    /// Display name.
    pub name: String,
    /// What kind of tool it is.
    pub kind: ToolKind,
    /// How to invoke it.
    pub plan: CommandPlan,
}

/// A rendered explanation of a diagnostic code (e.g. `rustc --explain` output).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ErrorExplanation {
    /// The code explained.
    pub code: String,
    /// The explanation body (markdown or plain text).
    pub body: String,
}

/// Stable identifier for an exercise (e.g. `ownership/01_move`).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ExerciseId(pub String);

/// A reference into the embedded book: chapter id + optional anchor + the "why".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BookRef {
    /// Chapter id (e.g. `ch04-01-what-is-ownership`).
    pub chapter: String,
    /// Optional in-page anchor.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<String>,
    /// One sentence: why this section helps with this exercise.
    pub why: String,
}

/// The on-disk source a [`Language`] turns into a [`CommandPlan`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExerciseSource {
    /// The exercise's id.
    pub id: ExerciseId,
    /// Directory containing the exercise.
    pub dir: String,
    /// Entry source file within `dir`.
    pub entry: String,
}

/// What a toolchain probe found.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolchainStatus {
    /// Is the toolchain present at all?
    pub present: bool,
    /// Version string, if detected.
    pub version: Option<String>,
    /// Installed components (e.g. `clippy`, `rust-src`).
    pub components: Vec<String>,
}

/// Error from running a [`CommandPlan`] via a [`Toolchain`].
#[derive(Debug, thiserror::Error)]
pub enum ToolError {
    /// The toolchain / program was not found.
    #[error("toolchain not found: {0}")]
    NotFound(String),
    /// The process failed to spawn.
    #[error("spawn failed: {0}")]
    Spawn(String),
    /// A remote (backend) toolchain reported an error.
    #[error("remote error: {0}")]
    Remote(String),
}

/// Error from a [`Storage`] backend.
#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    /// Underlying I/O or transport failure.
    #[error("io: {0}")]
    Io(String),
    /// Serialized data could not be decoded.
    #[error("decode: {0}")]
    Decode(String),
    /// The requested item does not exist.
    #[error("not found: {0}")]
    NotFound(String),
}

// ───────────────────────────── the seams ─────────────────────────────

/// Every language-specific decision behind one interface.
///
/// Pure: it **returns plans and parses output**; the effectful [`Toolchain`]
/// executes the plans. This is the only place a `Language` implementation may
/// know about `cargo`, `rustc`, error codes, rust-analyzer, or `doc.rust-lang.org`.
pub trait Language: Send + Sync {
    /// Stable id; selects `content/languages/<id>/`. (e.g. `"rust"`)
    fn id(&self) -> &str;
    /// Source file extension, without the dot (e.g. `"rs"`).
    fn source_ext(&self) -> &str;
    /// The starter file the course hands the learner for a concept.
    fn exercise_template(&self, concept: &str) -> ExerciseTemplate;
    /// Scrape structured diagnostics from raw compiler output (additive — raw is always kept).
    fn parse_diagnostics(&self, raw: &str) -> Vec<Diagnostic>;
    /// A plan that explains a diagnostic code, if the language supports it (e.g. `rustc --explain`).
    fn explain_plan(&self, code: &str) -> Option<CommandPlan>;
    /// Build the documentation URL for a [`BookRef`].
    fn book_ref_url(&self, r: &BookRef) -> String;
    /// How to launch the language server.
    fn lsp(&self) -> LspSpec;
    /// Plan a neutral [`RunOp`] against an exercise.
    fn command_plan(&self, ex: &ExerciseSource, op: &RunOp) -> CommandPlan;
    /// A plan that probes the local toolchain (e.g. `rustup show`).
    fn detect_plan(&self) -> CommandPlan;
    /// Interpret the output of [`detect_plan`](Language::detect_plan).
    fn parse_detect(&self, raw: &str) -> ToolchainStatus;
    /// The learning tools this language offers.
    fn tools(&self) -> Vec<LangTool>;
}

/// Executes [`CommandPlan`]s. Local (process) on desktop / Android / backend;
/// remote (HTTP) on web. Async because wasm has no `block_on`.
#[async_trait::async_trait(?Send)]
pub trait Toolchain {
    /// Probe whether the toolchain is available and usable.
    async fn detect(&self, plan: &CommandPlan) -> Result<Outcome, ToolError>;
    /// Run one plan (the verb is already encoded in the plan).
    async fn run(&self, plan: &CommandPlan) -> Result<Outcome, ToolError>;
}

/// Persists the learner's progress and code. Filesystem on native; state-API on web.
///
/// Note: progress/bookmarks/annotations are exchanged here in their **serialized**
/// (JSON) form — the typed model lives in `rpro-state`, kept out of this pure seam
/// so the crate stays wasm-safe and dependency-light.
#[async_trait::async_trait(?Send)]
pub trait Storage {
    /// Load the serialized progress document.
    async fn load_progress(&self) -> Result<String, StoreError>;
    /// Save the serialized progress document.
    async fn save_progress(&self, json: &str) -> Result<(), StoreError>;
    /// Load the learner's own saved code for an exercise.
    async fn load_code(&self, ex: &ExerciseId) -> Result<String, StoreError>;
    /// Save the learner's own code for an exercise.
    async fn save_code(&self, ex: &ExerciseId, src: &str) -> Result<(), StoreError>;
}
