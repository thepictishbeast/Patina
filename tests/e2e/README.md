# Tempered Studio — web E2E + a11y (#12)

Playwright browser tests + axe-core accessibility audit of the live web surface.

## Run
1. Start the server: `cargo run -p rpro-serve`  (serves http://127.0.0.1:8787/)
2. `cd tests/e2e && npm install && npx playwright install chromium`
3. `npx playwright test`

Override the target with `RPRO_BASE=http://host:port`.
