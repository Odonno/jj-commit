use clap::Parser;
use color_eyre::eyre::Result;

mod commit;
mod convention;
mod jj;

#[derive(Debug, Parser)]
#[command(name = "jjc", about = "Simplify the jj commit experience")]
struct Cli {
    /// Commit message convention to use
    #[arg(short, long, value_enum)]
    convention: Option<convention::Convention>,

    /// Commit message (optional pre-fill; format depends on convention)
    #[arg(short, long)]
    message: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let convention = convention::resolve_convention(cli.convention)?;
    let commit_message = commit::build_commit_message(&convention, cli.message.as_deref())?;

    jj::commit(&commit_message)?;

    Ok(())
}
