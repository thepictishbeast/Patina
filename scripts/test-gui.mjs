#!/usr/bin/env node
// Tempered Studio — web GUI pure-function tests (no browser needed).
//
// `gui/index.html` carries two pure transforms that are easy to get subtly
// wrong and security-sensitive: `esc`-then-`mdToHtml` (markdown → safe HTML)
// and `highlightRust` (syntax-highlight an ALREADY-ESCAPED Rust code block).
// This extracts those functions straight out of the shipped HTML (so the test
// tracks the real code, not a copy), writes them to a throwaway ES module, and
// imports it — no eval / new Function. Run with `node scripts/test-gui.mjs`.
//
// The load-bearing invariant for the highlighter is XSS-safety by construction:
// it must only ever INSERT fixed-class <span> wrappers, never alter any other
// character — so stripping the spans returns the exact input. That single
// property guarantees a <script> escaped upstream stays inert.
import { readFileSync, writeFileSync, mkdtempSync, rmSync } from 'node:fs';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { dirname, join } from 'node:path';
import { tmpdir } from 'node:os';
import { execFileSync } from 'node:child_process';

const here = dirname(fileURLToPath(import.meta.url));
const html = readFileSync(join(here, '..', 'gui', 'index.html'), 'utf8');

// Step 0 — the WHOLE app <script> must parse. The per-function tests below only
// import mdToHtml/highlightRust; this guards the rest (showView, renderBook,
// event wiring) against a syntax error slipping in. `node --check` parses without
// executing, so browser globals (document/window/fetch) are irrelevant.
{
  const m = [...html.matchAll(/<script>([\s\S]*?)<\/script>/g)].map(x => x[1]).find(s => s.includes('function mdToHtml'));
  if (!m) { console.log('  FAIL — could not find the app <script> block'); process.exit(1); }
  const d = mkdtempSync(join(tmpdir(), 'ts-gui-chk-')), p = join(d, 'app.js');
  writeFileSync(p, m);
  try { execFileSync(process.execPath, ['--check', p], { stdio: 'pipe' }); }
  catch (e) { console.log('  FAIL — app <script> has a syntax error:\n' + (e.stderr || e).toString()); process.exit(1); }
  finally { rmSync(d, { recursive: true, force: true }); }
}

// Pull a `function NAME(...) { ... }` body out of the HTML by brace-matching.
function grabFn(name) {
  const start = html.indexOf('function ' + name + '(');
  if (start < 0) throw new Error('could not find function ' + name + ' in gui/index.html');
  let i = html.indexOf('{', start), depth = 0;
  for (; i < html.length; i++) {
    if (html[i] === '{') depth++;
    else if (html[i] === '}' && --depth === 0) { i++; break; }
  }
  return html.slice(start, i);
}

// Re-declare the tiny dependencies the two functions close over (kept in sync
// with gui/index.html; trivial + asserted indirectly by the tests below), then
// the real extracted bodies. Written to a temp module and imported.
const deps = [
  "const esc = s => (s == null ? '' : String(s)).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;');",
  "const RS_KW = /^(?:as|async|await|break|const|continue|crate|dyn|else|enum|extern|false|fn|for|if|impl|in|let|loop|match|mod|move|mut|pub|ref|return|self|Self|static|struct|super|trait|true|type|union|unsafe|use|where|while)$/;",
  "const RS_PRIM = /^(?:i8|i16|i32|i64|i128|isize|u8|u16|u32|u64|u128|usize|f32|f64|bool|char|str)$/;",
].join('\n');
const moduleSrc = deps + '\n' + grabFn('highlightRust') + '\n' + grabFn('mdToHtml') +
  '\nexport { esc, highlightRust, mdToHtml };\n';

const dir = mkdtempSync(join(tmpdir(), 'ts-gui-'));
const modPath = join(dir, 'gui-extract.mjs');
writeFileSync(modPath, moduleSrc);
let mod;
try { mod = await import(pathToFileURL(modPath).href); }
finally { rmSync(dir, { recursive: true, force: true }); }
const { esc, highlightRust, mdToHtml } = mod;

let fails = 0;
const ok = m => console.log('  ok   — ' + m);
const bad = m => { console.log('  FAIL — ' + m); fails++; };
const check = (cond, m) => cond ? ok(m) : bad(m);
const stripSpans = s => s.replace(/<span class="tok-[a-z]">/g, '').replace(/<\/span>/g, '');

console.log('== Tempered Studio — GUI transform tests ==');

// 1. THE invariant: highlighting only inserts spans; strip them → the input.
for (const sample of [
  'fn main() { let mut n: i32 = 42; println!("{n}"); }',
  'let v: Vec<&str> = vec!["a", "b"]; // a comment with fn let mut',
  'struct Point { x: f64, y: f64 }',
  'match x { Some(v) => v, None => 0 }',
  '/* block fn comment */ let s = "string with \\" quote and < > &";',
  "let c = 'a'; let lifetime: &'static str = \"hi\";",
]) {
  const e = esc(sample);
  check(stripSpans(highlightRust(e)) === e, 'strip(spans) === escaped input  [' + sample.slice(0, 32) + '…]');
}

// 2. Keywords / types / numbers / macros actually get classed.
const h = highlightRust(esc('fn main() { let n: i32 = 42; println!("hi"); }'));
check(h.includes('<span class="tok-k">fn</span>'), 'keyword `fn` → tok-k');
check(h.includes('<span class="tok-k">let</span>'), 'keyword `let` → tok-k');
check(h.includes('<span class="tok-t">i32</span>'), 'primitive `i32` → tok-t');
check(h.includes('<span class="tok-n">42</span>'), 'number `42` → tok-n');
check(h.includes('<span class="tok-m">println!</span>'), 'macro `println!` → tok-m');
check(highlightRust(esc('struct Foo;')).includes('<span class="tok-t">Foo</span>'), 'CamelCase `Foo` → tok-t');

// 3. A keyword inside a string / comment is NOT separately wrapped.
const inStr = highlightRust(esc('let s = "fn let mut";'));
check((inStr.match(/tok-s/g) || []).length === 1 && !inStr.includes('"<span'),
      'keyword inside a string literal is not re-tokenized');
const inCmt = highlightRust(esc('// fn let mut struct'));
check((inCmt.match(/tok-c/g) || []).length === 1 && !/tok-k/.test(inCmt),
      'keyword inside a comment is not re-tokenized');

// 4. XSS: an HTML tag inside a fenced code block is neutralised by escape-first.
//    Using a neutral `<b>` tag — if `<` is escaped, EVERY tag (incl. <script>)
//    is inert, so this proves the guarantee for the whole fenced path.
const md = '```rust\nlet x = "<b>not bold</b>";\n```';
const out = mdToHtml(md);
check(!out.includes('<b>') && out.includes('&lt;b&gt;'), 'fenced HTML tag stays inert (escape-first)');
check(out.includes('<pre><code>') && out.includes('<span class="tok-k">let</span>'),
      'fenced block is highlighted inside <pre><code>');

// 5. Entities pass through intact (a Rust reference `&str` survives).
const ref = highlightRust(esc('let r: &str = "x";'));
check(ref.includes('&amp;') && ref.includes('<span class="tok-t">str</span>'), 'reference `&str` → &amp; intact + str typed');

// 6. Inline emphasis (_x_ / *x*) → <em>, GFM-intraword-safe, escape-first.
check(mdToHtml('A _variable_ binds a value.').includes('<em>variable</em>'), '`_x_` → <em>');
check(mdToHtml('This is *very* important.').includes('<em>very</em>'), '`*x*` → <em>');
check(mdToHtml('Two _key_ _terms_ here.').includes('<em>key</em>') && mdToHtml('Two _key_ _terms_ here.').includes('<em>terms</em>'),
      'two emphases on one line both wrap');
const snake = mdToHtml('Call to_string and read foo_bar_baz now.');
check(!snake.includes('<em>') && snake.includes('to_string') && snake.includes('foo_bar_baz'),
      'intraword snake_case is NOT italicised');
const codeEm = mdToHtml('Use `a_b_c` carefully.');
check(codeEm.includes('<code>a_b_c</code>') && !codeEm.includes('<em>'), 'underscores inside `code` are not italicised');
const bothBI = mdToHtml('A **bold** and an _em_ word.');
check(bothBI.includes('<strong>bold</strong>') && bothBI.includes('<em>em</em>'), 'bold and italic coexist (bold not eaten by em)');
const emXss = mdToHtml('danger _<b>x</b>_ here');
check(!emXss.includes('<b>') && emXss.includes('<em>') && emXss.includes('&lt;b&gt;'), 'emphasis content stays escaped (no raw tag)');

// 7. Headings h1–h6 (corpus uses through h4); each carries a slug id.
check(mdToHtml('# Title').includes('<h1 id="title">Title</h1>'), '`# x` → <h1> (with id)');
check(mdToHtml('### Sub').includes('<h3 id="sub">Sub</h3>'), '`### x` → <h3> (with id)');
check(mdToHtml('#### Deeper').includes('<h4 id="deeper">Deeper</h4>'), '`#### x` → <h4> (was literal before)');
check(mdToHtml('###### Deepest').includes('<h6 id="deepest">Deepest</h6>'), '`###### x` → <h6>');
check(!mdToHtml('#nospace heading').includes('<h1'), '`#nospace` (no space) is NOT a heading');

// 8. GFM tables (the ch03 integer-types table renders as a real <table>).
const tbl = mdToHtml('| Length | Signed |\n|--------|--------|\n| 8-bit | `i8` |\n| 16-bit | `i16` |');
check(tbl.includes('<table>') && tbl.includes('</table>'), 'pipe table → <table>');
check((tbl.match(/<th>/g) || []).length === 2, 'header row → 2 <th>');
check((tbl.match(/<tr>/g) || []).length === 3, 'header + 2 body rows → 3 <tr>');
check(tbl.includes('<td><code>i8</code></td>'), 'table cells are inline-rendered (`i8` → <code>)');
check(!tbl.includes('<p>|'), 'table is not left as literal pipe paragraphs');
const tblXss = mdToHtml('| H |\n|---|\n| <b>x</b> |');
check(!tblXss.includes('<b>') && tblXss.includes('&lt;b&gt;'), 'table cell content stays escaped');
const notTbl = mdToHtml('costs are a | b in prose');
check(!notTbl.includes('<table>') && notTbl.includes('<p>'), 'a stray pipe with no separator row is NOT a table');

// 9. Heading slug ids (so book_ref anchors can scroll to the cited section).
check(mdToHtml('### The Rules of References').includes('<h3 id="the-rules-of-references">'),
      'heading → GitHub-style slug id (matches an authored anchor)');
const useId = mdToHtml('## Bringing Paths into Scope with the `use` Keyword');
check(useId.includes('id="bringing-paths-into-scope-with-the-use-keyword"'),
      'slug drops backticks/punctuation (matches the `use`-keyword anchor)');
const idMatch = mdToHtml('#### Mutable References').match(/id="([^"]*)"/);
check(idMatch && /^[a-z0-9_-]*$/.test(idMatch[1]), 'slug id is attribute-safe ([a-z0-9_-] only)');
// `<T>` headings: esc → &lt;/&gt;, which the slug must STRIP (else `arclttgt`),
// so the GitHub-style anchor `the-api-of-mutext` resolves.
check(mdToHtml('### The API of Mutex<T>').includes('id="the-api-of-mutext"'),
      'angle-bracket heading slug strips entities (Mutex<T> → the-api-of-mutext)');

console.log('');
if (fails === 0) console.log('GUI TRANSFORM TESTS PASS ✓');
else { console.log('GUI TRANSFORM TESTS FAILED: ' + fails); process.exit(1); }
