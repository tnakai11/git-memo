use clap::{CommandFactory, Parser, Subcommand};
use git_memo::{add_memo, archive_category, edit_memo, list_categories, list_memos, remove_memos};

#[derive(Parser)]
#[command(name = "git-memo", about = "Record memos using Git")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

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
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Add { category, message }) => {
            if let Err(e) = add_memo(&category, &message) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::List { category, json }) => {
            if let Err(e) = list_memos(&category, json) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::Remove { category }) => {
            if let Err(e) = remove_memos(&category) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::Categories { json }) => {
            if let Err(e) = list_categories(json) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::Edit { category, message }) => {
            if let Err(e) = edit_memo(&category, &message) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        Some(Commands::Archive { category }) => {
            if let Err(e) = archive_category(&category) {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        None => {
            // Default to showing help if no command is given
            Cli::command().print_help().unwrap();
        }
    }
}
