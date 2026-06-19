#!/usr/bin/env node
// Tempered Studio — book_ref anchor integrity gate.
//
// Every exercise `.toml` book_ref may carry an `anchor` (a section slug). The
// web Book reader (gui/index.html `mdToHtml`) gives each heading a GitHub-style
// slug `id` and scrolls a clicked book-ref to it. If an authored anchor doesn't
// match any heading in its chapter, the jump silently falls back to chapter-top
// — so this asserts every anchor resolves, catching drift when the bundled
// chapters are re-vendored or an anchor is typo'd.
//
// The slug here mirrors gui/index.html exactly: esc() runs first (so the only
// entities are &amp;/&lt;/&gt;), then lowercase → drop punctuation → spaces to
// hyphens. Run: `node scripts/verify-book-anchors.mjs`.
import { readFileSync, readdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const esc = s => s.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;');
const slug = s => esc(s).replace(/&(?:lt|gt|amp|quot|#\d+);/g, '').toLowerCase().replace(/[^\w\s-]/g, '').trim().replace(/\s+/g, '-');

// All heading slugs in a bundled chapter (null if the chapter file is missing).
const cache = {};
function chapterSlugs(ch) {
  if (ch in cache) return cache[ch];
  let txt;
  try { txt = readFileSync(join(root, 'book', ch + '.md'), 'utf8'); }
  catch { return (cache[ch] = null); }
  const out = [];
  for (const line of txt.split('\n')) {
    const m = line.match(/^#{1,6}\s+(.*)$/);
    if (m) out.push(slug(m[1]));
  }
  return (cache[ch] = out);
}

// Walk exercises/ for .toml files.
function walk(dir) {
  let out = [];
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name);
    if (e.isDirectory()) out = out.concat(walk(p));
    else if (e.name.endsWith('.toml')) out.push(p);
  }
  return out;
}

let total = 0, resolved = 0;
const misses = [];
for (const file of walk(join(root, 'exercises'))) {
  const txt = readFileSync(file, 'utf8');
  let ch = null;
  // book_ref blocks are `chapter = "..."` then (optionally) `anchor = "..."`.
  for (const line of txt.split('\n')) {
    const cm = line.match(/^\s*chapter\s*=\s*"([^"]+)"/);
    if (cm) { ch = cm[1]; continue; }
    const am = line.match(/^\s*anchor\s*=\s*"([^"]+)"/);
    if (am && ch) {
      total++;
      const slugs = chapterSlugs(ch);
      if (slugs && slugs.includes(am[1])) resolved++;
      else misses.push({ file: file.replace(root + '/', ''), ch, anchor: am[1], slugs });
    }
  }
}

console.log(`== book_ref anchor integrity: ${resolved}/${total} resolve ==`);
if (misses.length) {
  for (const m of misses) {
    console.log(`\nFAIL ${m.file}\n  chapter: ${m.ch}\n  anchor : ${m.anchor}`);
    console.log(m.slugs ? '  available: ' + m.slugs.join(', ') : '  (chapter file missing!)');
  }
  console.log(`\n${misses.length} anchor(s) do not resolve.`);
  process.exit(1);
}
console.log('ALL ANCHORS RESOLVE ✓');
