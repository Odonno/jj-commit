/// Returns `true` if the message starts with a gitmoji (`:name:` or actual emoji codepoint).
pub fn is_gitmoji(msg: &str) -> bool {
    let first_line = msg.lines().next().unwrap_or("").trim();
    // Shortcode form: :word:
    if let Some(inner) = first_line.strip_prefix(':')
        && let Some(end) = inner.find(':')
    {
        let code = &inner[..end];
        return !code.is_empty()
            && code
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-');
    }

    // Actual emoji: first char has a high codepoint (emoji range starts around U+1F300)
    if let Some(ch) = first_line.chars().next() {
        return ch as u32 > 0x00FF;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
