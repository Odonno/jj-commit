use color_eyre::eyre::{eyre, Result, WrapErr};
use std::process::Command;

/// Fetch the last `n` commit descriptions from the jj repository.
pub fn fetch_commit_messages(n: usize) -> Result<Vec<String>> {
    let revset = format!("ancestors(@, {})", n);
    let output = Command::new("jj")
        .args([
            "log",
            "--no-graph",
            "-r",
            &revset,
            "-T",
            "description ++ \"\\n---\\n\"",
        ])
        .output()
        .wrap_err("Failed to run `jj log`")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(eyre!("`jj log` failed: {}", stderr));
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let messages: Vec<String> = raw
        .split("---")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(messages)
}

/// Run `jj commit -m <message>` to create a new commit from the working copy.
pub fn commit(message: &str) -> Result<()> {
    let status = Command::new("jj")
        .args(["commit", "-m", message])
        .status()
        .wrap_err("Failed to run `jj commit`")?;

    if !status.success() {
        return Err(eyre!("`jj commit` exited with status {}", status));
    }

    Ok(())
}
