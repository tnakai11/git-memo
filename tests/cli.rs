use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("git-memo").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("git-memo"));
}
