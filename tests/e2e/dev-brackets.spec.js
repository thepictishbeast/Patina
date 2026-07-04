// E2E for the Dev-tier editor convenience: bracket auto-close — and the charter
// guarantee that Learn AND Assist leave the editor default (no assists outside
// Dev). The editor is now CodeMirror 6; Dev enables its closeBrackets extension,
// Learn/Assist don't. We drive real key events against the live .cm-content so we
// test the actual wired behaviour, not a reimplementation.
const { test, expect } = require('@playwright/test');

async function setTier(page, m) {
  await page.locator('#menuBtn').click(); // the tier switcher lives in the ⋯ menu
  await page.locator(`#modesw button[data-mode="${m}"]`).click();
}

// Focus the editor, set a known one-line value, and park the caret at the end.
async function seed(page, value) {
  await page.locator('.cm-content').click();
  await page.evaluate((v) => { window.__cm.set(v); window.__cm.focus(); window.__cm.caretEnd(); }, value);
}

const doc = (page) => page.evaluate(() => window.__cm.get());

test.describe('Dev-tier bracket assists (CodeMirror closeBrackets)', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('.cm-content')).toBeVisible();
  });

  test('Dev: typing an opener auto-closes it', async ({ page }) => {
    await setTier(page, 'dev');
    await seed(page, 'foo');
    await page.keyboard.press('(');               // '(' auto-closes → foo()
    expect(await doc(page)).toBe('foo()');
  });

  test('Dev: typing over the auto-inserted closer does not double it', async ({ page }) => {
    await setTier(page, 'dev');
    await seed(page, '');
    await page.keyboard.type('(');                // -> ()
    await page.keyboard.type(')');                // type-over, not a second )
    const v = await doc(page);
    expect(v).toBe('()');
  });

  test('Learn leaves the editor default: no auto-close', async ({ page }) => {
    // Learn is the default tier
    await seed(page, 'let x = 5');
    await page.keyboard.type('(');
    expect((await doc(page)).endsWith('(')).toBeTruthy(); // no ')' auto-added
  });

  test('Assist leaves the editor default: no auto-close', async ({ page }) => {
    await setTier(page, 'assist');
    await seed(page, 'let x = 5');
    await page.keyboard.type('(');
    expect((await doc(page)).endsWith('(')).toBeTruthy();
  });
});
