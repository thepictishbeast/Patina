// The in-app Book reader is a linear reading surface, but a chapter used to offer
// only a "← contents" link — finishing one meant bouncing back to the ToC to find
// the next. This pins the prev/next chapter nav: it must reflect the real ToC
// order, actually navigate, and correctly omit the missing side at the ends.
const { test, expect } = require('@playwright/test');

async function tocIds(page) {
  await page.evaluate(() => showView('book'));
  await page.waitForSelector('#docview .booktoc .bookjump');
  return page.evaluate(() =>
    [...document.querySelectorAll('#docview .booktoc .bookjump')].map((a) => a.dataset.ch));
}

test.describe('Book reader: prev/next chapter navigation', () => {
  test('a middle chapter links its real ToC neighbours and next actually moves', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    const ids = await tocIds(page);
    expect(ids.length, 'the book has several chapters').toBeGreaterThan(2);

    // Open the second chapter — it must have BOTH a prev and a next.
    await page.evaluate((id) => renderBook(id), ids[1]);
    await page.waitForSelector('#docview .booknav');
    const prev = page.locator('#docview .booknav-prev');
    const next = page.locator('#docview .booknav-next');
    await expect(prev).toHaveAttribute('data-ch', ids[0]);
    await expect(next).toHaveAttribute('data-ch', ids[2]);
    // The direction labels + a (possibly truncated) title are present.
    await expect(prev).toContainText('previous');
    await expect(next).toContainText('next');

    // Clicking next advances to ids[2] (shown in the back-line chapter id).
    await next.click();
    await page.waitForFunction(
      (want) => document.querySelector('#docview .bookback')?.parentElement?.querySelector('small')?.textContent === want,
      ids[2],
    );
    // And the new chapter's prev now points back to where we came from.
    await expect(page.locator('#docview .booknav-prev')).toHaveAttribute('data-ch', ids[1]);
  });

  test('the first chapter has no prev and the last has no next', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    const ids = await tocIds(page);

    await page.evaluate((id) => renderBook(id), ids[0]);
    await page.waitForSelector('#docview .booknav');
    await expect(page.locator('#docview .booknav-prev')).toHaveCount(0);
    await expect(page.locator('#docview .booknav-next')).toHaveAttribute('data-ch', ids[1]);

    await page.evaluate((id) => renderBook(id), ids[ids.length - 1]);
    await page.waitForSelector('#docview .booknav');
    await expect(page.locator('#docview .booknav-next')).toHaveCount(0);
    await expect(page.locator('#docview .booknav-prev')).toHaveAttribute('data-ch', ids[ids.length - 2]);
  });
});
