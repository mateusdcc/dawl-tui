/* Tab & View Navigation Router */
export function initNavigation(onRouteChanged) {
  const buttons = document.querySelectorAll(".nav-btn");
  buttons.forEach(btn => {
    btn.addEventListener("click", () => {
      const targetView = btn.getAttribute("data-view");
      setActiveNavButton(btn);
      onRouteChanged(targetView);
    });
  });
}

function setActiveNavButton(activeBtn) {
  const buttons = document.querySelectorAll(".nav-btn");
  buttons.forEach(b => b.classList.remove("active"));
  activeBtn.classList.add("active");
}
