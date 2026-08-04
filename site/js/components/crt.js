/* CRT Overlay & Theme Control Component */
export function initCrtController() {
  const crtBtn = document.getElementById("toggle-crt");
  if (crtBtn) {
    crtBtn.addEventListener("click", handleCrtToggle);
  }
  const themeSelect = document.getElementById("select-theme");
  if (themeSelect) {
    themeSelect.addEventListener("change", (e) => applyTheme(e.target.value));
  }
}

function handleCrtToggle() {
  const overlay = document.querySelector(".crt-overlay");
  if (!overlay) return;
  const isHidden = overlay.style.display === "none";
  overlay.style.display = isHidden ? "block" : "none";
}

function applyTheme(themeName) {
  document.body.classList.remove("theme-amber", "theme-cyan");
  if (themeName === "amber") {
    document.body.classList.add("theme-amber");
  } else if (themeName === "cyan") {
    document.body.classList.add("theme-cyan");
  }
}
