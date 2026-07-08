// E2E for the unified Sandbox (Paul 2026-07-08: "the sandbox and fullscreen ide
// should be the same thing"). The 🧪 Sandbox menu entry — and the legacy
// showView('sandbox') route — now open the ONE coding environment, the IDE,
// focused on a free-play scratch file. Running compiles via the same offline
// /api/sandbox path with NO effect on exercise progress. (Scratch-file CRUD +
// persistence are covered in ide-scratch.spec.js.)
const { test, expect } = require('@playwright/test');

test.describe('Sandbox = the unified IDE on a scratch file', () => {
  test('the 🧪 Sandbox menu opens the IDE on a scratch file; running it works; progress untouched', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    const progressBefore = await page.locator('#exTitle').textContent();

    // Open the Sandbox from the menu, as a user would → it lands in the IDE.
    await page.locator('#menuBtn').click();
    await page.locator('#menuSandbox').click();
    await page.waitForSelector('#idehost .cm-editor, #idehost textarea');
    // A scratch file is open (not empty, not an exercise) and the Scratch group is
    // in the explorer beside the exercise files — one environment, both kinds.
    await expect(page.locator('#ideopenname')).not.toHaveText(/no file open/);
    await expect(page.locator('.ide-group-h', { hasText: 'Scratch' })).toBeVisible();
    // Scratch toolbar: browser auto-save (no disk Save/Reset), Delete shown.
    await expect(page.locator('#idesave')).toBeHidden();
    await expect(page.locator('#idedelete')).toBeVisible();

    // Set a deterministic program and Run it → its stdout appears in the terminal.
    await page.evaluate(() =>
      window.ideView.dispatch({ changes: { from: 0, to: window.ideView.state.doc.length, insert: 'fn main() { println!("sbx-{}", 6 * 7); }' } }));
    await page.locator('#iderun').click();
    await expect(page.locator('#ideout')).toContainText('sbx-42', { timeout: 30_000 });

    // The whole point of free-play: exercise progress is NEVER touched.
    await page.locator('.tab[data-view="practice"]').click();
    await expect(page.locator('#exTitle')).toHaveText(progressBefore);
  });

  test("the legacy showView('sandbox') route also lands in the IDE scratch view", async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    await page.evaluate(() => showView('sandbox'));
    await page.waitForSelector('#idehost .cm-editor, #idehost textarea');
    await expect(page.locator('.ide-group-h', { hasText: 'Scratch' })).toBeVisible();
    await expect(page.locator('#ideopenname')).not.toHaveText(/no file open/);
    // There is no separate Sandbox surface anymore.
    await expect(page.locator('#sbxhost')).toHaveCount(0);
  });
});
