# DAWL TUI Diagram Engine — Implemented Design

## Purpose

`dawl-tui` is a standalone Rust renderer for dense, deterministic terminal diagrams. It accepts a compact line-oriented notation or DAWL-compatible JSON, lowers both to one canonical compound-graph model, and emits plain text, ANSI, SVG, or an interactive terminal viewport.

The reference approval workflow is an explicit `180 × 52` composition. Normal diagrams can omit coordinates and rely on deterministic automatic placement; exact compositions progressively add positions, sizes, ports, alignment, placement, and route waypoints without changing semantic IDs.

## Product Boundary

Repository: `mateusdcc/dawl-tui`.

The project owns:

- native diagram parsing and diagnostics;
- canonical nodes, edges, groups, constraints, and semantic kinds;
- normalization of DAWL graph JSON and finite NDJSON runtime events;
- deterministic layered placement and compound-group bounds;
- orthogonal routing with explicit-route escape hatches;
- Unicode-aware terminal-cell painting;
- text, ANSI, SVG, and interactive viewport output;
- acceptance tests, source-policy checks, examples, benchmarks, and research documentation.

It does not parse or execute DAWL source. DAWL remains responsible for workflow semantics and supplies its compiled graph and event records.

## Research Basis

The implementation combines established results from several domains:

1. **Layered graph drawing.** Sugiyama, Tagawa, and Toda motivate rank assignment and within-rank ordering. Gansner et al. motivate the staged directed-graph pipeline and deterministic coordinate assignment.
2. **Compound graphs.** Sander motivates global placement across nested rectangular groups instead of independently drawing each group.
3. **Orthogonal and VLSI routing.** Tamassia separates topology, shape, and metrics; Lee motivates grid search around obstacles. The renderer therefore routes after placement.
4. **Constraint-based layout.** Dwyer, Koren, and Marriott motivate optional separation and alignment constraints layered over automatic placement.
5. **Graph comprehension.** Purchase, Ware, and later mental-map studies motivate prioritizing crossings, path continuity, bends, and spatial stability.
6. **Visual-language usability.** Cognitive Dimensions motivates semantic defaults, stable symbolic IDs, and optional secondary notation for editorial control.

Primary references and implementation consequences are recorded in `docs/research.md`.

## Inputs

### Native notation

A declaration occupies one line. Coordinates and sizes are optional:

```dtui
diagram approval "Approval flow"
viewport 120x40
direction right
theme midnight

group loop "iteration 1…M" kind repeat dashed
node developer "Developer agent" kind agent in loop
node reviewer "Reviewer agent" kind reviewer in loop
decision pass "pass?" in loop
node failed "failed review" kind failure in loop

edge review developer -> reviewer
edge decide reviewer -> pass
edge no pass -> failed kind failure label "NO"
edge retry failed -> developer kind back label "findings"

align horizontal developer reviewer pass
place developer before reviewer
```

Exact authority can be added locally:

```dtui
node developer "Developer agent" at 45,9 size 15x4 kind agent in loop
edge retry failed -> developer kind back from_port west to_port south via 67,14 67,16 52,16 52,13
```

### DAWL graph protocol

The adapter accepts the current DAWL graph shape and the versioned envelope:

```json
{
  "schema": "dawl.diagram/v1",
  "title": "Approval flow",
  "nodes": [{"id":"developer","label":"Developer","kind":"agent","groupId":"loop"}],
  "edges": [{"id":"review","from":"developer","to":"reviewer","kind":"forward"}],
  "groups": [{"id":"loop","label":"iteration 1…M","kind":"repeat","parentId":"issues"}]
}
```

Missing optional envelope fields receive deterministic defaults. Camel-case `groupId` and `parentId` normalize to the internal `group` and `parent` fields. Current DAWL kinds including `fork`, `join`, `value`, `return`, and `function` are accepted.

Runtime updates are finite NDJSON records. Supported records include `node.started`, `node.completed`, `node.succeeded`, `node.failed`, `condition.evaluated`, `retry.scheduled`, and `edge.traversed`. Events update semantic style only and never relayout the graph.

## Architecture

The library keeps each responsibility in a focused module:

- `parser`: line-oriented syntax and source diagnostics;
- `input`: native/JSON detection, standard-input support, and DAWL normalization;
- `model`: canonical types, semantic kinds, and validation;
- `layout`: explicit placement, deterministic layered placement, constraints, and group bounds;
- `route`: ports, occupancy, orthogonal A* search, back-edge routing, and waypoint handling;
- `state`: runtime-event projection and fuzzy DAWL ID matching;
- `canvas`: Unicode cell storage, boxes, paths, and junction resolution;
- `theme`: semantic truecolor palette;
- `render`: groups, edges, nodes, labels, title, and text panels;
- `export`: plain text, ANSI, and SVG;
- `tui`: Ratatui viewport and terminal restoration;
- `cli` and `interactive`: command dispatch and finite event replay.

Batch output and the TUI use the same layout, routes, semantic state, and terminal-cell grid.

## Layout and Routing

The implemented pipeline is:

1. Validate unique IDs, references, group ancestry, constraints, and group cycles.
2. Preserve all explicitly positioned nodes.
3. Rank unpositioned nodes with a deterministic longest-path pass.
4. Order ranks with deterministic median sweeps and stable ID tie-breaking.
5. Assign integer terminal-cell coordinates from measured Unicode label width.
6. Infer nested group bounds bottom-up.
7. Apply hard `align` and `place` constraints to nodes or entire groups.
8. Refresh inferred group bounds after movement.
9. Select edge ports and route orthogonally over node-interior obstacles.
10. Use explicit ports and `via` points as authoritative route control.
11. Paint into a clipped terminal-cell canvas.

Automatic layout is intentionally deterministic rather than globally optimal. Explicit coordinates and waypoints are the exact-composition mechanism.

## Rendering Fidelity

`examples/approval.dtui` targets `180 × 52` cells and reproduces the reference composition’s structure:

- dark navy background;
- cyan compound boxes and phase nodes;
- blue developer and purple reviewer nodes;
- green success and red failure/retry paths;
- three vertically stacked issue lanes;
- a distinct merge approval region;
- cleanup and summary chain;
- bottom agent-count panel;
- title and dense non-overlapping labels.

Text and ANSI output are exact at the cell level. SVG uses a documented monospace cell geometry; the appearance of terminal glyphs still depends on the selected font.

## Commands and Interaction

```text
dawl-tui render INPUT [--format text|ansi|svg] [--output FILE]
                      [--width CELLS] [--height CELLS]
dawl-tui check INPUT
dawl-tui view INPUT
dawl-tui watch --graph GRAPH.json --events EVENTS.ndjson [--headless]
```

`render -` accepts a DAWL graph from standard input. `watch` consumes a finite event file and renders the final projected state. The interactive viewport supports arrow-key panning, `0` to reset, and `q` or Escape to exit. Resizing changes the visible viewport rather than recomputing the graph.

## Error Handling

Errors carry a stable code, category, message, optional hint, and deterministic exit code:

- `2`: native syntax, JSON, or model input error;
- `3`: layout or route error;
- `4`: I/O error.

The TUI uses Ratatui terminal initialization/restoration so raw mode and the alternate screen are restored on normal exit and through the installed panic hook.

## Testing and Quality Gates

The repository contains unit, integration, property-style, and real CLI tests for:

- native syntax and diagnostics;
- model references and cyclic compound groups;
- deterministic placement and hard constraints;
- routing connectivity and actionable failures;
- Unicode cell width and line junctions;
- ANSI and SVG output;
- exact reference composition;
- current DAWL graph normalization;
- runtime state and traversed-edge projection;
- standard-input rendering and command exit codes.

A Python source-policy checker enforces every Rust file at no more than 200 lines, every function at no more than 20 physical lines, low decision complexity, and no production `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!`. Cargo verification adds compilation, Clippy with warnings denied, and the complete test suite. The benchmark measures parse, layout, route, render, event repaint, and full-pipeline time.

## Delivery State

The archive contains the complete Git repository with focused commits, documentation, CI for Linux/macOS/Windows, the reference image and composition, DAWL fixtures, tests, benchmarks, and an MIT license.
