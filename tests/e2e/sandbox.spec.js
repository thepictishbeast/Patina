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

  test('multiple files: create, switch keeps them independent, run the active, delete', async ({ page }) => {
    page.on('dialog', (d) => (d.type() === 'prompt' ? d.accept('demo') : d.accept()));
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    await page.evaluate(() => { try { localStorage.removeItem('ts-sandbox-files'); localStorage.removeItem('ts-sandbox-code'); } catch (_) {} });
    await page.evaluate(() => showView('sandbox'));
    await page.waitForSelector('#sbxhost .cm-content');
    await expect(page.locator('#sbxfiles .sbxfile-name')).toHaveText(['scratch']);

    // + new → "demo", becomes the active file.
    await page.locator('#sbxfiles .sbxfile-new').click();
    await expect(page.locator('#sbxfiles .sbxfile.active .sbxfile-name')).toHaveText('demo');

    // Put distinct code in demo; switching to scratch shows scratch's own code.
    await page.evaluate(() => window.sbxView.dispatch({ changes: { from: 0, to: window.sbxView.state.doc.length, insert: 'fn main(){ println!("SBX-DEMO"); }' } }));
    await page.locator('#sbxfiles .sbxfile-name', { hasText: 'scratch' }).click();
    expect(await page.evaluate(() => window.sbxView.state.doc.toString()), 'files are independent').not.toContain('SBX-DEMO');

    // Back to demo → its code persisted; run it.
    await page.locator('#sbxfiles .sbxfile-name', { hasText: 'demo' }).click();
    expect(await page.evaluate(() => window.sbxView.state.doc.toString())).toContain('SBX-DEMO');
    await page.locator('#sbxrun').click();
    await expect(page.locator('#sbxout')).toContainText('SBX-DEMO', { timeout: 30_000 });

    // Delete the active file → back to a single file.
    await page.locator('#sbxfiles .sbxfile.active .sbxfile-x').click();
    await expect(page.locator('#sbxfiles .sbxfile-name')).toHaveText(['scratch']);
  });
});
