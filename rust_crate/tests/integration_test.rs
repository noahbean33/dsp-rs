use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_version_info() {
    let mut cmd = Command::cargo_bin("os-core-utils").unwrap();
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("os-core-utils v"));
}

#[test]
fn help_flag_works() {
    let mut cmd = Command::cargo_bin("os-core-utils").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("Rebuilding core OS system utilities"));
}

#[test]
fn verbose_flag_works() {
    let mut cmd = Command::cargo_bin("os-core-utils").unwrap();
    cmd.arg("--verbose")
        .assert()
        .success()
        .stdout(predicate::str::contains("Verbose mode enabled"));
}
