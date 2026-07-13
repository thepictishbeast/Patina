// E2E for the Dev tier's LIVE diagnostics: as the learner pauses, the editor
// POSTs the buffer to /api/diagnostics and squiggles the analyzer's findings
// inline — WITHOUT running the code. This locks the GUI wiring (debounce → POST →
// parsed panel + gutter mark, and the clean/tier-gate transitions). The endpoint's
// real behaviour against a language server is covered by the rpro-lsp / rpro-serve
// tests; here we stub it so the wiring is deterministic and independent of whether
// a server is installed in the test environment.
const { test, expect } = require('@playwright/test');

const EX = 'basics/01_immutable_assign'; // any exercise with a multi-line starter

test.describe('Dev tier: live diagnostics (analyzer, no run)', () => {
  test('broken code squiggles inline, then clears when the analyzer reports clean', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });

    // The stub flips between a type-error result and a clean one.
    let analyzerState = 'broken';
    await page.route('**/api/diagnostics', async (route) => {
      const body = analyzerState === 'broken'
        ? { available: true, diagnostics: [
            { code: 'E0308', level: 'Error', message: 'mismatched types', span: { file: 'f', line: 2, col: 5 } },
          ] }
        : { available: true, diagnostics: [] };
      await route.fulfill({ status: 200, contentType: 'application/json', body: JSON.stringify(body) });
    });

    await page.goto('/');
    await expect(page.locator('.cm-content')).toBeVisible();
    // Mode switcher lives in the ⋯ menu; Dev is the live-analysis tier.
    await page.locator('#menuBtn').click();
    await page.locator('#modesw button[data-mode="dev"]').click();

    // An edit is the debounced trigger; typing a char fires the live POST.
    await page.locator('.cm-content').click();
    await page.locator('.cm-content').press('x');

    // The parsed panel shows the analyzer's code and the gutter squiggles the
    // reported line — proof the wiring rendered live diagnostics without a run.
    await expect(page.locator('.diag .diagcode')).toContainText('E0308', { timeout: 10_000 });
    await expect(page.locator('.cm-lineNumbers .cm-gutterElement.cm-errline').first()).toBeVisible();

    // Analyzer now reports clean → the panel resolves to "no problems" and the
    // squiggle clears (never leaves the stale mark behind).
    analyzerState = 'clean';
    await page.locator('.cm-content').press('y');
    await expect(page.locator('#diag')).toContainText('no problems', { timeout: 10_000 });
    await expect(page.locator('.cm-lineNumbers .cm-gutterElement.cm-errline')).toHaveCount(0);
  });

  test('Learn tier does NOT call the analyzer (errors are read by hand)', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });

    // If the Learn tier ever POSTs, this fails the test loudly.
    let called = false;
    await page.route('**/api/diagnostics', async (route) => {
      called = true;
      await route.fulfill({ status: 200, contentType: 'application/json',
        body: JSON.stringify({ available: true, diagnostics: [] }) });
    });

    await page.goto('/'); // Learn is the default tier
    await expect(page.locator('.cm-content')).toBeVisible();
    await page.locator('.cm-content').click();
    await page.locator('.cm-content').press('x'); // an edit — but Learn must not analyze
    // Give any (erroneous) debounced POST well over its 700 ms window to fire.
    await page.waitForTimeout(1500);
    expect(called, 'Learn tier must not call /api/diagnostics').toBe(false);
  });
});
