# Tempered Studio v0.4 — DRAFT release notes

> **STATUS: DRAFT — do not publish.** Maintained by the improvement loop so the
> v0.4 cut is a one-step job whenever Paul calls it. Everything below is already
> merged on `textbook-integration` (web/CLI/TUI) and `main` (mobile), verified by
> the full CI mirror (baseline `f7978f2`, 12/12 gates). The batch policy is
> Paul's: no releases between v0.3.14 and one great v0.4.

## The headline: the screen is yours now

v0.4 is the **IDE-first release**. v0.3 showed everything at once — three tabs
plus a ⋯ More menu, auto-opening panels, a long scroll. v0.4 keeps the main
screen to **the task and the editor**, and puts everything else one tap away:

- **Two tabs.** 📝 **Practice** — the exercise, the editor, Run. 📚 **Learn** —
  a hub for *all* learning materials with your progress at the top: lessons,
  quizzes, cheatsheets, the Rust Book, the 9-book PDF library, the glossary,
  the Study Guide, and Your Rust Journey.
- **The Insights drawer stays closed** until a run gives it something worth
  showing (or you pin it with ▦). No more permanent long scroll.
- **A slimmer task header** — the predict-first coaching is one line now; the
  editor and Run are on the first screen, even on a phone.

## Progress you can actually see

- **Stage-grouped lessons** — the 37 lessons group into the 11 curriculum
  stages, each with a read count (`FOUNDATIONS 1/8`, green ✓ when complete).
- **Per-phase exercise counts** — the exercise list's topic headers show
  `done/total` the same way. So does `rpro progress` in the terminal.
- **A visible progress line on phones** — a full-width bar under the app bar
  (it used to be an invisible 15px sliver).
- The **Journey** ties it together: per-stage reads, quiz ✓s, cheatsheet and
  book links, in one map.

## Fully offline, now actually fully

- **Mobile serves everything on-device**: lessons, quizzes, cheatsheets, the
  61-term glossary (Copy, Clone and dereference joined it), and — new since
  v0.3.14 — the **hint ladder** (same gate as desktop: no hint until you've genuinely
  tried, one rung per attempt, never the solution) and **Explain** — 
  `rustc --explain` runs on-device through the Termux bridge.
- **Bundled color emoji** (Noto Color Emoji, OFL, ~2MB of unicode-range-gated
  subsets): no more tofu boxes on systems without an emoji font, still zero
  network requests.
- App updates now **refresh bundled content** (they used to pin your first
  install's lessons forever), and the APK finally reports its real version —
  Obtainium tracks correctly from v0.3.14 onward.

## The same curriculum on every surface

Web, Android, CLI, and now the **TUI** all carry the full path. The TUI gained
a **Quizzes tab with the predict-then-reveal rule enforced**: answers stay
hidden until you press `a`, and re-hide when you move on.

## Light theme, for real

The light theme was never audited — it shipped 43 contrast failures and forgot
your choice on every reload. v0.4: WCAG-AA-clean status palette in both themes
(now permanently gated in CI — every view is axe-audited in dark *and* light),
and the 🌓 toggle persists.

## Under the hood (quality you'll feel, not see)

- The complete learner loop is verified END-TO-END on a fresh store:
  predict-gate → run → verdict → hint rung 1 (earned, never given) → Assist
  re-render → red gutter mark matching the L# jump → edit clears marks →
  fix → pass → advance → the error joins the RECALL queue. Every seam held.

- Browser e2e runs on an **isolated throwaway store** — the suite can never
  touch a real learner's progress again — and is deterministic (43/43 twice
  consecutively, ~13s).
- A **stale-render race** is fixed: fast navigation can no longer flip a view
  back to a late-arriving previous one.
- New CI guards: every exercise concept must resolve to its lesson (the "read
  the lesson" link can't silently die) and every quiz must carry the Answers
  gate.
- The Study Guide teaches the *current* navigation.

## Numbers

37 lessons · 71 exercises · 11 quizzes · 11 cheatsheets · 61 glossary terms ·
33 Rust Book chapters · 9 original books (PDF) · 4 surfaces · 100% offline.

---

### Cut checklist (for the release tick — do not run until Paul says go)

1. `bash scripts/check.sh` — full mirror green.
2. Mobile: bump `versionCode`/`versionName` (10400 / "0.4.0") in
   `app/build.gradle.kts` — MUST move together with the tag.
3. `HOME=/home/paul TS_SIBLING=… bash build-apk.sh` (re-syncs all assets),
   verify versionName + content in the APK (`aapt dump badging`, `unzip -l`).
4. `gh release create v0.4.0` with the APK + sha256 — RELEASE url, not an
   Actions artifact.
5. One email to william@plausiden.com via
   `/home/paul/.claude/tempered-email.sh`, verify `status=sent`.
6. Update this file's status line to RELEASED + tick the loop doc.
