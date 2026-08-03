use std::process::Command;

const REFERENCE_LABELS: [&str; 10] = [
    "parallel(\"issues\")",
    "issue-65",
    "issue-66",
    "issue-N",
    "issueResults",
    "merge phase",
    "shell cleanup",
    "summary",
    "OUTPUT",
    "Agents spawned",
];

fn run_render() -> String {
    let bin = env!("CARGO_BIN_EXE_dawl-tui");
    let cmd = [
        "render",
        "examples/approval.dtui",
        "--format",
        "text",
        "--width",
        "202",
        "--height",
        "72",
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
    for label in REFERENCE_LABELS {
        assert!(text.contains(label), "missing {label}");
    }
    assert!(text.matches("Developer ag").count() >= 3);
    assert!(text.contains("Merge Dev"));
    check_route_artifacts(text);
    assert!(!text.contains("\u{1b}["));
}

fn check_route_artifacts(text: &str) {
    assert!(
        !text.contains('┼'),
        "approval flow contains a false crossing"
    );
}

#[test]
fn approval_example_matches_the_reference_composition() {
    let text = run_render();
    assert_eq!(text.split('\n').count(), 72);
    assert!(text
        .lines()
        .next()
        .unwrap_or_default()
        .contains("developIssuesUntilApproved: full agent flow"));
    check_approval_output_labels(&text);
}

#[test]
fn approval_svg_uses_the_compact_reference_scale() {
    let out = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .args(["render", "examples/approval.dtui", "--format", "svg"])
        .output()
        .expect("SVG render runs");
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let svg = String::from_utf8_lossy(&out.stdout);
    assert!(svg.contains("width=\"1212\" height=\"720\""));
    assert!(svg.contains("font-weight=\"700\""));
    assert!(svg.contains("text-anchor=\"middle\""));
}
