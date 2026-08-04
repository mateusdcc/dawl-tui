/* Interactive Live .DTUI DSL Editor View Component */
import { EXAMPLES_DATA } from "../data/examples.js";
import { renderDtuiToSvg } from "../components/dtui-svg-renderer.js";

export function renderEditorView(container) {
  container.innerHTML = getEditorLayoutHtml();
  bindEditorEvents(container);
  loadPreset(container, "graph-loop");
}

function getEditorLayoutHtml() {
  const options = EXAMPLES_DATA.map(ex => `<option value="${ex.id}">${ex.title}</option>`).join("");
  return `
    <div>
      <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 15px; flex-wrap: wrap; gap: 10px;">
        <div style="display: flex; gap: 10px; align-items: center;">
          <label style="color: var(--text-bright); font-weight: bold;">LOAD PRESET:</label>
          <select id="editor-preset" class="retro-btn" style="padding: 6px 12px; font-size: 0.9rem;">
            ${options}
          </select>
        </div>
        <div style="display: flex; gap: 8px; flex-wrap: wrap;">
          <button id="btn-render" class="retro-btn active" style="padding: 6px 14px; background: var(--text-main); color: var(--bg-primary);">RUN / RENDER</button>
          <button id="btn-clear" class="retro-btn" style="padding: 6px 12px;">CLEAR</button>
          <button id="btn-copy-code" class="retro-btn" style="padding: 6px 12px;">COPY CODE</button>
        </div>
      </div>

      <!-- Side-by-Side Dual Panel Layout -->
      <div class="grid-2col" style="align-items: stretch; grid-template-columns: 1fr 1fr;">
        <!-- Left Panel: Live Code Editor -->
        <div class="terminal-window" style="display: flex; flex-direction: column; height: 100%;">
          <div class="terminal-header">
            <span class="terminal-title">PANEL_1: LIVE_EDITOR.DTUI</span>
            <span class="badge badge-green" id="editor-status">LIVE AUTO-SYNC</span>
          </div>
          <div class="terminal-body" style="flex: 1; padding: 10px; display: flex; flex-direction: column;">
            <textarea id="editor-input" class="cmd-input" style="width: 100%; min-height: 450px; flex: 1; font-family: var(--font-mono); font-size: 1rem; line-height: 1.45; color: var(--text-bright); background: transparent; resize: vertical; outline: none; border: none;" spellcheck="false" placeholder="Type .dtui syntax here..."></textarea>
          </div>
        </div>

        <!-- Right Panel: Live SVG Render Preview -->
        <div class="terminal-window" style="display: flex; flex-direction: column; height: 100%;">
          <div class="terminal-header">
            <span class="terminal-title">PANEL_2: LIVE_SVG_RENDER.SVG</span>
            <span class="badge badge-cyan" id="render-badge">SVG OUTPUT</span>
          </div>
          <div class="terminal-body" id="editor-preview-body" style="flex: 1; min-height: 450px; display: flex; justify-content: center; align-items: center; overflow: auto; padding: 15px;">
            <span style="color: var(--text-dim);">Live SVG rendering...</span>
          </div>
        </div>
      </div>
    </div>
  `;
}

function bindEditorEvents(container) {
  const presetSelect = container.querySelector("#editor-preset");
  const btnRender = container.querySelector("#btn-render");
  const btnClear = container.querySelector("#btn-clear");
  const btnCopy = container.querySelector("#btn-copy-code");
  const textarea = container.querySelector("#editor-input");

  if (presetSelect) presetSelect.addEventListener("change", (e) => loadPreset(container, e.target.value));
  if (btnRender) btnRender.addEventListener("click", () => handleRender(container));
  if (btnClear) btnClear.addEventListener("click", () => handleClear(container));
  if (btnCopy && textarea) btnCopy.addEventListener("click", () => handleCopyCode(btnCopy, textarea.value));
  if (textarea) textarea.addEventListener("input", () => handleLiveInput(container));
}

function loadPreset(container, presetId) {
  const example = EXAMPLES_DATA.find(ex => ex.id === presetId);
  const textarea = container.querySelector("#editor-input");
  if (!example || !textarea) return;

  textarea.value = example.code;
  container.dataset.activeId = example.id;
  handleRender(container);
}

function handleRender(container) {
  const textarea = container.querySelector("#editor-input");
  const previewBody = container.querySelector("#editor-preview-body");
  const statusBadge = container.querySelector("#editor-status");
  if (!textarea || !previewBody) return;

  const currentId = container.dataset.activeId;
  const example = EXAMPLES_DATA.find(ex => ex.id === currentId && ex.code === textarea.value);

  if (statusBadge) {
    statusBadge.textContent = "SYNTAX OK";
    statusBadge.className = "badge badge-green";
  }

  if (example) {
    fetch(`assets/svg/${example.id}.svg`)
      .then(r => r.ok ? r.text() : Promise.reject())
      .then(svg => { previewBody.innerHTML = svg; })
      .catch(() => { previewBody.innerHTML = renderDtuiToSvg(textarea.value); });
  } else {
    previewBody.innerHTML = renderDtuiToSvg(textarea.value);
  }
}

function handleLiveInput(container) {
  container.dataset.activeId = "";
  handleRender(container);
}

function handleClear(container) {
  const textarea = container.querySelector("#editor-input");
  const previewBody = container.querySelector("#editor-preview-body");
  if (textarea) textarea.value = "";
  if (previewBody) previewBody.innerHTML = renderDtuiToSvg("");
}

function handleCopyCode(button, text) {
  navigator.clipboard.writeText(text);
  const old = button.textContent;
  button.textContent = "COPIED!";
  setTimeout(() => button.textContent = old, 1500);
}
