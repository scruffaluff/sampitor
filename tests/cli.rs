use assert_cmd::assert::OutputAssertExt;
use assert_cmd::cargo::CommandCargoExt;
use std::process::Command;

#[test]
fn missing_file_error() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();

    let actual = cmd.args(&["-f", "this_file_does_not_exist.wav"]).assert();
    actual.failure().code(1);
}
