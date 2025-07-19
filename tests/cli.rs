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
        .stderr(predicate::str::contains("user.name and user.email"));
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
        .stderr(predicate::str::contains("not valid"));
}
