// Batch A (#36/#37): every lesson's practice section now routes into the app
// (Sandbox + the "Practice this lesson" exercise links) instead of `cargo new`.
// This pins the lesson -> exercise handoff end-to-end on the ISOLATED store:
// the link select-jumps to Practice, and the soft-gate confirm ("locked — jump
// ahead anyway?") honours both answers. Also guards the Batch-A content shape:
// the Foundations practice sections must name the in-app Sandbox, not open with
// a bare `cargo new` (the audit's #1 systemic finding).
const { test, expect } = require('@playwright/test');

test.describe('Lesson → Practice handoff (Batch A)', () => {
  test('a locked exercise link asks to jump ahead; declining stays, accepting lands in Practice on it', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    await page.evaluate(() => showView('lessons', '01-bindings-and-immutability'));
    await page.waitForSelector('#docview .lessonpractice .practicelink');
    const target = await page.locator('#docview .practicelink').first().getAttribute('data-id');
    expect(target).toBeTruthy();

    // Decline the soft-gate confirm -> we stay on the lesson, nothing selected.
    page.once('dialog', (d) => d.dismiss());
    await page.locator('#docview .practicelink').first().click();
    await page.waitForTimeout(300);
    await expect(page.locator('#docview .lessonbody')).toBeVisible();

    // Accept it -> force-select + jump to Practice with that exercise current.
    page.once('dialog', (d) => d.accept());
    await page.locator('#docview .practicelink').first().click();
    await page.waitForFunction(
      (want) => (document.getElementById('exId')?.textContent || document.body.textContent).includes(want),
      target,
      { timeout: 5000 },
    );
    // The Practice surface is live (Run button present and view switched).
    await expect(page.locator('#runbtn')).toBeVisible();
  });

  test('Foundations practice sections route in-app (Sandbox named, no bare cargo-new opener)', async ({ page }) => {
    await page.goto('/');
    await page.waitForSelector('#exTitle');
    for (const id of ['00-hello-world', '04-constants', '07-functions']) {
      const md = await page.evaluate(async (lid) => {
        const j = await fetch('api/lessons?id=' + lid).then((r) => r.json());
        return j.lesson.markdown;
      }, id);
      const practice = md.split(/## 5\./)[1] || '';
      expect(practice, `${id} practice names the Sandbox`).toContain('Sandbox');
      // cargo new may only appear as the parenthesised own-machine aside.
      const opener = practice.trim().split('\n').slice(0, 3).join(' ');
      expect(opener.startsWith('`cargo new'), `${id} practice no longer OPENS with cargo new`).toBe(false);
    }
  });
});
