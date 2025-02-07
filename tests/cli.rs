use assert_cmd::Command;
use pretty_assertions::assert_eq;
use tempfile::{tempdir, TempDir};

struct Fixture {
    _tmp_dir: TempDir,
    tmp_dir_str: String,
}

impl Fixture {
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
    // WHEN
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.arg("--help");
    let output = cmd.output().expect("running command failed");

    // THEN
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("invalid utf-8 stdout");
    assert!(stdout.contains("tash \"stashes\" content that you can access later"));
}

#[test]
fn pushing_content_from_flag_works() {
    // GIVEN
    let fixture = Fixture::new();

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("p");
    cmd.arg("key");
    cmd.arg("-d=\"content goes here\"");
    let output = cmd.output().expect("command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "output wasn't a success");
}

#[test]
fn pushing_content_from_local_file_works() {
    // GIVEN
    let fixture = Fixture::new();

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("p");
    cmd.arg("key");
    cmd.arg("-f=tests/sample.txt");
    let output = cmd.output().expect("command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "output wasn't a success");
}

#[test]
fn listing_from_an_empty_stash_works() {
    // GIVEN
    let fixture = Fixture::new();

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("ls");
    cmd.output().expect("command should've been executed");
    let output = cmd.output().expect("command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "output wasn't a success");
    let stdout =
        String::from_utf8(output.stdout).expect("a string should've been created from cmd stdout");
    assert!(stdout.as_str().trim().is_empty())
}

#[test]
fn getting_content_works() {
    // GIVEN
    let fixture = Fixture::new();
    let mut push_cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("push command should've been created");
    push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    push_cmd.arg("p");
    push_cmd.arg("key");
    push_cmd.arg("-f=tests/sample.txt");
    push_cmd
        .output()
        .expect("push command should've been executed");

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("get command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("g");
    cmd.arg("key");
    let output = cmd.output().expect("get command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "output wasn't a success");
    let stdout =
        String::from_utf8(output.stdout).expect("a string should've been created from cmd stdout");
    assert_eq!(
        stdout.as_str().trim(),
        r#"A sample file for tash.

Content goes here."#
    )
}

#[test]
fn getting_content_and_popping_works() {
    // GIVEN
    let fixture = Fixture::new();
    let mut push_cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("push command should've been created");
    push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    push_cmd.arg("p");
    push_cmd.arg("key");
    push_cmd.arg("-f=tests/sample.txt");
    push_cmd
        .output()
        .expect("push command should've been executed");

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("get command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("g");
    cmd.arg("key");
    cmd.arg("-p");
    let output = cmd.output().expect("get command should've been executed");

    let mut second_get_cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("get command should've been created");
    second_get_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    second_get_cmd.arg("g");
    second_get_cmd.arg("key");
    let second_cmd_output = second_get_cmd
        .output()
        .expect("second get command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "output wasn't a success");
    let stdout =
        String::from_utf8(output.stdout).expect("a string should've been created from cmd stdout");
    assert_eq!(
        stdout.as_str().trim(),
        r#"A sample file for tash.

Content goes here."#
    );

    if second_cmd_output.status.success() {
        let stdout = String::from_utf8(second_cmd_output.stdout)
            .expect("a string should've been created from the second command stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(
        !second_cmd_output.status.success(),
        "second cmd wasn't a failure"
    );
    let stderr = String::from_utf8(second_cmd_output.stderr)
        .expect("a string should've been created from cmd stderr");
    assert!(stderr.as_str().contains("key doesn't exist in stash"));
}

#[test]
fn listing_content_works() {
    // GIVEN
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .expect("push command should've been created");
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("p");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("ls command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("ls");
    let output = cmd.output().expect("ls command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "output wasn't a success");
    let stdout =
        String::from_utf8(output.stdout).expect("a string should've been created from cmd stdout");
    assert_eq!(
        stdout.as_str(),
        r#"key-a
key-b
key-c
"#
    )
}

#[test]
fn deleting_content_items_works() {
    // GIVEN
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .expect("push command should've been created");
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("p");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("delete command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("d");
    cmd.arg("key-a");
    cmd.arg("key-b");
    let output = cmd
        .output()
        .expect("delete command should've been executed");

    let mut ls_cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("ls command should've been created");
    ls_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    ls_cmd.arg("ls");
    let ls_output = ls_cmd.output().expect("ls command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "delete output wasn't a success");

    if !ls_output.status.success() {
        let stderr = String::from_utf8(ls_output.stderr)
            .expect("a string should've been created from ls_command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(ls_output.status.success(), "ls_output wasn't a success");
    let ls_stdout = String::from_utf8(ls_output.stdout)
        .expect("a string should've been created from ls_cmd stdout");
    assert_eq!(ls_stdout.as_str().trim(), "key-c")
}

#[test]
fn emptying_stash_works() {
    // GIVEN
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .expect("push command should've been created");
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("p");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("empty command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("e");
    cmd.arg("-y");
    let output = cmd.output().expect("empty command should've been executed");

    // THEN
    if !output.status.success() {
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from command stderr");
        println!("stderr: \n{}", stderr);
    }
    assert!(output.status.success(), "output wasn't a success");
    let stdout =
        String::from_utf8(output.stdout).expect("a string should've been created from cmd stdout");
    assert_eq!(stdout.as_str().trim(), "Deleted 3 entries");
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

    for incorrect_key in incorrect_keys {
        // WHEN
        let mut cmd =
            Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("get command should've been created");
        cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        cmd.arg("p");
        cmd.arg(incorrect_key);
        cmd.arg("-f=tests/sample.txt");
        let output = cmd.output().expect("get command should've been executed");

        // THEN
        if output.status.success() {
            let stdout = String::from_utf8(output.stdout)
                .expect("a string should've been created from command stdout");
            println!("stdout: \n{}", stdout);
        }
        assert!(!output.status.success(), "output wasn't a failure");
        let stderr = String::from_utf8(output.stderr)
            .expect("a string should've been created from cmd stderr");
        assert!(stderr.as_str().contains("incorrect key provided"));
    }
}

#[test]
fn fails_if_key_doesnt_exist() {
    // GIVEN
    let fixture = Fixture::new();

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("get command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("g");
    cmd.arg("non-existent-key");
    let output = cmd.output().expect("get command should've been executed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)
            .expect("a string should've been created from command stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success(), "output wasn't a failure");
    let stderr =
        String::from_utf8(output.stderr).expect("a string should've been created from cmd stderr");
    assert!(stderr.as_str().contains("key doesn't exist in stash"));
}

#[test]
fn fails_if_content_overwrites_are_not_desired() {
    // GIVEN
    let fixture = Fixture::new();
    let mut first_push_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .expect("first push command should've been created");
    first_push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    first_push_cmd.arg("p");
    first_push_cmd.arg("key");
    first_push_cmd.arg("-d=\"content goes here\"");
    first_push_cmd
        .output()
        .expect("first push command should've been executed");

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("p");
    cmd.arg("key");
    cmd.arg("-d=\"content goes here\"");
    cmd.arg("-p");
    let output = cmd.output().expect("command should've been executed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)
            .expect("a string should've been created from command stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success(), "output wasn't a failure");
    let stderr =
        String::from_utf8(output.stderr).expect("a string should've been created from cmd stderr");
    assert!(stderr.as_str().contains("key already exists in the stash"));
}

#[test]
fn deletion_fails_if_one_or_more_keys_dont_exist() {
    // GIVEN
    let fixture = Fixture::new();
    let keys = vec!["key-b", "key-c", "key-a"];
    for key in keys {
        let mut push_cmd = Command::cargo_bin(env!("CARGO_PKG_NAME"))
            .expect("push command should've been created");
        push_cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
        push_cmd.arg("p");
        push_cmd.arg(key);
        push_cmd.arg("-f=tests/sample.txt");
        push_cmd
            .output()
            .expect("push command should've been executed");
    }

    // WHEN
    let mut cmd =
        Command::cargo_bin(env!("CARGO_PKG_NAME")).expect("get command should've been created");
    cmd.env("TASH_DATA_DIR", &fixture.tmp_dir_str);
    cmd.arg("d");
    cmd.arg("non-existent-key");
    cmd.arg("key-b");
    let output = cmd.output().expect("get command should've been executed");

    // THEN
    if output.status.success() {
        let stdout = String::from_utf8(output.stdout)
            .expect("a string should've been created from command stdout");
        println!("stdout: \n{}", stdout);
    }
    assert!(!output.status.success(), "output wasn't a failure");
    let stderr =
        String::from_utf8(output.stderr).expect("a string should've been created from cmd stderr");
    assert!(stderr.as_str().contains("keys don't exist in stash"));
}
