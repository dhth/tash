mod common;
use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn emptying_stash_works() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/static/sample.txt"]);
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = fx.cmd(["empty", "--yes"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r"
    success: true
    exit_code: 0
    ----- stdout -----
    Deleted 3 entries

    ----- stderr -----
    ");
}
