# Security review — `rpro-serve` (the local web surface)

A focused review of the only network-facing component, `crates/rpro-serve`. Every
control below is cited by file + symbol so the claim can be re-verified against the
code; findings carry an honest severity tied to the threat model.

> Last reviewed on branch `textbook-integration`; Controls table covers the
> `/api/book` chapter lookup (chapter = map key, never a path), its `?q=` search
> (in-memory substring match, no path/regex/shell), `/api/select` (id validated
> against the discovered set), and the read-only `/api/glossary` term lookup. The
> no-leak contract now also covers `expected_runtime_panic` (the runtime-outcome
> exercise model). Re-run when the run path, the wire protocol, the endpoint set,
> or the served frontend changes.

## 1. Scope & threat model

`rpro-serve` is a **single-user, loopback-only** tool: it binds `127.0.0.1`
exclusively (`main.rs`, `run` — `addr = (Ipv4Addr::LOCALHOST, port)`), never
`0.0.0.0`, and serves one local learner their own exercises.

The critical consequence: **there is no privilege boundary to cross.** The person
driving the browser is the same person who owns the machine and could run any
command directly. So the relevant goals are *not* "stop a remote attacker" (none
can reach the port) but:

- **Integrity of the learning contract** — the page must never leak the answer
  (predict-first), and the server must never run anything but the four learning
  ops on the learner's own current exercise.
- **Defence-in-depth / robustness** — the server should degrade safely on
  malformed or oversized input, and shouldn't be coaxed into running an arbitrary
  command, reading an arbitrary file, or executing injected script.

Out of scope: multi-tenant isolation, authn/authz (single user, no accounts),
transport encryption (loopback only), secret management (none are handled).

## 2. Controls (verified)

| Surface | Control | Where |
|---|---|---|
| Network | Binds loopback only; `PORT` clamped to ≥1024 | `main` / `run` |
| Op surface | `WireOp` is a **closed** enum `{run,check,test,explain}` (`#[serde(tag="op")]`) — anything off-list fails to deserialize, so no format/lint/arbitrary-tool run can be requested from the wire | `WireOp`, test `wireop_whitelist_rejects_unknown_and_parses_known` |
| Explain input | `sanitize_code`: trims, rejects empty / >16 chars / non-`[A-Za-z0-9]` — no path separators, dots, or whitespace reach the toolchain | `sanitize_code`, test `sanitize_code_accepts_alnum_rejects_junk` |
| Edited source | Clamped to `MAX_SOURCE_BYTES` (256 KiB) → `413` before it ever reaches the runner | `run_handler` |
| Request body | **Explicit `DefaultBodyLimit` of 1 MiB** at the transport layer (defence-in-depth below the 256 KiB source clamp; tightens axum's 2 MiB default) | router in `main`, `MAX_BODY_BYTES` |
| Run target | Resolved from on-disk **current** exercise, never from client input — there is no wire path to a different file (`resolve_current`); the scratch run dir name is `slug(id)`, which maps anything non-`[A-Za-z0-9_-]` to `_` | `resolve_current`, `rpro_runner::slug` |
| Command exec | `Command::new(program).args(&args)` — args passed as a **vector, no shell**, so no shell-injection; `program`/`args` come from the `Language` layer, never the wire | `rpro-toolchain-local` `LocalProcess::exec` |
| Answer leak | `current_json` / `exercises_handler` are **field allowlists** that omit `solution_outline`, `expected_error_code`, **and `expected_runtime_panic`** (the runtime-outcome model's server-side panic string) — no answer field reaches the page; the hint ladder gates the outline to the top rung only; `/api/review` surfaces only codes the learner already saw in their own output | `current_json`, `ExerciseMetadata::hint`, test `current_json_omits_the_answer` (asserts all three fields absent), smoke `/api/review` |
| Static files | `ServeDir` (tower-http) serves `gui/` with built-in path-traversal protection; the roadmap reads a **fixed, compile-time** path (`CARGO_MANIFEST_DIR/../../docs/ROADMAP.md`), no client input | router, `roadmap_handler` |
| Book lookup | `GET /api/book?chapter=ID` resolves `ID` as a **`BTreeMap` key** (`Book::get`), **never** path-joined — a traversal value (`../../etc/passwd`, percent-encoded) simply misses the map → `{chapter:null}`, never a file read. The book root is the server-seeded `book/` dir | `book_handler`, tests `book_traversal_is_a_miss_not_a_file_read` (3 URIs), smoke book-traversal probes |
| Book search | `GET /api/book?q=TERM` runs a **case-insensitive substring match over in-memory chapter markdown** (`Book::search`) — no filesystem path is built from `q`, no regex (no ReDoS), no shell. Output (chapter id, title, count, snippet) is derived **only from bundled chapter content**, never echoed user input, and the web client `esc`-es every field before display. A blank term returns no hits | `book_handler` (`?q=` branch), `rpro_book::Book::search`, tests `book_search_returns_hits_with_counts_and_snippets`, `book_search_blank_term_returns_no_hits`, smoke `/api/book?q=` |
| Select target | `POST /api/select` validates `id` against the **discovered** exercise set (`rpro_runner::discover`) before use — an unknown id is `400` (and is only ever a map key/lookup, never a path); a *completed* exercise is refused `409` (no gauge regression). Mutates only on-disk progress, never a file path from the wire | `select_handler`, tests `select_switches_current_and_validates_id`, `select_refuses_a_completed_exercise_with_409`, smoke select |
| Glossary lookup | `GET /api/glossary` is **read-only**: with no query it returns all bundled terms (`Glossary::all`); with `?term=T` it resolves `T` by alias-aware **map lookup** (`Glossary::get`), **never** path-joined — a traversal value simply misses → no term. Output is bundled glossary content only (server-seeded `glossary/`), never echoed wire input; no answer field is involved | `glossary_handler`, `rpro_glossary::Glossary::get` |
| HTTP headers | CSP, `X-Content-Type-Options: nosniff`, `Referrer-Policy: no-referrer`, `X-Frame-Options: DENY` on every response | `security_headers` |
| Memory safety | `unsafe_code = "forbid"` workspace-wide; **0** `unsafe` in `rpro-serve` | `Cargo.toml` |

## 3. Findings

### F1 — Run execution timeout *(Low; availability — resolved, default-on)*
Previously `LocalProcess::exec` called `cmd.output()` with no deadline, so a
runaway exercise (`fn main(){ loop{} }`) submitted via `/api/run` hung the
`spawn_blocking` worker until the server was killed. (Not a security issue under
the threat model — no privilege boundary, self-DoS only — but a robustness gap;
it also froze the web UI, since the never-resolving fetch left the buttons
disabled — flagged by the educational-fidelity audit.)
- **Resolved (default-on):** the executor now applies a **safe 30s cap on every
  surface by default**, so a learner freely experimenting can never hang the
  CLI/TUI or freeze the web UI — the run is killed and a result still returns.
  `RPRO_RUN_TIMEOUT_SECS` overrides: a positive value sets the limit; **`0` opts
  OUT** (uncapped, for operators who want it). The capped path spawns with piped
  output, **drains stdout/stderr on separate threads** (so a child that floods a
  pipe can't deadlock the poller — covered by a `yes`-flood test), kills the
  child past the deadline, and appends a `[rpro: run exceeded the Ns timeout…]`
  note to the raw output the learner reads.

### F2 — CSP allows `'unsafe-inline'` for script + style *(Low; informational)*
The whole GUI is a single self-contained `index.html` with an inline `<script>`
and inline styles (FOSS-first, no CDN, no build step), so the CSP must permit
`'unsafe-inline'`. Residual XSS risk is **low and mitigated**: all dynamic
content is escaped before it enters the DOM (`esc()` on every interpolation;
`mdToHtml` escapes first, then re-introduces only a fixed tag whitelist). There is
no user-generated content from another origin.
- **Recommendation:** if a build step is ever added, switch to nonce- or
  hash-based CSP and drop `'unsafe-inline'`. Until then, the escaping is the
  control — keep it; never `innerHTML` un-escaped server data.

### F3 — Unbounded request body *(Resolved this review)*
Previously the request body was bounded only by axum's 2 MiB default, with the
256 KiB source clamp applied post-parse. Added an explicit 1 MiB
`DefaultBodyLimit` so oversized bodies are rejected at the transport layer before
parsing. ✅

## 4. Dependency audit

- **Pinned.** The exact dependency versions are pinned in `Cargo.lock`, and the
  runtime makes **no outbound network calls** other than binding the loopback
  listener (the only network exposure is the loopback port itself).
- **`cargo audit` — run, clean.** A RustSec advisory scan (`cargo audit`,
  v0.22) reports **0 vulnerabilities** across the **236** `Cargo.lock` dependencies
  (checked against 1138 advisories), confirming no known-vulnerable dependency in
  the tree as reviewed.
- **Surface.** Network/runtime deps are `axum` / `hyper` / `tower-http` / `tokio`
  (widely used and audited) plus `serde`/`serde_json` for the wire. No crypto,
  no auth, no secret material is handled, so there is no key-management surface.
- **Memory safety.** `unsafe_code = "forbid"` across the workspace removes the
  `unsafe`-based class of dependency-triggered UB from first-party code.
- **`cargo deny` — wired into CI (2026-06-25).** `deny.toml` configures the four
  axes (advisories, licenses, bans, sources); the FOSS-first license allow-list is
  derived from `cargo metadata` over the real dependency tree (MIT/Apache-2.0 plus
  the few AND-clauses that pull in Unicode-3.0 / BSD-3-Clause / Zlib, and one
  file-level-copyleft MPL-2.0 dep). The **`cargo-deny`** job in `ci.yml` runs
  `cargo deny check advisories licenses bans sources` on every push via the
  maintained `EmbarkStudios/cargo-deny-action` (a pinned binary, no compile);
  `scripts/check.sh` runs it locally when the tool is installed. Together with the
  always-on `cargo audit` this scans dependency changes automatically.

## 5. Conclusion

For the stated threat model — loopback, single-user, no privilege boundary —
`rpro-serve`'s posture is **sound**: a closed op-whitelist, no shell execution,
server-resolved targets, validated + clamped input (now bounded at the transport
layer too), a held predict-first no-leak contract, secure-by-default headers, and
zero `unsafe`. The two residual items (F1 run timeout, F2 nonce-CSP) are
low-severity hardening/robustness improvements, are documented here, and are
tracked in the internal backlog rather than left implicit.
