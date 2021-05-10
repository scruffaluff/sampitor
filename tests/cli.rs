use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn no_args_error() {
    let mut cmd = Command::cargo_bin("sampitor").unwrap();
    let expected = predicates::str::contains("The following required arguments were not provided:");

    cmd.assert().failure().code(2).stderr(expected);
}
