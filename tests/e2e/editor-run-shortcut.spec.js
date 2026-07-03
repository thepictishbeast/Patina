// E2E for the Ctrl/Cmd+Enter "Run from the editor" keyboard shortcut. Running is
// the core action, so the shortcut works in EVERY tier — but it goes through the
// same runOp() path, so the Learn predict-first gate still applies (no free runs).
const { test, expect } = require('@playwright/test');

const EX = 'basics/01_immutable_assign'; // compile-error exercise (E0384), deterministic

test.describe('editor Run shortcut (Ctrl/Cmd+Enter)', () => {
  test('Assist: Ctrl+Enter from the editor runs the exercise', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });
    await page.goto('/');
    await expect(page.locator('#editorCode')).toBeVisible();
    await page.locator('#modesw button[data-mode="assist"]').click(); // no predict-gate in Assist
    await page.locator('#editorCode').click();
    await page.locator('#editorCode').press('Control+Enter');
    await expect(page.locator('#statusline'), 'the run lands from the keyboard')
      .toContainText(/failed|passed/, { timeout: 30_000 });
    // a real diagnostic surfaced (proves Run actually executed, not a no-op)
    await expect(page.locator('.diag .diagcode')).toContainText(/E0\d{3}/, { timeout: 30_000 });
  });

  test('Learn: Ctrl+Enter still honors the predict-first gate', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });
    await page.goto('/'); // Learn is the default mode
    await expect(page.locator('#editorCode')).toBeVisible();
    await page.locator('#editorCode').click();
    await page.locator('#editorCode').press('Control+Enter'); // no prediction locked yet
    await expect(page.locator('#statusline'), 'gated, not run')
      .toContainText(/predict/i, { timeout: 10_000 });
    // now lock a prediction, then the shortcut runs
    await page.locator('.pbtn[data-pred="fails"]').click();
    await page.locator('#editorCode').click();
    await page.locator('#editorCode').press('Control+Enter');
    await expect(page.locator('#statusline'), 'runs once the guess is locked')
      .toContainText(/failed|passed/, { timeout: 30_000 });
  });

  test('Enter in the error-code box runs — but only once the outcome is locked', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });
    await page.goto('/'); // Learn is the default mode
    await expect(page.locator('#editorCode')).toBeVisible();
    // Type a code guess but pick NO outcome: Enter must be a no-op (gate holds).
    await page.locator('#predcode').fill('E0384');
    await page.locator('#predcode').press('Enter');
    await expect(page.locator('#runbtn'), 'still gated with no outcome picked').toBeDisabled();
    // Lock the outcome; now Enter from the same box fires the run.
    await page.locator('.pbtn[data-pred="fails"]').click();
    await page.locator('#predcode').press('Enter');
    await expect(page.locator('#statusline'), 'Enter finishes the prediction gesture')
      .toContainText(/failed|passed/, { timeout: 30_000 });
  });

  test('Ctrl+Enter does not insert a newline in the editor (Dev)', async ({ page, request }) => {
    await request.post('/api/select', { data: { id: EX, force: true } });
    await page.goto('/');
    await page.locator('#modesw button[data-mode="dev"]').click();
    await page.locator('#editorCode').evaluate((el) => {
      el.value = 'fn main() {}';
      el.selectionStart = el.selectionEnd = el.value.length;
      el.focus();
    });
    await page.locator('#editorCode').press('Control+Enter');
    const v = await page.locator('#editorCode').evaluate((el) => el.value);
    expect(v, 'no stray newline inserted by the shortcut').toBe('fn main() {}');
  });
});
