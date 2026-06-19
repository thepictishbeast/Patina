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

# Root the throwaway store under an EXEC-capable base: the success-path assertion
# runs the compiled exercise binary, and some systems mount /tmp `noexec`. Default
# to ~/.cache (where rpro-serve itself stores state); override with TS_SMOKE_DIR.
SMOKE_BASE="${TS_SMOKE_DIR:-${HOME:-/tmp}/.cache}"
mkdir -p "$SMOKE_BASE" 2>/dev/null || SMOKE_BASE="${TMPDIR:-/tmp}"
STATE="$(mktemp -d "$SMOKE_BASE/ts-smoke.XXXXXX")"
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

# 5b. SUCCESS path, end-to-end through the real toolchain: submit a correct fix
# for the first exercise (add `mut`) → it compiles, RUNS, passes, and the learner
# advances to the next exercise. This is the app's core payoff — previously only
# the failing path was exercised here, and `advance` only unit-tested.
python3 - >"$STATE/fix.json" <<'PY'
import json
src = "fn main() {\n    let mut count = 0;\n    count = count + 1;\n    println!(\"count is {count}\");\n}\n"
print(json.dumps({"op": "run", "source": src}))
PY
curl -s -X POST "$BASE/api/run" -H 'Content-Type: application/json' --data-binary @"$STATE/fix.json" \
  | python3 -c '
import sys, json
d = json.load(sys.stdin)
assert d.get("passed") is True, "the corrected source must pass: " + json.dumps(d)[:200]
assert d.get("advanced_to"), "a passing Run must advance the learner to the next exercise"
print("  ok   — correct fix runs + passes + advances to " + str(d["advanced_to"]))
' || fail "/api/run success path (compile + run + pass + advance)"

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

# 7. request-body limit (defence-in-depth): a body over the 1 MiB cap is rejected
# at the transport layer (413) before parsing. ~1.1 MiB of payload.
python3 -c 'open("'"$STATE"'/big.json", "w").write("{\"op\":\"run\",\"source\":\"" + "x" * 1_100_000 + "\"}")'
body_code="$(curl -s -o /dev/null -w '%{http_code}' -X POST "$BASE/api/run" \
  -H 'Content-Type: application/json' --data-binary @"$STATE/big.json")"
if [ "$body_code" = "413" ]; then
  ok "request body > 1 MiB rejected (413)"
else
  fail "body-limit: expected 413, got $body_code"
fi

# 8. embedded Book: the table of contents seeds with real chapters (ids+titles).
curl -s "$BASE/api/book" | python3 -c '
import sys, json
d = json.load(sys.stdin)
chs = d.get("chapters", [])
assert len(chs) >= 10, "too few book chapters seeded: " + str(len(chs))
assert all(c.get("id") and c.get("title") for c in chs), "a chapter is missing id/title"
ids = {c["id"] for c in chs}
assert "ch04-01-what-is-ownership" in ids, "ownership chapter not seeded"
print("  ok   — " + str(len(chs)) + " book chapters in the TOC")
' || fail "/api/book TOC"

# 9. a chapter fetch returns that chapter's real markdown body, cleaned for
# display: no raw mdBook include directives, un-bundled listings linked out.
curl -s "$BASE/api/book?chapter=ch04-01-what-is-ownership" | python3 -c '
import sys, json
d = json.load(sys.stdin)
md = d.get("markdown") or ""
assert d.get("id") == "ch04-01-what-is-ownership", "wrong chapter id"
assert "Ownership" in md, "chapter markdown missing"
assert "{{#" not in md, "raw mdBook directive leaked into /api/book"
assert "doc.rust-lang.org/book/ch04-01-what-is-ownership.html" in md, "listing link-out missing"
print("  ok   — /api/book?chapter= returns cleaned markdown (directives stripped, listings linked)")
' || fail "/api/book chapter fetch"

# 10. SECURITY: the chapter param is a map KEY, never a path. A traversal value
# must MISS the map (chapter:null) and never return a file from disk. Asserted
# raw and percent-encoded, against the live server (the network-surface twin of
# the book_traversal_* unit tests).
for q in "../../etc/passwd" "..%2F..%2F..%2Fetc%2Fpasswd" "%2Fetc%2Fpasswd"; do
  out="$(curl -s "$BASE/api/book?chapter=$q")"
  if printf '%s' "$out" | python3 -c '
import sys, json
d = json.load(sys.stdin)
assert d.get("chapter", "MISSING") is None, "did not miss the map"
' && ! printf '%s' "$out" | grep -q "root:"; then
    ok "book traversal [$q] -> null (no file read)"
  else
    fail "book traversal [$q] leaked or resolved"
  fi
done

# 11. POST /api/select switches the current exercise (validated against the
# DISCOVERED ids — an unknown id is rejected 400, never used as a path). Run last
# so it doesn't disturb the current-exercise the earlier assertions rely on.
second="$(curl -s "$BASE/api/exercises" | python3 -c 'import sys,json; print(json.load(sys.stdin)["exercises"][1]["id"])')"
sel_code="$(curl -s -o /dev/null -w '%{http_code}' -X POST "$BASE/api/select" \
  -H 'Content-Type: application/json' -d "{\"id\":\"$second\"}")"
now="$(curl -s "$BASE/api/current" | python3 -c 'import sys,json; print(json.load(sys.stdin).get("exercise",""))')"
if [ "$sel_code" = "200" ] && [ "$now" = "$second" ]; then
  ok "POST /api/select switched current -> $second"
else
  fail "select: code=$sel_code current=$now (expected $second)"
fi
bad_code="$(curl -s -o /dev/null -w '%{http_code}' -X POST "$BASE/api/select" \
  -H 'Content-Type: application/json' -d '{"id":"../../etc/passwd"}')"
[ "$bad_code" = "400" ] && ok "select rejects unknown id (400)" \
  || fail "select unknown id: expected 400, got $bad_code"

echo
if [ "$fails" -eq 0 ]; then
  echo "SMOKE PASS ✓"
else
  echo "SMOKE FAILED: $fails assertion(s)"; exit 1
fi
