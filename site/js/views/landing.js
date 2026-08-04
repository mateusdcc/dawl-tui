/* Landing Page View Component */
import { initTerminalSimulator } from "../components/terminal.js";

export function renderLandingView(container) {
  container.innerHTML = getLandingHtml();
  initTerminalSimulator("landing-term");
}

function getLandingHtml() {
  return `
    <section class="hero-section">
      <pre class="ascii-logo">
 ____    _    __500__  _       _____ _   _ ___ 
|  _ \\  / \\  |  _ \\ \\| |     |_   _| | | |_ _|
| | | |/ _ \\ | |_) | | |  _____ | | | | | || | 
| |_| / ___ \\|  __/| | |__|_____|| | | |_| || | 
|____/_/   \\_\\_|   |_|_____|     |_|  \\___/|___|
      </pre>
      <p class="hero-subtitle">Deterministic Compound Workflow Diagrams for the Terminal & Vector Graphics</p>
      <div style="margin-bottom: 20px;">
        <span class="badge badge-green">RUST 1.88+</span>
        <span class="badge badge-cyan">ANSI TRUECOLOR</span>
        <span class="badge badge-amber">SVG EXPORT</span>
        <span class="badge badge-green">DAWL PROTOCOL</span>
      </div>
    </section>

    <div class="grid-2col">
      <div>
        <div class="terminal-window">
          <div class="terminal-header">
            <span class="terminal-title">SYS_CAPABILITIES.TXT</span>
          </div>
          <div class="terminal-body">
            <ul>
              <li><strong>Layered Layout Engine:</strong> Median sweeps for minimal crossing reduction.</li>
              <li><strong>Orthogonal Routing:</strong> A* maze solver with bend and obstacle avoidance.</li>
              <li><strong>Nested Compounds:</strong> Support for lanes, parallel scopes, and decision nodes.</li>
              <li><strong>Zero Dependencies GUI:</strong> Pure Unicode ANSI truecolor output in standard terminals.</li>
            </ul>
          </div>
        </div>
      </div>
      <div id="landing-term"></div>
    </div>
  `;
}
