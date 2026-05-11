use std::{fs, path::PathBuf};

use serial_test::serial;

use crate::common::TestEnv;
mod common;

fn profiles_path(test_env: &TestEnv) -> PathBuf {
    test_env.xdg.join("chgpf").join("profiles")
}

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
        profiles_path(&test_env).to_str().unwrap()
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

#[test]
#[serial]
fn init_with_clean_state() {
    let test_env = common::TestEnv::new();
    let mut fixture = test_env.cmd();
    fixture.arg("init");

    fixture.assert().success();

    let profiles_path = profiles_path(&test_env);
    let profiles = fs::read_to_string(profiles_path).unwrap();
    insta::assert_snapshot!(profiles);
}

#[test]
#[serial]
fn init_twice_or_force() {
    let test_env = common::TestEnv::new();

    // 1st run of "init"
    let mut fixture = test_env.cmd();
    fixture.arg("init");
    fixture.assert().success();

    // 2nd "init": failed
    let mut fixture = test_env.cmd();
    fixture.arg("init");
    fixture.assert().failure();

    // additional "init" with "--force" flag
    let mut fixture = test_env.cmd();
    fixture.args(["init", "--force"]);

    fixture.assert().success();
}
