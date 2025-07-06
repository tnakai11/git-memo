use clap::{CommandFactory, Parser, Subcommand};
use serde_json::json;

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

fn add_memo(category: &str, message: &str) -> Result<(), git2::Error> {
    use git2::Repository;
    use std::io::Read;

    let repo = Repository::discover(".")?;

    // Read message from stdin if requested
    let mut stdin_message = String::new();
    let message = if message == "-" {
        std::io::stdin()
            .read_to_string(&mut stdin_message)
            .map_err(|e| git2::Error::from_str(&format!("Failed to read stdin: {e}")))?;
        while stdin_message.ends_with('\n') {
            stdin_message.pop();
        }
        &stdin_message
    } else {
        message
    };

    // Determine tree for the commit: use HEAD tree if exists, else empty tree
    let tree = match repo.head() {
        Ok(head) => {
            let commit = head.peel_to_commit()?;
            commit.tree()?
        }
        Err(_) => {
            let builder = repo.treebuilder(None)?;
            let oid = builder.write()?;
            repo.find_tree(oid)?
        }
    };

    // Prepare author/committer signature from git config
    // Allow missing user.email but still require user.name
    let config = repo.config()?;
    let name = config.get_string("user.name").map_err(|_| {
        git2::Error::from_str(
            "Git user.name must be set.\nRun `git config --global user.name <name>`",
        )
    })?;
    let mut email = config.get_string("user.email").unwrap_or_default();
    if email.trim().is_empty() {
        email = "none".to_string();
    }
    let sig = git2::Signature::now(&name, &email)?;

    // Parent is refs/memo/<category> if exists
    let refname = format!("refs/memo/{category}");
    let parent = repo
        .refname_to_id(&refname)
        .ok()
        .and_then(|oid| repo.find_commit(oid).ok());
    let parents = parent.iter().collect::<Vec<_>>();

    let commit_oid = repo.commit(Some(&refname), &sig, &sig, message, &tree, &parents)?;
    println!("Recorded memo {commit_oid} under {refname}");
    Ok(())
}

fn list_memos(category: &str, json_output: bool) -> Result<(), git2::Error> {
    use git2::{Repository, Sort};

    let repo = Repository::discover(".")?;
    let refname = format!("refs/memo/{category}");
    if repo.refname_to_id(&refname).is_err() {
        println!("No memos found for category {category}");
        return Ok(());
    }
    let mut revwalk = repo.revwalk()?;
    revwalk.set_sorting(Sort::REVERSE)?;
    revwalk.push_ref(&refname)?;
    let mut memos = Vec::new();
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let message = commit.summary().unwrap_or("").to_string();
        if json_output {
            memos.push(json!({ "oid": oid.to_string(), "message": message }));
        } else {
            println!("{oid} {message}");
        }
    }
    if json_output {
        println!("{}", serde_json::to_string_pretty(&memos).unwrap());
    }
    Ok(())
}

fn remove_memos(category: &str) -> Result<(), git2::Error> {
    use git2::Repository;

    let repo = Repository::discover(".")?;
    let refname = format!("refs/memo/{category}");
    match repo.find_reference(&refname) {
        Ok(mut reference) => {
            reference.delete()?;
            println!("Removed {refname}");
        }
        Err(_) => {
            println!("No memos found for category {category}");
        }
    }
    Ok(())
}

fn list_categories(json_output: bool) -> Result<(), git2::Error> {
    use git2::Repository;
    use std::collections::BTreeSet;

    let repo = Repository::discover(".")?;
    let refs = repo.references_glob("refs/memo/*")?;
    let mut categories = BTreeSet::new();
    for reference in refs {
        let reference = reference?;
        if let Some(cat) = reference
            .name()
            .and_then(|name| name.strip_prefix("refs/memo/"))
        {
            categories.insert(cat.to_string());
        }
    }
    if json_output {
        println!("{}", serde_json::to_string_pretty(&categories).unwrap());
    } else {
        for cat in categories {
            println!("{cat}");
        }
    }
    Ok(())
}

fn edit_memo(category: &str, message: &str) -> Result<(), git2::Error> {
    use git2::Repository;

    let repo = Repository::discover(".")?;
    let refname = format!("refs/memo/{category}");
    let oid = match repo.refname_to_id(&refname) {
        Ok(id) => id,
        Err(_) => {
            println!("No memos found for category {category}");
            return Ok(());
        }
    };
    let commit = repo.find_commit(oid)?;
    let tree = commit.tree()?;
    let sig = repo.signature()?;
    let new_oid = commit.amend(
        Some(&refname),
        Some(&sig),
        Some(&sig),
        None,
        Some(message),
        Some(&tree),
    )?;
    println!("Updated memo {new_oid} under {refname}");
    Ok(())
}

fn archive_category(category: &str) -> Result<(), git2::Error> {
    use git2::Repository;

    let repo = Repository::discover(".")?;
    let src = format!("refs/memo/{category}");
    let dst = format!("refs/archive/{category}");
    match repo.find_reference(&src) {
        Ok(mut reference) => {
            reference.rename(&dst, true, "archive")?;
            println!("Archived {src} to {dst}");
        }
        Err(_) => {
            println!("No memos found for category {category}");
        }
    }
    Ok(())
}
