use git2::{Repository, Signature, Sort};
use serde_json::json;

use std::collections::BTreeSet;

/// Discover the nearest Git repository starting from the current directory.
///
/// This is a small wrapper around [`Repository::discover`] that searches
/// parent directories until a repository is found.
pub fn open_repo() -> Result<Repository, git2::Error> {
    Repository::discover(".")
}

/// Create a signature using the repository's `user.name` and `user.email`.
///
/// `user.name` must be set while `user.email` is optional. If no email is
/// configured, "none" is used.
pub fn make_signature(repo: &Repository) -> Result<Signature<'_>, git2::Error> {
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
    git2::Signature::now(&name, &email)
}

/// Add a memo as a Git commit under `refs/memo/<category>`.
///
/// The commit author is determined from the repository's `user.name` and
/// `user.email` configuration. Pass `"-"` as `message` to read the contents
/// from standard input.
///
/// # Parameters
/// - `category`: Name of the memo category.
/// - `message`: Commit message or `"-"` to read from stdin.
///
/// # Examples
/// ```no_run
/// use git_memo::add_memo;
///
/// fn main() -> Result<(), git2::Error> {
///     add_memo("todo", "write docs")?;
///     Ok(())
/// }
/// ```
pub fn add_memo(category: &str, message: &str) -> Result<(), git2::Error> {
    use std::io::Read;

    let repo = open_repo()?;

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
    let sig = make_signature(&repo)?;

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

/// Print all memos recorded for `category`.
///
/// When `json_output` is `true`, a JSON array of objects containing the memo
/// OID and message is written to stdout instead of plain text.
///
/// # Parameters
/// - `category`: The memo category to display.
/// - `json_output`: Enable JSON output when set to `true`.
pub fn list_memos(category: &str, json_output: bool) -> Result<(), git2::Error> {
    let repo = open_repo()?;
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

/// Delete the reference storing all memos for `category`.
///
/// # Parameters
/// - `category`: The memo category to remove.
pub fn remove_memos(category: &str) -> Result<(), git2::Error> {
    let repo = open_repo()?;
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

/// Display all known memo categories.
///
/// When `json_output` is true, the category names are printed as a JSON array.
///
/// # Parameters
/// - `json_output`: Enable JSON output when set to `true`.
pub fn list_categories(json_output: bool) -> Result<(), git2::Error> {
    let repo = open_repo()?;
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

/// Amend the latest memo commit for `category` with a new message.
///
/// # Parameters
/// - `category`: The memo category containing the commit.
/// - `message`: The new commit message.
pub fn edit_memo(category: &str, message: &str) -> Result<(), git2::Error> {
    let repo = open_repo()?;
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
    let sig = make_signature(&repo)?;
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

/// Move `refs/memo/<category>` to `refs/archive/<category>` if it exists.
///
/// # Parameters
/// - `category`: The memo category to archive.
pub fn archive_category(category: &str) -> Result<(), git2::Error> {
    let repo = open_repo()?;
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

/// Search all memo commits for a pattern.
///
/// This runs `git log --grep=<pattern> refs/memo/*` and prints the matching
/// commit messages to stdout.
pub fn grep_memos(pattern: &str) -> Result<(), git2::Error> {
    use std::path::Path;
    use std::process::Command;

    let repo = open_repo()?;
    let workdir = repo.workdir().unwrap_or_else(|| Path::new("."));

    let refs = repo.references_glob("refs/memo/*")?;
    let mut args = vec![
        "log".to_string(),
        "--format=%s".into(),
        "--grep".into(),
        pattern.to_string(),
    ];
    for reference in refs {
        let reference = reference?;
        if let Some(name) = reference.name() {
            args.push(name.to_string());
        }
    }

    if args.len() == 4 {
        println!("No memos found");
        return Ok(());
    }

    let output = Command::new("git")
        .args(&args)
        .current_dir(workdir)
        .output()
        .map_err(|e| git2::Error::from_str(&format!("Failed to run git log: {e}")))?;

    if !output.status.success() {
        return Err(git2::Error::from_str(&String::from_utf8_lossy(
            &output.stderr,
        )));
    }

    print!("{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}
