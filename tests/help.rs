mod common;
use common::Fixture;
use insta_cmd::assert_cmd_snapshot;

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
