// Regression guard for the motion-polish pass (Paul 2026-07-07, #40: "fun and
// cool and modern … animations"). Two properties matter and both are easy for a
// refactor to break: (1) the delight cues exist — the XP bar fills smoothly, a
// view fades in, a level-up pulses the chip; (2) they ALL collapse under
// prefers-reduced-motion, so the app stays usable/accessible for opt-outs.
const { test, expect } = require('@playwright/test');

test.describe('Motion polish', () => {
  test('progress + view + level-up cues are present', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#xpchip');
    // The XP bar animates its fill (matching the progress gauge) rather than jumping.
    expect(await page.evaluate(() =>
      getComputedStyle(document.querySelector('#xpchip .xp-bar i')).transitionProperty)).toContain('width');
    // A level-up pulses the chip: seed at 0, jump to Lv 2, the .levelup class fires.
    await page.evaluate(() => { localStorage.removeItem('ts-gamify'); gamifyUpdate(0); gamifyUpdate(5); });
    expect(await page.evaluate(() => document.getElementById('xpchip').classList.contains('levelup'))).toBe(true);
    // A document view fades in on navigation.
    await page.evaluate(() => showView('lessons'));
    await page.waitForSelector('.docview');
    expect(await page.evaluate(() => getComputedStyle(document.querySelector('.docview')).animationName)).toBe('fadein');
  });

  test('prefers-reduced-motion collapses all of it', async ({ browser }) => {
    const ctx = await browser.newContext({ reducedMotion: 'reduce' });
    const page = await ctx.newPage();
    await page.goto('/');
    await page.waitForSelector('#xpchip');
    // The global guard forces near-zero durations, so nothing animates for opt-outs.
    const barDur = await page.evaluate(() => getComputedStyle(document.querySelector('#xpchip .xp-bar i')).transitionDuration);
    expect(parseFloat(barDur)).toBeLessThan(0.01);
    await page.evaluate(() => showView('lessons'));
    await page.waitForSelector('.docview');
    const viewDur = await page.evaluate(() => getComputedStyle(document.querySelector('.docview')).animationDuration);
    expect(parseFloat(viewDur)).toBeLessThan(0.01);
    await ctx.close();
  });
});
