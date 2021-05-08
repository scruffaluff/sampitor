use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn no_args() {
    let mut cmd = Command::cargo_bin("sampitor").unwrap();
    cmd.assert().failure();
}
