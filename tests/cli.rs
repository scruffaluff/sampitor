use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn missing_file_error() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let actual = cmd.arg("this_file_does_not_exist.wav").assert();
    actual.failure().code(1);
}

#[test]
fn no_args_error() {
    let expected = predicates::str::contains("The following required arguments were not provided:");

    let actual = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap().assert();
    actual.failure().code(2).stderr(expected);
}
