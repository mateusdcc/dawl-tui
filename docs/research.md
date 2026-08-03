# Research Basis

The implementation combines graph drawing, VLSI routing, human-computer interaction, and notation-design research. It does not claim globally optimal layout. It uses deterministic approximations selected for terminal constraints, predictable output, and explicit user authority.

## Layered directed layout

Sugiyama, Tagawa, and Toda introduced the now-standard decomposition of hierarchical drawing into level assignment, within-level ordering to reduce crossings, and coordinate assignment. Gansner, Koutsofios, North, and Vo developed a practical four-pass directed-graph method using network-simplex ranking, crossing reduction, coordinate assignment, and edge drawing. `dawl-tui` adopts this separation: rank assignment is independent from coordinate placement and routing.

- K. Sugiyama, S. Tagawa, M. Toda, “Methods for Visual Understanding of Hierarchical System Structures,” IEEE TSMC 11(2), 1981. DOI: 10.1109/TSMC.1981.4308636.
- E. R. Gansner, E. Koutsofios, S. C. North, K.-P. Vo, “A Technique for Drawing Directed Graphs,” IEEE TSE 19(3), 1993. DOI: 10.1109/32.221135.

## Compound graphs and constraints

Workflow diagrams are compound graphs: issue lanes contain worktree scopes, retry loops, and agents. Sander’s compound-layout work motivates bottom-up group bounds around globally layered children. Constraint-layout research shows why automatic layout should accept separation, ordering, grouping, and topology-preserving overrides instead of forcing users to choose between automation and manual coordinates.

- G. Sander, “Layout of Compound Directed Graphs,” Universität des Saarlandes Technical Report A/03/96, 1996. DOI: 10.22028/D291-25806.
- T. Dwyer, Y. Koren, K. Marriott, “IPSep-CoLa,” IEEE TVCG 12(5), 2006. DOI: 10.1109/TVCG.2006.156.
- T. Dwyer, Y. Koren, K. Marriott, “Constrained Graph Layout by Stress Majorization and Gradient Projection,” Discrete Mathematics 309(7), 2009. DOI: 10.1016/j.disc.2007.12.103.
- T. Dwyer, K. Marriott, M. Wybrow, “Topology Preserving Constrained Graph Layout,” GD 2008, 2009. DOI: 10.1007/978-3-642-00219-9_22.

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
3. Prefer deterministic tie-breaking over randomized aesthetic search.
4. Preserve coordinates while applying runtime events.
5. Allow exact ports, waypoints, alignment, and position constraints.
6. Benchmark the complete pipeline, not only individual graph algorithms.
