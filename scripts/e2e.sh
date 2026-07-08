#!/usr/bin/env bash
# Run the Playwright browser e2e suite against an ISOLATED rpro-serve — a
# throwaway store on its own port — never the live learner server.
#
# WHY THIS EXISTS: the suite used to be pointed at the live :8099 instance.
# Several specs MUTATE state (recall-chip passes an exercise for real; web.spec
# force-selects), so every suite run advanced the live store's progress — the
# learner's "done" count inflated one exercise per run — and the specs poisoned
# each other across runs (a Done exercise refuses force-select with 409, so
# web.spec's pin silently landed on whatever was current — a runtime-panic
# exercise with no E-code — and its Diagnostics assertion "flaked"). A fresh
# store per run makes the suite deterministic AND keeps hands off real progress.
set -euo pipefail
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

# Skip gracefully where the browser stack isn't installed (e.g. minimal CI).
if ! command -v npx >/dev/null 2>&1 || [ ! -d tests/e2e/node_modules ]; then
  echo "e2e: SKIP (npx or tests/e2e/node_modules missing)"; exit 0
fi

PORT="${TS_E2E_PORT:-8098}"
BASE="http://127.0.0.1:${PORT}"
# Exec-capable base (some systems mount /tmp noexec; runs execute real binaries).
E2E_BASE="${TS_E2E_DIR:-${HOME:-/tmp}/.cache}"
mkdir -p "$E2E_BASE" 2>/dev/null || E2E_BASE="${TMPDIR:-/tmp}"
STATE="$(mktemp -d "$E2E_BASE/ts-e2e.XXXXXX")"
SRV_PID=""
cleanup() { [ -n "$SRV_PID" ] && kill "$SRV_PID" 2>/dev/null || true; rm -rf "$STATE"; }
trap cleanup EXIT

echo "== browser e2e (isolated store: $STATE, port $PORT) =="
cargo build -p rpro-serve --offline 2>/dev/null || cargo build -p rpro-serve
# Resolve the binary the way cargo does: CARGO_TARGET_DIR when set, ./target
# otherwise. Hardcoding ./target once launched a TWO-WEEK-STALE fossil server
# (local builds go to the cache) — tests then failed against endpoints that
# didn't exist yet, masquerading as flakes.
BIN="${CARGO_TARGET_DIR:-$ROOT_DIR/target}/debug/rpro-serve"
PORT="$PORT" TS_SERVE_ROOT="$STATE" "$BIN" >"$STATE/server.log" 2>&1 &
SRV_PID=$!
curl -s --retry-connrefused --retry 30 --retry-delay 1 -o /dev/null "$BASE/" \
  || { echo "e2e: server never came up"; cat "$STATE/server.log"; exit 1; }

cd tests/e2e
# The 'compile' specs (dev-diag-jump/tutor/recall-chip/web) mutate ONE shared
# store; run in parallel they race — a passing run marks whatever's CURRENTLY
# selected Done, so recall-chip's pass can mark another spec's just-selected
# exercise Done → a 409 flake. Fix: on a FULL run, run the 'ui' project parallel
# (fast, no such races) but the 'compile' project SERIALLY (--workers=1), so no
# two store-writing specs overlap. A targeted run (a named spec) keeps the single
# fast pass — one spec alone never races with itself.
if [ "$#" -eq 0 ]; then
  RPRO_BASE="$BASE" npx playwright test --project=ui;        ui_rc=$?
  RPRO_BASE="$BASE" npx playwright test --project=compile --workers=1; co_rc=$?
  exit $(( ui_rc + co_rc ))
else
  RPRO_BASE="$BASE" npx playwright test "$@"
fi
