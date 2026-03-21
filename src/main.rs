use clap::{Parser, ValueEnum};
use std::process::Command;

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Convention {
    Conventional,
    Gitmoji,
}

#[derive(Debug, Parser)]
#[command(name = "jjc", about = "Simplify the jj commit experience")]
struct Cli {
    /// Commit message convention to use
    #[arg(short, long, value_enum)]
    convention: Option<Convention>,
}

/// Returns `true` if the message follows the Conventional Commits spec.
/// Pattern: `type(optional-scope)!: description`
fn is_conventional(msg: &str) -> bool {
    let first_line = msg.lines().next().unwrap_or("").trim();
    // Minimal regex-free check: a word (type), optional "(scope)", optional "!", then ": "
    let after_type = first_line
        .find(':')
        .map(|i| &first_line[..i]);
    let Some(prefix) = after_type else {
        return false;
    };

    // Strip optional scope and breaking-change marker
    let base = if let Some(scope_start) = prefix.find('(') {
        &prefix[..scope_start]
    } else {
        prefix.trim_end_matches('!')
    };

    // The type must be a non-empty lowercase ASCII word
    !base.is_empty() && base.chars().all(|c| c.is_ascii_lowercase() || c == '-')
        && first_line.len() > prefix.len() + 1
        && first_line.chars().nth(prefix.len() + 1) == Some(' ')
}

/// Returns `true` if the message starts with a gitmoji (`:name:` or actual emoji codepoint).
fn is_gitmoji(msg: &str) -> bool {
    let first_line = msg.lines().next().unwrap_or("").trim();
    // Shortcode form: :word:
    if first_line.starts_with(':') {
        if let Some(end) = first_line[1..].find(':') {
            let code = &first_line[1..=end];
            return !code.is_empty() && code.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-');
        }
    }

    // Actual emoji: first char has a high codepoint (emoji range starts around U+1F300)
    if let Some(ch) = first_line.chars().next() {
        return ch as u32 > 0x00FF;
    }

    false
}

/// Fetch the last `n` commit descriptions from the jj repository.
fn fetch_commit_messages(n: usize) -> Result<Vec<String>, String> {
    let revset = format!("ancestors(@, {})", n);
    let output = Command::new("jj")
        .args(["log", "--no-graph", "-r", &revset, "-T", "description ++ \"\\n---\\n\""])
        .output()
        .map_err(|e| format!("Failed to run `jj log`: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("`jj log` failed: {}", stderr));
    }

    let raw = String::from_utf8_lossy(&output.stdout);
    let messages: Vec<String> = raw
        .split("---")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();

    Ok(messages)
}

/// Detect the dominant commit convention from a slice of commit messages.
///
/// - Returns `Err` if no commit matches any convention.
/// - Returns `Err` if both conventions are tied.
/// - Returns the convention with the highest match count otherwise.
fn detect_convention(messages: &[String]) -> Result<Convention, String> {
    let conventional_count = messages.iter().filter(|m| is_conventional(m)).count();
    let gitmoji_count = messages.iter().filter(|m| is_gitmoji(m)).count();

    match (conventional_count, gitmoji_count) {
        (0, 0) => Err("No commit adheres to a known convention (conventional commits or gitmoji).".to_string()),
        (c, g) if c == g => Err(format!(
            "Cannot detect convention: tie between conventional commits ({c}) and gitmoji ({g})."
        )),
        (c, g) if c > g => Ok(Convention::Conventional),
        _ => Ok(Convention::Gitmoji),
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let num_commits_for_detection = 10;

    let convention = if let Some(c) = cli.convention {
        c
    } else {
        let messages = match fetch_commit_messages(num_commits_for_detection) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("Error fetching commits: {}", e);
                std::process::exit(1);
            }
        };

        match detect_convention(&messages) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error detecting convention: {}", e);
                std::process::exit(1);
            }
        }
    };

    println!("{:?}", convention);
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- is_conventional ---
    #[test]
    fn test_conventional_simple() {
        assert!(is_conventional("feat: add login"));
        assert!(is_conventional("fix(auth): correct token expiry"));
        assert!(is_conventional("chore!: drop support for node 12"));
    }

    #[test]
    fn test_conventional_negative() {
        assert!(!is_conventional(":sparkles: add login"));
        assert!(!is_conventional("✨ add login"));
        assert!(!is_conventional("just a plain message"));
    }

    // --- is_gitmoji ---
    #[test]
    fn test_gitmoji_shortcode() {
        assert!(is_gitmoji(":sparkles: add login"));
        assert!(is_gitmoji(":bug: fix crash"));
    }

    #[test]
    fn test_gitmoji_emoji() {
        assert!(is_gitmoji("✨ add login"));
        assert!(is_gitmoji("🎉 initial commit"));
    }

    #[test]
    fn test_gitmoji_negative() {
        assert!(!is_gitmoji("feat: add login"));
        assert!(!is_gitmoji("just a plain message"));
    }

    // --- detect_convention ---
    #[test]
    fn test_detect_no_convention() {
        let msgs = vec!["plain message".to_string(), "another plain".to_string()];
        assert!(detect_convention(&msgs).is_err());
    }

    #[test]
    fn test_detect_tie() {
        let msgs = vec![
            "feat: something".to_string(),
            ":sparkles: something".to_string(),
        ];
        assert!(detect_convention(&msgs).is_err());
    }

    #[test]
    fn test_detect_conventional_wins() {
        let msgs = vec![
            "feat: a".to_string(),
            "fix: b".to_string(),
            ":sparkles: c".to_string(),
        ];
        assert_eq!(detect_convention(&msgs).unwrap(), Convention::Conventional);
    }

    #[test]
    fn test_detect_gitmoji_wins() {
        let msgs = vec![
            ":sparkles: a".to_string(),
            "🎉 b".to_string(),
            "feat: c".to_string(),
        ];
        assert_eq!(detect_convention(&msgs).unwrap(), Convention::Gitmoji);
    }
}
