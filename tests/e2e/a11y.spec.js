// Accessibility audit of the web surface (the "a11y" half of #12), via axe-core
// driven through the same Playwright Chromium. Hard gate: zero CRITICAL or SERIOUS
// WCAG 2 A/AA violations. Moderate/minor issues are logged as findings, not failures.
const { test, expect } = require('@playwright/test');
const AxeBuilder = require('@axe-core/playwright').default;

test('a11y: no critical or serious WCAG 2 A/AA violations on the main view', async ({ page }) => {
  // Assert against the settled UI, not a transient render frame: opt into
  // reduced motion (which the page now honours) so the fade-in animation is
  // collapsed and axe never composites a mid-fade opacity into its contrast
  // math. This mirrors what a reduced-motion user actually sees.
  await page.emulateMedia({ reducedMotion: 'reduce' });
  await page.goto('/');
  await page.locator('#exTitle').waitFor();
  await page.locator('.exlist .ex').first().waitFor(); // exercise list populated

  const { violations } = await new AxeBuilder({ page })
    .withTags(['wcag2a', 'wcag2aa'])
    .analyze();

  for (const v of violations) {
    console.log(`  a11y [${v.impact}] ${v.id} — ${v.help} (${v.nodes.length} node(s))`);
    if (v.impact === 'critical' || v.impact === 'serious') {
      for (const n of v.nodes.slice(0, 8)) {
        console.log(`      @ ${n.target}  ::  ${(n.html || '').replace(/\s+/g, ' ').slice(0, 100)}`);
      }
    }
  }
  const blocking = violations.filter((v) => v.impact === 'critical' || v.impact === 'serious');
  expect(blocking.map((v) => v.id), 'no critical/serious a11y violations').toEqual([]);
});
