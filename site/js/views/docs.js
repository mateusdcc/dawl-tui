/* Documentation View Component */
import { DOCS_DATA } from "../data/docs.js";

export function renderDocsView(container) {
  container.innerHTML = getDocsLayoutHtml();
  bindDocsSidebar(container);
  loadDocContent(container, DOCS_DATA[0].id);
}

function getDocsLayoutHtml() {
  const sidebarItemsHtml = DOCS_DATA.map((doc, idx) => `
    <li class="sidebar-item ${idx === 0 ? 'active' : ''}" data-doc="${doc.id}">
      ${doc.title}
    </li>
  `).join("");

  return `
    <div class="docs-layout">
      <div class="docs-sidebar">
        <h4 style="margin-bottom: 10px; color: var(--text-bright);">DOCUMENTATION</h4>
        <ul class="sidebar-menu">
          ${sidebarItemsHtml}
        </ul>
      </div>
      <div class="terminal-window">
        <div class="terminal-header">
          <span class="terminal-title" id="doc-title-header">MANUAL_PAGE</span>
        </div>
        <div class="terminal-body" id="doc-content-body">
        </div>
      </div>
    </div>
  `;
}

function bindDocsSidebar(container) {
  const items = container.querySelectorAll(".sidebar-item");
  items.forEach(item => {
    item.addEventListener("click", () => {
      items.forEach(i => i.classList.remove("active"));
      item.classList.add("active");
      const docId = item.getAttribute("data-doc");
      loadDocContent(container, docId);
    });
  });
}

function loadDocContent(container, docId) {
  const doc = DOCS_DATA.find(d => d.id === docId);
  if (!doc) return;
  const headerEl = container.querySelector("#doc-title-header");
  const bodyEl = container.querySelector("#doc-content-body");
  if (headerEl) headerEl.textContent = doc.title;
  if (bodyEl) bodyEl.innerHTML = doc.content;
}
