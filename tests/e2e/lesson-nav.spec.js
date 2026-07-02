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
    await page.locator('.tab[data-view="lessons"]').click();
    const cta = page.locator('.continuecta');
    await expect(cta).toBeVisible();
    await expect(cta).toContainText('▶ Start'); // nothing read yet
    const firstId = await cta.getAttribute('data-id');
    expect(firstId).toBeTruthy();
    // Mark the first lesson read, then re-render (leave + return to the tab).
    await page.evaluate((id) => localStorage.setItem('ts-lessons-read', JSON.stringify([id])), firstId);
    await page.locator('.tab[data-view="practice"]').click();
    await page.locator('.tab[data-view="lessons"]').click();
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
    await page.locator('.tab[data-view="lessons"]').click();
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
});

test.describe('Books woven into the path (chapter deep-links)', () => {
  test('a lesson carries a book cross-link with a real chapter page', async ({ page }) => {
    await page.goto('/');
    await page.locator('.tab[data-view="lessons"]').click();
    await page.locator('.lessonjump').first().click();
    const book = page.locator('.lessonbook .booklink');
    await expect(book).toBeVisible();
    await expect(book).toHaveAttribute('data-file', /\.pdf$/);
    await expect(book).toHaveAttribute('data-page', /^\d+$/); // deep-links to a page
  });

  test('the Journey shows per-phase chapter book links with pages', async ({ page }) => {
    await page.goto('/');
    await page.locator('#moreBtn').click();
    await page.locator('.menuitem[data-view="roadmap"]').click();
    const links = page.locator('.roadphase .booklink');
    await expect(links.first()).toBeVisible();
    await expect(links.first()).toHaveAttribute('data-page', /^\d+$/);
    expect(await links.count()).toBeGreaterThan(1); // one per phase
  });
});
