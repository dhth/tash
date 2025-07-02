mod common;
use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn listing_from_an_empty_stash_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["ls"]);

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
fn listing_content_works() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/static/sample.txt"]);
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = fx.cmd(["ls"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    key-a
    key-b
    key-c

    ----- stderr -----
    ");
}
