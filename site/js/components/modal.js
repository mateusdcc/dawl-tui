/* Floating Modal Lightbox Component */
export function initModalController() {
  createModalContainer();
  bindModalCloseEvents();
}

function createModalContainer() {
  if (document.getElementById("modal-overlay")) return;
  const modalHtml = `
    <div id="modal-overlay" class="modal-overlay" style="display: none;">
      <div class="terminal-window modal-window">
        <div class="terminal-header">
          <span class="terminal-title" id="modal-title">DIAGRAM_ZOOM_VIEWER.SVG</span>
          <div class="terminal-controls">
            <button id="modal-close-btn" class="retro-btn" style="padding: 0 6px; font-size: 0.8rem; background: #ff5f56; color: #fff; border: none;">X</button>
          </div>
        </div>
        <div class="terminal-body modal-body" id="modal-content"></div>
      </div>
    </div>
  `;
  document.body.insertAdjacentHTML("beforeend", modalHtml);
}

function bindModalCloseEvents() {
  const overlay = document.getElementById("modal-overlay");
  const closeBtn = document.getElementById("modal-close-btn");
  if (closeBtn) closeBtn.addEventListener("click", closeModal);
  if (overlay) {
    overlay.addEventListener("click", (e) => {
      if (e.target === overlay) closeModal();
    });
  }
  document.addEventListener("keydown", (e) => {
    if (e.key === "Escape") closeModal();
  });
}

export function openModal(title, contentHtml) {
  const overlay = document.getElementById("modal-overlay");
  const titleEl = document.getElementById("modal-title");
  const contentEl = document.getElementById("modal-content");
  if (!overlay || !contentEl) return;

  if (titleEl) titleEl.textContent = title;
  contentEl.innerHTML = contentHtml;
  overlay.style.display = "flex";
}

export function closeModal() {
  const overlay = document.getElementById("modal-overlay");
  if (overlay) overlay.style.display = "none";
}
