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
  // rustc emits the primary E0308 AND a cascading E0282 (type-annotations-needed),
  // so several .diagcode spans render — assert on the FIRST (the primary), matching
  // how the rest of this file targets .first(). A bare locator here trips
  // Playwright strict mode (>1 match) and is NOT a missing-diagnostic failure.
  await expect(page.locator('.diag .diagcode').first()).toContainText(/E0\d{3}/, { timeout: 30_000 });
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

  test('the gutter marks the diagnostic line red — and an edit clears it', async ({ page, request }) => {
    await runInAssist(page, request);
    const lineNo = parseInt(await page.locator('.diag .jumpline').first().getAttribute('data-line'), 10);
    // Jump so the flagged line is scrolled into view (CM6 virtualises the gutter),
    // then its gutter line-number element carries the .cm-errline class.
    await page.locator('.diag .jumpline').first().click();
    const marked = await page.evaluate((n) => {
      const el = [...document.querySelectorAll('.cm-lineNumbers .cm-gutterElement')]
        .find((e) => e.textContent.trim() === String(n));
      return el ? el.classList.contains('cm-errline') : null;
    }, lineNo);
    expect(marked, 'the flagged line number is marked in the gutter').toBe(true);
    // Editing the buffer invalidates the marks (they describe the OLD code).
    await page.evaluate(() => window.__cm.set('// edited\n' + window.__cm.get()));
    await expect(page.locator('.cm-lineNumbers .cm-errline'), 'edit clears stale marks').toHaveCount(0);
  });

  test('Learn never marks the gutter (by-hand stays by-hand)', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });
    await page.goto('/'); // Learn is the default mode
    await expect(page.locator('.cm-content')).toBeVisible();
    await page.locator('.pbtn[data-pred="fails"]').click();
    await page.locator('#runbtn').click();
    await expect(page.locator('#statusline')).toContainText(/failed|passed/, { timeout: 30_000 });
    await expect(page.locator('.cm-lineNumbers .cm-errline')).toHaveCount(0);
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
