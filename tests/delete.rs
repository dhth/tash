mod common;
use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn deleting_content_items_works() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/static/sample.txt"]);
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = fx.cmd(["delete", "key-a", "key-b"]);

    let mut ls_cmd = fx.cmd(["ls"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----

    ----- stderr -----
    ");

    assert_cmd_snapshot!(ls_cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    key-c

    ----- stderr -----
    ");
}

//------------//
//  FAILURES  //
//------------//

#[test]
fn deletion_fails_if_one_or_more_keys_dont_exist() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/static/sample.txt"]);
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = fx.cmd(["delete", "non-existent-key", "key-b"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: false
    exit_code: 1
    ----- stdout -----

    ----- stderr -----
    Error: couldn't delete content: keys don't exist in stash: ["non-existent-key"]
    "#);
}
