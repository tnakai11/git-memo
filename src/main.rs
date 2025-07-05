use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "git-memo", about = "Record memos using Git")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new memo
    Add {},
}

fn main() {
    let _cli = Cli::parse();
}
