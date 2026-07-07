// E2E for the Sandbox — a free-play Rust scratchpad (Paul's "Sandbox tab"). Its
// own CodeMirror editor + Run compile+run ARBITRARY code via POST /api/sandbox
// (the local toolchain, offline), with NO exercise context and — critically — NO
// effect on exercise progress. Reached from the ⋯ menu.
const { test, expect } = require('@playwright/test');

test.describe('Sandbox: free-play scratchpad', () => {
  test('run arbitrary code → output; a compile error surfaces; progress untouched', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    const progressBefore = await page.locator('#exTitle').textContent();

    // Open the Sandbox from the menu (as a user would).
    await page.locator('#menuBtn').click();
    await page.locator('#menuSandbox').click();
    await page.waitForSelector('#sbxhost .cm-content');
    await expect(page.locator('#sbxout'), 'a fresh sandbox prompts to Run').toContainText(/Run to see/);

    // Set a deterministic program and Run it → its stdout appears.
    await page.evaluate(() =>
      window.sbxView.dispatch({ changes: { from: 0, to: window.sbxView.state.doc.length, insert: 'fn main() { println!("sbx-{}", 6 * 7); }' } }));
    await page.locator('#sbxrun').click();
    await expect(page.locator('#sbxout')).toContainText('sbx-42', { timeout: 30_000 });
    await expect(page.locator('#sbxstatus')).toContainText('ran');

    // A compile error surfaces (error styling), without crashing the view.
    await page.evaluate(() =>
      window.sbxView.dispatch({ changes: { from: 0, to: window.sbxView.state.doc.length, insert: 'fn main() { let x = 5 }' } }));
    await page.locator('#sbxrun').click();
    await expect(page.locator('#sbxout.err')).toBeVisible({ timeout: 30_000 });
    await expect(page.locator('#sbxout')).toContainText(/error/i);
    // The status distinguishes a compile failure from a runtime one (not "exited 101").
    await expect(page.locator('#sbxstatus')).toContainText(/compile error/i);

    // The whole point: the Sandbox NEVER touches exercise progress.
    await page.locator('.tab[data-view="practice"]').click();
    await expect(page.locator('#exTitle')).toHaveText(progressBefore);
  });
});
