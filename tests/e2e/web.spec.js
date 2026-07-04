// End-to-end browser test of the Tempered Studio web flow against a live
// rpro-serve. Covers: real exercise renders, predict-first integrity (no answer
// leak), and Run compiling the real toolchain + surfacing the error code in
// Diagnostics. Run with rpro-serve already up (see tests/e2e/README.md).
const { test, expect } = require('@playwright/test');

test.describe('Tempered Studio web flow', () => {
  test('renders the real current exercise', async ({ page, request }) => {
    const cur = await (await request.get('/api/current')).json();
    expect(cur.code, '/api/current returns exercise code').toBeTruthy();

    await page.goto('/');
    await expect(page.locator('#exTitle'), 'exercise title is populated').toContainText(/\S/);
    await expect(page.locator('#editorCode'), 'editable code pane is shown').toBeVisible();
    // the real exercise source is loaded into the editable pane (every exercise has fn main)
    const editorText = await page.locator('#editorCode')
      .evaluate((el) => el.value ?? el.innerText ?? el.textContent ?? '');
    expect(editorText, 'editor holds the real exercise source').toContain('fn main');
  });

  test('predict-first integrity: the answer never leaks to the client', async ({ page, request }) => {
    await page.goto('/');
    const html = await page.content();
    expect(html, 'page HTML has no solution_outline').not.toContain('solution_outline');
    expect(html, 'page HTML has no expected_error_code').not.toContain('expected_error_code');
    for (const ep of ['/api/current', '/api/exercises']) {
      const body = await (await request.get(ep)).text();
      expect(body, `${ep} has no solution_outline`).not.toContain('solution_outline');
      expect(body, `${ep} has no expected_error_code`).not.toContain('expected_error_code');
    }
  });

  test('tier differentiation: Learn withholds the parsed code, Assist surfaces it', async ({ page, request }) => {
    // Pin a COMPILE-ERROR exercise so the parsed-code path is deterministic: some
    // exercises compile but panic at run time (runtime-outcome model) and so carry
    // no E-code in Diagnostics. 01_immutable_assign fails to compile (E0384).
    // MUST land: force bypasses Locked but NOT Done (409 "reset to practise") —
    // against a polluted store this select silently failed and the test ran some
    // other exercise (a runtime-panic one has no E-code). Assert the pin so a
    // state problem fails HERE, not as a misleading Diagnostics "flake".
    const sel = await request.post('/api/select', { data: { id: 'basics/01_immutable_assign', force: true } });
    expect(sel.ok(), 'pin-select basics/01 must succeed (run against a FRESH store — scripts/e2e.sh)').toBeTruthy();
    await page.goto('/');
    await expect(page.locator('#exTitle')).toContainText(/\S/);
    // Learn mode (the default) is "by-hand errors only" (Paul's tier decision): it is
    // predict-first (Run gated until a compiles/fails guess is locked, 158c997), and
    // after Run the parsed Diagnostics panel is WITHHELD — the real rustc error code
    // lives only in the terminal, to be read by hand. Lock the honest guess for the
    // seeded starter (01_immutable_assign FAILS to compile), then Run.
    await page.locator('.pbtn[data-pred="fails"]').click();
    await page.locator('#runbtn').click();
    await expect(page.locator('#statusline'), 'the run lands')
      .toContainText(/failed|passed/, { timeout: 25_000 });
    await expect(page.locator('#diag'), 'Learn withholds the parsed error code (by hand)')
      .not.toContainText(/E0\d{3}/);
    // Switch to Assist: the SAME last run re-renders with the parsed Diagnostics panel,
    // surfacing the real rustc error code — no re-run needed (the tier re-render path).
    await page.locator('#menuBtn').click(); // open the ⋯ menu (mode switcher lives there now)
    await page.locator('#modesw button[data-mode="assist"]').click();
    await expect(page.locator('#diag'), 'Assist surfaces the parsed rustc error code')
      .toContainText(/E0\d{3}/, { timeout: 25_000 });
  });
});
