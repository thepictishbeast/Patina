// Regression guard for the fullscreen IDE (Paul 2026-07-07): a file explorer,
// editor, and terminal as three SEPARATE sub-views, each fullscreen-able. It is
// a progress-neutral WORKSHOP over the course's exercise files — it must never
// leak an answer, never advance progress, and honour the same locked/done gates
// as Practice. These are render- + fetch-driven and easy to break silently.
const { test, expect } = require('@playwright/test');

test.describe('Fullscreen IDE', () => {
  test('opens from the ⋯ menu with three sub-views and a populated explorer', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    // Reachable via the ⋯ menu (NO third top tab — the 2-tab design is preserved).
    await expect(page.locator('.tab[data-view]')).toHaveCount(2);
    await page.evaluate(() => showView('ide'));
    await expect(page.locator('.ide-tab[data-sub="files"]')).toBeVisible();
    await expect(page.locator('.ide-tab[data-sub="editor"]')).toBeVisible();
    await expect(page.locator('.ide-tab[data-sub="terminal"]')).toBeVisible();
    // The explorer lists exercise sources, grouped by phase.
    await page.waitForSelector('.ide-file');
    expect(await page.locator('.ide-group').count()).toBeGreaterThan(1);
    expect(await page.locator('.ide-file').count()).toBeGreaterThan(10);
  });

  test('the explorer never renders answer-bearing files or metadata', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('ide'));
    await page.waitForSelector('.ide-file');
    const tree = await page.locator('#idetree').innerHTML();
    // Only .rs sources appear; the answer-bearing .toml files never do.
    expect(tree).not.toContain('.toml');
    expect(tree).not.toContain('solution_outline');
    expect(tree).not.toContain('expected_error_code');
    for (const name of await page.locator('.ide-fname').allInnerTexts()) {
      expect(name.trim()).toMatch(/\.rs$/);
    }
  });

  test('sub-views switch and fullscreen toggles, clearing on exit', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('ide'));
    await page.waitForSelector('.ide-tab[data-sub="terminal"]');
    // Switch to the terminal sub-view — its panel becomes the visible one.
    await page.locator('.ide-tab[data-sub="terminal"]').click();
    await expect(page.locator('.ide-panel[data-sub="terminal"]')).toHaveClass(/active/);
    await expect(page.locator('.ide-panel[data-sub="files"]')).not.toHaveClass(/active/);
    // Fullscreen adds the body class; leaving the IDE must remove it (so the
    // hidden header/footer can never bleed into another view).
    await page.locator('#idefull').click();
    expect(await page.evaluate(() => document.body.classList.contains('ide-full-on'))).toBe(true);
    await page.evaluate(() => showView('practice'));
    expect(await page.evaluate(() => document.body.classList.contains('ide-full-on'))).toBe(false);
  });

  test('opening a locked file is gated behind a jump-ahead confirm', async ({ page }) => {
    await page.goto('/');
    // Find a file the SERVER actually gates (423). A file's tree *status* of
    // "locked" only means "no progress entry yet"; the real gate is is_unlocked
    // (a file opens once its predecessor is Done). So probe from the END — where
    // the unlock frontier can't have reached in these specs — for the first 423.
    const { lockedId, gatedStatus } = await page.evaluate(async () => {
      const d = await (await fetch('api/workspace')).json();
      const ids = d.groups.flatMap(g => g.files.map(f => f.id));
      for (let i = ids.length - 1; i >= 0; i--) {
        const s = (await fetch('api/workspace/file?id=' + encodeURIComponent(ids[i]))).status;
        if (s === 423) return { lockedId: ids[i], gatedStatus: s };
      }
      return { lockedId: null, gatedStatus: null };
    });
    expect(lockedId, 'some exercise is still behind the baby-steps gate').toBeTruthy();
    expect(gatedStatus).toBe(423);
    // In the UI, opening a gated file prompts the jump-ahead confirm; declining
    // it opens nothing.
    await page.evaluate(() => localStorage.setItem('ts-ide-state', JSON.stringify({ sub: 'files', openId: null })));
    await page.evaluate(() => showView('ide'));
    await page.waitForSelector('.ide-file');
    expect(await page.locator('#ideopenname').textContent()).toContain('no file open');
    page.once('dialog', d => d.dismiss());
    await page.evaluate((id) => ideOpenFile(id, 'locked'), lockedId);
    await page.waitForTimeout(150);
    expect(await page.locator('#ideopenname').textContent()).toContain('no file open');
  });
});
