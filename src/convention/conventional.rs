/// Returns `true` if the message follows the Conventional Commits spec.
/// Pattern: `type(optional-scope)!: description`
pub fn is_conventional(msg: &str) -> bool {
    let first_line = msg.lines().next().unwrap_or("").trim();
    // Minimal regex-free check: a word (type), optional "(scope)", optional "!", then ": "
    let after_type = first_line.find(':').map(|i| &first_line[..i]);
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
    !base.is_empty()
        && base.chars().all(|c| c.is_ascii_lowercase() || c == '-')
        && first_line.len() > prefix.len() + 1
        && first_line.chars().nth(prefix.len() + 1) == Some(' ')
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
