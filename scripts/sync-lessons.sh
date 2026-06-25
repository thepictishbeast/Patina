#!/usr/bin/env bash
# Sync the authored Patina textbook lessons from the upstream rust-textbook repo
# into this repo's bundled `lessons/` dir, so the offline app can serve them
# (the same way `book/` holds the embedded Rust Book chapters). rust-textbook is
# the source of truth; re-run this after editing lessons there.
#
#   scripts/sync-lessons.sh [path-to-rust-textbook]
#
# Defaults to the sibling checkout, overridable via TEXTBOOK or $1.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
src="${1:-${TEXTBOOK:-$here/../rust-textbook}}/lessons"
dst="$here/lessons"

if [ ! -d "$src" ]; then
  echo "error: lessons source not found at $src" >&2
  echo "       pass the rust-textbook path: scripts/sync-lessons.sh /path/to/rust-textbook" >&2
  exit 1
fi

mkdir -p "$dst"
# Mirror only the lesson markdown (not the repo's other files).
count=0
for f in "$src"/*.md; do
  [ -e "$f" ] || continue
  cp -f "$f" "$dst/"
  count=$((count + 1))
done

echo "synced $count lesson(s) from $src -> $dst"
