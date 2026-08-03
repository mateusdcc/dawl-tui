use std::hint::black_box;
use std::time::Instant;

use dawl_tui::export;
use dawl_tui::layout::layout_diagram;
use dawl_tui::parser::parse;
use dawl_tui::render::render_diagram;

const SOURCE: &str = include_str!("../examples/approval.dtui");
const ITERATIONS: u32 = 100;

fn main() {
    benchmark("parse", || { black_box(parse(SOURCE).expect("benchmark fixture parses")); });
    benchmark("full pipeline", full_pipeline);
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
