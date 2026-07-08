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

  // B3 ("remember your place"): a long chapter used to snap back to the top every
  // time you reopened it — so leaving to look something up lost your spot. Position
  // is now persisted per chapter (localStorage, fully offline). This pins that a
  // plain reopen resumes where you left off, while an explicit anchor still wins.
  test('a chapter resumes your scroll position on a plain reopen', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 }); // guarantees the chapter scrolls
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    const ids = await tocIds(page);
    const chId = ids[2] || ids[1];

    // Open the chapter and scroll partway down; the save is debounced (~250ms).
    await page.evaluate((id) => renderBook(id), chId);
    await page.waitForSelector('#docview .bookbody');
    const target = await page.evaluate(() => {
      const dv = document.querySelector('#docview');
      const max = dv.scrollHeight - dv.clientHeight;
      const t = Math.max(80, Math.min(500, Math.round(max * 0.5)));
      dv.scrollTop = t;
      dv.dispatchEvent(new Event('scroll'));
      return t;
    });
    expect(target, 'the chapter is tall enough to scroll').toBeGreaterThan(40);
    await page.waitForFunction((id) => localStorage.getItem('ts-book-pos:' + id) !== null, chId);

    // Leave to the contents list, then reopen the SAME chapter with no anchor/term.
    await page.evaluate(() => renderBook(null));
    await page.waitForSelector('#docview .booktoc');
    await page.evaluate((id) => renderBook(id), chId);
    await page.waitForSelector('#docview .bookbody');

    // It restores the saved offset (restore runs after an awaited linkify, so poll).
    await page.waitForFunction(
      (want) => Math.abs((document.querySelector('#docview')?.scrollTop || 0) - want) <= 3,
      target,
      { timeout: 4000 },
    );
    const restored = await page.evaluate(() => document.querySelector('#docview').scrollTop);
    expect(restored, 'resumed where we left off, not the top').toBeGreaterThan(10);
  });

  // Long chapters (some are 15k+ px tall) now offer a subtle "↑ Top" button once you
  // scroll down. It lives on <body> (viewport-anchored, since the .docview's fade
  // transform makes it a containing block), hides near the bottom so it never covers
  // the prev/next footer, and resets when you leave the view.
  test('a long chapter shows a "↑ Top" button that returns you to the top', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    await page.evaluate(() => showView('book', 'ch04-01-what-is-ownership'));
    await page.waitForSelector('#docview .bookbody');
    await page.waitForFunction(() => {
      const dv = document.querySelector('#docview');
      return dv && dv.scrollHeight - dv.clientHeight > 900;
    }, null, { timeout: 5000 });

    const btn = page.locator('#totop');
    // Hidden at the top.
    await page.evaluate(() => { const dv = document.querySelector('#docview'); dv.scrollTop = 0; dv.dispatchEvent(new Event('scroll')); });
    await expect(btn).not.toHaveClass(/\bshow\b/);
    // Scrolled well down → it appears, anchored to the viewport's bottom-right corner.
    await page.evaluate(() => { const dv = document.querySelector('#docview'); dv.scrollTop = 1200; dv.dispatchEvent(new Event('scroll')); });
    await expect(btn).toHaveClass(/\bshow\b/);
    const box = await btn.boundingBox();
    expect(box.x + box.width, 'near the right edge').toBeGreaterThan(300);
    expect(box.x + box.width, 'within the viewport').toBeLessThanOrEqual(390);
    expect(box.y, 'near the bottom').toBeGreaterThan(700);
    expect(box.y + box.height, 'within the viewport').toBeLessThanOrEqual(844);
    // Clicking returns to the top.
    await btn.click();
    await page.waitForFunction(() => (document.querySelector('#docview')?.scrollTop || 0) < 5, null, { timeout: 2000 });
    // Leaving the reading view hides it again (reset in freshDocview).
    await page.evaluate(() => showView('lessons'));
    await expect(btn).not.toHaveClass(/\bshow\b/);
  });
});
