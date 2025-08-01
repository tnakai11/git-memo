use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn shows_help() {
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("add"))
        .stdout(predicate::str::contains(env!("CARGO_PKG_VERSION")));
}

#[test]
fn adds_memo_commit() {
    let dir = tempdir().unwrap();

    // init repo
    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();

    // config user
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    // run git-memo add
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();

    // verify commit message
    let output = Command::new("git")
        .args(["log", "-1", "--format=%s", "refs/memo/todo"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("first memo"));
}

#[test]
fn adds_memo_from_stdin() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "-"])
        .write_stdin("line one\nline two\n")
        .assert()
        .success();

    let output = Command::new("git")
        .args(["log", "-1", "--format=%B", "refs/memo/todo"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("line one"));
    assert!(String::from_utf8_lossy(&output.stdout).contains("line two"));
}

#[test]
fn lists_memos() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    // add a memo
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();

    // list memos
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["list", "todo"])
        .assert()
        .success()
        .stdout(predicate::str::contains("first memo"));
}

#[test]
fn lists_memos_json() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["list", "todo", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("first memo"))
        .stdout(predicate::str::contains("\"oid\""));
}

#[test]
fn lists_categories() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "idea", "another"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .arg("categories")
        .assert()
        .success()
        .stdout(predicate::str::contains("todo"))
        .stdout(predicate::str::contains("idea"));
}

#[test]
fn lists_categories_json() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "idea", "another"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["categories", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("todo"))
        .stdout(predicate::str::contains("idea"))
        .stdout(predicate::str::starts_with("["));
}

#[test]
fn edits_latest_memo() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["edit", "todo", "edited memo"])
        .assert()
        .success();

    let output = Command::new("git")
        .args(["log", "-1", "--format=%s", "refs/memo/todo"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("edited memo"));
}

#[test]
fn archives_category() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["archive", "todo"])
        .assert()
        .success();

    Command::new("git")
        .args(["show-ref", "--verify", "--quiet", "refs/memo/todo"])
        .current_dir(&dir)
        .assert()
        .failure();
    Command::new("git")
        .args(["show-ref", "--verify", "--quiet", "refs/archive/todo"])
        .current_dir(&dir)
        .assert()
        .success();
}

#[test]
fn lists_archive_categories() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "idea", "another"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["archive", "todo"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .arg("archive-categories")
        .assert()
        .success()
        .stdout(predicate::str::contains("todo"))
        .stdout(predicate::str::contains("idea").not());
}

#[test]
fn lists_archive_categories_json() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["archive", "todo"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["archive-categories", "--json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("todo"))
        .stdout(predicate::str::starts_with("["));
}

#[test]
fn removes_memos() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    // add and then remove memo
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["remove", "todo"])
        .assert()
        .success();

    Command::new("git")
        .args(["show-ref", "--verify", "--quiet", "refs/memo/todo"])
        .current_dir(&dir)
        .assert()
        .failure();
}

#[test]
fn errors_when_missing_git_config() {
    let dir = tempdir().unwrap();
    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();

    // Use empty HOME so no global git config is found
    let empty_home = tempdir().unwrap();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .env("HOME", empty_home.path())
        .args(["add", "todo", "msg"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("user.name must be set"));
}

#[test]
fn adds_memo_without_email() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();

    // Set only user.name and use empty HOME so no global config provides email
    let empty_home = tempdir().unwrap();
    Command::new("git")
        .env("HOME", empty_home.path())
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .env("HOME", empty_home.path())
        .args(["add", "todo", "msg"])
        .assert()
        .success();

    let output = Command::new("git")
        .args(["log", "-1", "--format=%ae", "refs/memo/todo"])
        .current_dir(&dir)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("none"));
}

#[test]
fn errors_on_invalid_category() {
    let dir = tempdir().unwrap();
    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "bad category", "msg"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Invalid category name"));
}

#[test]
fn greps_memos() {
    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "hello world"])
        .assert()
        .success();
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "another note"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["grep", "hello"])
        .assert()
        .success()
        .stdout(predicate::str::contains("hello world"))
        .stdout(predicate::str::contains("another note").not());
}

#[test]
fn handles_parallel_commits() {
    use std::sync::{Arc, Barrier};
    use std::thread;

    let dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let msgs = ["first", "second"];
    // Seed the reference so concurrent additions must handle a parent commit.
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "initial"])
        .assert()
        .success();

    let barrier = Arc::new(Barrier::new(msgs.len() + 1));
    let mut handles = Vec::new();
    for msg in msgs {
        let b = barrier.clone();
        let path = dir.path().to_path_buf();
        handles.push(thread::spawn(move || {
            let mut cmd = Command::cargo_bin("git-memo").unwrap();
            b.wait();
            cmd.current_dir(path)
                .args(["add", "todo", msg])
                .assert()
                .success();
        }));
    }

    barrier.wait();
    for h in handles {
        h.join().unwrap();
    }

    let output = Command::new("git")
        .args(["log", "--format=%s", "refs/memo/todo"])
        .current_dir(&dir)
        .output()
        .unwrap();
    let log = String::from_utf8_lossy(&output.stdout);
    assert!(log.contains("first"));
    assert!(log.contains("second"));
}

#[test]
fn pushes_memos_to_remote() {
    let dir = tempdir().unwrap();
    let remote_dir = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["init", "--bare"])
        .current_dir(&remote_dir)
        .assert()
        .success();
    Command::new("git")
        .args([
            "remote",
            "add",
            "origin",
            remote_dir.path().to_str().unwrap(),
        ])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&dir)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&dir)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["add", "todo", "first memo"])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&dir)
        .args(["push", "origin"])
        .assert()
        .success();

    Command::new("git")
        .args([
            "--git-dir",
            remote_dir.path().to_str().unwrap(),
            "show-ref",
            "--verify",
            "--quiet",
            "refs/memo/todo",
        ])
        .assert()
        .success();
}

#[test]
fn adds_memo_with_relative_repo_path() {
    let base = tempdir().unwrap();
    let repo = base.path().join("repo");
    std::fs::create_dir(&repo).unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&repo)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&repo)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&base)
        .args(["--repo", "repo", "add", "todo", "msg"])
        .assert()
        .success();

    let output = Command::new("git")
        .args(["log", "-1", "--format=%s", "refs/memo/todo"])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("msg"));
}

#[test]
fn adds_memo_with_absolute_repo_path() {
    let repo = tempdir().unwrap();
    let cwd = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&repo)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&repo)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&cwd)
        .args([
            "--repo",
            repo.path().to_str().unwrap(),
            "add",
            "todo",
            "msg",
        ])
        .assert()
        .success();

    let output = Command::new("git")
        .args(["log", "-1", "--format=%s", "refs/memo/todo"])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("msg"));
}

#[test]
fn pushes_memos_with_repo_flag() {
    let repo = tempdir().unwrap();
    let remote_dir = tempdir().unwrap();
    let cwd = tempdir().unwrap();

    Command::new("git")
        .arg("init")
        .current_dir(&repo)
        .assert()
        .success();
    Command::new("git")
        .args(["init", "--bare"])
        .current_dir(&remote_dir)
        .assert()
        .success();
    Command::new("git")
        .args([
            "remote",
            "add",
            "origin",
            remote_dir.path().to_str().unwrap(),
        ])
        .current_dir(&repo)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.name", "Test"])
        .current_dir(&repo)
        .assert()
        .success();
    Command::new("git")
        .args(["config", "user.email", "test@example.com"])
        .current_dir(&repo)
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&cwd)
        .args([
            "--repo",
            repo.path().to_str().unwrap(),
            "add",
            "todo",
            "first memo",
        ])
        .assert()
        .success();

    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.current_dir(&cwd)
        .args(["--repo", repo.path().to_str().unwrap(), "push", "origin"])
        .assert()
        .success();

    Command::new("git")
        .args([
            "--git-dir",
            remote_dir.path().to_str().unwrap(),
            "show-ref",
            "--verify",
            "--quiet",
            "refs/memo/todo",
        ])
        .assert()
        .success();
}
