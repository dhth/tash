use insta_cmd::assert_cmd_snapshot;
use tempfile::{tempdir, TempDir};

mod common;

struct Fixture {
    _tmp_dir: TempDir,
    tmp_dir_str: String,
}

impl Fixture {
    #[allow(clippy::expect_used)]
    fn new() -> Self {
        let tmp_dir = tempdir().expect("temporary directory should've been created");
        let tmp_dir_str = tmp_dir
            .path()
            .to_str()
            .expect("temporary directory should've been converted to a string")
            .to_string();

        Self {
            _tmp_dir: tmp_dir,
            tmp_dir_str,
        }
    }
}

//-------------//
//  SUCCESSES  //
//-------------//

#[test]
fn shows_help() {
    // GIVEN
    let mut cmd = common::base_command();
    cmd.arg("--help");

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
    let fixture = Fixture::new();
    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("push");
    cmd.arg("key");
    cmd.arg("-d=content goes here");

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
    let fixture = Fixture::new();
    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("push");
    cmd.arg("key");
    cmd.arg("-f=tests/sample.txt");

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
    let fixture = Fixture::new();
    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("push");
    cmd.arg("key");
    cmd.arg("-d=content goes here");
    cmd.arg("-e");

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
    let fixture = Fixture::new();
    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("ls");

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
    let fixture = Fixture::new();
    let mut push_cmd = common::base_command();
    push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    push_cmd.arg("push");
    push_cmd.arg("key");
    push_cmd.arg("-f=tests/sample.txt");
    push_cmd
        .output()
        .expect("push command should've been executed");

    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("get");
    cmd.arg("key");

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
    let fixture = Fixture::new();
    let mut push_cmd = common::base_command();
    push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    push_cmd.arg("push");
    push_cmd.arg("key");
    push_cmd.arg("-f=tests/sample.txt");
    push_cmd
        .output()
        .expect("push command should've been executed");

    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("get");
    cmd.arg("key");
    cmd.arg("-p");

    let mut second_get_cmd = common::base_command();
    second_get_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    second_get_cmd.arg("get");
    second_get_cmd.arg("key");

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
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = common::base_command();
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("push");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("ls");

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
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = common::base_command();
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("push");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("delete");
    cmd.arg("key-a");
    cmd.arg("key-b");

    let mut ls_cmd = common::base_command();
    ls_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    ls_cmd.arg("ls");

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
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = common::base_command();
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("push");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("empty");
    cmd.arg("-y");

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
    let fixture = Fixture::new();
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
            let mut cmd = common::base_command();
            cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
            cmd.arg("push");
            cmd.arg(incorrect_key);
            cmd.arg("-f=tests/sample.txt");

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
    let fixture = Fixture::new();
    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("get");
    cmd.arg("non-existent-key");

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
    let fixture = Fixture::new();
    let mut first_push_cmd = common::base_command();
    first_push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    first_push_cmd.arg("push");
    first_push_cmd.arg("key");
    first_push_cmd.arg("-d=\"content goes here\"");
    first_push_cmd
        .output()
        .expect("first push command should've been executed");

    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("push");
    cmd.arg("key");
    cmd.arg("-d=content goes here");
    cmd.arg("-p");

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
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = common::base_command();
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("push");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    let mut cmd = common::base_command();
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("delete");
    cmd.arg("non-existent-key");
    cmd.arg("key-b");

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
