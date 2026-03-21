use std::fmt;

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

    pub fn from_str(s: &str) -> Option<Self> {
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
