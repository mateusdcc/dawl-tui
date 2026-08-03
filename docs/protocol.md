# DAWL Diagram Protocol

`dawl-tui` accepts the canonical `dawl.diagram/v1` JSON graph emitted by DAWL and a native `.dtui` notation. Both inputs normalize to the same model before validation, layout, routing, painting, and export.

## Canonical graph

```json
{
  "schema": "dawl.diagram/v1",
  "id": "approval",
  "title": "Approval flow",
  "direction": "right",
  "nodes": [
    { "id": "developer", "label": "Developer", "kind": "agent", "groupId": "loop" }
  ],
  "edges": [
    { "id": "review", "from": "developer", "to": "reviewer", "kind": "forward" }
  ],
  "groups": [
    { "id": "loop", "label": "iteration 1…M", "kind": "repeat", "parentId": "issues" }
  ]
}
```

`groupId` and `parentId` are accepted aliases for the normalized `group` and `parent` fields. IDs are semantic identity, not display labels. They must remain stable across runtime events and re-renders.

## Runtime events

The event stream is newline-delimited JSON. Supported events are:

```json
{"type":"node.started","nodeId":"developer"}
{"type":"node.completed","nodeId":"developer"}
{"type":"node.failed","nodeId":"reviewer"}
{"type":"condition.evaluated","nodeId":"pass","result":false}
{"type":"retry.scheduled","nodeId":"approval.repeat"}
{"type":"edge.traversed","edgeId":"review","status":"active"}
```

Snake-case `node_id` and `edge_id` are accepted, and `node.succeeded` is an alias of `node.completed`. Condition and retry records project onto semantic success, failure, and back edges. Events update styling only; they do not trigger layout, which preserves spatial stability.

## Native notation

```dtui
diagram approval "Approval flow"
viewport 120x40
theme midnight
direction right

group loop "iteration 1…M" at 20,4 size 45x16 kind repeat dashed
node developer "Developer agent" at 23,8 size 16x4 kind agent in loop
node reviewer "Reviewer agent" at 44,8 size 16x4 kind reviewer in loop
edge review developer -> reviewer kind forward
```

Automatic placement is the default when `at` and `size` are omitted. Exact compositions can add deterministic constraints:

```dtui
align horizontal developer reviewer
place developer before reviewer
edge retry failed -> developer kind back from_port west to_port south via 60,18 30,18
```

The syntax deliberately separates semantic content from optional layout authority. DAWL generators can emit only the graph; handcrafted diagrams can progressively add positions, ports, alignments, and route points without changing the renderer.
