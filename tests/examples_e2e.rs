use std::process::Command;

const GRAPH_GALLERY: [&str; 4] = [
    "examples/graph-loop.dtui",
    "examples/graph-diamond.dtui",
    "examples/graph-barrier.dtui",
    "examples/graph-verification.dtui",
];

#[test]
fn simple_example_renders_at_the_documented_svg_size() {
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .args(["render", "examples/simple.dtui", "--format", "svg"])
        .output()
        .expect("simple SVG render runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let svg = String::from_utf8_lossy(&output.stdout);
    assert!(svg.contains("width=\"720\" height=\"320\""));
    assert!(svg.contains("Healthy?"));
    assert!(svg.contains("M"), "simple SVG includes vector paths");
}

#[test]
fn graph_gallery_delegates_geometry_to_the_layout_engine() {
    for path in GRAPH_GALLERY {
        assert_topology_only(path);
        assert_svg_renders(path);
    }
}

fn assert_topology_only(path: &str) {
    let source = std::fs::read_to_string(path).expect("example source is readable");
    for manual in [
        "viewport ",
        " at ",
        " size ",
        " via ",
        "from_port",
        "to_port",
    ] {
        assert!(!source.contains(manual), "{path} contains {manual}");
    }
}

fn assert_svg_renders(path: &str) {
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .args(["render", path, "--format", "svg"])
        .output()
        .expect("gallery SVG render runs");
    assert!(
        output.status.success(),
        "{path}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("<svg"));
}
