// E2E for the navigation surface added across recent ticks — it had ZERO
// dedicated coverage even though it's now central to the UX:
//   1. the ⋯ More menu (reference surfaces folded out of the tab bar),
//   2. the lesson → phase-cheatsheet companion link (6th surface cross-link),
//   3. the ? keyboard-shortcuts overlay (and the regression that it must NOT
//      swallow clicks when closed — an author display:flex once overrode the
//      [hidden] attribute and covered the whole viewport).
const { test, expect } = require('@playwright/test');

test.describe('⋯ More menu (reference surfaces)', () => {
  test('the bar shows 3 primary tabs; reference surfaces live in the menu', async ({ page }) => {
    await page.goto('/');
    // Exactly the active-learning surfaces are primary tabs.
    await expect(page.locator('.tab[data-view]')).toHaveCount(3);
    await expect(page.locator('.tab[data-view="practice"]')).toBeVisible();
    await expect(page.locator('.tab[data-view="lessons"]')).toBeVisible();
    await expect(page.locator('.tab[data-view="quizzes"]')).toBeVisible();
    // The reference surfaces are NOT tabs — they're menu items.
    await expect(page.locator('.tab[data-view="book"]')).toHaveCount(0);
    await expect(page.locator('#moremenu-list .menuitem')).toHaveCount(4);
  });

  test('opening the menu and choosing Book navigates + closes + marks the trigger active', async ({ page }) => {
    await page.goto('/');
    const list = page.locator('#moremenu-list');
    await expect(list).toBeHidden();
    await page.locator('#moreBtn').click();
    await expect(list).toBeVisible();
    await expect(page.locator('#moreBtn')).toHaveAttribute('aria-expanded', 'true');
    await page.locator('#moremenu-list .menuitem[data-view="book"]').click();
    // navigated to the doc view, menu closed, trigger reflects the active reference view
    await expect(list).toBeHidden();
    await expect(page.locator('#docview')).toBeVisible();
    await expect(page.locator('#moreBtn')).toHaveClass(/active/);
    await expect(page.locator('.menuitem[data-view="book"]')).toHaveAttribute('aria-current', 'true');
  });

  test('Escape closes the open menu', async ({ page }) => {
    await page.goto('/');
    await page.locator('#moreBtn').click();
    await expect(page.locator('#moremenu-list')).toBeVisible();
    await page.locator('#moremenu-list .menuitem').first().focus();
    await page.keyboard.press('Escape');
    await expect(page.locator('#moremenu-list')).toBeHidden();
  });
});

test.describe('lesson → phase cheatsheet cross-link', () => {
  test('a lesson carries a working "this phase\'s cheat sheet" companion link', async ({ page }) => {
    await page.goto('/');
    await page.locator('.tab[data-view="lessons"]').click();
    // open the first lesson from the list
    await expect(page.locator('.lessonjump').first()).toBeVisible();
    await page.locator('.lessonjump').first().click();
    // the companion cheat link is present and targets a real phase id
    const cheat = page.locator('.cheatlink');
    await expect(cheat).toBeVisible();
    await expect(cheat).toHaveAttribute('data-cheat', /^(phase\d|phase6-generics|tooling)$/);
    // clicking it opens a single cheatsheet (the "← all cheatsheets" back link appears)
    await cheat.click();
    await expect(page.locator('.cheatback')).toBeVisible();
    await expect(page.locator('#moreBtn')).toHaveClass(/active/); // cheatsheets is a menu view
  });
});

test.describe('? keyboard-shortcuts overlay', () => {
  test('? opens the overlay and Escape closes it; it never blocks clicks when closed', async ({ page }) => {
    await page.goto('/');
    const help = page.locator('#keyshelp');
    await expect(help).toBeHidden();
    // Regression guard: a closed overlay must not intercept clicks. If it covered
    // the viewport (the old [hidden]-override bug), this tab click would time out.
    await page.locator('.tab[data-view="lessons"]').click();
    await expect(page.locator('#docview')).toBeVisible();
    // open via the '?' shortcut, then close with Escape. type('?') reliably
    // yields a keydown with key === '?' (Shift+/ can vary by mapping).
    await page.keyboard.type('?');
    await expect(help).toBeVisible();
    await expect(help).toContainText(/shortcut/i);
    await page.keyboard.press('Escape');
    await expect(help).toBeHidden();
  });
});
