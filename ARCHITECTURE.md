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

Source: <https://github.com/rust-lang/book> (MIT/Apache-2.0). At `rpro init`, we either:

- **Vendor at build time** — embed the markdown via `include_str!` so the binary is self-contained. Pros: no network at install. Cons: binary size, freshness.
- **Fetch at runtime** — download a frozen snapshot during `rpro init`. Pros: small binary, easier updates. Cons: needs network on first run.

v0 picks runtime fetch (smaller binary, network is acceptable on first install). A future `--offline` flag will switch to vendored content for air-gapped installs.

## Workspace crates

| Crate | Responsibility | Public API |
|---|---|---|
| `rpro-state` | Disk state — load + save progress, bookmarks, annotations, config | `Progress`, `Bookmarks`, `Annotations`, `Config` types + `Store::load` / `save` |
| `rpro-runner` | Exercise discovery + `cargo run` + result parsing | `discover()`, `run(exercise)`, `Outcome` enum |
| `rpro-book` | Book content loader + markdown → ratatui spans | `Book::load`, `Chapter::render(area)`, `Search::query` |
| `rpro-tui` | ratatui screens + keybindings (book reader, exercise screen, dashboard) | `Tui::run(&mut store, &mut book, &mut runner)` |
| `rpro-cli` | Top-level `rpro` binary — clap subcommands, dispatch | `main()` |

## Iced GUI (future)

`rpro-gui` will be a peer crate that consumes the same `rpro-state`, `rpro-runner`, `rpro-book` APIs. It uses [PlausiDen-Loom][loom]'s typed design tokens via the [`thundercrab-theme` bridge][bridge] (which is the prior-art for Loom→Iced projection).

The CLI and GUI write to the same `~/.rustlings-pro/` directory; a watcher hook in either makes them update in real time, so a learner can have the GUI open while running `rpro exercise run` from a terminal and see the progress flip.

[loom]: https://github.com/thepictishbeast/PlausiDen-Loom
[bridge]: https://github.com/thepictishbeast/thundercrab/tree/loom-iced-bridge-pilot/thundercrab-theme

## Android / Termux

[Termux](https://termux.dev/en/) is a Linux-like userland on Android. The `rpro` binary built for `aarch64-linux-musl` runs there without modification — same install script, same TUI. The GUI on Android is harder (Iced + Android = experimental); v0 says "use the TUI on Android, GUI is desktop-first."

A native Android app (no Termux) would mean running the runner against a bundled Rust toolchain, which Android does not ship. That's a deferred problem — not a v0 target.

## Test strategy

- `rpro-state`: round-trip tests on every type (load → save → load equals original).
- `rpro-runner`: integration test that runs a known-good exercise and asserts `Outcome::Pass`, plus a known-bad exercise and asserts `Outcome::Fail` with the expected error code.
- `rpro-book`: parser test that the bundled chapters all render without panicking.
- `rpro-tui`: snapshot tests of each screen via [insta](https://crates.io/crates/insta) — terminal output is text, easy to snapshot.
- `rpro-cli`: end-to-end tests in a temp `HOME=…` so they don't touch the user's real `~/.rustlings-pro/`.
