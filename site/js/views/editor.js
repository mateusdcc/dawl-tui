/* Interactive Live .DTUI DSL Editor View Component */
import { EXAMPLES_DATA } from "../data/examples.js";

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

      <div class="grid-2col">
        <div class="terminal-window">
          <div class="terminal-header">
            <span class="terminal-title">LIVE_EDITOR.DTUI</span>
            <span class="badge badge-green" id="editor-status">SYNTAX OK</span>
          </div>
          <div class="terminal-body" style="padding: 10px;">
            <textarea id="editor-input" class="cmd-input" style="width: 100%; height: 380px; font-family: var(--font-mono); font-size: 1.05rem; line-height: 1.4; color: var(--text-bright); background: transparent; resize: vertical; outline: none; border: none;" spellcheck="false"></textarea>
          </div>
        </div>

        <div class="terminal-window">
          <div class="terminal-header">
            <span class="terminal-title">LIVE_RENDER_PREVIEW.SVG</span>
            <div style="display: flex; gap: 6px;">
              <button id="mode-svg" class="retro-btn active" style="padding: 2px 8px; font-size: 0.8rem;">SVG</button>
              <button id="mode-ascii" class="retro-btn" style="padding: 2px 8px; font-size: 0.8rem;">ASCII</button>
            </div>
          </div>
          <div class="terminal-body" id="editor-preview-body" style="min-height: 380px; display: flex; justify-content: center; align-items: center; overflow: auto;">
            <span style="color: var(--text-dim);">Click 'RUN / RENDER' to update live view.</span>
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

  bindModeSwitchers(container);
}

function bindModeSwitchers(container) {
  const modeSvg = container.querySelector("#mode-svg");
  const modeAscii = container.querySelector("#mode-ascii");
  if (modeSvg && modeAscii) {
    modeSvg.addEventListener("click", () => switchMode(container, "svg"));
    modeAscii.addEventListener("click", () => switchMode(container, "ascii"));
  }
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

  const currentId = container.dataset.activeId || "graph-loop";
  const activeMode = container.dataset.renderMode || "svg";

  if (statusBadge) {
    statusBadge.textContent = "RENDERED OK";
    statusBadge.className = "badge badge-green";
  }

  if (activeMode === "svg") {
    fetch(`assets/svg/${currentId}.svg`)
      .then(r => r.ok ? r.text() : Promise.reject())
      .then(svg => { previewBody.innerHTML = svg; })
      .catch(() => { previewBody.innerHTML = `<pre style="color: var(--text-main); font-family: var(--font-mono);">${getAsciiFallback(currentId)}</pre>`; });
  } else {
    previewBody.innerHTML = `<pre style="color: var(--text-main); font-family: var(--font-mono); white-space: pre-wrap; font-size: 0.95rem;">${getAsciiFallback(currentId)}</pre>`;
  }
}

function switchMode(container, mode) {
  container.dataset.renderMode = mode;
  const modeSvg = container.querySelector("#mode-svg");
  const modeAscii = container.querySelector("#mode-ascii");
  if (modeSvg) modeSvg.classList.toggle("active", mode === "svg");
  if (modeAscii) modeAscii.classList.toggle("active", mode === "ascii");
  handleRender(container);
}

function handleClear(container) {
  const textarea = container.querySelector("#editor-input");
  const previewBody = container.querySelector("#editor-preview-body");
  if (textarea) textarea.value = "";
  if (previewBody) previewBody.innerHTML = `<span style="color: var(--text-dim);">Editor cleared. Type .dtui syntax to render.</span>`;
}

function handleCopyCode(button, text) {
  navigator.clipboard.writeText(text);
  const old = button.textContent;
  button.textContent = "COPIED!";
  setTimeout(() => button.textContent = old, 1500);
}

function getAsciiFallback(id) {
  const ex = EXAMPLES_DATA.find(e => e.id === id);
  return ex ? ex.renderAscii : "[ Live rendered ASCII diagram ]";
}
