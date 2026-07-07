#!/usr/bin/env node
// Tempered Studio — lesson-corpus integrity check (no browser, no server).
//
// The curriculum is 78 baby-step lessons chained by a hand-written
// `<!-- lesson-nav -->` footer (← prev · ↑ Study Guide · next →). Every split
// touches three or four files' footers plus the Study Guide index; a single
// mistyped filename silently breaks navigation with no test to catch it. This
// walks the whole corpus and asserts the invariants a split can break:
//
//   1. every lesson has a `# Lesson …` H1 and the seven `## N.` sections,
//   2. a `<!-- lesson-nav -->` footer whose links all resolve to real files,
//   3. footer-chain continuity — lesson[i]'s "next →" is lesson[i+1]'s file and
//      lesson[i+1]'s "← prev" is lesson[i]'s file (in course order),
//   4. no body link points at a `NN-*.md` lesson file that doesn't exist
//      (a ghost id left behind by a rename),
//   5. the Study Guide's "It's N lessons" count matches the file count.
//
// Run with `node scripts/check-lessons.mjs`. Exit 0 = clean, 1 = problems.
import { readFileSync, readdirSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const here = dirname(fileURLToPath(import.meta.url));
const lessonsDir = join(here, '..', 'lessons');
const problems = [];
const bad = (file, msg) => problems.push(`${file}: ${msg}`);

// Course order: numeric prefix, then letter suffix ('' < 'b' < 'c' …).
const key = (name) => {
  const m = name.match(/^(\d+)([a-z]*)-/);
  return m ? [parseInt(m[1], 10), m[2] || ''] : [1e9, name];
};
const files = readdirSync(lessonsDir)
  .filter((f) => f.endsWith('.md') && /^\d/.test(f))
  .sort((a, b) => {
    const [an, al] = key(a), [bn, bl] = key(b);
    return an - bn || (al < bl ? -1 : al > bl ? 1 : 0);
  });

if (files.length === 0) { console.log('FAIL — no lesson files found'); process.exit(1); }

// A lesson-nav footer link: `[label](target.md)`. We only care about the .md
// targets that are lesson files (skip the Study Guide's ../ link).
const linkRe = /\[[^\]]*\]\(([^)]+\.md)\)/g;
const parsed = new Map(); // file -> { navPrev, navNext, navTargets: Set }

for (const f of files) {
  const text = readFileSync(join(lessonsDir, f), 'utf8');

  // (1) H1 + seven numbered sections.
  if (!/^#\s+Lesson\b/m.test(text)) bad(f, 'missing "# Lesson …" H1');
  for (let n = 1; n <= 7; n++) {
    if (!new RegExp(`^##\\s+${n}\\.`, 'm').test(text)) bad(f, `missing "## ${n}." section`);
  }

  // (2) the lesson-nav footer block.
  const navIdx = text.indexOf('<!-- lesson-nav -->');
  if (navIdx === -1) { bad(f, 'missing <!-- lesson-nav --> footer'); parsed.set(f, null); continue; }
  const nav = text.slice(navIdx);
  const navTargets = new Set();
  let prev = null, next = null, m;
  linkRe.lastIndex = 0;
  while ((m = linkRe.exec(nav))) {
    const target = m[1];
    if (target.startsWith('..')) continue; // the ↑ Study Guide link
    navTargets.add(target);
    // The label text tells prev (has "←") from next (has "→").
    const label = nav.slice(nav.lastIndexOf('[', m.index), m.index);
    if (label.includes('←')) prev = target;
    else if (label.includes('→') || /→\]/.test(nav.slice(m.index, m.index + m[0].length + 2))) next = target;
  }
  // The "→" sits at the END of the next-link label; detect via the segment order.
  // Re-derive prev/next robustly from the arrow positions in the footer text.
  const arrowPrev = nav.match(/\[←[^\]]*\]\(([^)]+\.md)\)/);
  const arrowNext = nav.match(/\[[^\]]*→\]\(([^)]+\.md)\)/);
  prev = arrowPrev ? arrowPrev[1] : null;
  next = arrowNext ? arrowNext[1] : null;

  // (2b) every footer target must resolve to a real file.
  for (const t of navTargets) {
    if (!files.includes(t)) bad(f, `footer link to missing file "${t}"`);
  }

  // (4) no BODY link to a non-existent lesson file (ghost id from a rename).
  const body = text.slice(0, navIdx);
  linkRe.lastIndex = 0;
  while ((m = linkRe.exec(body))) {
    const t = m[1];
    if (t.startsWith('..') || t.includes('/')) continue;
    if (/^\d/.test(t) && !files.includes(t)) bad(f, `body link to missing lesson "${t}"`);
  }

  parsed.set(f, { prev, next });
}

// (3) footer-chain continuity across the sorted order.
for (let i = 0; i < files.length; i++) {
  const p = parsed.get(files[i]);
  if (!p) continue;
  const expectPrev = i > 0 ? files[i - 1] : null;
  const expectNext = i < files.length - 1 ? files[i + 1] : null;
  if (expectPrev && p.prev !== expectPrev) {
    bad(files[i], `footer "← prev" is "${p.prev || 'missing'}", expected "${expectPrev}"`);
  }
  if (expectNext && p.next !== expectNext) {
    bad(files[i], `footer "next →" is "${p.next || 'missing'}", expected "${expectNext}"`);
  }
}

// (5) Study Guide count matches the file count.
try {
  const sg = readFileSync(join(here, '..', 'STUDY-GUIDE.md'), 'utf8');
  const m = sg.match(/It's (\d+) lessons/);
  if (!m) problems.push('STUDY-GUIDE.md: no "It\'s N lessons" count line');
  else if (parseInt(m[1], 10) !== files.length) {
    problems.push(`STUDY-GUIDE.md: count says ${m[1]} but there are ${files.length} lesson files`);
  }
  const b = sg.match(/(\d+) baby steps/);
  if (b && parseInt(b[1], 10) !== files.length) {
    problems.push(`STUDY-GUIDE.md: "${b[1]} baby steps" but there are ${files.length} lesson files`);
  }
} catch (_) { problems.push('STUDY-GUIDE.md: could not read'); }

if (problems.length) {
  console.log(`LESSON INTEGRITY: ${problems.length} problem(s) across ${files.length} lessons\n`);
  for (const p of problems) console.log('  ✗ ' + p);
  process.exit(1);
}
console.log(`  ok   — ${files.length} lessons: H1 + 7 sections, footer chain continuous, no dead/ghost links, Study Guide count matches`);
console.log('LESSON INTEGRITY CHECK PASSED ✓');
