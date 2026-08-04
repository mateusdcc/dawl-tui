/* Changelog & Roadmap Data Module */
export const CHANGELOG_DATA = [
  {
    version: "0.1.0",
    date: "2026-08-03",
    status: "CURRENT RELEASE",
    changes: [
      { type: "ADDED", desc: "Deterministic layered layout engine for compound workflow graphs." },
      { type: "ADDED", desc: "Native `.dtui` DSL parser and DAWL JSON graph format parser." },
      { type: "ADDED", desc: "Truecolor ANSI, SVG export, and interactive terminal viewports." },
      { type: "ADDED", desc: "Orthogonal A* routing with explicit ports and via point constraints." },
      { type: "ADDED", desc: "Finite DAWL NDJSON event stream replay engine." }
    ]
  },
  {
    version: "0.2.0",
    date: "UPCOMING (Q3 2026)",
    status: "IN DEVELOPMENT",
    changes: [
      { type: "FEATURE", desc: "WebAssembly (WASM) live in-browser rendering library." },
      { type: "FEATURE", desc: "Live WebSocket event streaming backend for real-time agent visualization." },
      { type: "IMPROVE", desc: "Enhanced median-sweep crossing reduction for 100+ node graphs." }
    ]
  }
];
