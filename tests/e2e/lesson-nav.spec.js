// Regression guard for the lesson-navigation features added this session, which
// had ZERO dedicated coverage even though they're now core UX:
//   1. the "Continue / Start" CTA that resumes at your first unread lesson,
//   2. the type-to-filter box over the 37-lesson list,
//   3. the per-phase book cross-links that DEEP-LINK to a chapter page.
// These are localStorage- + render-driven, exactly the kind of thing a stray
// refactor silently breaks — pin them down.
const { test, expect } = require('@playwright/test');

test.describe('Lessons: Continue / Start resume CTA', () => {
  test('Start when fresh → Continue to first unread → opens the lesson', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => localStorage.removeItem('ts-lessons-read'));
    await page.evaluate(() => showView('lessons'));
    const cta = page.locator('.continuecta');
    await expect(cta).toBeVisible();
    await expect(cta).toContainText('▶ Start'); // nothing read yet
    const firstId = await cta.getAttribute('data-id');
    expect(firstId).toBeTruthy();
    // Mark the first lesson read, then re-render (leave + return to the tab).
    await page.evaluate((id) => localStorage.setItem('ts-lessons-read', JSON.stringify([id])), firstId);
    await page.locator('.tab[data-view="practice"]').click();
    await page.evaluate(() => showView('lessons'));
    await expect(cta).toContainText('▶ Continue'); // now resumes, not starts
    await expect(cta).not.toHaveAttribute('data-id', firstId); // points past the read one
    // Clicking it opens a lesson (the "← all lessons" back link appears).
    await cta.click();
    await expect(page.locator('.lessonback')).toBeVisible();
  });
});

test.describe('Lessons: type-to-filter', () => {
  test('narrows the list, shows a no-match note, and clears back to all', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('lessons'));
    const filter = page.locator('.lessonfilter');
    await expect(filter).toBeVisible();
    const items = page.locator('.booktoc li');
    const total = await items.count();
    expect(total).toBeGreaterThan(8); // the filter only shows past a handful

    await filter.fill('ownership');
    const visible = page.locator('.booktoc li:visible');
    await expect(visible.first()).toContainText(/owner/i);
    expect(await visible.count()).toBeLessThan(total);
    await expect(page.locator('.lessonnomatch')).toBeHidden();

    await filter.fill('zzzzznope');
    await expect(page.locator('.lessonnomatch')).toBeVisible();
    await expect(page.locator('.booktoc li:visible')).toHaveCount(0);

    await filter.fill('');
    await expect(page.locator('.booktoc li:visible')).toHaveCount(total);
    await expect(page.locator('.lessonnomatch')).toBeHidden();
  });

  test('full-text search surfaces body matches (not just titles) and opens them', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('lessons'));
    await page.waitForSelector('.lessonfilter');
    const bh = page.locator('.lessonbodyhits');
    // "borrow" appears in more lesson BODIES than titles → a "found in the text" list
    // (server /api/lessons?q=, ranked; the desktop e2e server supports it).
    await page.locator('.lessonfilter').fill('borrow');
    await expect.poll(() => bh.locator('.lbh-list li').count(), { timeout: 6000 }).toBeGreaterThan(0);
    await expect(bh.locator('.lbh-h')).toContainText(/found in the text/i);
    const first = bh.locator('.lbh-list li').first();
    await expect(first.locator('small'), 'each body hit shows a snippet').not.toHaveText('');
    await expect(first.locator('.bscount'), 'and a match count').toHaveText(/\d+/);
    // opening a body hit navigates into that lesson
    await first.locator('.lessonjump').click();
    await expect(page.locator('.lessonback')).toBeVisible();
    // gibberish → no title AND no body match → the no-match note; body section hidden
    await page.evaluate(() => showView('lessons'));
    await page.waitForSelector('.lessonfilter');
    await page.locator('.lessonfilter').fill('zzznotawordzz');
    await expect(page.locator('.lessonnomatch')).toBeVisible();
    await expect(bh).toBeHidden();
    // clearing restores the full stage list; the body section stays hidden
    await page.locator('.lessonfilter').fill('');
    await expect(bh).toBeHidden();
    await expect.poll(() => page.locator('.lstage:visible').count()).toBeGreaterThan(1);
  });
});

test.describe('Quizzes: completion tracking', () => {
  test('revealing every answer marks the quiz done (✓ + count on the list)', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => localStorage.removeItem('ts-quizzes-done'));
    await page.evaluate(() => showView('quizzes'));
    await expect(page.locator('.readbadge')).toHaveCount(0); // nothing done yet
    await page.locator('.quizjump').first().click();
    // Wait for the quiz (async fetch) to actually render before counting.
    await expect(page.locator('.quizanswers').first()).toBeVisible();
    // Reveal every answer (predict-then-verify) — that's what "done" means.
    const summaries = page.locator('.quizanswers > summary');
    const n = await summaries.count();
    expect(n).toBeGreaterThan(0);
    for (let i = 0; i < n; i++) await summaries.nth(i).click();
    // Back to the list → ✓ + "1 of M done".
    await page.locator('.quizback').click();
    await expect(page.locator('.readbadge')).toContainText(/1 of \d+ done/);
    await expect(page.locator('.booktoc li.read .readmark').first()).toBeVisible();
  });

  test('a partially-revealed quiz is NOT marked done', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => localStorage.removeItem('ts-quizzes-done'));
    await page.evaluate(() => showView('quizzes'));
    await page.locator('.quizjump').first().click();
    await expect(page.locator('.quizanswers').first()).toBeVisible();
    await page.locator('.quizanswers > summary').first().click(); // reveal only one
    await page.locator('.quizback').click();
    await expect(page.locator('.readbadge')).toHaveCount(0); // still nothing done
  });
});

test.describe('Books woven into the path (chapter deep-links)', () => {
  test('a lesson carries a book cross-link — precise in-app chapter when mapped, PDF page otherwise', async ({ page }) => {
    await page.goto('/');
    // A lesson mapped (LESSON_BOOK) to a bundled Rust Book chapter → a topic-precise
    // IN-APP chapter link, and clicking it opens that exact bundled chapter.
    await page.evaluate(() => showView('lessons', '01-bindings-and-immutability'));
    await page.waitForSelector('#docview .lessonbody');
    const ch = page.locator('.lessonbook .bookchlink');
    await expect(ch).toBeVisible();
    const chId = await ch.getAttribute('data-ch');
    expect(chId).toMatch(/^ch\d/);
    const chapterOk = await page.evaluate(
      async (id) => ((await fetch('api/book?chapter=' + id).then((r) => r.json())).markdown || '').length > 0,
      chId,
    );
    expect(chapterOk, 'maps to a real bundled chapter').toBe(true);
    await ch.click();
    await expect(page.locator('#docview')).toContainText('contents'); // landed in the in-app Book

    // A phased lesson WITHOUT a chapter mapping falls back to the per-phase PDF page.
    await page.evaluate(() => showView('lessons', '17-slices-in-depth'));
    await page.waitForSelector('#docview .lessonbody');
    const pdf = page.locator('.lessonbook .booklink');
    await expect(pdf).toBeVisible();
    await expect(pdf).toHaveAttribute('data-file', /\.pdf$/);
    await expect(pdf).toHaveAttribute('data-page', /^\d+$/); // deep-links to a page
  });

  test('the Journey shows per-phase chapter book links with pages', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('roadmap'));
    const links = page.locator('.roadphase .booklink');
    await expect(links.first()).toBeVisible();
    await expect(links.first()).toHaveAttribute('data-page', /^\d+$/);
    expect(await links.count()).toBeGreaterThan(1); // one per phase
  });

  test('the Journey shows an always-present per-stage progress bar that fills with reads', async ({ page }) => {
    await page.goto('/');
    // Fresh: every stage still shows a progress bar (measurable from day one).
    await page.evaluate(() => localStorage.setItem('ts-lessons-read', '[]'));
    await page.evaluate(() => showView('roadmap'));
    const bars = page.locator('.roadphase .roadprog');
    const phases = await page.locator('.roadphase').count();
    expect(await bars.count(), 'a progress bar on every stage').toBe(phases);
    await expect(bars.first().locator('.roadprog-n')).toHaveText(/^0\/\d+$/); // 0/N when nothing read
    // Read the first few lessons → the first stage(s) fill and can complete (✓ + .done).
    const ids = await page.evaluate(async () => (await (await fetch('api/lessons')).json()).lessons.map(l => l.id));
    await page.evaluate((s) => localStorage.setItem('ts-lessons-read', JSON.stringify(s)), ids.slice(0, 6));
    await page.evaluate(() => showView('roadmap'));
    // The bar fill width animates from 0 toward the read fraction (>0 once settled).
    const firstBar = page.locator('.roadphase .roadprog-bar i').first();
    await expect.poll(() => firstBar.evaluate(el => parseFloat(el.style.width) || 0)).toBeGreaterThan(0);
    expect(await page.locator('.roadphase .roadprog.done').count(), 'a fully-read stage is marked done').toBeGreaterThan(0);
  });

  test('a Rust Book chapter offers the gentler Patina lesson, and it opens', async ({ page }) => {
    await page.goto('/');
    // A chapter with a primary Patina lesson (ch04-01 → 15-ownership) shows the
    // reciprocal switch; clicking it opens that lesson.
    await page.evaluate(() => showView('book', 'ch04-01-what-is-ownership'));
    await page.waitForSelector('#docview .bookbody');
    const patina = page.locator('.bookpatina .patinalink');
    await expect(patina).toBeVisible();
    await expect(patina).toHaveAttribute('data-lesson', /^\d/); // a real lesson stem
    await patina.click();
    await expect(page.locator('#docview .lessonbody')).toBeVisible(); // landed in a lesson
  });
});

test.describe('Lessons: glossary tap-to-define', () => {
  test('jargon in lesson prose is tappable and reveals a definition — never in code', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('lessons', '01-bindings-and-immutability'));
    await page.waitForSelector('#docview .lessonbody');
    const defs = page.locator('#docview .lessonbody .glossdef');
    await expect(defs.first()).toBeVisible(); // at least one term got linked
    // Jargon-in-code / editor stays untouched: no glossdef inside code/link/heading.
    const inCode = await page.evaluate(() =>
      [...document.querySelectorAll('#docview .lessonbody .glossdef')]
        .filter((el) => el.closest('code,pre,a,h1,h2,h3,h4,h5,h6')).length);
    expect(inCode, 'no term linked inside code/link/heading').toBe(0);
    // Tap → an inline definition popover appears; tap again → it closes (one at a time).
    await defs.first().click();
    const pop = page.locator('#docview .lessonbody .glosspop');
    await expect(pop).toBeVisible();
    await expect(pop).not.toBeEmpty();
    // Definitions render markdown `code` spans as real inline <code>, not literal backticks.
    expect(await pop.first().textContent(), 'no raw backtick in the definition').not.toContain('`');
    await page.locator('#docview .lessonbody .glossdef.open').first().click();
    await expect(pop).toHaveCount(0);
  });

  test('the Book chapters get tap-to-define too (forward-reference jargon), minus common-word false positives', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    await page.evaluate(() => showView('book', 'ch04-01-what-is-ownership'));
    await page.waitForSelector('#docview .bookbody');
    await page.waitForTimeout(300); // async terms + linkify
    const defs = page.locator('#docview .bookbody .glossdef');
    await expect(defs.first(), 'the chapter linked at least one term').toBeVisible();
    // Never in code (the book is full of examples), and the common-English stoplist
    // holds (e.g. the ordinary word "result" must NOT be linked to the Rust term).
    const bad = await page.evaluate(() => {
      const els = [...document.querySelectorAll('#docview .bookbody .glossdef')];
      return {
        inCode: els.filter((e) => e.closest('code,pre,a,h1,h2,h3,h4,h5,h6')).length,
        stoplisted: els.filter((e) => ['result', 'method', 'expect', 'collect', 'moved'].includes(e.textContent.trim().toLowerCase())).length,
      };
    });
    expect(bad.inCode, 'no term linked inside code/heading').toBe(0);
    expect(bad.stoplisted, 'no common-English word linked (stoplist)').toBe(0);
  });
});
