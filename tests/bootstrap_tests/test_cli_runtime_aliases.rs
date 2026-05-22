use std::process::Command;

fn run(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .args(["run", "-p", "everarcade-cli", "--"])
        .args(args)
        .output()
        .expect("failed to run everarcade-cli")
}

#[test]
fn cli_help_alias_and_unknown_contract() {
    let ok_commands: &[&[&str]] = &[
        &["help"],
        &["start"],
        &["start-game", "2d-arena"],
        &["init-game", "2d-arena"],
        &["build-game"],
        &["package-game"],
        &["run-local-federation"],
        &["replay-world"],
        &["inspect-simulation"],
    ];

    for cmd in ok_commands {
        let output = run(cmd);
        assert!(
            output.status.success(),
            "command should succeed: {:?}\nstdout={}\nstderr={}",
            cmd,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let unknown = run(&["definitely-not-a-command"]);
    assert!(!unknown.status.success(), "unknown command should fail");

    let stdout = String::from_utf8_lossy(&unknown.stdout);
    let stderr = String::from_utf8_lossy(&unknown.stderr);
    assert!(
        stdout.contains("everarcade <") || stderr.contains("everarcade <"),
        "unknown command should print help\nstdout={}\nstderr={}",
        stdout,
        stderr
    );
}
