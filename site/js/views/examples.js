/* Examples Library View Component */
import { EXAMPLES_DATA } from "../data/examples.js";
import { openModal } from "../components/modal.js";

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
      <div style="margin-bottom: 15px;">
        <input type="text" class="search-input" id="example-search" placeholder="Search workflow examples..." />
      </div>
      <div class="filter-bar">${filterBtns}</div>
      <div class="grid-2col" id="examples-grid"></div>
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
    grid.innerHTML = `<div style="grid-column: 1/-1; padding: 20px; text-align: center;">No examples found.</div>`;
    return;
  }
  grid.innerHTML = list.map(ex => createCardHtml(ex)).join("");
  bindCardInteractions(grid);
  list.forEach(ex => loadCardSvg(ex.id));
}

function createCardHtml(ex) {
  return `
    <div class="example-card" data-id="${ex.id}">
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px; flex-wrap: wrap; gap: 6px;">
        <h3 style="color: var(--text-bright); font-size: 1.1rem;">${ex.title}</h3>
        <span class="badge badge-cyan">${ex.category}</span>
      </div>
      <p style="font-size: 0.9rem; color: var(--text-dim); margin-bottom: 10px;">${ex.description}</p>
      <div style="display: flex; gap: 6px; margin-bottom: 8px; flex-wrap: wrap;">
        <button class="retro-btn btn-mode-svg active" style="padding: 2px 8px; font-size: 0.8rem;">SVG RENDER</button>
        <button class="retro-btn btn-mode-ascii" style="padding: 2px 8px; font-size: 0.8rem;">ANSI / ASCII</button>
        <button class="retro-btn btn-mode-code" style="padding: 2px 8px; font-size: 0.8rem;">.DTUI CODE</button>
        <button class="retro-btn btn-copy" style="padding: 2px 8px; font-size: 0.8rem; margin-left: auto;">COPY</button>
      </div>
      <div class="svg-preview-container svg-box-${ex.id}" id="svg-box-${ex.id}" style="cursor: zoom-in;" title="Click to zoom in floating window">
        <span style="color: var(--text-dim);">Loading SVG output...</span>
      </div>
      <pre class="example-preview code-box-${ex.id}" id="code-box-${ex.id}" style="display: none; cursor: zoom-in;" title="Click to expand"></pre>
    </div>
  `;
}

function loadCardSvg(id) {
  const box = document.getElementById(`svg-box-${id}`);
  if (!box) return;
  fetch(`assets/svg/${id}.svg`)
    .then(res => res.ok ? res.text() : Promise.reject())
    .then(svgText => { box.innerHTML = svgText; })
    .catch(() => { box.innerHTML = `<span style="color: var(--text-amber);">SVG preview available via CLI render command.</span>`; });
}

function bindCardInteractions(grid) {
  grid.querySelectorAll(".example-card").forEach(card => {
    const id = card.getAttribute("data-id");
    const example = EXAMPLES_DATA.find(ex => ex.id === id);
    if (!example) return;
    setupTabButtons(card, example);
    bindZoomEvents(card, example);
  });
}

function bindZoomEvents(card, example) {
  const svgBox = card.querySelector(`.svg-box-${example.id}`);
  const codeBox = card.querySelector(`.code-box-${example.id}`);
  if (svgBox) {
    svgBox.addEventListener("click", () => {
      openModal(`${example.title} — SVG DIAGRAM`, svgBox.innerHTML);
    });
  }
  if (codeBox) {
    codeBox.addEventListener("click", () => {
      openModal(`${example.title} — CODE PREVIEW`, `<pre style="white-space: pre-wrap; font-family: var(--font-mono); color: var(--text-main);">${codeBox.textContent}</pre>`);
    });
  }
}

function setupTabButtons(card, example) {
  const svgBox = card.querySelector(`.svg-box-${example.id}`);
  const codeBox = card.querySelector(`.code-box-${example.id}`);
  const btnSvg = card.querySelector(".btn-mode-svg");
  const btnAscii = card.querySelector(".btn-mode-ascii");
  const btnCode = card.querySelector(".btn-mode-code");
  const btnCopy = card.querySelector(".btn-copy");

  const setActive = (activeBtn, showSvg, contentText) => {
    [btnSvg, btnAscii, btnCode].forEach(b => b && b.classList.remove("active"));
    activeBtn.classList.add("active");
    if (showSvg) {
      svgBox.style.display = "flex";
      codeBox.style.display = "none";
    } else {
      svgBox.style.display = "none";
      codeBox.style.display = "block";
      codeBox.textContent = contentText;
    }
  };

  if (btnSvg) btnSvg.addEventListener("click", () => setActive(btnSvg, true, ""));
  if (btnAscii) btnAscii.addEventListener("click", () => setActive(btnAscii, false, example.renderAscii));
  if (btnCode) btnCode.addEventListener("click", () => setActive(btnCode, false, example.code));
  if (btnCopy) btnCopy.addEventListener("click", () => handleCopy(btnCopy, example.code));
}

function handleCopy(button, text) {
  navigator.clipboard.writeText(text);
  const old = button.textContent;
  button.textContent = "COPIED!";
  setTimeout(() => button.textContent = old, 1500);
}
