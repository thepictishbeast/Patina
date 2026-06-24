// E2E for the Dev-tier editor convenience: bracket auto-close, type-over, and
// empty-pair backspace — and the charter guarantee that Learn AND Assist leave
// the textarea untouched (no editor assists outside Dev). Drives the real key
// events against the live #editorCode textarea so we test the actual handler,
// not a reimplementation.
const { test, expect } = require('@playwright/test');

// Put the caret at the END of the editor and clear it to a known one-line value,
// so each assertion starts from a clean, deterministic slate.
async function seed(page, value) {
  await page.locator('#editorCode').click();
  await page.locator('#editorCode').evaluate((el, v) => {
    el.value = v;
    el.selectionStart = el.selectionEnd = v.length;
    el.focus();
  }, value);
}

async function state(page) {
  return page.locator('#editorCode').evaluate((el) => ({
    value: el.value,
    caret: el.selectionStart,
  }));
}

test.describe('Dev-tier bracket assists', () => {
  test.beforeEach(async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#editorCode')).toBeVisible();
  });

  test('Dev: typing an opener at end-of-line auto-closes with caret between', async ({ page }) => {
    await page.locator('#modesw button[data-mode="dev"]').click();
    await seed(page, 'let v = vec!');
    await page.locator('#editorCode').press('[');
    let s = await state(page);
    expect(s.value, 'auto-closed the bracket pair').toBe('let v = vec![]');
    expect(s.caret, 'caret sits between the brackets').toBe('let v = vec!['.length);

    // type-over: pressing the matching close skips over it instead of doubling
    await page.locator('#editorCode').press(']');
    s = await state(page);
    expect(s.value, 'no doubled close bracket').toBe('let v = vec![]');
    expect(s.caret, 'caret advanced past the close').toBe('let v = vec![]'.length);
  });

  test('Dev: backspace inside an empty pair deletes both brackets', async ({ page }) => {
    await page.locator('#modesw button[data-mode="dev"]').click();
    await seed(page, 'foo');
    await page.locator('#editorCode').press('(');           // -> foo(|)
    expect((await state(page)).value).toBe('foo()');
    await page.locator('#editorCode').press('Backspace');   // -> foo|
    const s = await state(page);
    expect(s.value, 'both brackets removed').toBe('foo');
    expect(s.caret).toBe('foo'.length);
  });

  test('Dev: an opener glued onto a word is NOT auto-closed', async ({ page }) => {
    await page.locator('#modesw button[data-mode="dev"]').click();
    // caret before the "x": typing "(" must not produce "()x" — only a bare "("
    await page.locator('#editorCode').evaluate((el) => {
      el.value = 'fx';
      el.selectionStart = el.selectionEnd = 1; // between f and x
      el.focus();
    });
    await page.locator('#editorCode').press('(');
    expect((await state(page)).value, 'no auto-close before a word char').toBe('f(x');
  });

  test('Learn leaves the textarea default: no auto-close', async ({ page }) => {
    // Learn is the default mode; do not switch. Typing "(" inserts a lone "(".
    await seed(page, 'bar');
    await page.locator('#editorCode').press('(');
    expect((await state(page)).value, 'Learn does not auto-close').toBe('bar(');
  });

  test('Assist leaves the textarea default: no auto-close', async ({ page }) => {
    await page.locator('#modesw button[data-mode="assist"]').click();
    await seed(page, 'baz');
    await page.locator('#editorCode').press('(');
    expect((await state(page)).value, 'Assist does not auto-close').toBe('baz(');
  });
});
