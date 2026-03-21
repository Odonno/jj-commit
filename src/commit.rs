use color_eyre::eyre::Result;
use inquire::{Select, Text};
use std::fmt;

use crate::convention::Convention;

#[derive(Debug, Clone, PartialEq)]
pub enum ConventionalType {
    Build,
    Chore,
    Ci,
    Docs,
    Feat,
    Fix,
    Perf,
    Refactor,
    Revert,
    Style,
    Test,
}

impl ConventionalType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Build,
            Self::Chore,
            Self::Ci,
            Self::Docs,
            Self::Feat,
            Self::Fix,
            Self::Perf,
            Self::Refactor,
            Self::Revert,
            Self::Style,
            Self::Test,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Build => "build",
            Self::Chore => "chore",
            Self::Ci => "ci",
            Self::Docs => "docs",
            Self::Feat => "feat",
            Self::Fix => "fix",
            Self::Perf => "perf",
            Self::Refactor => "refactor",
            Self::Revert => "revert",
            Self::Style => "style",
            Self::Test => "test",
        }
    }

    fn from_str(s: &str) -> Option<Self> {
        match s {
            "build" => Some(Self::Build),
            "chore" => Some(Self::Chore),
            "ci" => Some(Self::Ci),
            "docs" => Some(Self::Docs),
            "feat" => Some(Self::Feat),
            "fix" => Some(Self::Fix),
            "perf" => Some(Self::Perf),
            "refactor" => Some(Self::Refactor),
            "revert" => Some(Self::Revert),
            "style" => Some(Self::Style),
            "test" => Some(Self::Test),
            _ => None,
        }
    }
}

impl fmt::Display for ConventionalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GitmojiType {
    pub emoji: &'static str,
    pub code: &'static str,
    pub description: &'static str,
    pub name: &'static str,
}

impl fmt::Display for GitmojiType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.emoji, self.description)
    }
}

impl GitmojiType {
    pub fn all() -> Vec<&'static GitmojiType> {
        GITMOJIS.iter().collect()
    }

    pub fn from_code(code: &str) -> Option<&'static GitmojiType> {
        GITMOJIS.iter().find(|g| g.code == code)
    }

    pub fn from_emoji(emoji: &str) -> Option<&'static GitmojiType> {
        GITMOJIS.iter().find(|g| g.emoji == emoji)
    }
}

static GITMOJIS: &[GitmojiType] = &[
    GitmojiType {
        emoji: "🎨",
        code: ":art:",
        description: "Improve structure / format of the code.",
        name: "art",
    },
    GitmojiType {
        emoji: "⚡️",
        code: ":zap:",
        description: "Improve performance.",
        name: "zap",
    },
    GitmojiType {
        emoji: "🔥",
        code: ":fire:",
        description: "Remove code or files.",
        name: "fire",
    },
    GitmojiType {
        emoji: "🐛",
        code: ":bug:",
        description: "Fix a bug.",
        name: "bug",
    },
    GitmojiType {
        emoji: "🚑️",
        code: ":ambulance:",
        description: "Critical hotfix.",
        name: "ambulance",
    },
    GitmojiType {
        emoji: "✨",
        code: ":sparkles:",
        description: "Introduce new features.",
        name: "sparkles",
    },
    GitmojiType {
        emoji: "📝",
        code: ":memo:",
        description: "Add or update documentation.",
        name: "memo",
    },
    GitmojiType {
        emoji: "🚀",
        code: ":rocket:",
        description: "Deploy stuff.",
        name: "rocket",
    },
    GitmojiType {
        emoji: "💄",
        code: ":lipstick:",
        description: "Add or update the UI and style files.",
        name: "lipstick",
    },
    GitmojiType {
        emoji: "🎉",
        code: ":tada:",
        description: "Begin a project.",
        name: "tada",
    },
    GitmojiType {
        emoji: "✅",
        code: ":white_check_mark:",
        description: "Add, update, or pass tests.",
        name: "white-check-mark",
    },
    GitmojiType {
        emoji: "🔒️",
        code: ":lock:",
        description: "Fix security or privacy issues.",
        name: "lock",
    },
    GitmojiType {
        emoji: "🔐",
        code: ":closed_lock_with_key:",
        description: "Add or update secrets.",
        name: "closed-lock-with-key",
    },
    GitmojiType {
        emoji: "🔖",
        code: ":bookmark:",
        description: "Release / Version tags.",
        name: "bookmark",
    },
    GitmojiType {
        emoji: "🚨",
        code: ":rotating_light:",
        description: "Fix compiler / linter warnings.",
        name: "rotating-light",
    },
    GitmojiType {
        emoji: "🚧",
        code: ":construction:",
        description: "Work in progress.",
        name: "construction",
    },
    GitmojiType {
        emoji: "💚",
        code: ":green_heart:",
        description: "Fix CI Build.",
        name: "green-heart",
    },
    GitmojiType {
        emoji: "⬇️",
        code: ":arrow_down:",
        description: "Downgrade dependencies.",
        name: "arrow-down",
    },
    GitmojiType {
        emoji: "⬆️",
        code: ":arrow_up:",
        description: "Upgrade dependencies.",
        name: "arrow-up",
    },
    GitmojiType {
        emoji: "📌",
        code: ":pushpin:",
        description: "Pin dependencies to specific versions.",
        name: "pushpin",
    },
    GitmojiType {
        emoji: "👷",
        code: ":construction_worker:",
        description: "Add or update CI build system.",
        name: "construction-worker",
    },
    GitmojiType {
        emoji: "📈",
        code: ":chart_with_upwards_trend:",
        description: "Add or update analytics or track code.",
        name: "chart-with-upwards-trend",
    },
    GitmojiType {
        emoji: "♻️",
        code: ":recycle:",
        description: "Refactor code.",
        name: "recycle",
    },
    GitmojiType {
        emoji: "➕",
        code: ":heavy_plus_sign:",
        description: "Add a dependency.",
        name: "heavy-plus-sign",
    },
    GitmojiType {
        emoji: "➖",
        code: ":heavy_minus_sign:",
        description: "Remove a dependency.",
        name: "heavy-minus-sign",
    },
    GitmojiType {
        emoji: "🔧",
        code: ":wrench:",
        description: "Add or update configuration files.",
        name: "wrench",
    },
    GitmojiType {
        emoji: "🔨",
        code: ":hammer:",
        description: "Add or update development scripts.",
        name: "hammer",
    },
    GitmojiType {
        emoji: "🌐",
        code: ":globe_with_meridians:",
        description: "Internationalization and localization.",
        name: "globe-with-meridians",
    },
    GitmojiType {
        emoji: "✏️",
        code: ":pencil2:",
        description: "Fix typos.",
        name: "pencil2",
    },
    GitmojiType {
        emoji: "💩",
        code: ":poop:",
        description: "Write bad code that needs to be improved.",
        name: "poop",
    },
    GitmojiType {
        emoji: "⏪️",
        code: ":rewind:",
        description: "Revert changes.",
        name: "rewind",
    },
    GitmojiType {
        emoji: "🔀",
        code: ":twisted_rightwards_arrows:",
        description: "Merge branches.",
        name: "twisted-rightwards-arrows",
    },
    GitmojiType {
        emoji: "📦️",
        code: ":package:",
        description: "Add or update compiled files or packages.",
        name: "package",
    },
    GitmojiType {
        emoji: "👽️",
        code: ":alien:",
        description: "Update code due to external API changes.",
        name: "alien",
    },
    GitmojiType {
        emoji: "🚚",
        code: ":truck:",
        description: "Move or rename resources (e.g.: files, paths, routes).",
        name: "truck",
    },
    GitmojiType {
        emoji: "📄",
        code: ":page_facing_up:",
        description: "Add or update license.",
        name: "page-facing-up",
    },
    GitmojiType {
        emoji: "💥",
        code: ":boom:",
        description: "Introduce breaking changes.",
        name: "boom",
    },
    GitmojiType {
        emoji: "🍱",
        code: ":bento:",
        description: "Add or update assets.",
        name: "bento",
    },
    GitmojiType {
        emoji: "♿️",
        code: ":wheelchair:",
        description: "Improve accessibility.",
        name: "wheelchair",
    },
    GitmojiType {
        emoji: "💡",
        code: ":bulb:",
        description: "Add or update comments in source code.",
        name: "bulb",
    },
    GitmojiType {
        emoji: "🍻",
        code: ":beers:",
        description: "Write code drunkenly.",
        name: "beers",
    },
    GitmojiType {
        emoji: "💬",
        code: ":speech_balloon:",
        description: "Add or update text and literals.",
        name: "speech-balloon",
    },
    GitmojiType {
        emoji: "🗃️",
        code: ":card_file_box:",
        description: "Perform database related changes.",
        name: "card-file-box",
    },
    GitmojiType {
        emoji: "🔊",
        code: ":loud_sound:",
        description: "Add or update logs.",
        name: "loud-sound",
    },
    GitmojiType {
        emoji: "🔇",
        code: ":mute:",
        description: "Remove logs.",
        name: "mute",
    },
    GitmojiType {
        emoji: "👥",
        code: ":busts_in_silhouette:",
        description: "Add or update contributor(s).",
        name: "busts-in-silhouette",
    },
    GitmojiType {
        emoji: "🚸",
        code: ":children_crossing:",
        description: "Improve user experience / usability.",
        name: "children-crossing",
    },
    GitmojiType {
        emoji: "🏗️",
        code: ":building_construction:",
        description: "Make architectural changes.",
        name: "building-construction",
    },
    GitmojiType {
        emoji: "📱",
        code: ":iphone:",
        description: "Work on responsive design.",
        name: "iphone",
    },
    GitmojiType {
        emoji: "🤡",
        code: ":clown_face:",
        description: "Mock things.",
        name: "clown-face",
    },
    GitmojiType {
        emoji: "🥚",
        code: ":egg:",
        description: "Add or update an easter egg.",
        name: "egg",
    },
    GitmojiType {
        emoji: "🙈",
        code: ":see_no_evil:",
        description: "Add or update a .gitignore file.",
        name: "see-no-evil",
    },
    GitmojiType {
        emoji: "📸",
        code: ":camera_flash:",
        description: "Add or update snapshots.",
        name: "camera-flash",
    },
    GitmojiType {
        emoji: "⚗️",
        code: ":alembic:",
        description: "Perform experiments.",
        name: "alembic",
    },
    GitmojiType {
        emoji: "🔍️",
        code: ":mag:",
        description: "Improve SEO.",
        name: "mag",
    },
    GitmojiType {
        emoji: "🏷️",
        code: ":label:",
        description: "Add or update types.",
        name: "label",
    },
    GitmojiType {
        emoji: "🌱",
        code: ":seedling:",
        description: "Add or update seed files.",
        name: "seedling",
    },
    GitmojiType {
        emoji: "🚩",
        code: ":triangular_flag_on_post:",
        description: "Add, update, or remove feature flags.",
        name: "triangular-flag-on-post",
    },
    GitmojiType {
        emoji: "🥅",
        code: ":goal_net:",
        description: "Catch errors.",
        name: "goal-net",
    },
    GitmojiType {
        emoji: "💫",
        code: ":dizzy:",
        description: "Add or update animations and transitions.",
        name: "dizzy",
    },
    GitmojiType {
        emoji: "🗑️",
        code: ":wastebasket:",
        description: "Deprecate code that needs to be cleaned up.",
        name: "wastebasket",
    },
    GitmojiType {
        emoji: "🛂",
        code: ":passport_control:",
        description: "Work on code related to authorization, roles and permissions.",
        name: "passport-control",
    },
    GitmojiType {
        emoji: "🩹",
        code: ":adhesive_bandage:",
        description: "Simple fix for a non-critical issue.",
        name: "adhesive-bandage",
    },
    GitmojiType {
        emoji: "🧐",
        code: ":monocle_face:",
        description: "Data exploration/inspection.",
        name: "monocle-face",
    },
    GitmojiType {
        emoji: "⚰️",
        code: ":coffin:",
        description: "Remove dead code.",
        name: "coffin",
    },
    GitmojiType {
        emoji: "🧪",
        code: ":test_tube:",
        description: "Add a failing test.",
        name: "test-tube",
    },
    GitmojiType {
        emoji: "👔",
        code: ":necktie:",
        description: "Add or update business logic.",
        name: "necktie",
    },
    GitmojiType {
        emoji: "🩺",
        code: ":stethoscope:",
        description: "Add or update healthcheck.",
        name: "stethoscope",
    },
    GitmojiType {
        emoji: "🧱",
        code: ":bricks:",
        description: "Infrastructure related changes.",
        name: "bricks",
    },
    GitmojiType {
        emoji: "🧑‍💻",
        code: ":technologist:",
        description: "Improve developer experience.",
        name: "technologist",
    },
    GitmojiType {
        emoji: "💸",
        code: ":money_with_wings:",
        description: "Add sponsorships or money related infrastructure.",
        name: "money-with-wings",
    },
    GitmojiType {
        emoji: "🧵",
        code: ":thread:",
        description: "Add or update code related to multithreading or concurrency.",
        name: "thread",
    },
    GitmojiType {
        emoji: "🦺",
        code: ":safety_vest:",
        description: "Add or update code related to validation.",
        name: "safety-vest",
    },
    GitmojiType {
        emoji: "✈️",
        code: ":airplane:",
        description: "Improve offline support.",
        name: "airplane",
    },
    GitmojiType {
        emoji: "🦖",
        code: ":t-rex:",
        description: "Code that adds backwards compatibility.",
        name: "t-rex",
    },
];

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
        Convention::Conventional => parse_conventional(message),
        Convention::Gitmoji => parse_gitmoji(message),
    }
}

fn parse_conventional(message: &str) -> CommitMessage {
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

fn parse_gitmoji(message: &str) -> CommitMessage {
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

/// Run interactive prompts to fill in any missing commit message fields,
/// then return the formatted commit message string.
pub fn build_commit_message(convention: &Convention, raw_message: Option<&str>) -> Result<String> {
    let pre = raw_message
        .map(|m| parse_message(convention, m))
        .unwrap_or_else(CommitMessage::empty);

    match convention {
        Convention::Conventional => build_conventional(pre),
        Convention::Gitmoji => build_gitmoji(pre),
    }
}

fn build_conventional(pre: CommitMessage) -> Result<String> {
    let commit_type = prompt_conventional_type(pre.commit_type.as_deref())?;
    let scopes = prompt_scopes(pre.scopes)?;
    let description = prompt_description(pre.description.as_deref())?;

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

fn build_gitmoji(pre: CommitMessage) -> Result<String> {
    let gitmoji = prompt_gitmoji_type(pre.commit_type.as_deref())?;
    let description = prompt_description(pre.description.as_deref())?;

    // Build the final string: `<emoji> description`
    Ok(format!("{} {}", gitmoji.emoji, description))
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

/// Prompt for scopes one at a time; empty input ends the loop.
fn prompt_scopes(prefill: Option<Vec<String>>) -> Result<Vec<String>> {
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

fn prompt_description(prefill: Option<&str>) -> Result<String> {
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
