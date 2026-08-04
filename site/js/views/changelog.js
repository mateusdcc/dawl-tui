/* Changelog View Component */
import { CHANGELOG_DATA } from "../data/changelog.js";

export function renderChangelogView(container) {
  container.innerHTML = getChangelogHtml();
}

function getChangelogHtml() {
  const listHtml = CHANGELOG_DATA.map(rel => createReleaseCard(rel)).join("");
  return `
    <div style="max-width: 800px; margin: 0 auto;">
      <h2 style="color: var(--text-bright); margin-bottom: 15px;">SYS_CHANGELOG.LOG</h2>
      ${listHtml}
    </div>
  `;
}

function createReleaseCard(release) {
  const changesHtml = release.changes.map(c => `
    <li style="margin-bottom: 6px;">
      <span class="badge ${getBadgeClass(c.type)}">${c.type}</span>
      <span>${c.desc}</span>
    </li>
  `).join("");

  return `
    <div class="terminal-window">
      <div class="terminal-header">
        <span class="terminal-title">VERSION ${release.version} (${release.date})</span>
        <span class="badge badge-amber">${release.status}</span>
      </div>
      <div class="terminal-body">
        <ul style="list-style: none;">
          ${changesHtml}
        </ul>
      </div>
    </div>
  `;
}

function getBadgeClass(type) {
  if (type === "ADDED" || type === "FEATURE") return "badge-green";
  if (type === "IMPROVE") return "badge-cyan";
  return "badge-amber";
}
