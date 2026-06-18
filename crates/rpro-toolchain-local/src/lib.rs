//! `rpro-toolchain-local` — runs [`CommandPlan`]s on the local machine via
//! `std::process`. Serves desktop, Android/Termux, and the backend's sandbox.
//!
//! Language-neutral: it never inspects *what* the command is (that lives behind
//! the `Language` seam), it just executes the plan and returns the raw output.
//! Not wasm-safe (uses `std::process`); the web surface uses a remote executor.

use std::process::Command;
use std::time::Instant;

use rpro_lang::{CommandPlan, Outcome, ToolError, Toolchain};

/// Executes commands locally through `std::process`.
#[derive(Debug, Clone, Copy, Default)]
pub struct LocalProcess;

impl LocalProcess {
    /// Run one plan and capture its raw output.
    ///
    /// `diagnostics` is left empty — turning raw compiler output into structured
    /// [`rpro_lang::Diagnostic`]s is the `Language` layer's job, which keeps this
    /// executor dumb and reusable across every language.
    fn exec(plan: &CommandPlan) -> Result<Outcome, ToolError> {
        let mut cmd = Command::new(&plan.program);
        cmd.args(&plan.args);
        if let Some(cwd) = &plan.cwd {
            cmd.current_dir(cwd);
        }
        for (k, v) in &plan.env {
            cmd.env(k, v);
        }
        let started = Instant::now();
        let output = cmd.output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                ToolError::NotFound(plan.program.clone())
            } else {
                ToolError::Spawn(e.to_string())
            }
        })?;
        Ok(Outcome {
            status: output.status.code(),
            raw_stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            raw_stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            diagnostics: Vec::new(),
            duration_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        })
    }
}

// Blocking `std::process` inside the async method is fine for the single-threaded
// CLI/TUI today; once a runtime is wired in (action 5) this moves to
// `spawn_blocking` so it never stalls an executor.
#[async_trait::async_trait(?Send)]
impl Toolchain for LocalProcess {
    async fn detect(&self, plan: &CommandPlan) -> Result<Outcome, ToolError> {
        Self::exec(plan)
    }

    async fn run(&self, plan: &CommandPlan) -> Result<Outcome, ToolError> {
        Self::exec(plan)
    }
}

#[cfg(test)]
mod tests {
    use super::LocalProcess;
    use rpro_lang::{CommandPlan, ToolError};

    #[test]
    fn runs_a_local_command_and_captures_output() {
        let plan = CommandPlan {
            program: "echo".into(),
            args: vec!["hello-rpro".into()],
            cwd: None,
            env: vec![],
            display: "echo hello-rpro".into(),
        };
        let out = LocalProcess::exec(&plan).expect("echo should run");
        assert_eq!(out.status, Some(0));
        assert!(out.raw_stdout.contains("hello-rpro"));
    }

    #[test]
    fn missing_program_is_not_found() {
        let plan = CommandPlan {
            program: "definitely-not-a-real-binary-xyz".into(),
            args: vec![],
            cwd: None,
            env: vec![],
            display: String::new(),
        };
        assert!(matches!(LocalProcess::exec(&plan), Err(ToolError::NotFound(_))));
    }
}
