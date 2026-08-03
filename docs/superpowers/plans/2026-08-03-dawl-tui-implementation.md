# DAWL TUI Diagram Engine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a standalone Rust CLI and library that parses a concise diagram DSL or DAWL graph/event JSON, computes deterministic compound layered layouts, routes orthogonal edges, and renders reference-quality interactive terminal diagrams plus text/ANSI/SVG snapshots.

**Architecture:** All inputs lower to a versioned canonical `Diagram` model. A deterministic pipeline validates, measures, ranks, orders, places, routes, and paints the graph into a terminal-cell `Scene`; batch and interactive frontends consume the same scene. The first release prioritizes the supplied approval-workflow composition and stable extensibility over general graph-layout optimality.

**Tech Stack:** Rust 1.85+, Cargo workspace, clap, serde/serde_json, pest, unicode-width, ratatui, crossterm, thiserror, indexmap, criterion, insta, proptest, assert_cmd, predicates, tempfile, syn/proc-macro2 for source-policy checks.

## Global Constraints

- Repository is standalone: `mateusdcc/dawl-tui`.
- Every Rust source file is at most 200 physical lines.
- Every Rust function is at most 20 physical lines.
- Production code has low decision complexity and no `unwrap`, `expect`, `panic`, `todo`, or `unimplemented`.
- Automatic layout is the default; optional constraints provide deterministic exact control.
- DAWL compatibility uses versioned JSON graph records and NDJSON runtime events with stable IDs.
- Runtime state updates do not relayout a structurally unchanged graph.
- Regression fixes begin with an E2E reproduction.
- Every task ends in a focused commit.

## File Map

- `Cargo.toml`: workspace metadata and shared dependencies.
- `crates/dawl-tui-core/src/model/*`: canonical diagram, constraints, validation, events.
- `crates/dawl-tui-core/src/syntax/*`: `.dtui` parser and diagnostics.
- `crates/dawl-tui-core/src/layout/*`: deterministic compound layered layout.
- `crates/dawl-tui-core/src/route/*`: ports, occupancy, orthogonal A* routing.
- `crates/dawl-tui-core/src/canvas/*`: Unicode terminal-cell scene and junction masks.
- `crates/dawl-tui-core/src/render/*`: theme, painting, text/ANSI/SVG export.
- `crates/dawl-tui-app/src/*`: Ratatui interactive view and event watch mode.
- `crates/dawl-tui-cli/src/*`: command parsing, IO, diagnostics, process exit codes.
- `xtask/src/*`: quality policy and verification orchestration.
- `tests/e2e/*`: end-user CLI tests.
- `examples/approval.dtui`: reference-image-equivalent diagram.
- `fixtures/dawl/*.json`: DAWL graph/event compatibility fixtures.
- `benches/*`: criterion benchmarks.

---

### Task 1: Workspace, Quality Gate, and CLI Smoke Test

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `clippy.toml`
- Create: `.gitignore`
- Create: `crates/dawl-tui-core/Cargo.toml`
- Create: `crates/dawl-tui-core/src/lib.rs`
- Create: `crates/dawl-tui-cli/Cargo.toml`
- Create: `crates/dawl-tui-cli/src/main.rs`
- Create: `xtask/Cargo.toml`
- Create: `xtask/src/main.rs`
- Create: `tests/e2e/help.rs`

**Interfaces:**
- Produces: binary `dawl-tui`; library crate `dawl_tui_core`; command `cargo xtask verify`.

- [ ] **Step 1: Write the failing CLI test**

```rust
#[test]
fn help_names_the_render_command() {
    let mut cmd = assert_cmd::Command::cargo_bin("dawl-tui").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("render"));
}
```

- [ ] **Step 2: Run the test and verify failure**

Run: `cargo test --test help`
Expected: FAIL because the workspace and binary do not exist.

- [ ] **Step 3: Create the minimal workspace and clap CLI**

```rust
#[derive(clap::Parser)]
#[command(name = "dawl-tui")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    Render { input: std::path::PathBuf },
}
```

- [ ] **Step 4: Implement `cargo xtask verify`**

The command runs `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` using `std::process::Command`, returning a nonzero status on the first failure.

- [ ] **Step 5: Run verification**

Run: `cargo xtask verify`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml rust-toolchain.toml clippy.toml .gitignore crates xtask tests
 git commit -m "build: initialize rust workspace and quality gate"
```

### Task 2: Canonical Diagram Model and Validation

**Files:**
- Create: `crates/dawl-tui-core/src/model/mod.rs`
- Create: `crates/dawl-tui-core/src/model/diagram.rs`
- Create: `crates/dawl-tui-core/src/model/kind.rs`
- Create: `crates/dawl-tui-core/src/model/constraint.rs`
- Create: `crates/dawl-tui-core/src/model/validate.rs`
- Create: `crates/dawl-tui-core/tests/model.rs`

**Interfaces:**
- Produces: `Diagram`, `Node`, `Edge`, `Group`, `Constraint`, `Diagram::validate() -> Result<(), Vec<ModelError>>`.

- [ ] **Step 1: Write validation tests**

```rust
#[test]
fn rejects_an_edge_with_an_unknown_endpoint() {
    let diagram = fixture().with_edge("a", "missing");
    let errors = diagram.validate().unwrap_err();
    assert!(errors.iter().any(|error| error.code() == "MODEL_UNKNOWN_NODE"));
}

#[test]
fn rejects_group_parent_cycles() {
    let diagram = cycle_fixture();
    assert!(diagram.validate().is_err());
}
```

- [ ] **Step 2: Run tests and verify failure**

Run: `cargo test -p dawl-tui-core --test model`
Expected: FAIL with unresolved model types.

- [ ] **Step 3: Implement serializable model types**

```rust
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Diagram {
    pub schema: String,
    pub id: String,
    pub title: String,
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
    pub groups: Vec<Group>,
    #[serde(default)]
    pub constraints: Vec<Constraint>,
}
```

Use string IDs, semantic enums with `#[serde(rename_all = "snake_case")]`, and optional parent/group references.

- [ ] **Step 4: Implement deterministic validation**

Validation checks schema major, duplicate IDs, endpoint references, group references, parent cycles, self-edges, and constraint references. Sort returned errors by `(code, path)`.

- [ ] **Step 5: Run tests and commit**

Run: `cargo test -p dawl-tui-core --test model`
Expected: PASS.

```bash
git add crates/dawl-tui-core
 git commit -m "feat: add canonical diagram model and validation"
```

### Task 3: DAWL JSON and Runtime Event Adapter

**Files:**
- Create: `crates/dawl-tui-core/src/model/event.rs`
- Create: `crates/dawl-tui-core/src/model/state.rs`
- Create: `crates/dawl-tui-core/src/adapter/mod.rs`
- Create: `crates/dawl-tui-core/src/adapter/json.rs`
- Create: `fixtures/dawl/approval-graph.json`
- Create: `fixtures/dawl/approval-events.ndjson`
- Create: `crates/dawl-tui-core/tests/dawl_adapter.rs`

**Interfaces:**
- Consumes: `Diagram` from Task 2.
- Produces: `read_dawl_graph(&str) -> Result<Diagram, AdapterError>`; `read_event(&str) -> Result<DiagramEvent, AdapterError>`; `DiagramState::apply(&DiagramEvent)`.

- [ ] **Step 1: Add a current-DAWL compatibility fixture**

The fixture uses DAWL's existing `{title,nodes,edges,groups}` shape, including `groupId`, `parentId`, and edge kinds `forward`, `success`, `failure`, and `back`.

- [ ] **Step 2: Write failing adapter tests**

```rust
#[test]
fn adapts_current_dawl_graph_shape() {
    let source = include_str!("../../../fixtures/dawl/approval-graph.json");
    let graph = read_dawl_graph(source).unwrap();
    assert_eq!(graph.nodes.len(), 6);
    assert_eq!(graph.schema, "dawl.diagram/v1");
}
```

- [ ] **Step 3: Implement tolerant field aliases**

Deserialize into private wire types with aliases for `groupId/group`, `parentId/parent`, then convert into the canonical model. Ignore unknown additive fields with normal Serde behavior.

- [ ] **Step 4: Implement state projection**

```rust
pub enum DiagramEvent {
    NodeStarted { node_id: String },
    NodeSucceeded { node_id: String },
    NodeFailed { node_id: String },
    EdgeTraversed { edge_id: String },
}
```

`DiagramState` stores node and edge statuses in `IndexMap` and never mutates layout data.

- [ ] **Step 5: Verify and commit**

Run: `cargo test -p dawl-tui-core --test dawl_adapter`
Expected: PASS.

```bash
git add crates/dawl-tui-core fixtures
 git commit -m "feat: adapt dawl graph and event streams"
```

### Task 4: Native `.dtui` Syntax and Diagnostics

**Files:**
- Create: `crates/dawl-tui-core/src/syntax/mod.rs`
- Create: `crates/dawl-tui-core/src/syntax/grammar.pest`
- Create: `crates/dawl-tui-core/src/syntax/ast.rs`
- Create: `crates/dawl-tui-core/src/syntax/parse.rs`
- Create: `crates/dawl-tui-core/src/syntax/lower.rs`
- Create: `crates/dawl-tui-core/src/error.rs`
- Create: `crates/dawl-tui-core/tests/syntax.rs`

**Interfaces:**
- Produces: `parse_dtui(source: &str) -> Result<Diagram, Vec<Diagnostic>>`.

- [ ] **Step 1: Write the syntax acceptance test**

```rust
#[test]
fn parses_groups_nodes_edges_and_constraints() {
    let graph = parse_dtui(SOURCE).unwrap();
    assert_eq!(graph.title, "Approval");
    assert_eq!(graph.groups[0].id, "issues");
    assert_eq!(graph.edges[0].from, "input");
    assert_eq!(graph.constraints.len(), 2);
}
```

- [ ] **Step 2: Define the grammar**

Support `diagram`, `viewport`, `direction`, `theme`, nested `group`, `node`, `decision`, `edge` chains using `->`, and constraints `align`, `place`, `port`, `route`, `size`, and `gap`. Strings support escapes; identifiers support `_-.`.

- [ ] **Step 3: Lower AST into the canonical model**

Group nesting supplies parent IDs. Chained edges get deterministic IDs derived from source order. Semantic defaults infer node kinds from declarations, never from label casing.

- [ ] **Step 4: Add span-rich error tests**

Malformed syntax must return `Diagnostic { code, message, span, hint }`. Assert the line/column for a missing closing brace and an unknown constraint target.

- [ ] **Step 5: Verify and commit**

Run: `cargo test -p dawl-tui-core --test syntax`
Expected: PASS.

```bash
git add crates/dawl-tui-core
 git commit -m "feat: parse declarative terminal diagram syntax"
```

### Task 5: Text Measurement and Scene Canvas

**Files:**
- Create: `crates/dawl-tui-core/src/canvas/mod.rs`
- Create: `crates/dawl-tui-core/src/canvas/cell.rs`
- Create: `crates/dawl-tui-core/src/canvas/grid.rs`
- Create: `crates/dawl-tui-core/src/canvas/line.rs`
- Create: `crates/dawl-tui-core/src/canvas/text.rs`
- Create: `crates/dawl-tui-core/tests/canvas.rs`

**Interfaces:**
- Produces: `Grid`, `Cell`, `StyleId`, `LineMask`, `display_width(&str)`, `Grid::write`, `Grid::connect`.

- [ ] **Step 1: Write Unicode and junction tests**

```rust
#[test]
fn wide_glyph_consumes_two_cells() {
    assert_eq!(display_width("界"), 2);
}

#[test]
fn four_way_mask_renders_cross() {
    assert_eq!(LineMask::all().glyph(), '┼');
}
```

- [ ] **Step 2: Implement a bounded cell grid**

Out-of-bounds writes return `false`; they never panic. Each cell stores glyph, style, and a line-connection mask. Text writing skips combining-width zero glyphs and marks continuation cells for width-two glyphs.

- [ ] **Step 3: Implement line junction resolution**

Resolve every 4-bit north/east/south/west mask to Unicode box-drawing glyphs. Dashed group borders use a separate paint path and do not merge with routed edges.

- [ ] **Step 4: Verify and commit**

Run: `cargo test -p dawl-tui-core --test canvas`
Expected: PASS.

```bash
git add crates/dawl-tui-core
 git commit -m "feat: add unicode terminal scene canvas"
```

### Task 6: Deterministic Layered Node Placement

**Files:**
- Create: `crates/dawl-tui-core/src/layout/mod.rs`
- Create: `crates/dawl-tui-core/src/layout/geometry.rs`
- Create: `crates/dawl-tui-core/src/layout/measure.rs`
- Create: `crates/dawl-tui-core/src/layout/rank.rs`
- Create: `crates/dawl-tui-core/src/layout/order.rs`
- Create: `crates/dawl-tui-core/src/layout/place.rs`
- Create: `crates/dawl-tui-core/src/layout/group.rs`
- Create: `crates/dawl-tui-core/tests/layout.rs`

**Interfaces:**
- Produces: `LayoutEngine::layout(&Diagram, &LayoutOptions) -> Result<Layout, LayoutError>`; `Layout { nodes, groups, size }`.

- [ ] **Step 1: Write invariant tests**

Tests assert that a chain ranks left-to-right, aligned lane groups stack vertically, node rectangles do not overlap, repeated runs yield identical coordinates, and child bounds remain inside parent bounds.

- [ ] **Step 2: Implement measurement**

Node width equals max label display width plus four cells, clamped by optional size constraints. Height equals label line count plus two. Groups reserve two title rows and one-cell inner padding.

- [ ] **Step 3: Implement rank assignment**

Compute strongly connected components, ignore semantic back-edges for forward rank assignment, condense the graph, and use deterministic longest-path ranks. Hard `place before/after` constraints become additional precedence edges.

- [ ] **Step 4: Implement rank ordering**

Initialize by source order. Run four alternating median sweeps and local adjacent transpositions, accepting swaps that reduce crossings while preserving hard order constraints.

- [ ] **Step 5: Assign integer coordinates and group bounds**

Place ranks with configurable horizontal gaps and nodes with vertical gaps. Apply alignment constraints, then calculate compound group rectangles bottom-up and shift colliding siblings apart.

- [ ] **Step 6: Verify and commit**

Run: `cargo test -p dawl-tui-core --test layout`
Expected: PASS.

```bash
git add crates/dawl-tui-core
 git commit -m "feat: lay out deterministic compound flow graphs"
```

### Task 7: Orthogonal Port Selection and A* Routing

**Files:**
- Create: `crates/dawl-tui-core/src/route/mod.rs`
- Create: `crates/dawl-tui-core/src/route/port.rs`
- Create: `crates/dawl-tui-core/src/route/grid.rs`
- Create: `crates/dawl-tui-core/src/route/search.rs`
- Create: `crates/dawl-tui-core/src/route/simplify.rs`
- Create: `crates/dawl-tui-core/tests/route.rs`

**Interfaces:**
- Consumes: `Layout` from Task 6.
- Produces: `Router::route(&Diagram, &Layout) -> Result<Vec<RoutedEdge>, RouteError>`.

- [ ] **Step 1: Write end-to-end route tests**

Assert paths begin/end at node borders, avoid node interiors, use only horizontal/vertical segments, honor an explicit south route hint, and route a back-edge below its decision loop.

- [ ] **Step 2: Build the occupancy grid**

Mark node interiors as blocked, borders as port-only, group titles as blocked, and group borders as gated. Reserve one-cell clearance around labels when space permits.

- [ ] **Step 3: Implement semantic port selection**

Forward edges prefer east-to-west, success/failure edges prefer east with vertical separation, and back edges prefer south-to-west. Explicit port constraints override defaults.

- [ ] **Step 4: Implement direction-aware A***

State is `(Point, Direction)`. Cost is distance plus bend, crossing, overlap, and proximity penalties. Use a binary heap with deterministic point/direction tie-breaking.

- [ ] **Step 5: Simplify paths**

Remove collinear intermediate points, calculate label anchors, and reject routes that cannot leave the endpoint border. Return an error carrying the edge ID and attempted ports.

- [ ] **Step 6: Verify and commit**

Run: `cargo test -p dawl-tui-core --test route`
Expected: PASS.

```bash
git add crates/dawl-tui-core
 git commit -m "feat: route semantic orthogonal connectors"
```

### Task 8: Semantic Theme and Diagram Painting

**Files:**
- Create: `crates/dawl-tui-core/src/render/mod.rs`
- Create: `crates/dawl-tui-core/src/render/theme.rs`
- Create: `crates/dawl-tui-core/src/render/paint.rs`
- Create: `crates/dawl-tui-core/src/render/label.rs`
- Create: `crates/dawl-tui-core/src/render/metrics.rs`
- Create: `crates/dawl-tui-core/tests/render.rs`

**Interfaces:**
- Produces: `render_scene(&Diagram, &Layout, &[RoutedEdge], &DiagramState, &Theme) -> Grid`.

- [ ] **Step 1: Write snapshot-oriented rendering tests**

Assert title, nested group labels, semantic node styles, green `YES`, red `NO`, retry loop, and the bottom metrics strip are present in the plain-cell projection.

- [ ] **Step 2: Define the midnight theme**

Map semantic styles to RGB values: navy background, cyan structure, blue agents, purple reviewers, green success, red failure, amber running, white title, and dim explanatory text.

- [ ] **Step 3: Paint in stable z-order**

Paint background, outer groups, inner groups, edge paths, edge labels/arrows, nodes, node labels, title, and metrics. Node paint overwrites edges; group borders do not overwrite node boxes.

- [ ] **Step 4: Apply runtime state without relayout**

Running/succeeded/failed state changes node border and label style. Traversed edges use active state while retaining success/failure hue.

- [ ] **Step 5: Verify and commit**

Run: `cargo test -p dawl-tui-core --test render`
Expected: PASS.

```bash
git add crates/dawl-tui-core
 git commit -m "feat: paint semantic terminal workflow scenes"
```

### Task 9: Text, ANSI, and SVG Export

**Files:**
- Create: `crates/dawl-tui-core/src/export/mod.rs`
- Create: `crates/dawl-tui-core/src/export/text.rs`
- Create: `crates/dawl-tui-core/src/export/ansi.rs`
- Create: `crates/dawl-tui-core/src/export/svg.rs`
- Create: `crates/dawl-tui-core/tests/export.rs`

**Interfaces:**
- Produces: `export_text(&Grid) -> String`; `export_ansi(&Grid, &Theme) -> String`; `export_svg(&Grid, &Theme, CellSize) -> String`.

- [ ] **Step 1: Write export tests**

Assert text contains no ANSI escapes, ANSI coalesces adjacent equal styles, SVG escapes labels, and SVG dimensions equal `grid_size * cell_size`.

- [ ] **Step 2: Implement text trimming**

Trim trailing blank cells per line but preserve internal spaces and blank lines inside the scene.

- [ ] **Step 3: Implement minimal ANSI transitions**

Emit truecolor foreground/background sequences only when the style changes and reset once at line end.

- [ ] **Step 4: Implement SVG terminal emulation**

Render one background rect per style run and one `<text>` element per glyph run using a generic monospace font family. Use configurable cell size defaulting to 9x18.

- [ ] **Step 5: Verify and commit**

Run: `cargo test -p dawl-tui-core --test export`
Expected: PASS.

```bash
git add crates/dawl-tui-core
 git commit -m "feat: export terminal scenes as text ansi and svg"
```

### Task 10: Reference Approval Diagram and Golden E2E Test

**Files:**
- Create: `examples/approval.dtui`
- Create: `tests/e2e/render_approval.rs`
- Create: `tests/snapshots/render_approval__approval_text.snap`
- Create: `tests/snapshots/render_approval__approval_svg.snap`
- Modify: `crates/dawl-tui-cli/src/main.rs`
- Create: `crates/dawl-tui-cli/src/command.rs`
- Create: `crates/dawl-tui-cli/src/io.rs`
- Create: `crates/dawl-tui-cli/src/render.rs`

**Interfaces:**
- Produces: working `dawl-tui render INPUT --format text|ansi|svg --width 180 --height 52`.

- [ ] **Step 1: Write the E2E test first**

```rust
#[test]
fn approval_example_matches_the_reference_composition() {
    let output = command().args([
        "render", "examples/approval.dtui", "--format", "text",
        "--width", "180", "--height", "52",
    ]).output().unwrap();
    insta::assert_snapshot!(String::from_utf8_lossy(&output.stdout));
}
```

- [ ] **Step 2: Run and verify failure**

Run: `cargo test --test render_approval`
Expected: FAIL because render command execution is incomplete.

- [ ] **Step 3: Author the full reference diagram**

Model input, phase nodes, the three issue groups, nested approval loops, issue-results join, merge approval group, cleanup, summary, output, and agent metrics. Add explicit align/place/route constraints only where automatic layout cannot reproduce the target composition.

- [ ] **Step 4: Wire the complete render pipeline**

CLI detects `.dtui` or JSON, validates, lays out, routes, paints, clips to viewport, and exports. Diagnostics go to stderr; successful diagram bytes go to stdout.

- [ ] **Step 5: Review snapshots visually**

Generate text and SVG. Compare SVG to the supplied image for hierarchy, spacing, colors, and routing; adjust only syntax constraints and algorithm constants, not fixture-specific code.

- [ ] **Step 6: Verify and commit**

Run: `cargo test --test render_approval`
Expected: PASS.

```bash
git add examples tests crates/dawl-tui-cli
 git commit -m "feat: reproduce the approval workflow diagram"
```

### Task 11: Interactive Ratatui Viewer and Event Watch Mode

**Files:**
- Create: `crates/dawl-tui-app/Cargo.toml`
- Create: `crates/dawl-tui-app/src/lib.rs`
- Create: `crates/dawl-tui-app/src/app.rs`
- Create: `crates/dawl-tui-app/src/input.rs`
- Create: `crates/dawl-tui-app/src/view.rs`
- Create: `crates/dawl-tui-app/src/terminal.rs`
- Create: `crates/dawl-tui-app/src/watch.rs`
- Modify: `crates/dawl-tui-cli/Cargo.toml`
- Modify: `crates/dawl-tui-cli/src/command.rs`
- Create: `tests/e2e/check.rs`

**Interfaces:**
- Produces: `dawl-tui view FILE`, `dawl-tui watch --graph FILE --events FILE|-`, and `dawl-tui check FILE`.

- [ ] **Step 1: Write noninteractive command tests**

`check` exits 0 for valid input and 2 for syntax/model errors. `watch` with a finite events file emits a final text snapshot under `--headless`.

- [ ] **Step 2: Implement RAII terminal guard**

Enter raw mode and alternate screen on construction. Restore cursor, raw mode, and main screen in `Drop`; install a panic hook that restores before delegating.

- [ ] **Step 3: Implement viewport state**

Store x/y offsets, detail level, search query, toggles, and scene. Resize changes only the visible rectangle. Keyboard handlers are table-driven to keep complexity low.

- [ ] **Step 4: Implement stateful Ratatui widget**

Translate each `Grid` cell into Ratatui buffer symbol/style. Reuse external app state, consistent with `StatefulWidget` semantics.

- [ ] **Step 5: Implement NDJSON watch loop**

Read one event per line, apply it to `DiagramState`, repaint with existing layout/routes, and redraw. Invalid lines report diagnostics but do not corrupt prior state.

- [ ] **Step 6: Verify and commit**

Run: `cargo test --workspace`
Expected: PASS.

```bash
git add crates/dawl-tui-app crates/dawl-tui-cli tests
 git commit -m "feat: add interactive viewer and live event watch"
```

### Task 12: Source Policy, Property Tests, and Failure Diagnostics

**Files:**
- Create: `xtask/src/policy.rs`
- Create: `xtask/src/functions.rs`
- Create: `xtask/src/complexity.rs`
- Modify: `xtask/src/main.rs`
- Create: `crates/dawl-tui-core/tests/properties.rs`
- Create: `tests/e2e/errors.rs`

**Interfaces:**
- Produces: `cargo xtask policy`; extended `cargo xtask verify`.

- [ ] **Step 1: Write policy self-tests**

Create temporary Rust files and assert detection of 201 lines, a 21-line function, forbidden macros/methods, and complexity above 8.

- [ ] **Step 2: Parse Rust source with `syn`**

Use span locations to calculate function line ranges. A visitor counts `if`, loop, match-arm beyond one, `&&`, `||`, and `?` as decisions. Skip test modules for forbidden `unwrap/expect`, but enforce size and function length everywhere.

- [ ] **Step 3: Add model/layout property tests**

Generate small DAGs; assert serialization round trips, layout determinism, no node overlap, and routed endpoint connectivity.

- [ ] **Step 4: Add actionable CLI diagnostic tests**

Assert errors contain stable code, file path, line/column or JSON pointer, message, and hint. Exit codes: 2 input error, 3 layout/routing error, 4 IO error.

- [ ] **Step 5: Verify and commit**

Run: `cargo xtask verify`
Expected: PASS.

```bash
git add xtask crates tests
 git commit -m "test: enforce source policy and layout invariants"
```

### Task 13: Benchmarks and Research Documentation

**Files:**
- Create: `benches/pipeline.rs`
- Create: `fixtures/bench/small.dtui`
- Create: `fixtures/bench/stress.json`
- Create: `docs/research.md`
- Create: `docs/protocol.md`
- Modify: `crates/dawl-tui-core/Cargo.toml`
- Modify: `Cargo.toml`

**Interfaces:**
- Produces: `cargo bench`; published DAWL graph/event protocol documentation.

- [ ] **Step 1: Add Criterion benchmark groups**

Benchmark parsing, validation, layout, routing, rendering, and event repaint for small, approval, and stress fixtures. Use `black_box` and fixed inputs.

- [ ] **Step 2: Add research synthesis**

Document algorithmic decisions and limitations with primary references: Sugiyama et al. 1981; Gansner et al. 1993; Tamassia 1987; Sander 1996; Lee 1961; Dwyer et al. 2006/2009; Purchase et al. 1996/1997/2000; Ware et al. 2002; Archambault and Purchase 2013; Green and Petre 1996.

- [ ] **Step 3: Document protocol guarantees**

Specify schemas, field aliases, additive compatibility, unknown-major rejection, event semantics, stable IDs, ordering, and examples for piping current DAWL output.

- [ ] **Step 4: Run benchmarks and commit**

Run: `cargo bench --bench pipeline -- --sample-size 10`
Expected: all benchmark groups complete without errors.

```bash
git add benches fixtures docs Cargo.toml crates/dawl-tui-core/Cargo.toml
 git commit -m "docs: add research protocol and pipeline benchmarks"
```

### Task 14: README, CI, and Final Verification

**Files:**
- Create: `README.md`
- Create: `LICENSE`
- Create: `.github/workflows/ci.yml`
- Modify: `.gitignore`

**Interfaces:**
- Produces: release-ready archive with full Git history, documented commands, and cross-platform verification.

- [ ] **Step 1: Write README usage**

Include installation, native syntax, DAWL JSON/NDJSON pipelines, render/view/watch/check commands, reference example, interaction keys, architecture, tests, benchmarks, and limitations.

- [ ] **Step 2: Add CI**

Run source-policy checks, `cargo check`, Clippy with warnings denied, and tests on Linux, macOS, and Windows. Run the benchmark smoke test on Linux.

- [ ] **Step 3: Run full verification**

Run:

```bash
python3 -m unittest scripts/test_quality.py
python3 scripts/quality.py
cargo check --all-targets
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo run -- render examples/approval.dtui --format text --width 180 --height 52
cargo run -- render examples/approval.dtui --format svg --output artifacts/approval.svg
cargo bench --bench pipeline
```

Expected: all commands succeed; generated diagram has no overlaps or clipped required labels.

- [ ] **Step 4: Inspect repository policy and history**

Run: `git status --short && git log --oneline --decorate`
Expected: clean worktree and focused commits.

- [ ] **Step 5: Commit**

```bash
git add README.md LICENSE .github .gitignore docs
 git commit -m "docs: finalize release workflow and usage"
```
