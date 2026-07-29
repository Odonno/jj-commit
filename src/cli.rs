use clap::Parser;

use crate::{convention::Convention, types::ConventionalType};

#[derive(Debug, Parser)]
#[command(name = "jjc", about = "Simplify the jj commit experience", version)]
pub struct Cli {
    /// Commit message convention to use
    #[arg(short, long, value_enum)]
    pub convention: Option<Convention>,

    /// Commit message (optional pre-fill; format depends on convention)
    #[arg(short, long)]
    pub message: Option<String>,

    /// Conventional commit type, only from the conventional convention
    #[arg(short, long, value_enum, value_name = "TYPE")]
    pub r#type: Option<ConventionalType>,

    /// Conventional commit scopes, only from the conventional convention (repeatable)
    #[arg(short, long, value_name = "SCOPE")]
    pub scopes: Vec<String>,

    /// Advance the bookmark from the closest ancestor to point to the newly created commit
    #[arg(short = 'a', long)]
    pub advance_bookmark: bool,
}
