/* Examples Library View Component */
import { EXAMPLES_DATA } from "../data/examples.js";

export function renderExamplesView(container) {
  container.innerHTML = getExamplesLayoutHtml();
  bindFilterAndSearch(container);
  renderExampleCards(container, EXAMPLES_DATA);
}

function getExamplesLayoutHtml() {
  const categories = ["ALL", "Agent Workflows", "CI/CD Pipelines", "Approval Loops", "Security & Barriers", "Microservices"];
  const filterBtns = categories.map((cat, idx) => `
    <button class="nav-btn filter-btn ${idx === 0 ? 'active' : ''}" data-cat="${cat}">${cat}</button>
  `).join("");

  return `
    <div>
      <div style="display: flex; gap: 15px; margin-bottom: 15px; flex-wrap: wrap; align-items: center;">
        <input type="text" class="search-input" id="example-search" placeholder="Search workflow examples by name or keyword..." style="flex: 1;" />
      </div>
      <div class="filter-bar">
        ${filterBtns}
      </div>
      <div class="grid-2col" id="examples-grid">
      </div>
    </div>
  `;
}

function bindFilterAndSearch(container) {
  let activeCat = "ALL";
  const searchInput = container.querySelector("#example-search");
  const filterBtns = container.querySelectorAll(".filter-btn");

  const applyFilters = () => {
    const query = searchInput ? searchInput.value.toLowerCase().trim() : "";
    const filtered = EXAMPLES_DATA.filter(ex => {
      const matchCat = activeCat === "ALL" || ex.category === activeCat;
      const matchQuery = ex.title.toLowerCase().includes(query) || ex.description.toLowerCase().includes(query) || ex.id.includes(query);
      return matchCat && matchQuery;
    });
    renderExampleCards(container, filtered);
  };

  if (searchInput) searchInput.addEventListener("input", applyFilters);
  filterBtns.forEach(btn => {
    btn.addEventListener("click", () => {
      filterBtns.forEach(b => b.classList.remove("active"));
      btn.classList.add("active");
      activeCat = btn.getAttribute("data-cat");
      applyFilters();
    });
  });
}

function renderExampleCards(container, list) {
  const grid = container.querySelector("#examples-grid");
  if (!grid) return;
  if (list.length === 0) {
    grid.innerHTML = `<div style="grid-column: 1/-1; padding: 20px; text-align: center;">No examples match search criteria.</div>`;
    return;
  }
  grid.innerHTML = list.map(ex => createCardHtml(ex)).join("");
  bindCardInteractions(grid);
}

function createCardHtml(ex) {
  return `
    <div class="example-card" data-id="${ex.id}">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
        <h3 style="color: var(--text-bright); font-size: 1.1rem;">${ex.title}</h3>
        <span class="badge badge-cyan">${ex.category}</span>
      </div>
      <p style="font-size: 0.9rem; color: var(--text-dim); margin-bottom: 10px;">${ex.description}</p>
      <div style="display: flex; gap: 8px; margin-bottom: 8px;">
        <button class="retro-btn btn-mode-ascii" style="padding: 2px 8px; font-size: 0.8rem;">ASCII</button>
        <button class="retro-btn btn-mode-code" style="padding: 2px 8px; font-size: 0.8rem;">.DTUI CODE</button>
        <button class="retro-btn btn-copy" style="padding: 2px 8px; font-size: 0.8rem; margin-left: auto;">COPY CODE</button>
      </div>
      <pre class="example-preview card-view-${ex.id}">${ex.renderAscii}</pre>
    </div>
  `;
}

function bindCardInteractions(grid) {
  grid.querySelectorAll(".example-card").forEach(card => {
    const id = card.getAttribute("data-id");
    const example = EXAMPLES_DATA.find(ex => ex.id === id);
    if (!example) return;
    const preview = card.querySelector(`.card-view-${id}`);
    const btnAscii = card.querySelector(".btn-mode-ascii");
    const btnCode = card.querySelector(".btn-mode-code");
    const btnCopy = card.querySelector(".btn-copy");

    if (btnAscii && preview) btnAscii.addEventListener("click", () => preview.textContent = example.renderAscii);
    if (btnCode && preview) btnCode.addEventListener("click", () => preview.textContent = example.code);
    if (btnCopy) btnCopy.addEventListener("click", () => handleCopyCode(btnCopy, example.code));
  });
}

function handleCopyCode(button, text) {
  navigator.clipboard.writeText(text);
  const oldText = button.textContent;
  button.textContent = "COPIED!";
  setTimeout(() => button.textContent = oldText, 1500);
}
