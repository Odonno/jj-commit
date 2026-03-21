use clap::Parser;
use color_eyre::eyre::Result;

mod convention;
mod jj;

#[derive(Debug, Parser)]
#[command(name = "jjc", about = "Simplify the jj commit experience")]
struct Cli {
    /// Commit message convention to use
    #[arg(short, long, value_enum)]
    convention: Option<convention::Convention>,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let cli = Cli::parse();

    let convention = convention::resolve_convention(cli.convention)?;

    println!("{:?}", convention);

    Ok(())
}
