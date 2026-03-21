use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, ValueEnum)]
pub enum Convention {
    Conventional,
    Gitmoji,
}

#[derive(Debug, Parser)]
#[command(name = "jjc", about = "Simplify the jj commit experience")]
struct Cli {
    /// Commit message convention to use
    #[arg(short, long, value_enum)]
    convention: Option<Convention>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("{:?}", cli);
}
