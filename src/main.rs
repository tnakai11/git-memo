use clap::{CommandFactory, Parser, Subcommand};
use git_memo::{
    add_memo, archive_category, edit_memo, grep_memos, list_categories, list_memos, remove_memos,
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
        /// Set the author date to midnight today
        #[arg(long, conflicts_with = "at")]
        today: bool,
        /// Set a specific author datetime in RFC3339
        #[arg(long, value_name = "datetime", conflicts_with = "today")]
        at: Option<String>,
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
    /// Search memos matching a pattern
    Grep {
        /// Pattern to search for
        pattern: String,
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
        Commands::Add {
            category,
            message,
            today,
            at,
        } => {
            let when = if today {
                use chrono::{Datelike, TimeZone, Utc};
                let now = Utc::now();
                let dt = Utc
                    .with_ymd_and_hms(now.year(), now.month(), now.day(), 0, 0, 0)
                    .single()
                    .unwrap();
                Some(git2::Time::new(dt.timestamp(), 0))
            } else if let Some(at) = at {
                let dt = chrono::DateTime::parse_from_rfc3339(&at)
                    .map_err(|e| git2::Error::from_str(&format!("invalid datetime: {e}")))?;
                let offset = dt.offset().local_minus_utc() / 60;
                Some(git2::Time::new(dt.timestamp(), offset))
            } else {
                None
            };
            add_memo(&category, &message, when)
        }
        Commands::List { category, json } => list_memos(&category, json),
        Commands::Remove { category } => remove_memos(&category),
        Commands::Categories { json } => list_categories(json),
        Commands::Edit { category, message } => edit_memo(&category, &message),
        Commands::Archive { category } => archive_category(&category),
        Commands::Grep { pattern } => grep_memos(&pattern),
    }
}
