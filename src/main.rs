use clap::{CommandFactory, Parser, Subcommand};

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
        message: String,
    },
    /// List memos for a category
    List {
        /// Category to list
        category: String,
    },
    /// Remove all memos for a category
    Remove {
        /// Category to remove
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
        Some(Commands::List { category }) => {
            if let Err(e) = list_memos(&category) {
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
        None => {
            // Default to showing help if no command is given
            Cli::command().print_help().unwrap();
        }
    }
}

fn add_memo(category: &str, message: &str) -> Result<(), git2::Error> {
    use git2::Repository;

    let repo = Repository::discover(".")?;

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
    // Provide a clearer error if user.name or user.email are missing
    let sig = repo.signature().map_err(|_| {
        git2::Error::from_str(
            "Git user.name and user.email must be set.\n\
Run `git config --global user.name <name>` and `git config --global user.email <email>`",
        )
    })?;

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

fn list_memos(category: &str) -> Result<(), git2::Error> {
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
    for oid in revwalk {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        let message = commit.summary().unwrap_or("");
        println!("{oid} {message}");
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
