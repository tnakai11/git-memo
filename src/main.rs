use clap::{CommandFactory, Parser, Subcommand};
use git_memo::{
    add_memo, archive_category, edit_memo, grep_memos, list_archive_categories, list_categories,
    list_memos, push_memos, remove_memos,
};

/// Top-level command line interface for the git-memo application.
#[derive(Parser)]
#[command(name = "git-memo", about = "Record memos using Git")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Available subcommands for the CLI.
#[derive(Subcommand)]
enum Commands {
    /// Add a new memo
    Add {
        /// Category for the memo
        category: String,
        /// Memo message
        #[arg(allow_hyphen_values = true)]
        message: String,
    },
    /// List memos for a category
    List {
        /// Category to list
        category: String,
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Remove all memos for a category
    Remove {
        /// Category to remove
        category: String,
    },
    /// List all memo categories
    #[command(alias = "list-categories")]
    Categories {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// List archived memo categories
    #[command(alias = "list-archive-categories")]
    ArchiveCategories {
        /// Output in JSON format
        #[arg(long)]
        json: bool,
    },
    /// Edit the most recent memo in a category
    Edit {
        /// Category containing the memo
        category: String,
        /// New message
        message: String,
    },
    /// Archive a category under refs/archive/
    Archive {
        /// Category to archive
        category: String,
    },
    /// Search memos matching a pattern
    Grep {
        /// Pattern to search for
        pattern: String,
    },
    /// Push all memo refs to a remote
    Push {
        /// Remote name to push to
        remote: String,
    },
}

/// Application entry point.
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

/// Parse command line arguments and dispatch the requested subcommand.
fn run() -> Result<(), git2::Error> {
    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => handle_command(cmd),
        None => {
            // Default to showing help if no command is given
            Cli::command().print_help().unwrap();
            Ok(())
        }
    }
}

/// Execute an individual CLI command.
fn handle_command(cmd: Commands) -> Result<(), git2::Error> {
    match cmd {
        Commands::Add { category, message } => add_memo(&category, &message),
        Commands::List { category, json } => list_memos(&category, json),
        Commands::Remove { category } => remove_memos(&category),
        Commands::Categories { json } => list_categories(json),
        Commands::ArchiveCategories { json } => list_archive_categories(json),
        Commands::Edit { category, message } => edit_memo(&category, &message),
        Commands::Archive { category } => archive_category(&category),
        Commands::Grep { pattern } => grep_memos(&pattern),
        Commands::Push { remote } => push_memos(&remote),
    }
}
