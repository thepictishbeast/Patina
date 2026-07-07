// Regression guard for the in-app "← Studio" back button added to the vendored
// pdf.js viewer. The Library opens each book as a FULL-PAGE nav to viewer.html,
// so without this button the only way back was the browser / Android back button
// (Paul flagged the missing close control).
//
// The viewer's CSP is `script-src 'self'` (no 'unsafe-inline'), so the click
// handler CANNOT be an inline onclick — it lives in tempered-back.js. This guard
// asserts BOTH the button and its wiring script survive (e.g. a future pdf.js
// re-vendor could clobber the viewer.html edit). The click→history.back()
// behaviour itself is standard and is verified live at 390×844.
const { test, expect } = require('@playwright/test');

test.describe('PDF library: in-app back button', () => {
  test('the vendored viewer exposes a CSP-wired "← Studio" back button', async ({ page }) => {
    await page.goto('/vendor/pdfjs/web/viewer.html');
    const back = page.locator('#temperedBack');
    await expect(back, 'the in-app back button is present in the viewer toolbar').toBeVisible();
    await expect(back).toHaveText(/Studio/);
    // The CSP-safe handler script is BOTH referenced by the page AND served
    // (inline onclick would be silently blocked by script-src 'self').
    const wired = await page.evaluate(async () => {
      const referenced = !!document.querySelector('script[src="tempered-back.js"]');
      const served = (await fetch('tempered-back.js')).ok;
      const noInlineHandler = !document.getElementById('temperedBack').getAttribute('onclick');
      return { referenced, served, noInlineHandler };
    });
    expect(wired.referenced, 'tempered-back.js is referenced').toBe(true);
    expect(wired.served, 'tempered-back.js is served (200)').toBe(true);
    expect(wired.noInlineHandler, 'no dead CSP-blocked inline onclick remains').toBe(true);
  });
});
