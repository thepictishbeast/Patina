// Regression guard for the gamification layer (Paul 2026-07-07: "gamify
// learning … fun and cool"). The load-bearing CHARTER rule is "never reward a
// reveal/hint": XP/level/streak/badges are a pure function of the server's
// `done` count, so a hint or answer-reveal — which never changes `done` —
// cannot move any of them. This pins that invariant plus the level/badge/streak
// behaviour, all offline and localStorage-driven.
const { test, expect } = require('@playwright/test');

test.describe('Gamification', () => {
  test('the XP chip renders and reflects progress; a reveal grants NOTHING', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#xpchip');
    // Deterministic control over the server-authoritative `done` count.
    await page.evaluate(() => { localStorage.removeItem('ts-gamify'); gamifyUpdate(0); });
    await expect(page.locator('#xpLv')).toHaveText('Lv 1');
    await expect(page.locator('#xpStreak')).toHaveText('🔥0');

    // Completing 5 exercises → Lv 2, a 1-day streak, the first+five badges.
    await page.evaluate(() => gamifyUpdate(5));
    await expect(page.locator('#xpLv')).toHaveText('Lv 2');
    await expect(page.locator('#xpStreak')).toHaveText('🔥1');
    const at5 = await page.evaluate(() => localStorage.getItem('ts-gamify'));

    // THE INVARIANT: a reveal/hint does NOT change `done`, so folding the SAME
    // count again must leave the stored state byte-identical — no XP, no level,
    // no streak, no badge can come from a reveal.
    await page.evaluate(() => gamifyUpdate(5));
    const afterReveal = await page.evaluate(() => localStorage.getItem('ts-gamify'));
    expect(afterReveal).toBe(at5);

    // Reset-to-redo drops `done`, but the level must NOT fall (bestDone is
    // monotonic — you don't lose a level for practising again).
    await page.evaluate(() => gamifyUpdate(4));
    await expect(page.locator('#xpLv')).toHaveText('Lv 2');
  });

  test('the badge panel opens with earned + locked badges', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#xpchip');
    await page.evaluate(() => { localStorage.removeItem('ts-gamify'); gamifyUpdate(10); });
    await page.locator('#xpchip').click();
    await expect(page.locator('#xppanel')).toBeVisible();
    // Seven badges total; at 10 done, first/five/ten are earned, the rest locked.
    await expect(page.locator('#xppanel .badge')).toHaveCount(7);
    expect(await page.locator('#xppanel .badge.earned').count()).toBeGreaterThanOrEqual(3);
    expect(await page.locator('#xppanel .badge.locked').count()).toBeGreaterThan(0);
    // Esc closes it.
    await page.keyboard.press('Escape');
    await expect(page.locator('#xppanel')).toHaveCount(0);
  });
});
