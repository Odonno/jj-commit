mod conventional;
mod gitmoji;

use clap::ValueEnum;
use color_eyre::eyre::{bail, Result, WrapErr};

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub enum Convention {
    Conventional,
    Gitmoji,
}

/// Resolve the convention: use the provided one directly, or auto-detect from
/// the last `n` commits in the repository.
pub fn resolve_convention(convention: Option<Convention>) -> Result<Convention> {
    if let Some(c) = convention {
        return Ok(c);
    }

    let num_commits_for_detection = 10;

    let messages = crate::jj::fetch_commit_messages(num_commits_for_detection)
        .wrap_err("Error fetching commits")?;
    detect_convention(&messages).wrap_err("Error detecting convention")
}

/// Detect the dominant commit convention from a slice of commit messages.
///
/// - Returns `Err` if no commit matches any convention.
/// - Returns `Err` if both conventions are tied.
/// - Returns the convention with the highest match count otherwise.
fn detect_convention(messages: &[String]) -> Result<Convention> {
    let conventional_count = messages
        .iter()
        .filter(|m| conventional::is_conventional(m))
        .count();
    let gitmoji_count = messages.iter().filter(|m| gitmoji::is_gitmoji(m)).count();

    match (conventional_count, gitmoji_count) {
        (0, 0) => {
            bail!("No commit adheres to a known convention (conventional commits or gitmoji).")
        }
        (c, g) if c == g => bail!(
            "Cannot detect convention: tie between conventional commits ({c}) and gitmoji ({g})."
        ),
        (c, g) if c > g => Ok(Convention::Conventional),
        _ => Ok(Convention::Gitmoji),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
