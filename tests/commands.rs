use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

#[test]
fn check_uses_input_exit_code_for_invalid_syntax() {
    let path = std::env::temp_dir().join(format!("dawl-tui-invalid-{}.dtui", std::process::id()));
    fs::write(&path, "diagram broken \"Broken\"\nnode").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .arg("check")
        .arg(&path)
        .output()
        .unwrap();
    let _ = fs::remove_file(path);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("line 2"));
}

#[test]
fn headless_watch_applies_a_finite_event_stream() {
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .args([
            "watch",
            "--graph",
            "fixtures/dawl/approval-graph.json",
            "--events",
            "fixtures/dawl/approval-events.ndjson",
            "--headless",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Developer"));
}

#[test]
fn render_accepts_dawl_json_from_standard_input() {
    let source = fs::read("fixtures/dawl/approval-graph.json").unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .args([
            "render", "-", "--format", "text", "--width", "180", "--height", "52",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&source).unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("Developer"));
}
