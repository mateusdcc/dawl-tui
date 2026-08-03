use std::fs;
use std::process::Command;

#[test]
fn check_uses_input_exit_code_for_invalid_syntax() {
    let path = std::env::temp_dir().join(format!("dawl-tui-invalid-{}.dtui", std::process::id()));
    fs::write(&path, "diagram broken \"Broken\"\nnode").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui")).arg("check").arg(&path).output().unwrap();
    let _ = fs::remove_file(path);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("line 2"));
}

#[test]
fn headless_watch_applies_a_finite_event_stream() {
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .args(["watch", "--graph", "fixtures/dawl/approval-graph.json", "--events", "fixtures/dawl/approval-events.ndjson", "--headless"])
        .output().unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Developer"));
}
