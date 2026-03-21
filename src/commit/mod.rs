mod conventional;
mod gitmoji;

use color_eyre::eyre::Result;
use inquire::Text;

use crate::convention::Convention;

#[derive(Debug)]
pub struct CommitMessage {
    /// The commit type (stored as the string key, e.g. "feat" or ":sparkles:")
    pub commit_type: Option<String>,
    /// Scopes (only used by Conventional Commits)
    pub scopes: Option<Vec<String>>,
    /// The human-readable description
    pub description: Option<String>,
}

impl CommitMessage {
    fn empty() -> Self {
        Self {
            commit_type: None,
            scopes: None,
            description: None,
        }
    }
}

/// Parse a raw `--message` string into a `CommitMessage` based on the active convention.
///
/// Conventional format:  `type(scope1,scope2): description`
/// Gitmoji format:       `:code: description`  or  `<emoji> description`
fn parse_message(convention: &Convention, message: &str) -> CommitMessage {
    match convention {
        Convention::Conventional => conventional::parse_conventional(message),
        Convention::Gitmoji => gitmoji::parse_gitmoji(message),
    }
}

/// Run interactive prompts to fill in any missing commit message fields,
/// then return the formatted commit message string.
pub fn build_commit_message(convention: &Convention, raw_message: Option<&str>) -> Result<String> {
    let pre = raw_message
        .map(|m| parse_message(convention, m))
        .unwrap_or_else(CommitMessage::empty);

    match convention {
        Convention::Conventional => conventional::build_conventional(pre),
        Convention::Gitmoji => gitmoji::build_gitmoji(pre),
    }
}

/// Prompt for scopes one at a time; empty input ends the loop.
pub fn prompt_scopes(prefill: Option<Vec<String>>) -> Result<Vec<String>> {
    let mut scopes: Vec<String> = prefill.unwrap_or_default();

    loop {
        let prompt_text = if scopes.is_empty() {
            "Scope (press Enter to skip):".to_string()
        } else {
            format!(
                "Add another scope (current: {}) or press Enter to finish:",
                scopes.join(", ")
            )
        };

        let input = Text::new(&prompt_text).prompt()?;
        let trimmed = input.trim().to_string();

        if trimmed.is_empty() {
            break;
        }

        scopes.push(trimmed);
    }

    Ok(scopes)
}

pub fn prompt_description(prefill: Option<&str>) -> Result<String> {
    let mut prompt = Text::new("Description:");

    let prefill_owned;
    if let Some(p) = prefill {
        prefill_owned = p.to_string();
        prompt = prompt.with_initial_value(&prefill_owned);
    }

    loop {
        let input = prompt.prompt()?;
        let trimmed = input.trim().to_string();
        if !trimmed.is_empty() {
            return Ok(trimmed);
        }
        eprintln!("Description cannot be empty. Please enter a commit description.");
        prompt = Text::new("Description:");
    }
}
