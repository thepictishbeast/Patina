// The lesson DRAWER (#lessonDrawerBody) is the "read a lesson while you code"
// panel that slides over the Practice tab. It reuses the same rendered markup as
// the Learn tab (inline `code`, code blocks, ▷ Run + ⧉ Copy toolkit, glossary
// tap-to-define), but it is NOT inside `.docview` — so the rich-text CSS has to
// list `.ldrawer-body` explicitly. That's fragile: a stray refactor of the
// `.docview …` selectors would silently strip the drawer back to unstyled prose
// (inline code with a transparent background + the browser's default monospace,
// naked code blocks, default-chrome buttons). This spec pins the styling down.
const { test, expect } = require('@playwright/test');

async function openDrawer(page) {
  await page.goto('/');
  await page.waitForSelector('#exTitle');
  await page.evaluate(() => openLessonDrawer('01-bindings-and-immutability', 'Bindings'));
  await page.waitForSelector('#lessonDrawerBody .lessonbody');
  await page.waitForTimeout(300); // async terms + linkify + wire toolkit
}

test.describe('Lesson drawer: rich-text parity with the Learn tab', () => {
  test('inline code is a styled chip (real bg + monospace), never plain prose', async ({ page }) => {
    await openDrawer(page);
    const styled = await page.evaluate(() => {
      const body = document.getElementById('lessonDrawerBody');
      const code = body.querySelector('p code, :not(pre) > code');
      if (!code) return { found: false };
      const cs = getComputedStyle(code);
      const bg = cs.backgroundColor;
      // "transparent" / rgba(...,0) means the .docview rule never reached us.
      const opaque = bg !== 'transparent' && !/,\s*0\s*\)$/.test(bg) && bg !== 'rgba(0, 0, 0, 0)';
      return {
        found: true,
        opaque,
        mono: /mono|Fira|Cascadia|Menlo|Consol|monospace/i.test(cs.fontFamily),
      };
    });
    expect(styled.found, 'the lesson has inline code to style').toBe(true);
    expect(styled.opaque, 'inline code has a non-transparent chip background').toBe(true);
    expect(styled.mono, 'inline code uses a monospace font').toBe(true);
  });

  test('code blocks and the ▷ Run / ⧉ Copy toolkit render (styled), matching the Learn tab', async ({ page }) => {
    await openDrawer(page);
    const info = await page.evaluate(() => {
      const body = document.getElementById('lessonDrawerBody');
      const pre = body.querySelector('pre');
      const preBg = pre ? getComputedStyle(pre).backgroundColor : null;
      const preBorder = pre ? getComputedStyle(pre).borderTopWidth : null;
      const runBtn = body.querySelector('.coderun-btn');
      const runRadius = runBtn ? getComputedStyle(runBtn).borderRadius : null;
      const copy = body.querySelector('.codecopy');
      const copyPos = copy ? getComputedStyle(copy).position : null;
      return {
        hasPre: !!pre, preBg, preBorder,
        hasRun: !!runBtn, runRadius,
        hasCopy: !!copy, copyPos,
      };
    });
    expect(info.hasPre, 'the lesson has a code block').toBe(true);
    expect(info.preBg, 'code block has a filled background (not transparent)').not.toBe('rgba(0, 0, 0, 0)');
    expect(info.preBorder, 'code block has a border').not.toBe('0px');
    expect(info.hasRun, 'a ▷ Run button is wired into the drawer').toBe(true);
    expect(info.runRadius, 'the Run button carries the styled pill radius').toBe('8px');
    expect(info.hasCopy, 'a ⧉ Copy button is wired into the drawer').toBe(true);
    expect(info.copyPos, 'the Copy button is absolutely positioned in the corner').toBe('absolute');
  });

  test('glossary tap-to-define works inside the drawer, and never inside code', async ({ page }) => {
    await openDrawer(page);
    const defs = page.locator('#lessonDrawerBody .glossdef');
    await expect(defs.first(), 'at least one term got linked in the drawer').toBeVisible();
    const inCode = await page.evaluate(() =>
      [...document.querySelectorAll('#lessonDrawerBody .glossdef')]
        .filter((el) => el.closest('code,pre,a,h1,h2,h3,h4,h5,h6')).length);
    expect(inCode, 'no term linked inside code/link/heading').toBe(0);
    await defs.first().click();
    const pop = page.locator('#lessonDrawerBody .glosspop');
    await expect(pop).toBeVisible();
    await expect(pop).not.toBeEmpty();
  });
});
