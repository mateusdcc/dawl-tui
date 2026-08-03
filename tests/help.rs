use std::process::Command;

#[test]
fn help_describes_the_public_commands() {
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .arg("--help")
        .output()
        .expect("binary runs");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(text.contains("Render a native .dtui or DAWL JSON diagram"));
    assert!(text.contains("Parse and validate a diagram without rendering it"));
    assert!(text.contains("Replay DAWL runtime events over a graph"));
}
