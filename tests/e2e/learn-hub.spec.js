// E2E for the Learn hub's unified "search everything" box: one place to find a
// concept across lessons, the Rust Book, cheatsheets, and the glossary (server
// ?q= for the first three, client-filtered glossary), each hit with a snippet.
// Fully offline (every endpoint is mirrored on mobile). The per-view searches
// stay; this is the global discovery entry.
const { test, expect } = require('@playwright/test');

test.describe('Learn hub: search everything', () => {
  test('finds across categories, opens a hit, and clears back to the cards', async ({ page }) => {
    await page.goto('/');
    await page.evaluate(() => showView('learn'));
    await page.waitForSelector('#hubsearch');
    await expect(page.locator('#hubbody'), 'the card grid shows before searching').toBeVisible();
    await expect(page.locator('#hubresults')).toBeHidden();

    // A concept in the material → ranked hits replace the cards, spanning >1 category.
    await page.locator('#hubsearch').fill('borrow');
    await expect.poll(() => page.locator('#hubresults .hubhit').count(), { timeout: 6000 }).toBeGreaterThan(0);
    await expect(page.locator('#hubbody'), 'cards hide while results show').toBeHidden();
    expect(await page.locator('#hubresults .hubsec').count(), 'more than one category matched').toBeGreaterThan(1);
    // each hit carries a title link (lessons/book/cheats also carry a snippet)
    await expect(page.locator('#hubresults .hubhit').first()).not.toHaveText('');

    // Clicking a lesson hit navigates into that lesson.
    await page.locator('#hubresults .hubhit[data-kind="lesson"]').first().click();
    await expect(page.locator('.lessonback')).toBeVisible();

    // A nonsense query → an explicit no-match note; clearing restores the cards.
    await page.evaluate(() => showView('learn'));
    await page.waitForSelector('#hubsearch');
    await page.locator('#hubsearch').fill('zzzznotawordzz');
    await expect(page.locator('.hubnomatch')).toBeVisible();
    await page.locator('#hubsearch').fill('');
    await expect(page.locator('#hubbody')).toBeVisible();
    await expect(page.locator('#hubresults')).toBeHidden();
  });
});
