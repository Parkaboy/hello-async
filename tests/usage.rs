// ...new test file...
use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn prints_usage_and_exits_nonzero_when_no_args() {
    let mut cmd = Command::cargo_bin("hello-async").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage:"));
}