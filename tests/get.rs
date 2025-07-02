mod common;
use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn getting_content_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut push_cmd = fx.cmd(["push", "key", "--file-path", "tests/static/sample.txt"]);
    push_cmd
        .output()
        .expect("push command should've been executed");

    let mut cmd = fx.cmd(["get", "key"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    A sample file for tash.

    Content goes here.

    ----- stderr -----
    ");
}

#[test]
fn getting_content_and_popping_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut push_cmd = fx.cmd(["push", "key", "--file-path", "tests/static/sample.txt"]);
    push_cmd
        .output()
        .expect("push command should've been executed");

    let mut cmd = fx.cmd(["get", "key", "--pop"]);
    let mut second_get_cmd = fx.cmd(["get", "key"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    A sample file for tash.

    Content goes here.

    ----- stderr -----
    ");
    assert_cmd_snapshot!(second_get_cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get content: key doesn't exist in stash
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn fails_if_key_doesnt_exist() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["get", "non-existent-key"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't get content: key doesn't exist in stash
    ");
}
