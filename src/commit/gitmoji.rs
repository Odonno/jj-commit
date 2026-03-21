use color_eyre::eyre::Result;
use inquire::Select;

use crate::commit::CommitMessage;
use crate::types::GitmojiType;

pub fn parse_gitmoji(message: &str) -> CommitMessage {
    let first_line = message.lines().next().unwrap_or("").trim();

    // Try `:code: description` form
    if first_line.starts_with(':') {
        if let Some(end) = first_line[1..].find(':') {
            let code = format!(":{}:", &first_line[1..=end]);
            let rest = first_line[end + 2..].trim();
            return CommitMessage {
                commit_type: Some(code),
                scopes: None,
                description: if rest.is_empty() {
                    None
                } else {
                    Some(rest.to_string())
                },
            };
        }
    }

    // Try emoji prefix form: first char is high-codepoint emoji
    if let Some(ch) = first_line.chars().next() {
        if ch as u32 > 0x00FF {
            let emoji = ch.to_string();
            let rest = first_line[ch.len_utf8()..].trim();
            return CommitMessage {
                commit_type: Some(emoji),
                scopes: None,
                description: if rest.is_empty() {
                    None
                } else {
                    Some(rest.to_string())
                },
            };
        }
    }

    // Fallback: treat entire string as description
    CommitMessage {
        commit_type: None,
        scopes: None,
        description: Some(first_line.to_string()),
    }
}

pub fn build_gitmoji(pre: CommitMessage) -> Result<String> {
    let gitmoji = prompt_gitmoji_type(pre.commit_type.as_deref())?;
    let description = crate::commit::prompt_description(pre.description.as_deref())?;

    // Build the final string: `<emoji> description`
    Ok(format!("{} {}", gitmoji.emoji, description))
}

fn prompt_gitmoji_type(prefill: Option<&str>) -> Result<&'static GitmojiType> {
    let options: Vec<&'static GitmojiType> = GitmojiType::all();

    // Match prefill against code (`:sparkles:`) or emoji (`✨`)
    let starting_cursor = prefill
        .and_then(|s| {
            if s.starts_with(':') {
                GitmojiType::from_code(s)
            } else {
                GitmojiType::from_emoji(s)
            }
        })
        .and_then(|g| options.iter().position(|o| o.code == g.code))
        .unwrap_or(0);

    let selected = Select::new("Commit type:", options)
        .with_starting_cursor(starting_cursor)
        .prompt()?;

    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_gitmoji_code() {
        let msg = parse_gitmoji(":sparkles: add login");
        assert_eq!(msg.commit_type.as_deref(), Some(":sparkles:"));
        assert_eq!(msg.description.as_deref(), Some("add login"));
    }

    #[test]
    fn test_parse_gitmoji_emoji() {
        let msg = parse_gitmoji("✨ add login");
        assert_eq!(msg.commit_type.as_deref(), Some("✨"));
        assert_eq!(msg.description.as_deref(), Some("add login"));
    }

    #[test]
    fn test_parse_gitmoji_plain() {
        let msg = parse_gitmoji("just a plain message");
        assert_eq!(msg.commit_type, None);
        assert_eq!(msg.description.as_deref(), Some("just a plain message"));
    }
}
