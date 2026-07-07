// Regression guard for the reading-size control (Paul 2026-07-07, #41: make the
// textbooks more readable). A ⋯-menu A−/A+ stepper scales ALL long-form reading
// text (the Book, lessons, the lesson drawer — everything that renders into
// .bookbody) via the --read-scale CSS variable, clamped and persisted. This is
// pure GUI/localStorage and easy for a refactor to silently break.
const { test, expect } = require('@playwright/test');

test.describe('Reading size', () => {
  test('scales long-form text, persists across reload, and clamps', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => localStorage.removeItem('ts-read-scale'));
    await page.evaluate(() => showView('lessons', '15-ownership'));
    await page.waitForSelector('.bookbody p');
    const base = await page.evaluate(() => parseFloat(getComputedStyle(document.querySelector('.bookbody p')).fontSize));
    expect(base).toBeCloseTo(15.5, 1); // the tuned default

    // Two steps up → the reading text and the % label both grow; state persists.
    await page.evaluate(() => { setReadScale(1); setReadScale(1); });
    const bigger = await page.evaluate(() => parseFloat(getComputedStyle(document.querySelector('.bookbody p')).fontSize));
    expect(bigger).toBeGreaterThan(base);
    await expect(page.locator('#rsVal')).toHaveText('130%');
    expect(await page.evaluate(() => localStorage.getItem('ts-read-scale'))).toBe('1.3');

    // Persistence: after a reload the larger size is restored.
    await page.reload();
    await page.evaluate(() => showView('lessons', '15-ownership'));
    await page.waitForSelector('.bookbody p');
    const afterReload = await page.evaluate(() => parseFloat(getComputedStyle(document.querySelector('.bookbody p')).fontSize));
    expect(afterReload).toBeCloseTo(bigger, 1);

    // Clamp: hammering "smaller" never drops below the smallest step (layout-safe).
    await page.evaluate(() => { for (let i = 0; i < 12; i++) setReadScale(-1); });
    expect(await page.evaluate(() => localStorage.getItem('ts-read-scale'))).toBe('0.85');
    // ...and "larger" never exceeds the largest step.
    await page.evaluate(() => { for (let i = 0; i < 12; i++) setReadScale(1); });
    expect(await page.evaluate(() => localStorage.getItem('ts-read-scale'))).toBe('1.5');
  });
});
