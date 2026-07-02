// Accessibility audit of the web surface (the "a11y" half of #12), via axe-core
// driven through the same Playwright Chromium. Hard gate: zero CRITICAL or SERIOUS
// WCAG 2 A/AA violations. Moderate/minor issues are logged as findings, not failures.
//
// Covers the main Practice view AND the content surfaces built out since — the
// Lessons list (Continue CTA + filter), an open lesson (book cross-links + Tutor-
// adjacent chrome), the offline Library list, and the Book TOC — so an a11y
// regression in any of them is caught, not just on the landing screen.
const { test, expect } = require('@playwright/test');
const AxeBuilder = require('@axe-core/playwright').default;

/** Run axe on the current DOM; log every violation, return the blocking ids. */
async function blockingViolations(page, label) {
  const { violations } = await new AxeBuilder({ page }).withTags(['wcag2a', 'wcag2aa']).analyze();
  for (const v of violations) {
    console.log(`  [${label}] a11y [${v.impact}] ${v.id} — ${v.help} (${v.nodes.length} node(s))`);
    if (v.impact === 'critical' || v.impact === 'serious') {
      for (const n of v.nodes.slice(0, 8)) {
        console.log(`      @ ${n.target}  ::  ${(n.html || '').replace(/\s+/g, ' ').slice(0, 100)}`);
      }
    }
  }
  return violations
    .filter((v) => v.impact === 'critical' || v.impact === 'serious')
    .map((v) => v.id);
}

test('a11y: no critical or serious violations on the main view', async ({ page }) => {
  // Assert against the settled UI, not a transient render frame: opt into reduced
  // motion (which the page honours) so the fade-in is collapsed and axe never
  // composites a mid-fade opacity into its contrast math.
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.locator('#exTitle').waitFor();
  await page.locator('.exlist .ex').first().waitFor(); // exercise list populated
  expect(await blockingViolations(page, 'main'), 'main view').toEqual([]);
});

test('a11y: Lessons list + an open lesson', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.evaluate(() => showView('lessons'));
  await page.locator('.continuecta, .booktoc li').first().waitFor();
  expect(await blockingViolations(page, 'lessons-list'), 'lessons list').toEqual([]);
  await page.locator('.lessonjump').first().click();
  await page.locator('.lessonback').waitFor();
  expect(await blockingViolations(page, 'lesson-open'), 'open lesson').toEqual([]);
});

test('a11y: offline Library list', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.evaluate(() => showView('library'));
  await page.locator('.libopen').first().waitFor();
  expect(await blockingViolations(page, 'library'), 'library list').toEqual([]);
});

test('a11y: Book table of contents', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.evaluate(() => showView('book'));
  await page.locator('.booktoc li, .bookjump').first().waitFor();
  expect(await blockingViolations(page, 'book-toc'), 'book toc').toEqual([]);
});

test('a11y: Quizzes list', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.evaluate(() => showView('quizzes'));
  await page.locator('.quizjump').first().waitFor();
  expect(await blockingViolations(page, 'quizzes'), 'quizzes list').toEqual([]);
});

test('a11y: Glossary (search + term cards)', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.evaluate(() => showView('glossary'));
  await page.locator('.gloss-term').first().waitFor();
  expect(await blockingViolations(page, 'glossary'), 'glossary').toEqual([]);
});

test('a11y: Journey / roadmap', async ({ page }) => {
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.evaluate(() => showView('roadmap'));
  await page.locator('.roadphase').first().waitFor();
  expect(await blockingViolations(page, 'roadmap'), 'roadmap').toEqual([]);
});
