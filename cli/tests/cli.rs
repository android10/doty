use assert_cmd::Command;

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("doty").unwrap();
    cmd.assert().success().stdout("Hello, doty!");
}