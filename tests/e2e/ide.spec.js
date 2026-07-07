// Regression guard for the fullscreen IDE (Paul 2026-07-07). The redesign makes
// the file explorer PART of the IDE: a sidebar that sits BESIDE an always-visible
// editor, with a foldable terminal below it — NOT three mutually-exclusive tabs.
// You can browse and open files without ever losing sight of the editor. It stays
// a progress-neutral WORKSHOP over the course's exercise files: it must never leak
// an answer, never advance progress, and honour the same locked/done gates as
// Practice. These are render- + fetch-driven and easy to break silently.
const { test, expect } = require('@playwright/test');

test.describe('Fullscreen IDE', () => {
  test('opens from the ⋯ menu with explorer, editor AND terminal coexisting', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    // Reachable via the ⋯ menu (NO third top tab — the 2-tab design is preserved).
    await expect(page.locator('.tab[data-view]')).toHaveCount(2);
    await page.evaluate(() => showView('ide'));
    await page.waitForSelector('.ide-file');
    // The explorer and the editor host are visible AT THE SAME TIME — the whole
    // point of the redesign. The terminal exists too (folded until you Run).
    await expect(page.locator('#idetree')).toBeVisible();
    await expect(page.locator('#idehost')).toBeVisible();
    await expect(page.locator('#ideterm')).toHaveClass(/closed/);
    // The explorer lists exercise sources, grouped by phase.
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

  test('opening a file keeps the editor visible; toggling the explorer never hides it', async ({ page }) => {
    await page.goto('/');
    // Start clean so no previously-open file is restored.
    await page.evaluate(() => localStorage.removeItem('ts-ide-state'));
    await page.evaluate(() => showView('ide'));
    await page.waitForSelector('.ide-file');
    // Open the first file the server will actually serve (first exercise is
    // unlocked; skip any that answer 409 done / 423 locked).
    const openId = await page.evaluate(async () => {
      const d = await (await fetch('api/workspace')).json();
      const ids = d.groups.flatMap(g => g.files.map(f => f.id));
      for (const id of ids) {
        const s = (await fetch('api/workspace/file?id=' + encodeURIComponent(id))).status;
        if (s === 200) return id;
      }
      return null;
    });
    expect(openId, 'at least one exercise file opens').toBeTruthy();
    await page.evaluate((id) => ideOpenFile(id, null), openId);
    // The editor now holds the file AND the explorer is still beside it.
    await expect(page.locator('#idehost .cm-editor, #idehost textarea')).toBeVisible();
    await expect(page.locator('#idetree')).toBeVisible();
    await expect(page.locator('#ideopenname')).not.toHaveText(/no file open/);
    // Fold the explorer away — the editor must REMAIN visible (the old design
    // hid it; this is the exact regression Paul called out).
    await page.locator('#idefilesbtn').click();
    await expect(page.locator('#idetree')).toBeHidden();
    await expect(page.locator('#idehost .cm-editor, #idehost textarea')).toBeVisible();
    // Bring it back.
    await page.locator('#idefilesbtn').click();
    await expect(page.locator('#idetree')).toBeVisible();
  });

  test('terminal folds open/closed and fullscreen toggles, clearing on exit', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('ide'));
    await page.waitForSelector('.ide-file');
    // Terminal starts folded; its toggle opens it while the editor stays put.
    await expect(page.locator('#ideterm')).toHaveClass(/closed/);
    await page.locator('#idetermbtn').click();
    await expect(page.locator('#ideterm')).not.toHaveClass(/closed/);
    await expect(page.locator('#idehost')).toBeVisible();
    await page.locator('#idetermbtn').click();
    await expect(page.locator('#ideterm')).toHaveClass(/closed/);
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
    // it opens nothing (the open filename stays "no file open").
    await page.evaluate(() => localStorage.removeItem('ts-ide-state'));
    await page.evaluate(() => showView('ide'));
    await page.waitForSelector('.ide-file');
    expect(await page.locator('#ideopenname').textContent()).toContain('no file open');
    page.once('dialog', d => d.dismiss());
    await page.evaluate((id) => ideOpenFile(id, 'locked'), lockedId);
    await page.waitForTimeout(150);
    expect(await page.locator('#ideopenname').textContent()).toContain('no file open');
  });
});
