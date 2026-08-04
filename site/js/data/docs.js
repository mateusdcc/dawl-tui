/* Documentation Data Module */
export const DOCS_DATA = [
  {
    id: "getting-started",
    title: "1. Getting Started & Installation",
    content: `
<h3>Installation</h3>
<p>Ensure Rust 1.88+ is installed. Install <code>dawl-tui</code> via Cargo:</p>
<pre><code>cargo install dawl-tui --locked
dawl-tui --version</code></pre>

<h3>Quick Execution</h3>
<p>Render a workflow diagram directly from STDIN into ANSI color:</p>
<pre><code>dawl-tui render - --format ansi &lt;&lt;'DTUI'
diagram release "Release Gate"
direction right
theme midnight

node build "Build Service" kind activity
node deploy "Production" kind success
edge e1 build -> deploy
DTUI</code></pre>`
  },
  {
    id: "cli-commands",
    title: "2. CLI Command Reference",
    content: `
<h3>dawl-tui CLI Syntax</h3>
<p>Available commands and flags:</p>
<ul>
  <li><code>dawl-tui render &lt;FILE&gt; [--format ansi|svg|text]</code> - Renders a .dtui or JSON workflow file.</li>
  <li><code>dawl-tui inspect &lt;FILE&gt;</code> - Outputs parsed graph metrics, crossing counts, and layout grid dimensions.</li>
  <li><code>dawl-tui replay &lt;EVENTS.ndjson&gt;</code> - Plays finite NDJSON runtime event stream interactively in terminal.</li>
</ul>`
  },
  {
    id: "dtui-syntax",
    title: "3. .dtui Syntax Guide",
    content: `
<h3>Native Syntax Specification</h3>
<p>Definitions used in <code>.dtui</code> graph files:</p>
<ul>
  <li><code>diagram &lt;id&gt; "&lt;title&gt;"</code> - Header declaration.</li>
  <li><code>viewport &lt;WxH&gt;</code> - Optional canvas size constraint (e.g. 120x32).</li>
  <li><code>direction right|down</code> - Graph layout flow axis.</li>
  <li><code>node &lt;id&gt; "&lt;label&gt;" kind &lt;type&gt;</code> - Node with type: <em>activity, reviewer, agent, success, failure, input, output, fork, join</em>.</li>
  <li><code>group &lt;id&gt; "&lt;label&gt;" kind parallel|lane|repeat</code> - Compound container.</li>
  <li><code>edge &lt;id&gt; &lt;src&gt; -&gt; &lt;dst&gt; [kind &lt;k&gt;] [label "&lt;lbl&gt;"]</code> - Directed connection.</li>
  <li><code>decision &lt;id&gt; "&lt;label&gt;"</code> - True diamond decision node.</li>
  <li><code>align horizontal|vertical &lt;nodes...&gt;</code> - Hard grid alignment constraint.</li>
</ul>`
  },
  {
    id: "dawl-json",
    title: "4. DAWL JSON & NDJSON Protocol",
    content: `
<h3>JSON Schema Compatibility</h3>
<p>dawl-tui directly parses DAWL graph structures:</p>
<pre><code>{
  "title": "Approval System",
  "nodes": [
    { "id": "n1", "label": "Build", "kind": "activity" }
  ],
  "edges": [
    { "id": "e1", "source": "n1", "target": "n2" }
  ]
}</code></pre>`
  },
  {
    id: "rust-api",
    title: "5. Rust Library API",
    content: `
<h3>Embedding in Rust Applications</h3>
<p>Add to your <code>Cargo.toml</code>:</p>
<pre><code>[dependencies]
dawl-tui = "0.1"</code></pre>
<p>Render programmatically in Rust code:</p>
<pre><code>use dawl_tui::{Graph, RenderOptions, OutputFormat};

let graph = Graph::parse_dtui(dtui_source)?;
let ansi_output = graph.render(&RenderOptions {
    format: OutputFormat::Ansi,
    theme: "midnight".to_string(),
})?;
println!("{}", ansi_output);</code></pre>`
  }
];
