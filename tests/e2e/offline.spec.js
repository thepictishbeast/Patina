// Charter promise #1: Tempered Studio teaches Rust FULLY OFFLINE — no internet,
// no CDN, no account. This guard exercises the main study surfaces and FAILS if
// the app makes a single request to anything but the local server (data:/blob:
// are in-page, not network). If someone later adds a Google-Fonts <link> or a
// CDN <script>, this catches it before it ships and silently breaks offline use.
const { test, expect } = require('@playwright/test');

test('the app makes ZERO external network requests (fully offline)', async ({ page }) => {
  const external = [];
  const isLocal = (u) =>
    /^https?:\/\/(127\.0\.0\.1|localhost)([:/]|$)/.test(u) ||
    /^(data|blob|about):/.test(u);
  page.on('request', (req) => { if (!isLocal(req.url())) external.push(req.url()); });

  await page.goto('/');

  // Exercise every surface that could pull a resource.
  await page.locator('.tab[data-view="lessons"]').click();
  await page.locator('.lessonjump').first().click();            // renders a lesson (+ its book/cheat links)
  await page.locator('.tab[data-view="quizzes"]').click();
  await page.locator('.quizjump').first().click();              // renders a quiz
  await page.evaluate(() => showView('glossary'));              // 116-term glossary
  await expect(page.locator('.gloss-term').first()).toBeVisible();
  await page.evaluate(() => showView('book'));                  // Rust Book TOC
  await page.evaluate(() => showView('library'));               // offline PDF library list (fetches manifest)
  await expect(page.locator('.libopen').first()).toBeVisible();

  await page.waitForTimeout(400); // let any late fetch fire
  expect(external, `external requests leaked: ${external.join(', ')}`).toHaveLength(0);
});
