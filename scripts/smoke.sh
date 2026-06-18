#!/usr/bin/env bash
# Tempered Studio — end-to-end smoke test for the local web server (rpro-serve).
#
# Builds the server, seeds a throwaway state root, starts it on loopback, and
# asserts the contract end-to-end: static assets serve, the exercise list seeds
# in learning order, the current-exercise payload never leaks the answer, the
# hint ladder's low rungs never leak the solution, and a real op runs through.
#
# Usage:  scripts/smoke.sh [PORT]
# Exits nonzero on the first failed assertion. No external deps beyond the Rust
# toolchain + python3 + curl. Uses curl's own retry (no sleep) to wait for boot.
set -euo pipefail

PORT="${1:-8799}"
ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

STATE="$(mktemp -d "${TMPDIR:-/tmp}/ts-smoke.XXXXXX")"
BASE="http://127.0.0.1:${PORT}"
SRV_PID=""
fails=0

cleanup() { [ -n "$SRV_PID" ] && kill "$SRV_PID" 2>/dev/null || true; rm -rf "$STATE"; }
trap cleanup EXIT

ok()   { echo "  ok   — $1"; }
fail() { echo "  FAIL — $1"; fails=$((fails + 1)); }

echo "== Tempered Studio smoke test (port $PORT) =="

echo "-- build --"
cargo build -p rpro-serve --offline 2>/dev/null || cargo build -p rpro-serve

echo "-- start server (loopback only, seeded fresh) --"
PORT="$PORT" TS_SERVE_ROOT="$STATE" CARGO_TERM_COLOR=always \
  ./target/debug/rpro-serve >"$STATE/server.log" 2>&1 &
SRV_PID=$!
curl -s --retry-connrefused --retry 30 --retry-delay 1 -o /dev/null "$BASE/" \
  || { fail "server never came up"; cat "$STATE/server.log"; exit 1; }

echo "-- assertions --"

# 1. static shell + vendored xterm
[ "$(curl -s -o /dev/null -w '%{http_code}' "$BASE/")" = "200" ] \
  && ok "GET / -> 200" || fail "GET / not 200"
[ "$(curl -s -o /dev/null -w '%{http_code}' "$BASE/vendor/xterm/xterm.js")" = "200" ] \
  && ok "vendored xterm.js served" || fail "xterm.js not served"

# 2. exercise list seeds, in learning order, with a current
curl -s "$BASE/api/exercises" | python3 -c '
import sys, json
d = json.load(sys.stdin)
exs = d.get("exercises", [])
assert d.get("total", 0) >= 4, "too few exercises seeded"
assert exs and exs[0]["status"] == "current", "first exercise not current"
assert exs[0]["id"].startswith("basics/"), "first is " + exs[0]["id"] + ", expected basics/*"
print("  ok   — " + str(d["total"]) + " exercises seeded; first = " + exs[0]["id"] + " (current)")
' || fail "exercise list / order / current"

# 3. current-exercise payload must NOT leak the answer
curl -s "$BASE/api/current" | python3 -c '
import sys, json
d = json.load(sys.stdin)
assert "code" in d and "title" in d, "missing renderable fields"
assert "solution_outline" not in d, "LEAK: solution_outline in /api/current"
assert "expected_error_code" not in d, "LEAK: expected_error_code in /api/current"
print("  ok   — /api/current renders the exercise, no answer leaked")
' || fail "/api/current no-leak"

# 4. hint ladder: level 1 must not contain a solution outline
curl -s "$BASE/api/hint?level=1" | python3 -c '
import sys, json
d = json.load(sys.stdin)
t = (d.get("text") or "").lower()
assert d.get("level") == 1, "level 1 not returned"
assert "solution outline" not in t, "LEAK: level-1 hint contains the solution"
print("  ok   — hint L1 guides without revealing the solution")
' || fail "hint ladder gating"

# 5. a real op runs end-to-end (check compiles the current exercise)
curl -s -X POST "$BASE/api/run" -H 'Content-Type: application/json' -d '{"op":"check"}' \
  | python3 -c '
import sys, json
d = json.load(sys.stdin)
assert "passed" in d and "raw_stderr" in d and "exercise" in d, "run response shape"
print("  ok   — /api/run executed (exercise=" + str(d["exercise"]) + ", passed=" + str(d["passed"]) + ")")
' || fail "/api/run end-to-end"

# 6. spaced-repetition (RECALL) queue is wired + internally consistent. Every
# tracked code is either due (box < mastered) or mastered, so the counts must
# satisfy len(due) + mastered == tracked regardless of seeded content.
curl -s "$BASE/api/review" | python3 -c '
import sys, json
d = json.load(sys.stdin)
due, mastered, tracked = d["due"], d["mastered"], d["tracked"]
assert isinstance(due, list), "due must be a list"
assert isinstance(mastered, int) and isinstance(tracked, int), "counts must be ints"
assert len(due) + mastered == tracked, "queue inconsistent: " + str(len(due)) + "+" + str(mastered) + " != " + str(tracked)
print("  ok   — /api/review consistent (due=" + str(len(due)) + ", mastered=" + str(mastered) + ", tracked=" + str(tracked) + ")")
' || fail "/api/review consistency"

echo
if [ "$fails" -eq 0 ]; then
  echo "SMOKE PASS ✓"
else
  echo "SMOKE FAILED: $fails assertion(s)"; exit 1
fi
