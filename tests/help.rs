use std::process::Command;

#[test]
fn help_names_the_render_command() {
    let output = Command::new(env!("CARGO_BIN_EXE_dawl-tui"))
        .arg("--help")
        .output()
        .expect("binary runs");
    let text = String::from_utf8_lossy(&output.stdout);
    assert!(output.status.success());
    assert!(text.contains("render"));
}
