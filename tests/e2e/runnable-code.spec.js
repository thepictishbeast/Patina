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

    // Every code block also gets a "⧉ Copy" button (whole programs AND partial snippets).
    const codeBlocks = await page.locator('#docview .lessonbody pre code').count();
    const copyBtns = await page.locator('#docview .lessonbody .codecopy').count();
    expect(copyBtns, 'one Copy button per code block').toBe(codeBlocks);
    expect(codeBlocks, 'the lesson has code blocks').toBeGreaterThan(0);

    // Run it → output appears inline in its own console.
    await btns.first().click();
    const out = page.locator('#docview .coderun-out').first();
    await expect(out).toBeVisible();
    await expect(out).toContainText('Hello, world!', { timeout: 30_000 });

    // Running an example is free-play — exercise progress is never recorded.
    await page.locator('.tab[data-view="practice"]').click();
    await expect(page.locator('#exTitle')).toHaveText(progressBefore);
  });

  test('an example opens in the IDE as a NEW scratch file, without clobbering existing scratch', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    await page.evaluate(() => { try { localStorage.removeItem('ts-sandbox-files'); localStorage.removeItem('ts-sandbox-code'); } catch (_) {} });

    // Seed the default scratch file with distinct code (auto-persists to the store).
    await page.evaluate(() => showView('sandbox'));
    await page.waitForSelector('#idehost .cm-editor');
    await page.evaluate(() => window.ideView.dispatch({ changes: { from: 0, to: window.ideView.state.doc.length, insert: 'fn main(){ println!("KEEP-SCRATCH"); }' } }));
    await expect.poll(() => page.evaluate(() => (localStorage.getItem('ts-sandbox-files') || '').includes('KEEP-SCRATCH'))).toBe(true);

    // From a lesson example, tap "Open in the IDE" → the example opens in the IDE.
    await page.evaluate(() => showView('lessons', '00-hello-world'));
    await page.waitForSelector('#docview .coderun-bar');
    await page.locator('#docview .coderun-btn', { hasText: 'Open in the IDE' }).first().click();
    await page.waitForSelector('#idehost .cm-editor');

    // A new "example" scratch file is active with the example's code (not the scratch's)…
    await expect(page.locator('#ideopenname')).toContainText('example');
    expect(await page.evaluate(() => window.ideView.state.doc.toString())).not.toContain('KEEP-SCRATCH');
    // …and the seeded scratch file is preserved intact (open it from the tree).
    await page.locator('.ide-file.st-scratch .ide-fname', { hasText: /^scratch$/ }).click();
    await expect.poll(() => page.evaluate(() => window.ideView.state.doc.toString())).toContain('KEEP-SCRATCH');
  });
});
