# UI / UX & Visual System

Two tiers, kept honest:

- **Tier A — ACTIONABLE NOW:** the ratatui TUI (`rpro-tui`) is the only UI
  surface that exists today. The enrichment plan below targets the real crate
  and ships incrementally.
- **Tier B — DESIGN-AHEAD (pending a GUI surface):** the full visual system
  (thumbnails, motion, palette, component shapes) has **no GUI to live in yet**
  — it is designed now so it drops straight into the Android-first GUI (and then
  desktop/web) when that surface exists. Do not read Tier B as "shipped."

Both tiers share one status/icon vocabulary (next section). **ASCII text tokens
are canonical** (load-bearing for alignment on phone terminals); glyphs/emoji are
decorative and dropped under `ascii_only`/`NO_COLOR`; color is always a third
channel, never the only one.

---

## Shared status & icon legend

# Tempered Studio — Shared Status & Icon Legend

> One vocabulary for **both** the TUI (today) and the future GUI. **ASCII text token is canonical** (load-bearing for alignment on phone terminals); the glyph is the TUI display rune; the emoji is decorative and dropped under `ascii_only`/`NO_COLOR`; the GUI icon is the design-ahead shape. Color is always a *third* channel, never the only one.

## Exercise status (`rpro_state::ExerciseStatus`)
| State | ASCII | TUI glyph | Emoji | GUI icon | Color token | In progress %? |
|---|---|---|---|---|---|---|
| Locked | `[ ]` | `🔒`/`-` | 🔒 | closed padlock | `status.locked` (dim) | counts as not-done |
| Current | `[>]` | `▸` | ▶️ | filled triangle | `status.current` (bold) | not-done (in progress) |
| Done | `[x]` | `✓` | ✅ | check-in-circle | `status.done` | **counts toward fill** |
| Skipped | `[~]` | `»` | ⏭️ | double-chevron / dashed ring | `status.skipped` | **not-done** (excluded from fill) |

Overall completion (Gauge / ring): `done_count / total`. **Skipped never fills the bar.**

## Diagnostics (`rpro_lang::DiagLevel`)
| Level | ASCII | TUI glyph | GUI icon | Color token |
|---|---|---|---|---|
| Error | `[!]` | `✗` | filled circle + `!` | `diag.error` |
| Warning | `[w]` | `▲` | triangle + `!` | `diag.warning` |
| Note | `[i]` | `·` | circle + `i` | `diag.note` |

## Run state (verdict from `rpro_lang::Outcome`)
| State | ASCII | TUI glyph | Notes |
|---|---|---|---|
| Running | `...` | braille `⠋⠙⠹…` (ASCII `\|/-\` fallback) | frame-stepped on event-loop tick |
| Passed | `OK` | `✓` | + `duration_ms` (e.g. `✓ 412ms`) |
| Failed | `FAIL` | `✗` | raw `stderr` shown verbatim |

## Mode badge (`rpro_lang::EditorAssists`)
| Mode | ASCII | Glyph | Meaning |
|---|---|---|---|
| Free | `[FREE]` | `⠿` | all assists on (default) |
| Learning | `[LEARN]` | `◐` | one or more assists off |

## UI / action icons
| Action | ASCII key | TUI glyph | Emoji | GUI icon |
|---|---|---|---|---|
| Run | `r` | `▷` | ▶️ | play |
| Check | `c` | `✓?` | — | check-outline |
| Hint / Book | `h`/`b` | `📖`/`B` | 📖 | open book |
| Book ref jump | `1` `2` `3` | numbered | — | numbered chip |
| Edit (`Config.editor`) | `e` | `✎` | ✏️ | pencil |
| Bookmark | `m` | `★` | 🔖 | ribbon |
| Highlight | `H` | `▍` | 🖍️ | marker |
| Roadmap/Tasks | `t` | checklist | 🗒️ | checklist |
| Help overlay | `?` | `?` | ❓ | question |
| Quit | `q` | — | — | — |

## Roadmap / verification (`docs/ROADMAP.md` + AVP-2)
| Item | ASCII | Glyph | Meaning |
|---|---|---|---|
| Task open | `[ ]` | `▢` | not started |
| Task in progress | `[>]` | `▸` | active (reuses Current) |
| Task done | `[x]` | `✓` | complete (reuses Done) |
| Verification | `N/36` | `LineGauge` | AVP-2 passes; `<36` ⇒ **STILL BROKEN** (text canonical), `36/36` ⇒ `candidate` |

> Fallback rule: when `ascii_only` or `NO_COLOR` is set, use **only** the ASCII column above — glyphs and emoji are dropped, weight/`BOLD`/`DIM` + the text token carry all meaning.

---

# Tier A — TUI enrichment (actionable)

# Tempered Studio — `rpro-tui` Enrichment Plan (ACTIONABLE)

Targets the real crate `crates/rpro-tui` (today a v0 stub: `run_dashboard`, `run_book_reader`). Built on **ratatui 0.30.0** + **crossterm 0.29** (`default-features=false, features=["crossterm"]`, per workspace `Cargo.toml`). Data comes from the existing types: `rpro_state::{ExerciseStatus, Progress, ProgressEntry, Config}`, `rpro_lang::{Outcome, Diagnostic, DiagLevel, EditorAssists, BookRef}`.

> **Status vocabulary, glyphs, and emoji are defined once in `legend_markdown` and used identically here. ASCII text is the load-bearing form; emoji are decorative.**

---

## 0. Hard constraints baked into every screen

1. **Raw output is the primary surface.** `Outcome::raw_stderr` is shown verbatim ("what the learner reads"); `Outcome::diagnostics` are *strictly additive* (a sidebar/overlay, never a replacement). This is the by-hand-error / CLI-first thesis from the architecture — the exercise view must not prettify stderr away.
2. **Never color-only.** Every status carries an **ASCII text token + a glyph**; color is the third channel. Many emoji are double-width and render inconsistently on Termux/Android, so the ASCII token is canonical and load-bearing for alignment.
3. **Phone-terminal reflow.** Inspect `area.width` each frame; below a `NARROW` threshold (~50 cols, with a hard floor at ~40) swap the `Layout` from side-by-side `Constraint::Percentage` columns to a single stacked column with `Constraint::Length`/`Min`. Truncate titles with an ellipsis rune rather than wrapping the chrome.
4. **Free/Learning mode is visible.** Render an `EditorAssists` badge in the exercise view (all assists on = **Free**; any off = **Learning**). It is a status line element, not buried in config.
5. **`Skipped` counts as not-done** in any percentage (`progress.rs`): Gauge ratio = `done_count / total`; Skipped gets its own glyph but never fills the bar.

---

## 1. Color theme (`theme.rs`)

A single `Theme` struct of `ratatui::style::Color`s, selected from `Config.theme` (`"dark"`/`"light"`, auto-detected, override-able). Use named ANSI indices (not 24-bit) as the **base** layer so 16-color and light/dark terminals stay legible; offer an opt-in truecolor ramp.

| Role | Dark | Light | Used by |
|---|---|---|---|
| `bg` / `fg` | Reset / `Gray` | Reset / `Black` | all `Block`/`Paragraph` |
| `accent` (brand) | `Rgb(247,76,0)` Ferris-orange | `Rgb(183,65,14)` | titles, selected tab, focus border |
| `done` | `Green` | `Green` | done glyph, gauge fill |
| `current` | `Cyan` | `Blue` | current glyph, cursor row |
| `locked` | `DarkGray` | `Gray` | locked rows (dim) |
| `skipped` | `Yellow` | `Rgb(150,120,0)` | skipped glyph |
| `error` | `Red` | `Red` | `DiagLevel::Error` |
| `warn` | `Yellow` | `Yellow` | `DiagLevel::Warning` |
| `note` | `Blue` | `Blue` | `DiagLevel::Note` |
| `muted` | `DarkGray` | `Gray` | keybinding hints, footers |

Accessibility: never pair `done`/`error` on hue alone — the glyph (`[✓]` vs `[x]`) disambiguates. Honor `NO_COLOR` env → fall back to a monochrome theme where weight/`Modifier::BOLD`/`DIM` and glyphs carry all meaning.

---

## 2. Status icons (drives `ExerciseStatus`)

One `fn status_cell(s: ExerciseStatus) -> Span` returning glyph + style; the ASCII token sits beside it in the List label. (Full table lives in `legend_markdown`.)

| `ExerciseStatus` | ASCII token (canonical) | Glyph | Emoji (decorative) | Style |
|---|---|---|---|---|
| `Locked` | `[ ]` | `🔒`→`-` fallback | 🔒 | `locked`, `DIM` |
| `Current` | `[>]` | `▸` | ▶️ | `current`, `BOLD` |
| `Done` | `[x]` | `✓` | ✅ | `done` |
| `Skipped` | `[~]` | `»` | ⏭️ | `skipped` |

`config.toml` gets an `ascii_only = true` switch (auto-on when `TERM`/locale isn't UTF-8) that drops the emoji column entirely and uses only `[ ] [>] [x] [~]`.

---

## 3. Progress bars + spinner/throbber

- **Overall completion:** `ratatui::widgets::Gauge` — `.ratio(done as f64 / total as f64)`, `.label(format!("{done}/{total} · {pct}%"))`, fill `done` color. Skipped excluded from the ratio.
- **Compact / per-section progress:** `LineGauge` (single-row, ideal in the narrow stacked layout and in each section header of the exercise list).
- **Attempt heat / streak:** `Sparkline` fed from `ProgressEntry.attempts` per recent exercise, or a 7-day completion strip from `completed_at` — turns the `attempts` field into a "you've been grinding this one" signal.
- **Long lists:** `Scrollbar` + `ScrollbarState` on the exercise List and book reader.
- **Spinner / throbber (pattern, not a widget):** ratatui has no animation primitive. Implement a `Throbber` helper:
  - Frame array, advanced by a `tick: u64` counter: `frames[(tick / 2) as usize % frames.len()]`.
  - **Braille set** `["⠋","⠙","⠹","⠸","⠼","⠴","⠦","⠧","⠇","⠏"]`; **ASCII fallback** `["|","/","-","\\"]` when `ascii_only`.
  - Drawn as a `Span` inside the exercise view's status line *while a `cargo run`/check is in flight* (the `rpro-runner` future is unresolved), e.g. `⠹ running cargo run …`. On completion swap to `Outcome::status` verdict + `Outcome::duration_ms` (`✓ passed in 412ms` / `✗ failed`).
  - **The event loop is what animates it:** `crossterm::event::poll(Duration::from_millis(80))` in the main loop; on timeout, increment `tick` and redraw. No timeout = redraw only on input (battery-friendly on Termux when idle).

---

## 4. Dashboard (entry screen — implements `run_dashboard`)

`Tabs` widget across the top (`Dashboard │ Exercise │ Book │ Roadmap`), each a screen module. Dashboard body:

```
┌ Tempered Studio ──────────────── Free mode ⠿ ┐
│ Progress  [██████████░░░░░░░░] 12/40 · 30%    │   ← Gauge
│ Streak    ▁▂▄▆█▅▃  (Sparkline of recent days) │
│                                               │
│ ▸ Current: ownership/01_move                  │   ← from Progress (Current)
│   "Move semantics: when assignment transfers" │
│   tried 3× · started 8m ago                   │   ← ProgressEntry.attempts/started_at
│                                               │
│ Book refs for this exercise:                  │   ← BookRef list
│  [1] ch04-01 · The three ownership rules      │
│  [2] ch04-01 · Move diagrams (s1 → s2)        │
│                                               │
│ Up next (locked until current passes):        │
│  [ ] ownership/02_clone                       │
└ r run · h hint · b book · t tasks · q quit ───┘
```

- Header right shows the **Free/Learning** badge + the throbber when a run is active.
- Section rows use `List` + `ListState` (selectable); status cell + ASCII token per row.
- `started_at`/`completed_at` rendered as humanized relative time.

---

## 5. Exercise view (the core screen)

Two regions on a wide terminal; **stacked** on narrow. **Raw stderr is the big region.**

```
┌ ownership/01_move ────── Learning mode (◐ assists) ──┐
│ ┌ Raw output (cargo) ─────────────┐ ┌ Diagnostics ──┐│
│ │ error[E0382]: borrow of moved   │ │ ✗ E0382  L7   ││  ← additive sidebar
│ │   value: `s1`                   │ │   borrow of   ││    from Outcome
│ │  --> src/main.rs:7:20           │ │   moved value ││    .diagnostics
│ │   |                             │ │ ⚠ unused var  ││    (color+glyph by
│ │ 5 |  let s2 = s1;               │ │   `s2`  L5    ││     DiagLevel)
│ │   |           -- value moved    │ └───────────────┘│
│ │ … (raw_stderr verbatim) …       │ ⠹ running…  412ms │
│ └─────────────────────────────────┘                  │
│ Book refs: [1] ch04-01 ownership-rules               │
│ [2] ch04-01 move-diagrams   why: shows s1→s2          │
└ r run · c check · e edit · 1/2/3 jump · ? keys ──────┘
```

- **Left/main:** `Paragraph` with `Wrap { trim:false }` rendering `Outcome::raw_stderr` (then `raw_stdout`) verbatim, vertical `Scrollbar`. This is always present, never collapsed.
- **Right/overlay:** `List` of `Outcome::diagnostics`; each item styled by `DiagLevel` (`error`/`warn`/`note`), shows `code` + `span` (`L{line}:{col}`) + truncated `message`. Strictly additive; hideable with a key, but raw is not.
- **Mode badge** from `EditorAssists`: `Free` (all true) vs `Learning (◐ assists)`; a sub-line can spell which are off (`syntax · ⨯complete · ⨯inline`).
- **Status line:** throbber while running → verdict + `duration_ms` on `Outcome`. `expected_error_code` mismatch (from exercise `.toml`) surfaces a `note`-colored banner: "this isn't the error we expected — you may have changed too much."
- **`e`** shells out to `Config.editor` (suspend the terminal: leave raw mode + `LeaveAlternateScreen`, spawn, restore on return).
- **`1`/`2`/`3`** jump into the book reader at the matching `BookRef` (matches ARCHITECTURE's contract).

---

## 6. Book reader (implements `run_book_reader`)

Two-pane on wide, stacked on narrow:

- **Left:** chapter `List` (`ListState`) from the book's `SUMMARY.md`; current chapter highlighted with `accent`.
- **Right:** `Paragraph` of rendered markdown spans (rpro-book renders markdown → ratatui spans) with `Wrap`, a `Scrollbar`, and the scroll offset tracked in screen state.
- **Anchor jump + highlight:** when entered via a `BookRef` anchor, scroll the offset to the heading and paint that section's span run with a reversed/`accent` background for ~1s of ticks, then settle. Footer shows **"← Back to exercise"** (per ARCHITECTURE).
- **Bookmarks / highlights / annotations** (the `~/.rustlings-pro/*.json` files): `m` bookmark, `h` highlight selection, `a` annotate. A gutter glyph (`★` bookmark, `▍` highlight) marks annotated lines.
- **Search** (`rpro book search`): a one-line `Paragraph` input at the bottom; results as a `List`; `n`/`N` to cycle matches.

---

## 7. Roadmap / Tasks view (NEW — `t` tab)

**Data source (named, not hand-waved):** today `docs/` contains only `DISTRIBUTION.md`. Add a tracked **`docs/ROADMAP.md`** (or a structured `docs/tasks.toml`) that the TUI reads at startup; the view renders it so both the learner and Paul see what's in flight. Render strategy:

- If markdown: parse top-level `## Section` → tab/section headers; `- [ ]` / `- [x]` task lines → a `List` with the same `[ ]`/`[x]` ASCII status tokens reused from §2; prose → `Paragraph`.
- If `tasks.toml`: typed `{ title, status, owner, notes }` rows → `List` with status glyphs + an owner column.

Surface the repo's **AVP-2 doctrine** as a first-class badge (README/AVP-2: default verdict "STILL BROKEN", ≥36 verification passes before a `SHIP-DECISION:`):

```
┌ Roadmap / Tasks ──────────── source: docs/ROADMAP.md ┐
│ Now                                                   │
│  [>] rpro-tui: dashboard + exercise view   ▶ in prog  │
│  [ ] rpro-runner: cargo run + Outcome parse           │
│ Next                                                  │
│  [ ] book reader anchor-jump + highlight              │
│ Verification (AVP-2)                                  │
│  rpro-tui   [████░░░░░░] 14/36 passes · STILL BROKEN  │  ← LineGauge per crate
│  rpro-state [██████████] 36/36 · ✓ candidate          │
└ ↑↓ move · enter open · t back · q quit ──────────────┘
```

Per-crate verification uses `LineGauge` (`passes/36`); the `STILL BROKEN` / `candidate` text label is canonical (color is secondary). Editing tasks stays out of the TUI in v0 — it reads the file (which is git-tracked, matching the "version-control your `~/.rustlings-pro/`" ethos).

---

## 8. Keybindings + hints

- **Global:** `Tab`/`Shift-Tab` cycle the `Tabs`; `q` quit; `?` toggle a help overlay (`Paragraph` in a centered `Block`, dim background). Reconciled with README's planned commands and ARCHITECTURE's `1/2/3` + Back-to-exercise.
- **Persistent footer:** every screen ends in a `muted` `Block` title or bottom `Paragraph` listing the 4–6 most relevant binds (see mockups). On narrow width, collapse to the top 3 + `?`.
- **Discoverability:** `?` help overlay groups binds by screen and shows that `e` uses `Config.editor` and `1/2/3` jump to the current exercise's `BookRef`s.

---

## 9. Implementation order (incremental, API-stable)

1. `theme.rs` + `status.rs` (glyph/token/style helpers, `ascii_only` + `NO_COLOR`).
2. App skeleton: `App` state, crossterm event loop with `poll(80ms)` tick + `Tabs` routing; reflow helper (`is_narrow(area)`).
3. Dashboard (`Gauge`/`LineGauge`/`Sparkline` + `List`).
4. Exercise view (raw `Paragraph` + diagnostics `List` + throbber + mode badge).
5. Book reader (`List` + `Paragraph` + `Scrollbar` + anchor highlight).
6. Roadmap view (read `docs/ROADMAP.md`).
7. Help overlay + footers.

Each lands behind the existing `run_dashboard` / `run_book_reader` signatures, so the CLI dispatch never reshuffles. Snapshot-test each screen with `insta` (per ARCHITECTURE test strategy) at both wide and ~40-col widths.

**ratatui 0.30 widgets cited:** `Gauge`, `LineGauge`, `Sparkline`, `List` + `ListState`, `Tabs`, `Paragraph` + `Wrap`, `Block`, `Scrollbar` + `ScrollbarState`, `Layout`/`Constraint`. Throbber = a frame-array Span pattern advanced on the poll-timeout tick (no built-in motion widget).

---

# Tier B — Visual system (DESIGN-AHEAD, pending a GUI surface)

> Nothing in this section ships today; it targets the future Android/desktop/web GUIs. Language-neutral where possible; Rust-specific visuals are flagged so a future Go/Kotlin/C++ build can swap them.

# Tempered Studio — Visual System (DESIGN-AHEAD)

> **PENDING A GUI SURFACE.** No GUI exists today; the only shipping UI is the ratatui TUI. Per ARCHITECTURE, the future `rpro-gui` is an **Iced** peer crate consuming the same `rpro-state`/`rpro-lang` APIs and rendering via **PlausiDen-Loom typed design tokens through the `thundercrab-theme` Loom→Iced bridge**. Everything below is a **token specification and a set of mockups to design against** — not a shipped spec. Nothing here ships now.

**Language-neutral by default; Rust-specific visuals are flagged 🦀 so a future Go/Kotlin/C++ skin can swap them.** The core (status states, progress, layout, motion) is language-agnostic; only the "skin" (mascot, error-code styling, concept diagrams) changes.

---

## 1. Design principles (carried from the product)

- **CLI-first stays visible.** Even in a GUI, raw compiler output (`Outcome::raw_stderr`) is the hero element; formatted diagnostics are an *additive* affordance layered on top. The GUI must not become a magic "fix it" button — that would break the by-hand-error thesis.
- **One progress model.** Same `ExerciseStatus` (Locked/Current/Done/Skipped) and same `done/total` math (Skipped excluded) as the TUI; visuals differ, semantics don't.
- **Parity with disk.** GUI and TUI both read `~/.rustlings-pro/`; a watcher flips state live, so visual state must be derivable purely from those files.

---

## 2. Color palette — as Loom-projectable tokens

Express as **semantic tokens** (not raw hex) so they project cleanly through Loom→Iced. Each token names a role; the bridge resolves it per theme. The TUI consumes the same names mapped to ANSI/truecolor.

| Token | Role | Dark value | Light value |
|---|---|---|---|
| `color.brand.primary` | brand / accent | `#F74C00` 🦀 Ferris-orange | `#B7410E` rust-oxide |
| `color.brand.secondary` | links, focus | `#3B82F6` | `#1D4ED8` |
| `color.surface.base` | app background | `#0E1116` | `#FBFBFD` |
| `color.surface.raised` | cards/panels | `#161B22` | `#FFFFFF` |
| `color.text.primary` | body | `#E6EDF3` | `#1A1A1A` |
| `color.text.muted` | hints, meta | `#8B949E` | `#6B7280` |
| `status.done` | Done | `#3FB950` | `#1A7F37` |
| `status.current` | Current | `#58A6FF` | `#0969DA` |
| `status.locked` | Locked | `#484F58` | `#AFB8C1` |
| `status.skipped` | Skipped | `#D29922` | `#9A6700` |
| `diag.error` | Error | `#F85149` | `#CF222E` |
| `diag.warning` | Warning | `#D29922` | `#9A6700` |
| `diag.note` | Note | `#58A6FF` | `#0969DA` |

> 🦀 **Flag:** only `color.brand.primary`'s *Ferris-orange* identity is Rust-specific. A Go skin would set `color.brand.primary = #00ADD8` (Gopher-cyan); Kotlin `#7F52FF`; C++ `#00599C`. The token name stays; the value swaps. All other tokens are language-neutral.

Contrast: every text/surface pair targets WCAG AA (≥4.5:1 body, ≥3:1 large/UI). Status meaning is never color-only — paired with an icon (§3) and a text label, mirroring the TUI's ASCII-token rule.

---

## 3. Icon set (concept thumbnails + status + UI)

Two-tier: **stroke icons** (UI chrome, 24px grid, 1.5px stroke, rounded joins) and **filled status pips**. Each status icon is shape-distinct so it reads without color (colorblind-safe + grayscale-print-safe):

| Concept | Icon | Notes |
|---|---|---|
| Locked | padlock, closed | shape ≠ color |
| Current | filled right-triangle ▸ | the only triangle in the set |
| Done | check inside a circle | the only check |
| Skipped | double-chevron » / dashed ring | distinct from Done |
| Error | filled circle + `!` | |
| Warning | triangle + `!` | |
| Note | circle + `i` | |
| Run | ▷ play | |
| Hint / Book | open book | |
| Bookmark | ribbon ★ | |
| Roadmap/Tasks | checklist | |
| Editor | pencil | maps to `Config.editor` |

**Per-lesson concept thumbnails** — one small illustrative glyph per concept, shown on dashboard cards and the lesson list, language-neutral metaphors:

| Concept | Thumbnail metaphor (neutral) | 🦀 Rust skin overlay |
|---|---|---|
| Variables / binding | a labeled box | — |
| Ownership | a key being handed between two boxes | "move" arrow s1 → s2 🦀 |
| Borrowing | a box with an outgoing dotted arrow (lent, not given) | `&` shared / `&mut` exclusive 🦀 |
| Lifetimes | two overlapping timelines | `'a` tick labels 🦀 |
| Error handling | a fork in a path (ok / err) | `Result<T,E>` 🦀 |
| Concurrency | two parallel lanes | — |
| Traits/Generics | a socket accepting multiple plug shapes | `impl Trait` 🦀 |

> 🦀 The base metaphor is shared; the overlay/label is the swappable skin. A Kotlin version replaces the ownership "key handoff" with a nullable/`?`-safety metaphor, etc.

**Mascot 🦀:** Ferris appears in empty states and the lesson-complete celebration. This is the most Rust-specific asset — a sibling repo's Go skin would substitute the Gopher; the *placement and animation* are reusable, the *sprite* is not.

---

## 4. Motion / animation guidelines

GUI-only (the TUI has no real motion; its throbber is frame-stepped text). Respect `prefers-reduced-motion` → all of the below degrade to instant state changes or a single cross-fade.

- **Progress fills:** Gauge/ring animates from old→new ratio with an ease-out cubic, ~350ms. When an exercise flips to `Done`, the overall ring tween + the row's status pip morphs Locked/Current → Done with a 200ms scale-pop.
- **Lesson-complete celebration:** on `set_done`, a brief (~900ms, skippable) moment: the row's check draws on (stroke-dasharray reveal), a soft confetti or a 🦀 Ferris bounce, and the streak Sparkline ticks up. Tasteful, one-shot, never blocks input. 🦀 Ferris is the Rust skin; neutral fallback is a checkmark burst.
- **Screen transitions:** tab/screen changes = 180ms horizontal slide + fade matching tab order; book anchor-jump = smooth scroll + a 1s highlight-then-settle on the target section (parity with the TUI's tick highlight).
- **Throbber → verdict:** while running, an indeterminate ring/braille spinner; on `Outcome`, cross-fade to ✓/✗ + `duration_ms`. Same state machine as the TUI.
- **Diagnostics reveal:** additive diagnostics slide in as a side panel; raw output never animates away (it's the hero, always present).

All motion specified as **Loom motion tokens** (`motion.duration.fast=180ms`, `motion.duration.celebrate=900ms`, `motion.easing.standard=ease-out-cubic`) so the bridge projects them and reduced-motion is a single token flip.

---

## 5. Component mockups (ASCII, GUI intent)

**Dashboard (wide GUI):**
```
┌───────────────────────────────────────────────────────────┐
│ 🦀 Tempered Studio        [Dashboard] Exercise Book Tasks   │
├───────────────────────────────────────────────────────────┤
│  Your progress                                  Free mode   │
│  ◜◝  12 / 40 done  (30%)        Streak ▁▂▄▆█▅▃  6 days       │
│  ◟◞   ↑ animated ring                                       │
│                                                             │
│  ▸ Current                                                  │
│  ┌───────────────────────────────────────────────────────┐ │
│  │ [key-handoff thumb] ownership/01_move                  │ │
│  │ Move semantics: when assignment transfers ownership    │ │
│  │ tried 3× · 8m   [ Open ▷ ]  [ Hint 📖 ]                 │ │
│  └───────────────────────────────────────────────────────┘ │
│  Up next  🔒 ownership/02_clone   🔒 ownership/03_borrow    │
└───────────────────────────────────────────────────────────┘
```

**Exercise view (raw output is the hero):**
```
┌ ownership/01_move ───────────────── Learning ◐ ────────────┐
│ ┌ cargo run (raw) ───────────────────────┐ ┌ Diagnostics ┐ │
│ │ error[E0382]: borrow of moved value `s1`│ │ ● E0382  L7 │ │  🦀 E-code
│ │  --> src/main.rs:7:20                    │ │   moved val │ │   styling
│ │ 5 | let s2 = s1;                         │ │ ▲ unused s2 │ │   flagged
│ │   |          -- value moved here         │ │   L5        │ │
│ │ … verbatim raw_stderr …                  │ └─────────────┘ │
│ └──────────────────────────────────────────┘  ◌ running…    │
│ Book refs: [1] ch04-01 ownership-rules  [2] move-diagrams   │
│ [ Run ▷ ]  [ Check ]  [ Edit ✎ ]   ← Back                   │
└─────────────────────────────────────────────────────────────┘
```
*The right "Diagnostics" panel is additive and collapsible; the left raw pane is permanent.*

**Mobile / narrow GUI (single column, mirrors TUI reflow):**
```
┌──────────────────────┐
│ 🦀 Tempered  ☰        │
│ ◜◝ 12/40 (30%)        │
│ ▸ ownership/01_move   │
│ [ Open ▷ ] [ Hint 📖 ]│
│ ── raw output ──      │
│ error[E0382]: borrow… │
│ [ Run ▷ ] [ Edit ✎ ]  │
└──────────────────────┘
```

**Lesson-complete celebration (one-shot overlay):**
```
        ✦   ✓   ✦
      🦀  Nice!  🦀
   ownership/01_move done
     +1 · streak 7 days
   [ Next exercise → ]
```

---

## 6. Dark / light + theming notes

- Ship **dark default** (matches `Config.theme = "dark"`), full light theme, and "follow system." All three resolve through the same token set — no duplicated component styles.
- The 🦀 brand orange is tuned per theme (brighter on dark, oxide on light) for contrast; status hues shift value, not hue, between themes so the icon-shape language stays constant.
- Because GUI + TUI share token *names*, a future change to `status.done` propagates to both surfaces; the TUI just resolves the token to its nearest ANSI color.

---

## 7. Language-neutral vs Rust-specific summary

| Layer | Neutral (reusable across language skins) | 🦀 Rust-specific (swap per language) |
|---|---|---|
| Status states & pips | ✅ Locked/Current/Done/Skipped shapes | — |
| Progress / streak / motion tokens | ✅ | — |
| Layout, reflow, component structure | ✅ | — |
| Diagnostic severity icons | ✅ Error/Warning/Note | error-*code* styling (`E0382`) 🦀 |
| Concept thumbnails | ✅ base metaphors | concept overlays (ownership/borrow/lifetime) 🦀 |
| Brand color value | token name ✅ | Ferris-orange value 🦀 |
| Mascot / celebration sprite | placement & animation ✅ | Ferris sprite 🦀 |

A Go/Kotlin/C++ edition keeps every neutral row verbatim and swaps only the flagged 🦀 cells — which is exactly what the language-seam architecture (`rpro-lang` neutral core + per-language crate) already enforces in code.