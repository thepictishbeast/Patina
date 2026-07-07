// Tempered Studio: wire the in-app "← Studio" back button. The viewer's CSP is
// `script-src 'self'` (no 'unsafe-inline'), so an inline onclick= attribute is
// silently blocked — the handler must live in a same-origin file like this one.
// history.back() returns to the SPA that opened the book; the fallback covers a
// PDF opened directly (no prior app page).
(function () {
  var btn = document.getElementById('temperedBack');
  if (!btn) return;
  btn.addEventListener('click', function () {
    if (history.length > 1) history.back();
    else location.href = '../../../index.html';
  });
})();
