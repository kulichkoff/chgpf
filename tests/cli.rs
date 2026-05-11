use serial_test::serial;
mod common;

#[test]
#[serial]
fn help_flag() {
    let test_env = common::TestEnv::new();
    let mut fixture = test_env.cmd();
    fixture.args(["--help"]);

    let output = fixture.assert().success().get_output().clone();
    let out_str = String::from_utf8(output.stdout).unwrap();

    insta::assert_snapshot!(out_str);
}
