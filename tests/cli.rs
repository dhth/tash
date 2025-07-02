use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

mod common;

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    let fx = Fixture::new();
    let mut cmd = fx.cmd(["--help"]);

    // WHEN
    // THEN
    assert_cmd_snapshot!(cmd, @r#"
    success: true
    exit_code: 0
    ----- stdout -----
    tash "stashes" content that you can access later

    Usage: tash <COMMAND>

    Commands:
      delete  Delete one or more content items
      empty   Empty entire stash
      ls      List stashed content keys
      get     Get content from stash
      push    Stash content
      help    Print this message or the help of the given subcommand(s)

    Options:
      -h, --help  Print help

    ----- stderr -----
    "#);
}

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
    let mut cmd = fx.cmd(["push", "key", "--file-path", "tests/sample.txt"]);

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
fn getting_content_works() {
    // GIVEN
    let fx = Fixture::new();
    let mut push_cmd = fx.cmd(["push", "key", "--file-path", "tests/sample.txt"]);
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
    let mut push_cmd = fx.cmd(["push", "key", "--file-path", "tests/sample.txt"]);
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

#[test]
fn listing_content_works() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/sample.txt"]);
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

#[test]
fn deleting_content_items_works() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/sample.txt"]);
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

#[test]
fn emptying_stash_works() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/sample.txt"]);
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
            let mut cmd = fx.cmd(["push", incorrect_key, "--file-path", "tests/sample.txt"]);

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

#[test]
fn deletion_fails_if_one_or_more_keys_dont_exist() {
    // GIVEN
    let fx = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = fx.cmd(["push", key, "--file-path", "tests/sample.txt"]);
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
