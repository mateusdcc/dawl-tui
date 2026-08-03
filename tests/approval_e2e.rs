use std::process::Command;

#[test]
fn approval_example_matches_the_reference_composition() {
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .args(["render", "examples/approval.dtui", "--format", "text", "--width", "180", "--height", "52"])
        .output()
        .expect("render command runs");
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    let text = String::from_utf8_lossy(&output.stdout);
    assert_eq!(text.split('\n').count(), 52);
    assert!(text.lines().next().unwrap_or_default().contains("developIssuesUntilApproved: full agent flow"));
    for label in ["parallel(\"issues\")", "issue-65", "issue-66", "issue-N", "issueResults", "phase: merge", "shell cleanup", "phase: summary", "OUTPUT", "Agents spawned"] {
        assert!(text.contains(label), "missing {label}");
    }
    assert!(text.matches("Developer agent").count() >= 4);
    assert!(!text.contains("\u{1b}["));
}
