mod common;
use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn pushing_content_from_flag_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["push", "key", "--data", "content goes here"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");
}

#[test]
fn pushing_content_from_local_file_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["push", "key", "--file-path", "tests/static/sample.txt"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");
}

#[test]
fn pushing_and_echoing_content_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["push", "key", "--data", "content goes here", "--echo"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    content goes here
    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn fails_if_key_doesnt_conform_to_regex() {
    // GIVEN
    let fx = Fixture::new();
    let incorrect_keys = vec![
        "incorrect key",
        "inco!!rectkey",
        "incorrect.key",
        "  ",
        "INCORRECTKEY",
        "incorrect,key",
    ];

    insta::allow_duplicates! {
        for incorrect_key in incorrect_keys {
            let mut cmd = fx.cmd(["push", incorrect_key, "--file-path", "tests/static/sample.txt"]);

            // WHEN
            // THEN
            assert_cmd_snapshot!(cmd, @r"
        success: false
        exit_code: 1
        ----- stdout -----

        ----- stderr -----
        Error: couldn't push content: incorrect key provided (valid regex: ^[a-z0-9_-]{1,30}$)
        ");
        }
    }
}

#[test]
fn fails_if_content_overwrites_are_not_desired() {
    // GIVEN
    let fx = Fixture::new();
    let mut first_push_cmd = fx.cmd(["push", "key", "--data", "content goes here"]);

    first_push_cmd
        .output()
        .expect("push command should've been executed");

    let mut cmd = fx.cmd([
        "push",
        "key",
        "--data",
        "content goes here",
        "--prevent-overwrite",
    ]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't push content: key already exists in the stash
    ");
}
