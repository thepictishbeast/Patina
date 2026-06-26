#!/usr/bin/env bash
# Sync the authored Patina textbook content (lessons + quizzes) from the upstream
# rust-textbook repo into this repo's bundled dirs, so the offline app can serve
# them (the same way `book/` holds the embedded Rust Book chapters). rust-textbook
# is the source of truth; re-run this after editing that content there.
#
#   scripts/sync-lessons.sh [path-to-rust-textbook]
#
# Defaults to the sibling checkout, overridable via TEXTBOOK or $1.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
root="${1:-${TEXTBOOK:-$here/../rust-textbook}}"

# Each content type is a flat dir of `<id>.md` files mirrored 1:1.
for kind in lessons quizzes cheatsheets; do
  src="$root/$kind"
  dst="$here/$kind"
  if [ ! -d "$src" ]; then
    echo "warn: $kind source not found at $src — skipping" >&2
    continue
  fi
  mkdir -p "$dst"
  count=0
  for f in "$src"/*.md; do
    [ -e "$f" ] || continue
    cp -f "$f" "$dst/"
    count=$((count + 1))
  done
  echo "synced $count $kind file(s) from $src -> $dst"
done
