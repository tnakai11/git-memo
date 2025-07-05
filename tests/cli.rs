use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn shows_help() {
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("add"));
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
