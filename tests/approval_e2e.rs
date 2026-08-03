use std::process::Command;

fn run_render() -> String {
    let bin = env!("CARGO_BIN_EXE_dawl-tui");
    let cmd = [
        "render",
        "examples/approval.dtui",
        "--format",
        "text",
        "--width",
        "180",
        "--height",
        "52",
    ];
    let out = Command::new(bin).args(cmd).output().expect("render runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn check_approval_output_labels(text: &str) {
    let labels = [
        "parallel(\"issues\")",
        "issue-65",
        "issue-66",
        "issue-N",
        "issueResu",
        "phase: mer",
        "shell cleanu",
        "phase: summar",
        "OUTPUT",
        "Agents spawned",
    ];
    for label in labels {
        assert!(text.contains(label), "missing {label}");
    }
    assert!(text.matches("Developer ag").count() >= 4);
    assert!(!text.contains("\u{1b}["));
}

#[test]
fn approval_example_matches_the_reference_composition() {
    let text = run_render();
    assert_eq!(text.split('\n').count(), 52);
    assert!(text
        .lines()
        .next()
        .unwrap_or_default()
        .contains("developIssuesUntilApproved: full agent flow"));
    check_approval_output_labels(&text);
}
