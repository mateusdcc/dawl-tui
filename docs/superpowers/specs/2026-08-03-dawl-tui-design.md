# DAWL TUI Diagram Engine — Design

## Purpose

Build a standalone Rust program that renders dense, deterministic terminal diagrams with nested groups, orthogonal connectors, semantic colors, runtime state, and layout constraints. The bundled approval-workflow example must reproduce the composition and visual hierarchy of the supplied reference image at a documented terminal size.

The renderer accepts both a human-authored diagram language and DAWL-compatible graph/event streams. Both inputs lower to one canonical intermediate representation before layout.

## Product Boundary

Repository: `mateusdcc/dawl-tui`.

The project owns:

- diagram syntax and parser;
- canonical graph/layout IR;
- DAWL JSON and NDJSON adapters;
- deterministic hierarchical layout;
- orthogonal edge routing;
- terminal-cell canvas and ANSI styling;
- interactive viewport and runtime-state updates;
- SVG/text snapshot export for verification;
- benchmarks, examples, and quality gates.

It does not parse or execute DAWL source. DAWL remains responsible for workflow semantics and emits the canonical graph/event protocol.

## Research Basis

The architecture combines results from several domains:

1. **Layered directed-graph drawing.** Sugiyama, Tagawa, and Toda separate hierarchy drawing into layer ordering and coordinate assignment. Gansner et al. refine this into cycle removal, rank assignment, crossing reduction, coordinate assignment, and edge construction. This becomes the main layout pipeline.
2. **Compound graphs.** Sander's compound directed-graph layout motivates globally ranked nodes inside nested rectangular groups rather than laying out each group independently.
3. **Orthogonal/VLSI routing.** Tamassia's topology–shape–metrics separation and Lee's grid routing motivate a routing stage independent from node placement, with explicit penalties for bends, crossings, occupied cells, and semantic back-edges.
4. **Constraint-based layout.** Dwyer, Koren, and Marriott show how separation/alignment constraints can coexist with automatic layout. The DSL therefore supports optional rank, order, alignment, port, size, and route-hint constraints.
5. **Human graph comprehension.** Purchase et al. and Ware et al. support prioritizing crossings, path continuity, bends, and symmetry rather than optimizing a single geometric metric.
6. **Dynamic diagrams.** Incremental graph-layout research shows a trade-off between stability and static quality. Runtime events do not relayout by default; stable IDs preserve node positions and only styles change. Structural changes may request an explicit incremental or full layout.
7. **Visual-language usability.** Cognitive Dimensions motivates low viscosity and useful secondary notation. Semantic defaults remove boilerplate while explicit constraints remain available for exact compositions.

Primary references and DOIs are listed in `docs/research.md`.

## Input Model

### Native syntax

The native `.dtui` language is declarative and automatic by default:

```dtui
diagram developIssuesUntilApproved "developIssuesUntilApproved: full agent flow" {
  viewport 180x52
  direction right
  theme midnight

  group issues "parallel(\"issues\")" {
    layout stack
    group issue65 "issue-65" { use approval(issue = 65) }
    group issue66 "issue-66" { use approval(issue = 66) }
    group issueN  "issue-N"  { use approval(issue = N) }
  }

  input -> phase_issues -> issues -> issue_results -> phase_merge
  phase_merge -> merge -> cleanup -> phase_summary -> summary -> output

  align issue65 issue66 issueN vertical
  place issues before phase_merge
  route rejected below pass
}
```

Reusable templates define repeated structures. Nodes, groups, edges, and constraints have stable symbolic IDs. Labels are separate from IDs.

### DAWL protocol

The canonical JSON graph uses versioned records:

```json
{
  "schema": "dawl.diagram/v1",
  "title": "developIssuesUntilApproved: full agent flow",
  "nodes": [{"id":"flow.issues.developer","label":"Developer agent","kind":"agent","group":"issue-65"}],
  "edges": [{"id":"e1","from":"a","to":"b","kind":"success","label":"YES"}],
  "groups": [{"id":"issue-65","label":"issue-65","kind":"lane","parent":"issues"}],
  "constraints": []
}
```

Runtime updates are NDJSON records with stable graph IDs:

```json
{"schema":"dawl.event/v1","type":"node.started","node_id":"flow.issues.developer"}
```

Unknown additive fields are ignored. Unknown schema major versions fail with an actionable diagnostic.

## Architecture

The crate is split into focused modules, each below 200 lines:

- `syntax`: lexer, parser, source spans, diagnostics;
- `model`: canonical graph, constraints, semantic kinds, validation;
- `adapter`: DAWL JSON/NDJSON conversion;
- `layout`: cycle policy, ranks, lane ordering, coordinate assignment, groups;
- `route`: ports, occupancy grid, A* orthogonal routing, edge bundling;
- `canvas`: Unicode-aware terminal cells and box/line junction resolution;
- `theme`: semantic palette and style inheritance;
- `render`: graph-to-canvas painting and clipping;
- `app`: Ratatui event loop, panning, zoom levels, resize, watch mode;
- `export`: plain text, ANSI, and SVG snapshots;
- `cli`: commands and error presentation.

The library API remains independent from Ratatui. Batch rendering and tests use the same `Scene` and `Canvas` as the interactive application.

## Layout Pipeline

1. Validate identifiers, references, group ancestry, and constraint satisfiability.
2. Collapse nested groups into a compound hierarchy and compute node extents from Unicode display width.
3. Assign flow ranks using a deterministic longest-path/network-simplex-inspired solver with fixed tie-breaking.
4. Expand long edges with virtual routing vertices.
5. Order ranks using deterministic median/barycenter sweeps plus local transpositions.
6. Apply hard ordering/alignment constraints; score soft constraints.
7. Assign integer terminal-cell coordinates using separation constraints.
8. Compute group bounds bottom-up with title, border, and padding reservations.
9. Choose node ports and route edges over an occupancy grid with A*.
10. Improve routes locally by reducing crossings, bends, detours, and label collisions.
11. Paint groups, edges, nodes, labels, title, and optional metrics strip.

The objective is lexicographic, not a fragile weighted sum:

1. satisfy hard constraints;
2. avoid node/group intersections;
3. minimize edge crossings;
4. preserve path continuity;
5. minimize bends;
6. minimize edge length;
7. minimize area;
8. preserve previous positions when requested.

This ordering reflects both empirical readability findings and the needs of dense terminal diagrams.

## Routing

Edges are routed after placement on a cell grid. Obstacles include node interiors, group title cells, and reserved padding. Border crossings are allowed only through calculated gates.

A* state includes cell position and incoming direction. Its cost includes:

- one unit per cell;
- bend penalty;
- crossing penalty;
- overlap penalty;
- proximity penalty near labels and borders;
- preferred-lane bonus;
- strong direction penalty for back-edges unless explicitly routed otherwise.

Edges may share a trunk when compatible, but diverging semantic edges retain individually colored terminal segments. Junction glyphs are resolved from a four-direction connectivity mask.

## Rendering Fidelity

The reference example ships as `examples/approval.dtui` and targets `180x52` cells. Its golden ANSI/SVG snapshots verify:

- dark navy background;
- cyan compound boxes and phase nodes;
- blue developer and purple reviewer nodes;
- green success and red failure/retry paths;
- three vertically stacked issue lanes;
- separate merge approval group;
- cleanup and summary chain;
- bottom agent-count metrics panel;
- title and dense but non-overlapping labels.

Exact pixels depend on the terminal font and cell aspect ratio. The deterministic cell scene, ANSI output, and SVG reference are exact for a specified theme and cell geometry.

## Interaction

Commands:

- `dawl-tui render FILE --format ansi|text|svg`;
- `dawl-tui view FILE`;
- `dawl-tui watch --graph GRAPH.json --events EVENTS.ndjson`;
- `dawl-tui check FILE`;
- `dawl-tui explain FILE` for layout diagnostics.

Interactive controls: arrows/WASD pan, `+/-` change detail level, `0` fit, `/` search, `n/N` navigate matches, `g` toggle groups, `e` toggle edge labels, `q` quit. Terminal resize triggers viewport recomputation, not graph relayout.

## Error Handling

All parsing and validation failures carry a stable error code, source span or JSON pointer, concise message, and remediation hint. Layout failures produce a minimal unsatisfied-constraint set when possible. Terminal teardown uses RAII and panic hooks so raw mode and the alternate screen are restored.

## Testing Strategy

Development is acceptance-test-first.

- Parser tests cover valid syntax, recovery, spans, and malformed constraints.
- Model property tests cover stable serialization and validation invariants.
- Layout tests assert no overlap, deterministic coordinates, rank/order constraints, and stable IDs.
- Router tests assert obstacle avoidance and endpoint connectivity.
- Canvas tests cover Unicode width and all line-junction masks.
- Snapshot tests compare text, ANSI-stripped cells, and SVG.
- E2E tests launch the real CLI against `examples/approval.dtui` and compare output.
- DAWL compatibility tests consume a fixture matching DAWL's current node/edge/group graph and runtime events.
- Regression fixes begin with an E2E reproduction before unit-level isolation.

## Quality Gates

`cargo xtask verify` runs formatting, Clippy with warnings denied, unit/integration/E2E tests, source-policy checks, examples, documentation tests, and release build. Source-policy checks enforce:

- Rust source files at most 200 lines;
- functions at most 20 physical lines;
- low decision complexity;
- no production `unwrap`, `expect`, `panic`, or `todo`;
- module-level single-responsibility boundaries.

Criterion benchmarks measure parsing, layout, routing, rendering, and event projection at small, reference, and stress sizes. README documents benchmark and example commands.

## Delivery

The archive contains the complete Git repository and atomic commits. An ignored executable bootstrap script configures the GitHub remote for `mateusdcc/dawl-tui`, creates the repository when needed, verifies the toolchain, runs tests/examples/benchmarks, and pushes the commit history.
