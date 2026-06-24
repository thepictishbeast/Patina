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
  await expect(page.locator('#editorCode')).toBeVisible();
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
    // The editor selection should now cover exactly that 1-based source line.
    const sel = await page.locator('#editorCode').evaluate((el) => {
      const before = el.value.slice(0, el.selectionStart);
      return {
        selStartLine: before.split('\n').length, // 1-based line the caret starts on
        selected: el.value.slice(el.selectionStart, el.selectionEnd),
        actualLine: el.value.split('\n')[before.split('\n').length - 1],
      };
    });
    expect(sel.selStartLine, 'caret jumped to the diagnostic line').toBe(lineNo);
    expect(sel.selected, 'the offending line is selected (caret moved there)').toBe(sel.actualLine);
    expect(sel.selected.length, 'a non-empty line was selected').toBeGreaterThan(0);
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
    await expect(page.locator('#editorCode')).toBeVisible();
    await page.locator('.pbtn[data-pred="fails"]').click();
    await page.locator('#runbtn').click();
    await expect(page.locator('#statusline')).toContainText(/failed|passed/, { timeout: 30_000 });
    await expect(page.locator('.diag .diagcode'), 'Learn shows no parsed code button').toHaveCount(0);
    await expect(page.locator('.diag .jumpline'), 'Learn shows no jump target').toHaveCount(0);
  });
});
