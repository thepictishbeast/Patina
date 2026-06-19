# Running Tempered Studio

Three ways to use it: a local **web** server, the **CLI**, or the **TUI**
dashboard. All run fully offline against your local Rust toolchain — there is no
telemetry and no CDN. Every mode shows you the *real* compiler output.

## 0. Install

You need a Rust toolchain, **1.85 or newer** (edition 2024).

- Recommended: install via [rustup](https://rustup.rs):
  `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
  then `rustup default stable` and make sure `cargo --version` reports ≥ 1.85.
- A distro-provided `rustc`/`cargo` ≥ 1.85 also works.

Build everything once (uses only the local crate cache; add `--offline` if your
network is restricted):

```
cargo build --workspace
```

## 1. WEB — run exercises in a browser

Start the local server (loopback only — it never listens on the network):

```
CARGO_TERM_COLOR=always cargo run -p rpro-serve
```

Then open <http://127.0.0.1:8787/> in any browser.

- `CARGO_TERM_COLOR=always` makes the compiler emit ANSI color; the page renders
  it in a real xterm.js terminal (vendored, no CDN). It is set on the launch
  command because the server inherits the environment to run the toolchain.
- The page shows the **current exercise**. Press **Run**, **Check**, or
  **Explain** — each posts to the server, which compiles the exercise locally and
  streams the real `rustc` output (errors, warnings, diagnostics) back into the
  terminal pane, colors and all. Read the error, edit, re-run.
- Override the port: `PORT=9000 cargo run -p rpro-serve` (must be ≥ 1024).
- Override the writable state/scratch root (exercises + run dir):
  `TS_SERVE_ROOT=/path/to/dir cargo run -p rpro-serve`. Default is
  `~/.cache/ts-serve`. On first start the server seeds this root from the
  workspace `exercises/` and marks the first exercise current, so a fresh run
  lands on a real, runnable exercise.
- Cap each run's wall-clock time (stops a runaway `loop{}` from hanging a
  worker): `RPRO_RUN_TIMEOUT_SECS=60 cargo run -p rpro-serve`. Unset / `0` = no
  cap (the default; the CLI/TUI are unaffected). Use a generous value so a cold
  compile isn't killed. See `docs/SECURITY.md` F1.

Stop the server with Ctrl-C.

## 2. CLI

The binary is `rpro` (crate `rpro-cli`). Run subcommands with:

```
cargo run -p rpro-cli -- <command>
```

Common commands:

- `init` — one-time setup (exercises + book + config)
- `exercise next` — jump to the next unfinished exercise (sets it current)
- `exercise list` — every exercise with status
- `run` / `check` / `test` — compile-and-run / type-check / test the current
  exercise (or pass an exercise id)
- `explain E0382` — full explanation of a diagnostic code

Two-line example:

```
cargo run -p rpro-cli -- exercise next
cargo run -p rpro-cli -- run
```

The second command compiles the current exercise and prints the real compiler
output (e.g. `error[E0382]: borrow of moved value`). Fix the source, run again.

## 3. TUI — the dashboard

Run `rpro` with no subcommand to open the full-screen terminal dashboard
(exercises, the embedded Rust Book reader, progress, and a live run pane):

```
cargo run -p rpro-cli
```

(equivalently `cargo run -p rpro-cli --bin rpro`). Navigate with the tab keys;
PgUp/PgDn scroll the book and exercise panes; press Run/Check inside the
exercise view to see live compiler output. Quit with `q`.

---

**Notes**
- First run in any mode compiles a small throwaway crate per exercise, so the
  very first Run/Check can take a few seconds; subsequent runs are fast.
- The web server is loopback-only (`127.0.0.1`) and accepts only a closed set of
  operations (run / check / test / explain) on the server-resolved current
  exercise — it cannot be pointed at arbitrary files or commands.

## Smoke test (end-to-end)

`scripts/smoke.sh [PORT]` builds the server, seeds a throwaway store, starts it on
loopback, and asserts the whole contract: static assets serve, the exercises seed
in learning order with a current one, `/api/current` never leaks the
answer (no `solution_outline` / `expected_error_code`), the hint ladder's first
rung doesn't reveal the solution, and a real op runs through. Exits nonzero on any
failure — suitable for CI. Needs only the Rust toolchain + `python3` + `curl`.
