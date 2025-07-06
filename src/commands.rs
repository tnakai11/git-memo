use git2::{Repository, Sort};
use serde_json::json;

use std::collections::BTreeSet;

pub fn open_repo() -> Result<Repository, git2::Error> {
    Repository::discover(".")
}

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
