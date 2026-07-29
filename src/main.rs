use clap::Parser;
use color_eyre::eyre::{Result, bail};
use inquire::MultiSelect;

use crate::cli::Cli;

mod cli;
mod commit;
mod convention;
mod jj;
mod types;

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

    if !cli.bookmarks.is_empty() {
        for name in &cli.bookmarks {
            jj::advance_bookmark(name, &new_commit_id).await?;
        }
    }
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
        eprintln!("warning: --advance-bookmark found no bookmarks in ancestors");
    }

    Ok(())
}
