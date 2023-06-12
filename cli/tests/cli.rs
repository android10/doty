use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("doty").unwrap();
    cmd.arg("bootstrap").assert().success();
}

#[test]
fn bootstrap_runs_correctly() {
    let mut cmd = Command::cargo_bin("doty").unwrap();
    cmd.arg("bootstrap").assert().success().stdout("doty: bootstrap!!!\n");
}

#[test]
fn dies_no_args() {
    let mut cmd: Command = Command::cargo_bin("doty").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicates::str::contains("Usage: doty <COMMAND>"));
}