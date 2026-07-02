// The Tutor USED to be a single hard-coded question that never changed — Paul:
// "there is no answer to the question it asks; the questions should disappear
// once it's satisfied." It now reacts to the last run. Guard both the reaction
// and the charter rule that it stays guide-only (never types the fix).
const { test, expect } = require('@playwright/test');

test('the Tutor reacts to the run and stays guide-only', async ({ page }) => {
  await page.goto('/');
  const tutor = page.locator('#tutorbox');
  // Pre-run: the generic predict prompt.
  await expect(tutor).toContainText(/prediction say/i);

  // Assist mode (no predict gate) + a definite compile error (E0384).
  await page.locator('.modesw [data-mode="assist"]').click();
  await page.locator('.editor textarea').fill('fn main() {\n    let x = 5;\n    x = 6;\n    println!("{x}");\n}');
  await page.locator('#runbtn').click();

  // It now names the real error and asks a contextual question — but never the fix.
  await expect(tutor).toContainText(/E0384/, { timeout: 30_000 });
  await expect(tutor).toContainText(/never type your fix/i);
});
