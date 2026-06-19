# Architecture

## State on disk: `~/.rustlings-pro/`

```
~/.rustlings-pro/
├── exercises/                    rustlings + Rustlings-Pro exercise sources
│   ├── 00-intro/
│   │   ├── 01_welcome.rs
│   │   └── 01_welcome.toml       (exercise metadata — see below)
│   └── …
├── book/                         Rust Book markdown content
│   ├── ch01-00-getting-started.md
│   ├── ch04-01-what-is-ownership.md
│   └── …
├── progress.json                 per-exercise state (locked / current / done + timestamps)
├── bookmarks.json                book bookmarks ({ chapter, anchor, label, ts })
├── annotations.json              book annotations ({ chapter, range, body, ts })
└── config.toml                   user prefs (theme, editor command, default keybindings)
```

Every file is JSON or TOML; nothing in a database. Plain text on disk means a user can `git init ~/.rustlings-pro` and version-control their progress and notes.

## Exercise metadata

Each exercise has a sibling `.toml` file with typed metadata. Example:

```toml
# exercises/04-ownership/01_move.toml
id = "ownership/01_move"
title = "Move semantics: when assignment transfers ownership"
difficulty = "beginner"
estimated_minutes = 8

# What concept this exercise is testing.
concept = "move-semantics"

# Book references — shown on `rpro hint`. Each ref has:
#   chapter:  Rust Book chapter ID (matches the file under ~/.rustlings-pro/book/)
#   anchor:   optional in-page anchor
#   why:      one sentence — why THIS section helps with THIS exercise
[[book_refs]]
chapter = "ch04-01-what-is-ownership"
anchor = "ownership-rules"
why = "The three ownership rules — read these before you try to fix the second `let` in this exercise."

[[book_refs]]
chapter = "ch04-01-what-is-ownership"
anchor = "ways-variables-and-data-interact-move"
why = "The diagrams here show what's happening to s1 and s2 in the failing line."

# Optional: which compiler error message this exercise is designed
# around. The runner cross-checks against this so a confusing OTHER
# error gets a "this isn't the error we expected — you may have
# changed too much" warning.
expected_error_code = "E0382"

# Hidden by default. Shown only on `rpro hint --solution`.
solution_outline = "Use `s1.clone()` to keep both variables valid, or restructure to pass ownership instead of borrowing."
```

The runner pairs each `.rs` file with its `.toml` metadata at exercise-discovery time. A file without a `.toml` sibling is rejected (`rpro init` checks this and refuses to install a malformed exercise set).

## Book references in the TUI

When `rpro hint` fires, the runner:

1. Loads the current exercise's `book_refs` list.
2. Renders each one as a card: chapter title (resolved from the book's `SUMMARY.md`), the `why` line, a press-to-jump key.
3. The user presses `1` / `2` / `3` to jump to that book reference inside the TUI book reader. The reader scrolls to the anchor, highlights the section, and shows a "Back to exercise" footer.

This works for both rustlings-derived exercises (where the metadata is generated at import time from `info.toml` + a hand-curated mapping) and Rustlings-Pro original exercises (where the metadata is hand-written alongside the `.rs` file).

## Book content

Source: <https://github.com/rust-lang/book> (MIT/Apache-2.0). The project is
**offline-first** — there is no install-time network fetch. `rpro init` seeds the
*bundled* exercise set from the workspace and sets the first one current; the web
server auto-seeds on first start. Embedding the referenced Book chapters as local
markdown (so the Book tab renders real content rather than placeholders) is
tracked but not yet done — see Content in `docs/BACKLOG.md`.

## Workspace crates — a language-agnostic seam

The defining choice: a **neutral seam** describes a run without naming a toolchain,
so the engine and every surface stay language-agnostic and the core stays pure
(it compiles to `wasm32`). Effects (a real process, the filesystem) live only at
the edges. Dependency direction flows one way — surfaces → engine → seam — and
two CI gates enforce it (`seam-gates.yml`): the pure crates must build for
`wasm32-unknown-unknown`, and no toolchain/error-code literals may leak outside
`crates/languages/`.

| Layer | Crate | Responsibility |
|---|---|---|
| **Seam** (pure) | `rpro-lang` | Neutral types — `RunOp`, `Outcome`, `Diagnostic`, `CommandPlan`, the `Toolchain`/`Language` traits. Names no toolchain. |
| **Language** | `languages/rust` (`rpro-lang-rust`) | The Rust adapter — scaffolds a project, builds the command plan, parses raw output into `Diagnostic`s. The *only* place `cargo`/`rustc`/`E0xxx` appear. |
| **Engine** (pure) | `rpro-core` | Drives a run over the seam (`Core::run`); `!Send`, so surfaces build it inside a worker and only plain data crosses `.await`. |
| **State** (pure) | `rpro-state` | On-disk model — `Progress` (incl. skip/reset), `ReviewState` (clock-free Leitner spaced-repetition), `ExerciseMetadata::hint` (the shared hint ladder), `tutor` (guide-not-solve scaffolding), bookmarks/annotations/config. |
| **Content** | `rpro-book` | Book chapter loader + markdown render. |
| **Session** | `rpro-runner` | Exercise discovery (`discover`) + `record_run` — the one helper every surface shares to fold a finished run into progress (attempt + spaced-rep + advance). |
| **Effects** | `rpro-storage-fs` | The `Store` — read/write state on disk. |
| **Effects** | `rpro-toolchain-local` | Runs `CommandPlan`s via `std::process` (with an optional run timeout). |
| **Surfaces** | `rpro-cli` / `rpro-tui` / `rpro-serve` | The `rpro` binary, the ratatui dashboard, and the loopback web server — each a thin front-end over the same engine + state. |

Because state and the hint/review/run-record logic are shared (pure) crates, the
three surfaces can't drift: a learner's progress, hints, and Recall queue are
identical whether they use the web, CLI, or TUI.

## Web GUI (`rpro-serve`) — shipped

The GUI is a **local web app**, not a native toolkit binding: `rpro-serve` is a
loopback-only [axum](https://github.com/tokio-rs/axum) server that is *another front-end over `Core`* (it builds the
`!Send` engine inside a `spawn_blocking` worker, exactly like the TUI's run
thread, so only plain data crosses the async boundary). It serves a single-file
`gui/index.html` with vendored xterm.js — no CDN, no build step, no JS framework.
This one frontend is also what a future desktop wrapper (e.g. Tauri) or an
on-device mobile browser would load, so it is not throwaway. Security model:
binds `127.0.0.1` only, a closed op-whitelist, validated + size-clamped input, and
it never sends the page the answer (see `docs/SECURITY.md`).

## Android / Termux

[Termux](https://termux.dev/en/) is a Linux-like userland on Android. The `rpro`
binary built for `aarch64` runs there without modification — same CLI, same TUI.
The web GUI helps here too: `rpro-serve` on loopback can be opened in the device's
browser, so the same frontend reaches mobile without a native toolkit. A native
Android app (no Termux) would need a bundled Rust toolchain, which Android does
not ship — a deferred problem, and likely a separate mobile repo (see
`docs/DISTRIBUTION.md`).

## Test strategy

94 unit/integration tests + an end-to-end smoke, all green; the full tally lives in
[docs/AUDIT.md](docs/AUDIT.md). In brief:

- `rpro-state` — round-trips + the pure logic (review/`fold_run`, hint ladder, tutor scaffolding, progress skip/reset).
- `rpro-runner` — discovery + `record_run` against a tempfile `Store`.
- `rpro-tui` — screen render assertions via ratatui's `TestBackend` (terminal output is text).
- `rpro-serve` — the network contract via `tower::ServiceExt::oneshot` against the extracted `build_router` (no-leak, op-whitelist, body limit, headers).
- `rpro-toolchain-local` — process exec incl. the run-timeout + a deadlock-avoidance test.
- `scripts/smoke.sh` — boots the server, seeds a throwaway store, asserts the API + serving contract end-to-end (also a CI job).
- Gates: `seam-grep` (no toolchain literals outside `crates/languages/`) + `wasm32` pure-core build.
