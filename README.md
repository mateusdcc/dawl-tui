# dawl-tui

Deterministic compound workflow diagrams for the terminal, written in Rust.

![Approval Workflow Diagram](docs/assets/approval.png)

`dawl-tui` turns either a compact native notation or a DAWL graph into a dense Unicode/ANSI diagram. It combines automatic layered placement with explicit constraints, ports, and route points, so ordinary graphs need little layout syntax while reference-grade compositions remain reproducible cell for cell.

## Capabilities

- Nested compound groups, lanes, repeat regions, panels, and function scopes.
- Deterministic layered placement with median sweeps for crossing reduction.
- Hard `align` and `place` constraints for nodes and complete groups.
- Orthogonal A* routing with bend, overlap, and obstacle-proximity costs.
- Directional track occupancy, repeated vector arrowheads, and true diamond decisions.
- Explicit ports and `via` points when exact routing is required.
- Semantic truecolor themes for agents, reviewers, decisions, phases, outcomes, and runtime state.
- Plain text, ANSI, SVG, interactive viewport, and finite event-stream replay.
- Direct compatibility with DAWL's current `{ title, nodes, edges, groups }` graph and NDJSON runtime events.

## Install

Rust 1.88 or newer is required.

```bash
cargo install --path .
```

For development:

```bash
cargo build --release
./target/release/dawl-tui --help
```

## Examples

### 1. Multi-Agent Approval Flow

Real output rendered from `examples/approval.dtui` (`202 × 72` scene):

![Approval Flow Diagram Output](docs/assets/approval.png)

```bash
dawl-tui render examples/approval.dtui --format ansi --width 202 --height 72
```

The SVG reference uses exact grid-aligned strokes, centered monospace labels, true decision diamonds, and periodic arrowheads on long routes so direction remains visible throughout the flow.

### 2. CI/CD Deployment Pipeline

Real output rendered from `examples/simple.dtui`:

![CI/CD Pipeline Diagram Output](docs/assets/simple.png)

```bash
dawl-tui render examples/simple.dtui --format ansi --width 100 --height 26
```

## Render diagrams

Other deterministic exports:

```bash
dawl-tui render examples/approval.dtui --format text --output approval.txt
dawl-tui render examples/approval.dtui --format svg --output approval.svg
```

Open an interactive viewport for terminals smaller than the scene:

```bash
dawl-tui view examples/approval.dtui
```

Arrow keys pan, `0` resets the viewport, and `q` or Escape exits.

## Native syntax

A declaration occupies one line. Coordinates and sizes are optional.

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

Automatic placement is the default. Exact layouts can progressively add authority without changing semantic IDs:

```dtui
node developer "Developer agent" at 46,9 size 15x5 kind agent in loop
edge retry failed -> developer kind back from_port west to_port south via 76,16 76,20 53,20 53,13
```

Supported placement relations are `before`, `after`, `above`, and `below`. Alignment is `horizontal` or `vertical`. Ports are `north`, `east`, `south`, and `west`.

## DAWL integration

The JSON adapter accepts DAWL's camel-case `groupId` and `parentId` fields, current semantic kinds such as `fork`, `join`, `value`, `return`, and `function`, and missing optional envelope fields.

Pipe a graph directly through standard input:

```bash
produce-dawl-graph | dawl-tui render - --format ansi
```

A minimal DAWL exporter can use DAWL's existing library API:

```js
import { readFileSync } from "node:fs";
import { compile, parse } from "dawl";

const source = readFileSync(process.argv[2], "utf8");
const graph = compile(parse(source), process.argv[3]);
process.stdout.write(JSON.stringify(graph));
```

Replay a finite DAWL event stream and emit the final state without entering the TUI:

```bash
dawl-tui watch \
  --graph fixtures/dawl/approval-graph.json \
  --events fixtures/dawl/approval-events.ndjson \
  --headless
```

Recognized runtime events include `node.started`, `node.completed`, `node.succeeded`, `node.failed`, `condition.evaluated`, `retry.scheduled`, and `edge.traversed`. Stable and fuzzy DAWL IDs are projected onto the graph without relayout, preserving spatial orientation.

The versioned protocol is specified in [`docs/protocol.md`](docs/protocol.md).

## Commands

```text
dawl-tui render INPUT [--format text|ansi|svg] [--output FILE]
                      [--width CELLS] [--height CELLS]
dawl-tui check INPUT
dawl-tui view INPUT
dawl-tui watch --graph GRAPH.json --events EVENTS.ndjson [--headless]
```

Exit codes are stable: `2` for input/model errors, `3` for layout/routing errors, and `4` for I/O errors.

## Architecture

The pipeline keeps semantic stages independent:

```text
.dtui / DAWL JSON
        │
        ▼
 parse + normalize ──► canonical graph ──► validate
        │                                      │
        ▼                                      ▼
 layered placement ──► compound constraints ──► group bounds
        │
        ▼
 orthogonal routing ──► terminal-cell canvas ──► text / ANSI / SVG / TUI
```

Runtime events update only `DiagramState`. Layout coordinates and routes remain stable unless the graph itself is rendered again.

## Verification

```bash
python3 -m unittest scripts/test_quality.py
python3 scripts/quality.py
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
```

The source policy checks every Rust file for the requested limits: at most 200 lines per file, at most 20 physical lines per function, low decision complexity, and no production `unwrap`, `expect`, `panic!`, `todo!`, or `unimplemented!`.

Integration tests exercise the real CLI, the exact approval composition, DAWL graph normalization, DAWL runtime-event projection, compound constraints, routing diagnostics, Unicode cell width, ANSI coalescing, and SVG escaping.

## Benchmarks

```bash
cargo bench --bench pipeline
```

The benchmark reports parse, layout, route, render, event repaint, and complete pipeline time over a fixed number of iterations. It prints measurements rather than embedding machine-specific results in the repository.

## Research basis

The design draws from layered graph layout, compound graph drawing, constrained layout, VLSI maze routing, orthogonal bend minimization, graph-comprehension experiments, dynamic mental-map research, and Cognitive Dimensions. The algorithms are deterministic approximations adapted to integer terminal cells; they do not claim globally optimal drawings.

The primary references, DOIs, implementation consequences, and limitations are documented in [`docs/research.md`](docs/research.md).

## Current limitations

- SVG fidelity assumes a monospace font and the selected cell geometry; terminal fonts can differ in glyph metrics.
- Automatic layout optimizes a deterministic hierarchy and crossing heuristic, not a global aesthetic optimum.
- Explicit `at`, `size`, ports, and waypoints remain the authoritative mechanism for exact editorial compositions.
- Event files are currently consumed as finite replays; following an indefinitely growing file is outside the first release boundary.

## License

MIT.
