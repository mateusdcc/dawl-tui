# Research Basis

The implementation combines graph drawing, VLSI routing, human-computer interaction, and notation-design research. It does not claim globally optimal layout. Output happens to include terminal cells, but terminal dimensions do not constrain the topology pass or the intrinsic canvas size.

## Layered directed layout

Sugiyama, Tagawa, and Toda introduced the now-standard decomposition of hierarchical drawing into level assignment, within-level ordering to reduce crossings, and coordinate assignment. Gansner, Koutsofios, North, and Vo developed a practical four-pass directed-graph method using network-simplex ranking, crossing reduction, coordinate assignment, and edge drawing.

Mermaid delegates flowchart topology to Dagre by default and exposes stable diagram padding, node spacing, rank spacing, and label padding instead of asking authors for rectangles. `dawl-tui` now follows that architecture directly: intrinsic label dimensions and compound membership enter a Rust port of Dagre's complete Sugiyama pipeline. Network-simplex ranking, crossing minimization, and Brandes–Köpf coordinate assignment operate in continuous coordinates; conversion to integer cells happens once, at the layout boundary. The local A* router remains a separate stage because terminal and SVG exports share orthogonal edge semantics.

- K. Sugiyama, S. Tagawa, M. Toda, “Methods for Visual Understanding of Hierarchical System Structures,” IEEE TSMC 11(2), 1981. DOI: 10.1109/TSMC.1981.4308636.
- E. R. Gansner, E. Koutsofios, S. C. North, K.-P. Vo, “A Technique for Drawing Directed Graphs,” IEEE TSE 19(3), 1993. DOI: 10.1109/32.221135.
- Mermaid, “Layouts,” <https://mermaid.ai/open-source/config/layouts.html>.
- Mermaid, “Flowchart Diagram Config Schema,” <https://mermaid.js.org/config/schema-docs/config-defs-flowchart-diagram-config.html>.
- Dagre, “Directed graph layout for JavaScript,” <https://github.com/dagrejs/dagre>.
- `dagre` Rust port, <https://docs.rs/dagre/0.1.1/dagre/>.

## Compound graphs and constraints

Workflow diagrams are compound graphs: issue lanes contain worktree scopes, retry loops, and agents. Sander’s compound-layout work motivates bottom-up group bounds around globally layered children. Constraint-layout research shows why automatic layout should accept separation, ordering, grouping, and topology-preserving overrides instead of forcing users to choose between automation and manual coordinates.

- G. Sander, “Layout of Compound Directed Graphs,” Universität des Saarlandes Technical Report A/03/96, 1996. DOI: 10.22028/D291-25806.
- T. Dwyer, Y. Koren, K. Marriott, “IPSep-CoLa,” IEEE TVCG 12(5), 2006. DOI: 10.1109/TVCG.2006.156.
- T. Dwyer, Y. Koren, K. Marriott, “Constrained Graph Layout by Stress Majorization and Gradient Projection,” Discrete Mathematics 309(7), 2009. DOI: 10.1016/j.disc.2007.12.103.
- T. Dwyer, K. Marriott, M. Wybrow, “Topology Preserving Constrained Graph Layout,” GD 2008, 2009. DOI: 10.1007/978-3-642-00219-9_22.

## Output scale is not a layout constraint

The canonical layout is an unbounded scene sized from its content. Text and ANSI exports quantize that scene to cells, while SVG uses the same scene at a configurable pixel scale. A small terminal is a viewport over the scene, not a reason to compress the graph or require manual coordinates.

Modern terminal graphics reinforce the separation between scene size and character cells. Kitty's graphics protocol accepts arbitrary raster graphics with pixel offsets and placements, and WezTerm's image tooling accepts pixel dimensions with a default 25-million-pixel frame budget. `dawl-tui` does not need either protocol to compute layout, but their existence rules out treating an 80×24 character grid as an architectural ceiling.

- Kitty, “Terminal graphics protocol,” <https://sw.kovidgoyal.net/kitty/graphics-protocol/>.
- WezTerm, “iTerm Image Protocol,” <https://wezterm.org/imgcat.html>.
- WezTerm, “`wezterm imgcat`,” <https://wezterm.org/cli/imgcat.html>.

## Orthogonal routing

Lee’s maze-routing algorithm established exhaustive grid path finding for wiring and diagram connection problems. Tamassia formulated bend minimization for orthogonal grid drawings through minimum-cost flow. The renderer uses direction-aware A* rather than claiming bend optimality: distance, bends, overlap, and obstacle proximity contribute to cost, while explicit ports and waypoints remain authoritative.

- C. Y. Lee, “An Algorithm for Path Connections and Its Applications,” IRE Transactions on Electronic Computers 10(3), 1961. DOI: 10.1109/TEC.1961.5219222.
- R. Tamassia, “On Embedding a Graph in the Grid with the Minimum Number of Bends,” SIAM Journal on Computing 16(3), 1987. DOI: 10.1137/0216030.

## Perceptual criteria

Graph aesthetics are not interchangeable. Controlled studies support treating crossings, path continuity, bends, and task context as separate concerns. The router therefore gives bend changes a substantial cost, semantic edge classes stable colors, and node boxes paint priority over connectors. Runtime events change style without moving geometry, preserving orientation for tasks where spatial memory matters while avoiding an unconditional claim that mental-map preservation helps every task.

- H. C. Purchase, R. F. Cohen, M. I. James, “An Experimental Study of the Basis for Graph Drawing Algorithms,” ACM JEA 2, 1997. DOI: 10.1145/264216.264222.
- C. Ware, H. Purchase, L. Colpoys, M. McGill, “Cognitive Measurements of Graph Aesthetics,” Information Visualization 1(2), 2002. DOI: 10.1057/palgrave.ivs.9500013.
- D. Archambault, H. C. Purchase, “Mental Map Preservation Helps User Orientation in Dynamic Graphs,” GD 2012, 2013. DOI: 10.1007/978-3-642-36763-2_42.
- D. Archambault, H. C. Purchase, “The ‘Map’ in the Mental Map,” IJHCS 71(11), 2013. DOI: 10.1016/j.ijhcs.2013.08.004.

## Notation design

Green and Petre’s cognitive-dimensions framework highlights viscosity, closeness of mapping, and secondary notation. The `.dtui` language therefore uses one semantic declaration per line, permits omission of layout detail, and exposes positions, colors through semantic kinds, and route constraints as optional secondary notation.

- T. R. G. Green, M. Petre, “Usability Analysis of Visual Programming Environments: A Cognitive Dimensions Framework,” Journal of Visual Languages & Computing 7(2), 1996. DOI: 10.1006/jvlc.1996.0009.

## Engineering consequences

1. Parse into a stable graph IR before any rendering decision.
2. Keep ranking, placement, routing, painting, and export independently testable.
3. Measure labels before layout and apply one standard padding and spacing policy.
4. Quantize continuous layout coordinates only when producing the shared grid.
5. Prefer deterministic tie-breaking over randomized aesthetic search.
6. Preserve coordinates while applying runtime events.
7. Keep exact ports, waypoints, alignment, and positions as optional overrides.
8. Benchmark the complete pipeline, not only individual graph algorithms.
