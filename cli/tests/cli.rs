use assert_cmd::Command;
use std::error::Error;

pub type TestResult = Result<(), Box<dyn Error>>;

#[test]
fn runs() -> TestResult {
    let mut cmd = Command::cargo_bin("doty")?;
    cmd.arg("bootstrap")
        .assert()
        .success();
    Ok(())
}

#[test]
fn bootstrap_runs_correctly() -> TestResult {
    let mut cmd = Command::cargo_bin("doty")?;
    cmd.arg("bootstrap")
        .assert()
        .success()
        .stdout("doty: bootstrap!!!");
    Ok(())
}

#[test]
fn dies_no_args() -> TestResult {
    let mut cmd: Command = Command::cargo_bin("doty")?;
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage: doty <COMMAND>"));
    Ok(())
}