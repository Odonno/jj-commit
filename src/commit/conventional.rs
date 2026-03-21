use color_eyre::eyre::Result;
use inquire::Select;

use crate::commit::CommitMessage;
use crate::types::ConventionalType;

pub fn parse_conventional(message: &str) -> CommitMessage {
    let first_line = message.lines().next().unwrap_or("").trim();

    // Find the `: ` separator
    let Some(colon_pos) = first_line.find(": ") else {
        return CommitMessage {
            commit_type: None,
            scopes: None,
            description: Some(first_line.to_string()),
        };
    };

    let prefix = &first_line[..colon_pos];
    let description = first_line[colon_pos + 2..].trim().to_string();

    // prefix may be `type`, `type(scope)`, or `type(scope1,scope2)`
    let (raw_type, scopes) = if let Some(open) = prefix.find('(') {
        let raw_type = &prefix[..open];
        let rest = &prefix[open + 1..];
        let scopes_str = rest.trim_end_matches(')').trim_end_matches('!');
        let scopes: Vec<String> = scopes_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();
        (raw_type.trim_end_matches('!'), Some(scopes))
    } else {
        (prefix.trim_end_matches('!'), None)
    };

    CommitMessage {
        commit_type: if raw_type.is_empty() {
            None
        } else {
            Some(raw_type.to_string())
        },
        scopes,
        description: if description.is_empty() {
            None
        } else {
            Some(description)
        },
    }
}

/// Build a conventional commit message.
///
/// `forced_type` is provided when the user passed `--type` on the CLI; in that case the type
/// selection prompt is skipped entirely.
pub fn build_conventional(
    pre: CommitMessage,
    forced_type: Option<ConventionalType>,
) -> Result<String> {
    let commit_type = match forced_type {
        Some(t) => t,
        None => prompt_conventional_type(pre.commit_type.as_deref())?,
    };
    let scopes = crate::commit::prompt_scopes(pre.scopes)?;
    let description = crate::commit::prompt_description(pre.description.as_deref())?;

    // Build the final string: `type(scope1,scope2): description`
    let type_str = commit_type.as_str();
    let message = if scopes.is_empty() {
        format!("{type_str}: {description}")
    } else {
        let scopes_str = scopes.join(",");
        format!("{type_str}({scopes_str}): {description}")
    };

    Ok(message)
}

fn prompt_conventional_type(prefill: Option<&str>) -> Result<ConventionalType> {
    let options = ConventionalType::all();

    // If we have a valid pre-filled type, use it as the default selection
    let starting_cursor = prefill
        .and_then(|s| ConventionalType::from_str(s))
        .and_then(|t| options.iter().position(|o| o == &t))
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
    fn test_parse_conventional_full() {
        let msg = parse_conventional("feat(auth,ui): add login page");
        assert_eq!(msg.commit_type.as_deref(), Some("feat"));
        assert_eq!(msg.scopes, Some(vec!["auth".to_string(), "ui".to_string()]));
        assert_eq!(msg.description.as_deref(), Some("add login page"));
    }

    #[test]
    fn test_parse_conventional_no_scope() {
        let msg = parse_conventional("fix: correct token expiry");
        assert_eq!(msg.commit_type.as_deref(), Some("fix"));
        assert_eq!(msg.scopes, None);
        assert_eq!(msg.description.as_deref(), Some("correct token expiry"));
    }

    #[test]
    fn test_parse_conventional_breaking() {
        let msg = parse_conventional("chore!: drop node 12");
        assert_eq!(msg.commit_type.as_deref(), Some("chore"));
        assert_eq!(msg.scopes, None);
        assert_eq!(msg.description.as_deref(), Some("drop node 12"));
    }

    #[test]
    fn test_parse_conventional_description_only() {
        let msg = parse_conventional("just a plain message");
        assert_eq!(msg.commit_type, None);
        assert_eq!(msg.description.as_deref(), Some("just a plain message"));
    }
}
