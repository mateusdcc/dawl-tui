/* Y2K UNIX Terminal Simulator Component */
import { EXAMPLES_DATA } from "../data/examples.js";

export function initTerminalSimulator(containerId) {
  const container = document.getElementById(containerId);
  if (!container) return;

  container.innerHTML = createTerminalHtml();
  bindTerminalInput(container);
}

function createTerminalHtml() {
  return `
    <div class="terminal-window">
      <div class="terminal-header">
        <span class="terminal-title">dawl-tui@unix: ~ (tty1)</span>
        <div class="terminal-controls">
          <span class="btn-ctrl btn-close"></span>
          <span class="btn-ctrl btn-min"></span>
          <span class="btn-ctrl btn-max"></span>
        </div>
      </div>
      <div class="terminal-body" id="term-body">
        <div class="terminal-output" id="term-out">dawl-tui interactive CLI v0.1.0\nType 'help' or 'list' to see available workflow examples.</div>
        <div class="cmd-line">
          <span class="cmd-prompt">dawl-tui@unix:~$</span>
          <input type="text" class="cmd-input" id="term-in" autocomplete="off" spellcheck="false" />
        </div>
      </div>
    </div>
  `;
}

function bindTerminalInput(container) {
  const input = container.querySelector("#term-in");
  if (!input) return;
  input.addEventListener("keydown", (e) => {
    if (e.key === "Enter") {
      processCommand(input.value.trim());
      input.value = "";
    }
  });
}

function processCommand(cmdLine) {
  const outEl = document.getElementById("term-out");
  if (!outEl) return;
  if (!cmdLine) return;

  const promptPrefix = `\ndawl-tui@unix:~$ ${cmdLine}\n`;
  const response = executeCli(cmdLine);

  if (cmdLine.toLowerCase() === "clear") {
    outEl.textContent = "dawl-tui interactive CLI v0.1.0\nType 'help' to begin.";
  } else {
    outEl.textContent += promptPrefix + response;
  }
  const body = document.getElementById("term-body");
  if (body) body.scrollTop = body.scrollHeight;
}

function executeCli(cmdLine) {
  const parts = cmdLine.split(/\s+/);
  const cmd = parts[0].toLowerCase();
  const arg = parts[1];

  const handlers = {
    help: () => "Available commands: render <id>, list, version, clear, theme <green|amber|cyan>, help",
    version: () => "dawl-tui 0.1.0 (built with rustc 1.88.0)",
    list: () => "Examples:\n" + EXAMPLES_DATA.map(e => `  - ${e.id}: ${e.title}`).join("\n"),
    render: () => handleRenderCmd(arg),
    theme: () => handleThemeCmd(arg)
  };

  return handlers[cmd] ? handlers[cmd]() : `bash: command not found: ${cmd}. Type 'help' for commands.`;
}

function handleRenderCmd(id) {
  if (!id) return "Error: Please specify example id. Usage: render <id> (e.g. 'render simple' or 'render approval')";
  const example = EXAMPLES_DATA.find(e => e.id === id || e.id.includes(id));
  if (!example) return `Error: Example '${id}' not found. Type 'list' to view available examples.`;
  return `=== Rendering ${example.title} ===\n${example.renderAscii}`;
}

function handleThemeCmd(theme) {
  if (!["green", "amber", "cyan"].includes(theme)) {
    return "Usage: theme <green|amber|cyan>";
  }
  document.body.classList.remove("theme-amber", "theme-cyan");
  if (theme !== "green") document.body.classList.add(`theme-${theme}`);
  return `Switched terminal phosphor theme to ${theme}.`;
}
