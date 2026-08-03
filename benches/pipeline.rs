use std::hint::black_box;
use std::time::Instant;

use dawl_tui::{export, layout_diagram, parser::parse, render_diagram};
use dawl_tui::route::route_diagram;
use dawl_tui::DiagramState;

const SOURCE: &str = include_str!("../examples/approval.dtui");
const EVENT: &str = r#"{"type":"node.started","nodeId":"dev65"}"#;
const ITERATIONS: u32 = 100;

fn main() {
    let diagram = parse(SOURCE).expect("benchmark fixture parses");
    let layout = layout_diagram(&diagram, &Default::default()).expect("layout succeeds");
    benchmark("parse", || { black_box(parse(SOURCE).expect("parse")); });
    benchmark("layout", || { black_box(layout_diagram(&diagram, &Default::default()).expect("layout")); });
    benchmark("route", || { black_box(route_diagram(&diagram, &layout).expect("route")); });
    benchmark("render", || { black_box(render_diagram(&diagram, &layout, &Default::default()).expect("render")); });
    benchmark("event repaint", || event_repaint(&diagram, &layout));
    benchmark("full pipeline", full_pipeline);
}

fn event_repaint(diagram: &dawl_tui::Diagram, layout: &dawl_tui::Layout) {
    let mut state = DiagramState::default();
    state.apply_json_with_graph(EVENT, diagram).expect("event applies");
    black_box(render_diagram(diagram, layout, &state).expect("repaint"));
}

fn full_pipeline() {
    let diagram = parse(SOURCE).expect("benchmark fixture parses");
    let layout = layout_diagram(&diagram, &Default::default()).expect("layout succeeds");
    let grid = render_diagram(&diagram, &layout, &Default::default()).expect("render succeeds");
    black_box(export::text(&grid));
}

fn benchmark(name: &str, mut operation: impl FnMut()) {
    let started = Instant::now();
    for _ in 0..ITERATIONS { operation(); }
    let elapsed = started.elapsed();
    let micros = elapsed.as_micros() / u128::from(ITERATIONS);
    println!("{name}: {micros} µs/iteration ({ITERATIONS} iterations)");
}
