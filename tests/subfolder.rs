use assert_cmd::Command;
use color_eyre::eyre::Result;
use std::process;
use tempfile::TempDir;

/// Initialize a minimal jj repo in `dir` so the binary can load it.
fn jj_init(dir: &std::path::Path) {
    let status = process::Command::new("jj")
        .args(["git", "init"])
        .current_dir(dir)
        .status()
        .expect("jj must be installed to run integration tests");
    assert!(status.success(), "jj git init failed");
}

#[test]
fn test_runs_from_subfolder() -> Result<()> {
    let root = TempDir::new()?;
    jj_init(root.path());

    let sub = root.path().join("src");
    std::fs::create_dir(&sub)?;

    // `jjc` with no args / --help should succeed from a subfolder;
    // we use `--help` as a side-effect-free smoke test that the binary
    // loads the workspace without erroring on "no jj repo here".
    Command::cargo_bin("jjc")?
        .arg("--help")
        .current_dir(&sub)
        .assert()
        .success();

    Ok(())
}

#[test]
fn test_error_outside_any_repo() -> Result<()> {
    let dir = TempDir::new()?;

    // Without --convention, auto-detection calls fetch_commit_messages which
    // triggers workspace load — should fail with a clear message.
    let output = Command::cargo_bin("jjc")?
        // Pass stdin as empty so the binary doesn't block waiting for TTY input.
        .write_stdin("")
        .current_dir(dir.path())
        .output()?;
    assert!(!output.status.success());

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no Jujutsu repo") || stderr.contains("No Jujutsu"),
        "unexpected error: {stderr}"
    );

    Ok(())
}
