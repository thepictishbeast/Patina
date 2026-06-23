//! `rpro-toolchain-local` — runs [`CommandPlan`]s on the local machine via
//! `std::process`. Serves desktop, Android/Termux, and the backend's sandbox.
//!
//! Language-neutral: it never inspects *what* the command is (that lives behind
//! the `Language` seam), it just executes the plan and returns the raw output.
//! Not wasm-safe (uses `std::process`); the web surface uses a remote executor.

use std::io::Read;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use rpro_lang::{CommandPlan, Outcome, ToolError, Toolchain};

/// Env knob to override the per-run cap (seconds): `0` opts OUT (uncapped); a
/// positive value sets it; unset = a safe **30s default on every surface**, so a
/// learner's runaway exercise (`loop{}`) is always killed with a message instead
/// of hanging the CLI/TUI or freezing the web UI (the learner keeps Ctrl-C on the
/// CLI for an earlier stop). See `docs/SECURITY.md` F1.
const RUN_TIMEOUT_ENV: &str = "RPRO_RUN_TIMEOUT_SECS";

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
        Self::exec_inner(plan, run_timeout_from_env())
    }

    /// The real executor, with the timeout passed explicitly so it's testable
    /// without touching process-global env. `timeout = None` is the original
    /// `Command::output()` path, byte-for-byte; `Some(dur)` spawns with piped
    /// output, **drains stdout/stderr on separate threads** (so a child that
    /// fills a pipe buffer can't deadlock against a non-reading parent), and
    /// kills the child if it outlives the deadline.
    fn exec_inner(plan: &CommandPlan, timeout: Option<Duration>) -> Result<Outcome, ToolError> {
        let mut cmd = Command::new(&plan.program);
        cmd.args(&plan.args);
        if let Some(cwd) = &plan.cwd {
            cmd.current_dir(cwd);
        }
        for (k, v) in &plan.env {
            cmd.env(k, v);
        }
        let started = Instant::now();

        let Some(limit) = timeout else {
            // No cap: the simple, well-trodden path. Unchanged behaviour.
            let output = cmd.output().map_err(|e| map_spawn_err(e, plan))?;
            return Ok(outcome(
                output.status.code(),
                String::from_utf8_lossy(&output.stdout).into_owned(),
                String::from_utf8_lossy(&output.stderr).into_owned(),
                started,
            ));
        };

        cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
        let mut child = cmd.spawn().map_err(|e| map_spawn_err(e, plan))?;
        // Drain both pipes concurrently so the child never blocks on a full
        // buffer while we poll for exit (the classic timeout+capture deadlock).
        let mut out = child.stdout.take();
        let mut err = child.stderr.take();
        let out_h = std::thread::spawn(move || drain(out.as_mut()));
        let err_h = std::thread::spawn(move || drain(err.as_mut()));

        let mut timed_out = false;
        let status = loop {
            match child
                .try_wait()
                .map_err(|e| ToolError::Spawn(e.to_string()))?
            {
                Some(s) => break s,
                None => {
                    if started.elapsed() >= limit {
                        let _ = child.kill();
                        let s = child.wait().map_err(|e| ToolError::Spawn(e.to_string()))?;
                        timed_out = true;
                        break s;
                    }
                    std::thread::sleep(Duration::from_millis(20));
                }
            }
        };

        let stdout = out_h.join().unwrap_or_default();
        let mut stderr = err_h.join().unwrap_or_default();
        if timed_out {
            // Surface *why* it stopped in the raw bytes the learner reads.
            stderr.push_str(&format!(
                "\n[rpro: run exceeded the {}s timeout and was stopped]\n",
                limit.as_secs()
            ));
        }
        Ok(outcome(status.code(), stdout, stderr, started))
    }
}

/// The per-run wall-clock cap. `RPRO_RUN_TIMEOUT_SECS` overrides: a positive
/// integer sets the limit; `0` opts out (uncapped — for operators who want it).
/// When unset, a safe default applies so a learner freely experimenting (the
/// classic `fn main() { loop {} }` or a blocking read) can never hang the server
/// or freeze the UI — the run is killed and a result still returns.
fn run_timeout_from_env() -> Option<Duration> {
    /// Generous enough for a debug build + run of a learner-sized program.
    const DEFAULT_SECS: u64 = 30;
    match std::env::var(RUN_TIMEOUT_ENV)
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
    {
        Some(0) => None, // explicit operator opt-out
        Some(n) => Some(Duration::from_secs(n)),
        None => Some(Duration::from_secs(DEFAULT_SECS)), // safe default
    }
}

/// Read a child pipe to EOF, lossily as UTF-8. `None` (pipe absent) → empty.
fn drain(pipe: Option<&mut impl Read>) -> String {
    let mut buf = Vec::new();
    if let Some(p) = pipe {
        let _ = p.read_to_end(&mut buf);
    }
    String::from_utf8_lossy(&buf).into_owned()
}

/// Build an [`Outcome`] (diagnostics stay empty — the `Language` layer fills them).
fn outcome(
    status: Option<i32>,
    raw_stdout: String,
    raw_stderr: String,
    started: Instant,
) -> Outcome {
    Outcome {
        status,
        raw_stdout,
        raw_stderr,
        diagnostics: Vec::new(),
        duration_ms: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    }
}

/// Map a spawn I/O error to the right [`ToolError`].
fn map_spawn_err(e: std::io::Error, plan: &CommandPlan) -> ToolError {
    if e.kind() == std::io::ErrorKind::NotFound {
        ToolError::NotFound(plan.program.clone())
    } else {
        ToolError::Spawn(e.to_string())
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
    use std::time::{Duration, Instant};

    fn plan(program: &str, args: &[&str]) -> CommandPlan {
        CommandPlan {
            program: program.into(),
            args: args.iter().map(|s| (*s).to_string()).collect(),
            cwd: None,
            env: vec![],
            display: String::new(),
        }
    }

    #[test]
    fn runs_a_local_command_and_captures_output() {
        let out = LocalProcess::exec(&plan("echo", &["hello-rpro"])).expect("echo should run");
        assert_eq!(out.status, Some(0));
        assert!(out.raw_stdout.contains("hello-rpro"));
    }

    #[test]
    fn missing_program_is_not_found() {
        let p = plan("definitely-not-a-real-binary-xyz", &[]);
        assert!(matches!(
            LocalProcess::exec(&p),
            Err(ToolError::NotFound(_))
        ));
    }

    #[test]
    fn timeout_path_still_captures_a_fast_command() {
        // A generous cap takes the piped+threaded path; output must round-trip.
        let out =
            LocalProcess::exec_inner(&plan("echo", &["hi-timed"]), Some(Duration::from_secs(10)))
                .expect("echo should run");
        assert_eq!(out.status, Some(0));
        assert!(out.raw_stdout.contains("hi-timed"));
    }

    #[test]
    fn timeout_kills_a_runaway_command() {
        let started = Instant::now();
        let out =
            LocalProcess::exec_inner(&plan("sleep", &["10"]), Some(Duration::from_millis(200)))
                .expect("spawn should succeed");
        // Killed well before its 10s — not waited out.
        assert!(
            started.elapsed() < Duration::from_secs(3),
            "should be killed promptly"
        );
        assert!(
            out.status.is_none(),
            "a signal-killed child has no exit code"
        );
        assert!(
            out.raw_stderr.contains("timeout"),
            "the stop reason is surfaced: {}",
            out.raw_stderr
        );
    }

    #[test]
    fn timeout_does_not_deadlock_on_heavy_output() {
        // `yes` floods stdout forever; if the parent polled try_wait WITHOUT
        // draining the pipe, the child would block on a full buffer and this test
        // would hang. The reader threads must keep it flowing until the kill.
        let started = Instant::now();
        let out = LocalProcess::exec_inner(&plan("yes", &[]), Some(Duration::from_millis(300)))
            .expect("spawn should succeed");
        assert!(
            started.elapsed() < Duration::from_secs(3),
            "must not deadlock on full pipe"
        );
        assert!(out.raw_stderr.contains("timeout"));
        assert!(
            !out.raw_stdout.is_empty(),
            "drained at least some flooded output"
        );
    }
}
