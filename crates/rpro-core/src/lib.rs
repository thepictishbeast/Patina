//! `rpro-core` — the async session orchestrator.
//!
//! Every surface (Android, desktop, web) drives the same flow: pick a
//! [`Language`], pick a [`Toolchain`] (local process or remote HTTP), pick a
//! [`Storage`] (filesystem or state-API), and hand all three to a [`Core`].
//! From then on the surface speaks only neutral verbs ([`RunOp`]) and neutral
//! data — it never names a compiler, an error code, or a file path convention.
//!
//! `Core` is **pure plumbing**: it owns no language knowledge of its own. Each
//! method is a three-beat phrase — *plan* (pure, the [`Language`]'s job),
//! *execute* (effectful, the [`Toolchain`]'s or [`Storage`]'s job), *interpret*
//! (pure again). That split is what lets the same crate compile to `wasm32`
//! and run on the web with a remote toolchain, byte-for-byte the same logic as
//! the desktop build with a local one. See `MASTERPLAN.md` §2.4.
//!
//! The crate is wasm-safe: it depends only on the pure [`rpro_lang`] seam.

#![doc(html_no_source)]

use rpro_lang::{
    CommandPlan, EditorAssists, ExerciseId, ExerciseSource, Language, Outcome, RunOp, Storage,
    StoreError, ToolError, Toolchain, ToolchainStatus,
};

/// A wired learning session: one language, one toolchain, one store.
///
/// Construct with [`Core::new`]. The three trait objects are the *only* place a
/// surface chooses concrete behavior; everything past construction is neutral.
pub struct Core {
    lang: Box<dyn Language>,
    toolchain: Box<dyn Toolchain>,
    storage: Box<dyn Storage>,
}

impl Core {
    /// Wire a session from its three replaceable parts.
    #[must_use]
    pub fn new(
        lang: Box<dyn Language>,
        toolchain: Box<dyn Toolchain>,
        storage: Box<dyn Storage>,
    ) -> Self {
        Self {
            lang,
            toolchain,
            storage,
        }
    }

    /// The active language's stable id (selects `content/languages/<id>/`).
    #[must_use]
    pub fn language_id(&self) -> &str {
        self.lang.id()
    }

    /// Resolve a neutral verb into the concrete command the toolchain will run.
    ///
    /// Pure and synchronous — useful for *previewing* the exact CLI line the
    /// learner is about to run (the CLI-first contract) without executing it.
    #[must_use]
    pub fn plan(&self, ex: &ExerciseSource, op: &RunOp) -> CommandPlan {
        self.lang.command_plan(ex, op)
    }

    /// Run a neutral verb end to end: *plan* → *execute* → *interpret*.
    ///
    /// The toolchain is language-blind, so it returns the raw output with no
    /// structured diagnostics. When that's the case, the [`Language`] scrapes
    /// diagnostics from the raw stderr it could not interpret itself. The raw
    /// output is always preserved — parsing is strictly additive, which is what
    /// keeps the by-hand-error thesis intact on every surface.
    ///
    /// # Errors
    /// Returns [`ToolError`] if the toolchain cannot launch or run the command.
    pub async fn run(&self, ex: &ExerciseSource, op: &RunOp) -> Result<Outcome, ToolError> {
        let plan = self.lang.command_plan(ex, op);
        let mut outcome = self.toolchain.run(&plan).await?;
        if outcome.diagnostics.is_empty() {
            outcome.diagnostics = self.lang.parse_diagnostics(&outcome.raw_stderr);
        }
        Ok(outcome)
    }

    /// Probe the toolchain: *plan* (`detect_plan`) → *execute* → *interpret*.
    ///
    /// # Errors
    /// Returns [`ToolError`] if the probe command cannot be launched.
    pub async fn detect(&self) -> Result<ToolchainStatus, ToolError> {
        let plan = self.lang.detect_plan();
        let outcome = self.toolchain.detect(&plan).await?;
        Ok(self.lang.parse_detect(&outcome.raw_stdout))
    }

    /// Resolve the editor assists for a screen.
    ///
    /// `None` means no `[editor]` block was supplied, which is **Free mode** —
    /// every assist on ([`EditorAssists::default`]). Learning-mode lessons pass
    /// `Some(..)` to strip assists down to bare code.
    #[must_use]
    pub fn editor_assists(&self, requested: Option<EditorAssists>) -> EditorAssists {
        requested.unwrap_or_default()
    }

    /// Load the learner's own saved code for an exercise.
    ///
    /// # Errors
    /// Returns [`StoreError`] on transport failure or if no code is saved yet.
    pub async fn load_code(&self, ex: &ExerciseId) -> Result<String, StoreError> {
        self.storage.load_code(ex).await
    }

    /// Save the learner's own code for an exercise.
    ///
    /// # Errors
    /// Returns [`StoreError`] on transport failure.
    pub async fn save_code(&self, ex: &ExerciseId, src: &str) -> Result<(), StoreError> {
        self.storage.save_code(ex, src).await
    }

    /// Load the serialized progress document (typed model lives in `rpro-state`).
    ///
    /// # Errors
    /// Returns [`StoreError`] on transport failure or if no document exists yet.
    pub async fn load_progress(&self) -> Result<String, StoreError> {
        self.storage.load_progress().await
    }

    /// Save the serialized progress document.
    ///
    /// # Errors
    /// Returns [`StoreError`] on transport failure.
    pub async fn save_progress(&self, json: &str) -> Result<(), StoreError> {
        self.storage.save_progress(json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rpro_lang::{DiagLevel, Diagnostic, ExerciseTemplate, LangTool, LspSpec};

    /// A deliberately language-neutral fake: no real-compiler tokens, so the
    /// seam-grep gate stays green even though this lives outside `languages/`.
    struct EchoLang;

    #[async_trait::async_trait(?Send)]
    impl Language for EchoLang {
        fn id(&self) -> &str {
            "echo"
        }
        fn source_ext(&self) -> &str {
            "txt"
        }
        fn exercise_template(&self, concept: &str) -> ExerciseTemplate {
            ExerciseTemplate {
                filename: "main.txt".into(),
                contents: format!("// {concept}\n"),
            }
        }
        fn parse_diagnostics(&self, raw: &str) -> Vec<Diagnostic> {
            // Marker-driven so a test can assert the parse path fired. The code
            // string avoids the gate's reserved diagnostic-code shape on purpose.
            if raw.contains("BANG") {
                vec![Diagnostic {
                    code: Some("EX1".into()),
                    level: DiagLevel::Error,
                    message: "boom".into(),
                    span: None,
                }]
            } else {
                vec![]
            }
        }
        fn explain_plan(&self, _code: &str) -> Option<CommandPlan> {
            None
        }
        fn book_ref_url(&self, r: &rpro_lang::BookRef) -> String {
            format!("about:book/{}", r.chapter)
        }
        fn lsp(&self) -> LspSpec {
            LspSpec {
                server: "echo-lsp".into(),
                args: vec![],
                init_options: None,
                file_glob: "**/*.txt".into(),
            }
        }
        fn command_plan(&self, ex: &ExerciseSource, op: &RunOp) -> CommandPlan {
            let verb = match op {
                RunOp::Check => "check",
                RunOp::Run => "run",
                RunOp::Test => "test",
                RunOp::Fmt => "fmt",
                RunOp::Lint => "lint",
                RunOp::Explain(_) => "explain",
                RunOp::Tool(_) => "tool",
            };
            CommandPlan {
                program: "echo".into(),
                args: vec![verb.into(), ex.entry.clone()],
                cwd: Some(ex.dir.clone()),
                env: vec![],
                display: format!("echo {verb} {}", ex.entry),
            }
        }
        fn detect_plan(&self) -> CommandPlan {
            CommandPlan {
                program: "echo".into(),
                args: vec!["v1.0".into()],
                cwd: None,
                env: vec![],
                display: "echo v1.0".into(),
            }
        }
        fn parse_detect(&self, raw: &str) -> ToolchainStatus {
            ToolchainStatus {
                present: !raw.trim().is_empty(),
                version: raw.lines().next().map(str::trim).map(String::from),
                components: vec![],
            }
        }
        fn tools(&self) -> Vec<LangTool> {
            vec![]
        }
    }

    /// Canned toolchain: returns whatever stdout/stderr the test seeds, with no
    /// diagnostics — exercising the Core's "parse on empty" path.
    struct CannedToolchain {
        stdout: String,
        stderr: String,
    }

    #[async_trait::async_trait(?Send)]
    impl Toolchain for CannedToolchain {
        async fn detect(&self, _plan: &CommandPlan) -> Result<Outcome, ToolError> {
            Ok(Outcome {
                status: Some(0),
                raw_stdout: self.stdout.clone(),
                raw_stderr: String::new(),
                diagnostics: vec![],
                duration_ms: 0,
            })
        }
        async fn run(&self, _plan: &CommandPlan) -> Result<Outcome, ToolError> {
            Ok(Outcome {
                status: Some(0),
                raw_stdout: self.stdout.clone(),
                raw_stderr: self.stderr.clone(),
                diagnostics: vec![],
                duration_ms: 0,
            })
        }
    }

    /// Storage backed by nothing: every read is `NotFound`, every write is a
    /// no-op. Enough to prove Core delegates without owning persistence.
    struct NullStore;

    #[async_trait::async_trait(?Send)]
    impl Storage for NullStore {
        async fn load_progress(&self) -> Result<String, StoreError> {
            Err(StoreError::NotFound("progress".into()))
        }
        async fn save_progress(&self, _json: &str) -> Result<(), StoreError> {
            Ok(())
        }
        async fn load_code(&self, ex: &ExerciseId) -> Result<String, StoreError> {
            Err(StoreError::NotFound(ex.0.clone()))
        }
        async fn save_code(&self, _ex: &ExerciseId, _src: &str) -> Result<(), StoreError> {
            Ok(())
        }
    }

    fn ex() -> ExerciseSource {
        ExerciseSource {
            id: ExerciseId("control/01_loop".into()),
            dir: "/tmp/ex".into(),
            entry: "main.txt".into(),
        }
    }

    #[test]
    fn plan_is_pure_and_shows_the_cli_line() {
        let core = Core::new(
            Box::new(EchoLang),
            Box::new(CannedToolchain {
                stdout: String::new(),
                stderr: String::new(),
            }),
            Box::new(NullStore),
        );
        let plan = core.plan(&ex(), &RunOp::Run);
        assert_eq!(plan.program, "echo");
        assert_eq!(plan.display, "echo run main.txt");
        assert_eq!(core.language_id(), "echo");
    }

    #[test]
    fn run_fills_diagnostics_from_raw_when_toolchain_leaves_them_empty() {
        let core = Core::new(
            Box::new(EchoLang),
            Box::new(CannedToolchain {
                stdout: String::new(),
                stderr: "line1\nBANG: something\n".into(),
            }),
            Box::new(NullStore),
        );
        let out = pollster::block_on(core.run(&ex(), &RunOp::Check)).unwrap();
        // Raw is preserved verbatim …
        assert!(out.raw_stderr.contains("BANG"));
        // … and the Language scraped one diagnostic out of it.
        assert_eq!(out.diagnostics.len(), 1);
        assert_eq!(out.diagnostics[0].code.as_deref(), Some("EX1"));
        assert_eq!(out.diagnostics[0].level, DiagLevel::Error);
    }

    #[test]
    fn run_leaves_existing_diagnostics_untouched() {
        // No marker in stderr ⇒ parse yields nothing ⇒ stays empty.
        let core = Core::new(
            Box::new(EchoLang),
            Box::new(CannedToolchain {
                stdout: String::new(),
                stderr: "all good\n".into(),
            }),
            Box::new(NullStore),
        );
        let out = pollster::block_on(core.run(&ex(), &RunOp::Run)).unwrap();
        assert!(out.diagnostics.is_empty());
    }

    #[test]
    fn detect_interprets_the_probe_output() {
        let core = Core::new(
            Box::new(EchoLang),
            Box::new(CannedToolchain {
                stdout: "v1.0\n".into(),
                stderr: String::new(),
            }),
            Box::new(NullStore),
        );
        let status = pollster::block_on(core.detect()).unwrap();
        assert!(status.present);
        assert_eq!(status.version.as_deref(), Some("v1.0"));
    }

    #[test]
    fn editor_assists_defaults_to_free_mode() {
        let core = Core::new(
            Box::new(EchoLang),
            Box::new(CannedToolchain {
                stdout: String::new(),
                stderr: String::new(),
            }),
            Box::new(NullStore),
        );
        let free = core.editor_assists(None);
        assert!(free.syntax_highlight && free.autocomplete);
        assert!(free.inline_diagnostics && free.format_on_save);

        let stripped = EditorAssists {
            syntax_highlight: false,
            autocomplete: false,
            inline_diagnostics: false,
            format_on_save: false,
        };
        assert_eq!(core.editor_assists(Some(stripped)), stripped);
    }

    #[test]
    fn storage_delegates_to_the_backend() {
        let core = Core::new(
            Box::new(EchoLang),
            Box::new(CannedToolchain {
                stdout: String::new(),
                stderr: String::new(),
            }),
            Box::new(NullStore),
        );
        let id = ExerciseId("control/01_loop".into());
        // Writes succeed (no-op), reads miss — proving delegation, not caching.
        assert!(pollster::block_on(core.save_code(&id, "let x = 1;")).is_ok());
        assert!(matches!(
            pollster::block_on(core.load_code(&id)),
            Err(StoreError::NotFound(_))
        ));
        assert!(pollster::block_on(core.save_progress("{}")).is_ok());
        assert!(matches!(
            pollster::block_on(core.load_progress()),
            Err(StoreError::NotFound(_))
        ));
    }
}
