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

#[test]
#[serial]
fn empty_args() {
    let test_env = common::TestEnv::new();
    let mut fixture = test_env.cmd();

    let output = fixture.assert().failure().get_output().clone();
    let out_str = String::from_utf8(output.stdout).unwrap();
    assert!(out_str.is_empty());
    let err_str = String::from_utf8(output.stderr).unwrap();

    insta::assert_snapshot!(err_str);
}

#[test]
#[serial]
fn list_without_config_exits_nonzero() {
    let test_env = common::TestEnv::new();
    let mut fixture = test_env.cmd();
    fixture.arg("list");

    let expected_err = format!(
        "Error: config file not found: {}\n",
        test_env.xdg.join("chgpf").join("profiles").to_str().unwrap()
    );

    fixture.assert().failure().stderr(expected_err);
}

#[test]
#[serial]
fn list_with_seeded_profiles_prints_stable_output() {
    let test_env = common::TestEnv::new();
    let mut fixture = test_env.cmd();
    fixture.arg("list");
    test_env.write_profiles(
        "
        [home]
        name = \"Daniel Kulichkov\"
        email = \"danchick03@gmail.com\"

        [test]
        name = \"Daniel Kulichkov\"
        email = \"dxack@yandex.ru\"

        [itshamans]
        name = \"IT Shaman\"
        email = \"shaman@itshamans.team\"
        ",
    );

    let output = fixture.assert().success().get_output().clone();
    let out_str = String::from_utf8(output.stdout).unwrap();

    insta::assert_snapshot!(out_str);
}
