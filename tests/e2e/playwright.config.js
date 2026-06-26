// Playwright E2E config for the Tempered Studio web surface (rpro-serve).
// Browser: Playwright's bundled Chromium headless shell (no system Chrome needed).
// Assumes rpro-serve is already running at RPRO_BASE (default 127.0.0.1:8787).
const { defineConfig } = require('@playwright/test');

module.exports = defineConfig({
  testDir: '.',
  timeout: 30_000,
  expect: { timeout: 10_000 },
  fullyParallel: false,
  // The first run-based spec occasionally exceeds the 30s timeout — not a cold
  // compile (a cold /api/run is ~0.1s; the run-scratch shares the warm workspace
  // target), but environmental: the first Chromium launch + page load under
  // machine load. One retry (warm browser) makes the suite reliable; a genuine
  // regression still fails both attempts, so this hides flakes, not real breaks.
  retries: 1,
  reporter: [['list']],
  use: {
    baseURL: process.env.RPRO_BASE || 'http://127.0.0.1:8787',
    headless: true,
    browserName: 'chromium',
  },
});
