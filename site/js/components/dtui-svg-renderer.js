/* Lightweight Client-Side .DTUI DSL to SVG Renderer */

export function renderDtuiToSvg(dtuiCode) {
  const nodes = parseNodes(dtuiCode);
  const edges = parseEdges(dtuiCode, nodes);
  return generateSvg(nodes, edges);
}

function parseNodes(code) {
  const nodes = [];
  const lines = code.split("\n");
  let x = 40, y = 40;

  lines.forEach(line => {
    const trimmed = line.trim();
    if (trimmed.startsWith("node ") || trimmed.startsWith("decision ")) {
      const isDecision = trimmed.startsWith("decision ");
      const parts = trimmed.split(/\s+/);
      const id = parts[1] || `n${nodes.length}`;
      const labelMatch = trimmed.match(/"([^"]+)"/);
      const label = labelMatch ? labelMatch[1].replace(/\\n/g, " ") : id;
      const isSuccess = trimmed.includes("kind success");
      const isFailure = trimmed.includes("kind failure");

      nodes.push({ id, label, isDecision, isSuccess, isFailure, x, y, w: 120, h: 50 });
      x += 160;
      if (x > 500) { x = 40; y += 90; }
    }
  });
  return nodes;
}

function parseEdges(code, nodes) {
  const edges = [];
  const lines = code.split("\n");

  lines.forEach(line => {
    const trimmed = line.trim();
    if (trimmed.startsWith("edge ")) {
      const match = trimmed.match(/edge\s+(\w+)\s+(\w+)\s*->\s*(\w+)/);
      if (match) {
        const [, id, srcId, dstId] = match;
        const src = nodes.find(n => n.id === srcId);
        const dst = nodes.find(n => n.id === dstId);
        if (src && dst) edges.push({ id, src, dst });
      }
    }
  });
  return edges;
}

function generateSvg(nodes, edges) {
  if (nodes.length === 0) {
    return `<div style="color: var(--text-dim); padding: 40px; text-align: center;">Type node & edge syntax to render SVG diagram.<br><br>Example:<br><code>node a "Build"<br>node b "Deploy"<br>edge e1 a -> b</code></div>`;
  }

  const svgNodes = nodes.map(n => renderNodeSvg(n)).join("\n");
  const svgEdges = edges.map(e => renderEdgeSvg(e)).join("\n");
  const width = Math.max(...nodes.map(n => n.x + n.w)) + 80;
  const height = Math.max(...nodes.map(n => n.y + n.h)) + 80;

  return `
    <svg width="${width}" height="${height}" viewBox="0 0 ${width} ${height}" xmlns="http://www.w3.org/2000/svg" style="max-width: 100%; height: auto; background: #030608;">
      <defs>
        <marker id="arrow" viewBox="0 0 10 10" refX="8" refY="5" markerWidth="6" markerHeight="6" orient="auto-start-reverse">
          <path d="M 0 0 L 10 5 L 0 10 z" fill="#00ff77"/>
        </marker>
      </defs>
      ${svgEdges}
      ${svgNodes}
    </svg>
  `;
}

function renderNodeSvg(n) {
  const color = n.isSuccess ? "#00ff77" : n.isFailure ? "#ff5f56" : "#00f3ff";
  if (n.isDecision) {
    const points = `${n.x + n.w/2},${n.y} ${n.x + n.w},${n.y + n.h/2} ${n.x + n.w/2},${n.y + n.h} ${n.x},${n.y + n.h/2}`;
    return `
      <polygon points="${points}" fill="#0f1519" stroke="#ffb000" stroke-width="2"/>
      <text x="${n.x + n.w/2}" y="${n.y + n.h/2 + 4}" fill="#ffffff" font-size="12" font-family="monospace" text-anchor="middle">${n.label}</text>
    `;
  }
  return `
    <rect x="${n.x}" y="${n.y}" width="${n.w}" height="${n.h}" rx="4" fill="#0f1519" stroke="${color}" stroke-width="2"/>
    <text x="${n.x + n.w/2}" y="${n.y + n.h/2 + 4}" fill="#ffffff" font-size="12" font-family="monospace" text-anchor="middle">${n.label}</text>
  `;
}

function renderEdgeSvg(e) {
  const x1 = e.src.x + e.src.w;
  const y1 = e.src.y + e.src.h / 2;
  const x2 = e.dst.x;
  const y2 = e.dst.y + e.dst.h / 2;
  return `<path d="M ${x1} ${y1} L ${x2} ${y2}" stroke="#00ff77" stroke-width="2" marker-end="url(#arrow)"/>`;
}
