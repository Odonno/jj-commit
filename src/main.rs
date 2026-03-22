use clap::Parser;
use color_eyre::eyre::{Result, bail};
use inquire::MultiSelect;

mod commit;
mod convention;
mod jj;
mod types;

#[derive(Debug, Parser)]
#[command(name = "jjc", about = "Simplify the jj commit experience", version)]
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

    /// Advance the bookmark from the closest ancestor to point to the newly created commit
    #[arg(short = 'a', long)]
    advance_bookmark: bool,
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

    let commit_message =
        commit::build_commit_message(&convention, cli.message.as_deref(), cli.r#type, cli.scopes)?;

    // Find the nearest ancestor bookmark *before* committing, while the WC is still the open change.
    // After commit() the topology shifts and the search would yield a different (or no) result.
    let ancestor_bookmarks = if cli.advance_bookmark {
        jj::find_nearest_ancestor_bookmarks().await?
    } else {
        None
    };

    let new_commit_id = jj::commit(&commit_message).await?;

    if let Some(bookmarks) = ancestor_bookmarks {
        let to_advance: Vec<String> = match bookmarks.len() {
            1 => bookmarks,
            _ => MultiSelect::new("Select bookmarks to advance to the new commit:", bookmarks)
                .prompt()?,
        };

        for name in to_advance {
            jj::advance_bookmark(&name, &new_commit_id).await?;
        }
    } else if cli.advance_bookmark {
        eprintln!("warning: no bookmarks found in ancestors; nothing to advance");
    }

    Ok(())
}
