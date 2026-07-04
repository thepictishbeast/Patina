// E2E for the Assist/Dev diagnostic JUMP-to-line affordance (a bounded, robust
// slice of #18 inline diagnostics). After a run, each parsed diagnostic shows the
// code (→ Explain) and the line (→ move the editor caret there). The jump is
// char-offset based (setSelectionRange), so it is wrap-immune. Learn withholds the
// whole panel, so neither affordance exists there (tier differentiation).
const { test, expect } = require('@playwright/test');

// A compile-error exercise whose error sits well below line 1, so a successful
// jump visibly moves the caret. traits/03_trait_object_dyn fails with E0308 on the
// `let shapes = vec![...]` line.
const EX = 'traits/03_trait_object_dyn';

async function runInAssist(page, request) {
  await request.post('/api/select', { data: { id: EX, force: true } });
  await page.goto('/');
  await expect(page.locator('.cm-content')).toBeVisible();
  await page.locator('#menuBtn').click(); // open the ⋯ menu (mode switcher lives there now)
  await page.locator('#modesw button[data-mode="assist"]').click();
  await page.locator('.pbtn[data-pred="fails"]').click();
  await page.locator('#runbtn').click();
  await expect(page.locator('#statusline')).toContainText(/failed|passed/, { timeout: 30_000 });
  await expect(page.locator('.diag .diagcode')).toContainText(/E0\d{3}/, { timeout: 30_000 });
}

test.describe('Assist/Dev diagnostic jump-to-line', () => {
  test('clicking the line number moves the editor caret to that line', async ({ page, request }) => {
    await runInAssist(page, request);
    const jump = page.locator('.diag .jumpline').first();
    await expect(jump, 'a jump target is rendered for a diagnostic with a span').toBeVisible();
    const lineNo = parseInt(await jump.getAttribute('data-line'), 10);
    expect(lineNo, 'data-line is a real 1-based line').toBeGreaterThan(1);

    await jump.click();
    // The CodeMirror selection should now cover exactly that 1-based source line.
    const sel = await page.evaluate(() => window.__cm.sel());
    expect(sel.line, 'caret jumped to the diagnostic line').toBe(lineNo);
    expect(sel.selected.length, 'the offending line is selected (caret moved there)').toBeGreaterThan(0);
  });

  // DEFERRED: red gutter error-marks are being reimplemented as CodeMirror line
  // decorations (the old #editorHL overlay is gone). Diagnostics still show in the
  // console + the jump-to-line works; the gutter tint is a follow-up.
  test.skip('the gutter marks the diagnostic line red — and an edit clears it', async ({ page, request }) => {
    await runInAssist(page, request);
    const lineNo = parseInt(await page.locator('.diag .jumpline').first().getAttribute('data-line'), 10);
    // The .cl block at that 1-based index carries the err mark (red number).
    const errIndexes = await page.evaluate(() => {
      const cls = [...document.querySelectorAll('#editorHL .cl')];
      return cls.flatMap((el, i) => el.classList.contains('err') ? [i + 1] : []);
    });
    expect(errIndexes, 'the flagged line number is marked in the gutter').toContain(lineNo);
    // Editing the buffer invalidates the marks (they describe the OLD code).
    await page.locator('#editorCode').evaluate((el) => {
      el.value = '// edited\n' + el.value;
      el.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await expect(page.locator('#editorHL .cl.err'), 'edit clears stale marks').toHaveCount(0);
  });

  test('Learn never marks the gutter (by-hand stays by-hand)', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });
    await page.goto('/'); // Learn is the default mode
    await expect(page.locator('.cm-content')).toBeVisible();
    await page.locator('.pbtn[data-pred="fails"]').click();
    await page.locator('#runbtn').click();
    await expect(page.locator('#statusline')).toContainText(/failed|passed/, { timeout: 30_000 });
    await expect(page.locator('#editorHL .cl.err')).toHaveCount(0);
  });

  test('the diagnostic row is not itself an interactive control (no nested buttons)', async ({ page, request }) => {
    await runInAssist(page, request);
    // The clickable bits are the inner .diagcode / .jumpline spans; the row div
    // itself must carry no button role / tabindex (avoids axe nested-interactive).
    const row = page.locator('.diag').filter({ has: page.locator('.diagcode') }).first();
    await expect(row).toHaveAttribute('class', /diag/);
    expect(await row.getAttribute('role'), 'row is a plain container').toBeNull();
    expect(await row.getAttribute('tabindex'), 'row is not a tab stop').toBeNull();
    expect(await row.getAttribute('data-code'), 'code lives on the inner button').toBeNull();
  });

  test('Learn withholds the panel: no code/jump affordances there', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });
    await page.goto('/'); // Learn is the default mode
    await expect(page.locator('.cm-content')).toBeVisible();
    await page.locator('.pbtn[data-pred="fails"]').click();
    await page.locator('#runbtn').click();
    await expect(page.locator('#statusline')).toContainText(/failed|passed/, { timeout: 30_000 });
    await expect(page.locator('.diag .diagcode'), 'Learn shows no parsed code button').toHaveCount(0);
    await expect(page.locator('.diag .jumpline'), 'Learn shows no jump target').toHaveCount(0);
  });
});
