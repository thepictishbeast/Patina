// Playwright E2E config for the Tempered Studio web surface (rpro-serve).
// Browser: Playwright's bundled Chromium headless shell (no system Chrome needed).
// Assumes rpro-serve is already running at RPRO_BASE (default 127.0.0.1:8787).
const { defineConfig } = require('@playwright/test');

module.exports = defineConfig({
  testDir: '.',
  timeout: 30_000,
  expect: { timeout: 10_000 },
  fullyParallel: false,
  retries: 0,
  reporter: [['list']],
  use: {
    baseURL: process.env.RPRO_BASE || 'http://127.0.0.1:8787',
    headless: true,
    browserName: 'chromium',
  },
});
