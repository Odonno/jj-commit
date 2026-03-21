use clap::Parser;
use color_eyre::eyre::{bail, Result};

mod commit;
mod convention;
mod jj;
mod types;

#[derive(Debug, Parser)]
#[command(name = "jjc", about = "Simplify the jj commit experience")]
struct Cli {
    /// Commit message convention to use
    #[arg(short, long, value_enum)]
    convention: Option<convention::Convention>,

    /// Commit message (optional pre-fill; format depends on convention)
    #[arg(short, long)]
    message: Option<String>,

    /// Conventional commit type, only from the conventional convention
    #[arg(short, long, value_enum, value_name = "TYPE")]
    r#type: Option<types::ConventionalType>,

    /// Conventional commit scopes, only from the conventional convention (repeatable)
    #[arg(short, long, value_name = "SCOPE")]
    scopes: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let convention = convention::resolve_convention(cli.convention).await?;

    // --type is only meaningful for the conventional convention
    if cli.r#type.is_some() && convention != convention::Convention::Conventional {
        bail!("--type is only valid when using the conventional commit convention");
    }

    // --scopes is only meaningful for the conventional convention
    if !cli.scopes.is_empty() && convention != convention::Convention::Conventional {
        bail!("--scopes is only valid when using the conventional commit convention");
    }

    let commit_message = commit::build_commit_message(
        &convention,
        cli.message.as_deref(),
        cli.r#type,
        cli.scopes,
    )?;

    jj::commit(&commit_message).await?;

    Ok(())
}
