use std::process::Command;

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
