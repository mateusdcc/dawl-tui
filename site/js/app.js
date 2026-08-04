/* Main Application Router & Entry Point */
import { initCrtController } from "./components/crt.js";
import { initNavigation } from "./components/navigation.js";
import { initModalController } from "./components/modal.js";
import { renderLandingView } from "./views/landing.js";
import { renderDocsView } from "./views/docs.js";
import { renderChangelogView } from "./views/changelog.js";
import { renderExamplesView } from "./views/examples.js";

document.addEventListener("DOMContentLoaded", () => {
  initCrtController();
  initModalController();
  initNavigation(handleRouteChanged);
  initSystemTicker();
  handleRouteChanged("landing");
});

function handleRouteChanged(viewName) {
  const container = document.getElementById("main-view");
  if (!container) return;

  const viewRenderers = {
    landing: renderLandingView,
    docs: renderDocsView,
    changelog: renderChangelogView,
    examples: renderExamplesView
  };

  const renderFn = viewRenderers[viewName] || renderLandingView;
  renderFn(container);
  window.scrollTo({ top: 0, behavior: "smooth" });
}

function initSystemTicker() {
  const uptimeEl = document.getElementById("sys-uptime");
  let seconds = 34820;
  setInterval(() => {
    seconds++;
    const hrs = Math.floor(seconds / 3600);
    const mins = Math.floor((seconds % 3600) / 60);
    const secs = seconds % 60;
    if (uptimeEl) {
      uptimeEl.textContent = `UPTIME: ${hrs}h ${mins}m ${secs}s`;
    }
  }, 1000);
}
