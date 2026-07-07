// E2E for runnable code examples in lessons/book. A whole-program example (one
// with a `fn main`) gets a ▷ Run button that compiles + runs it via the same
// offline path as the Sandbox (/api/sandbox), output inline — READING support, so
// it never touches exercise progress. Partial snippets are left un-runnable.
const { test, expect } = require('@playwright/test');

test.describe('Runnable code examples in lessons/book', () => {
  test('a whole-program example runs inline; every Run button is under a fn-main block; progress untouched', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    const progressBefore = await page.locator('#exTitle').textContent();

    await page.evaluate(() => showView('lessons', '00-hello-world'));
    await page.waitForSelector('#docview .lessonbody');
    const btns = page.locator('#docview .coderun-btn');
    await expect(btns.first(), 'the hello-world program is runnable').toBeVisible();

    // A Run button appears ONLY under a block that is a complete program.
    const allWhole = await page.evaluate(() =>
      [...document.querySelectorAll('#docview .coderun-bar')].every((bar) => {
        const pre = bar.previousElementSibling;
        return pre && /\bfn\s+main\s*\(/.test(pre.textContent || '');
      }));
    expect(allWhole, 'no Run button on a partial snippet').toBe(true);

    // Run it → output appears inline in its own console.
    await btns.first().click();
    const out = page.locator('#docview .coderun-out').first();
    await expect(out).toBeVisible();
    await expect(out).toContainText('Hello, world!', { timeout: 30_000 });

    // Running an example is free-play — exercise progress is never recorded.
    await page.locator('.tab[data-view="practice"]').click();
    await expect(page.locator('#exTitle')).toHaveText(progressBefore);
  });

  test('an example opens in the Sandbox as a NEW file, without clobbering existing scratch', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    await page.evaluate(() => { try { localStorage.removeItem('ts-sandbox-files'); localStorage.removeItem('ts-sandbox-code'); } catch (_) {} });

    // Seed the scratch file with distinct code.
    await page.evaluate(() => showView('sandbox'));
    await page.waitForSelector('#sbxhost .cm-content');
    await page.evaluate(() => window.sbxView.dispatch({ changes: { from: 0, to: window.sbxView.state.doc.length, insert: 'fn main(){ println!("KEEP-SCRATCH"); }' } }));

    // From a lesson example, tap "Open in Sandbox".
    await page.evaluate(() => showView('lessons', '00-hello-world'));
    await page.waitForSelector('#docview .coderun-bar');
    await page.locator('#docview .coderun-btn', { hasText: 'Open in Sandbox' }).first().click();
    await page.waitForSelector('#sbxhost .cm-content');

    // A new "example" file is active with the example's code (not the scratch's)…
    await expect(page.locator('#sbxfiles .sbxfile.active .sbxfile-name')).toContainText('example');
    expect(await page.evaluate(() => window.sbxView.state.doc.toString())).not.toContain('KEEP-SCRATCH');
    // …and the scratch file is preserved intact.
    await page.locator('#sbxfiles .sbxfile-name', { hasText: 'scratch' }).click();
    expect(await page.evaluate(() => window.sbxView.state.doc.toString())).toContain('KEEP-SCRATCH');
  });
});
