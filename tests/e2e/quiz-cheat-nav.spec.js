// Quizzes and cheatsheets are linear reference surfaces like the Book, but each
// item used to offer only a "← all …" link — finishing one meant bouncing back to
// the list. This pins the prev/next footer (shared prevNextNav / .booknav) for both:
// it must reflect real list order, actually navigate, and omit the missing side at
// the ends.
const { test, expect } = require('@playwright/test');

function navSuite(label, view, jumpSel) {
  test.describe(`${label}: prev/next navigation`, () => {
    async function ids(page) {
      await page.evaluate((v) => showView(v), view);
      await page.waitForSelector(`#docview ${jumpSel}`);
      return page.evaluate((s) => [...document.querySelectorAll(`#docview ${s}`)].map((a) => a.dataset.id), jumpSel);
    }

    test('a middle item links its real neighbours and next actually moves', async ({ page }) => {
      await page.goto('/');
      await page.waitForSelector('#exTitle');
      const list = await ids(page);
      expect(list.length, `${label} has several items`).toBeGreaterThan(2);

      await page.locator(`#docview ${jumpSel}`).nth(1).click();
      await page.waitForSelector('#docview .booknav');
      await expect(page.locator('#docview .booknav-prev')).toHaveAttribute('data-id', list[0]);
      await expect(page.locator('#docview .booknav-next')).toHaveAttribute('data-id', list[2]);

      await page.locator('#docview .booknav-next').click();
      // The new page's prev now points back to the item we came from (list[1]).
      await expect(page.locator('#docview .booknav-prev')).toHaveAttribute('data-id', list[1]);
    });

    test('first item has no prev, last has no next', async ({ page }) => {
      await page.goto('/');
      await page.waitForSelector('#exTitle');
      const list = await ids(page);

      await page.locator(`#docview ${jumpSel}`).first().click();
      await page.waitForSelector('#docview .booknav');
      await expect(page.locator('#docview .booknav-prev')).toHaveCount(0);
      await expect(page.locator('#docview .booknav-next')).toHaveAttribute('data-id', list[1]);

      await ids(page); // back to the list
      await page.locator(`#docview ${jumpSel}`).nth(list.length - 1).click();
      await page.waitForSelector('#docview .booknav');
      await expect(page.locator('#docview .booknav-next')).toHaveCount(0);
      await expect(page.locator('#docview .booknav-prev')).toHaveAttribute('data-id', list[list.length - 2]);
    });
  });
}

navSuite('Quizzes', 'quizzes', '.quizjump');
navSuite('Cheatsheets', 'cheatsheets', '.cheatjump');
