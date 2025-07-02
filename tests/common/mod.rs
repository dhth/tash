use insta_cmd::get_cargo_bin;
use std::ffi::OsStr;
use std::process::Command;
use tempfile::{tempdir, TempDir};

#[cfg(test)]
pub struct Fixture {
    _tmp_dir: TempDir,
    tmp_dir_str: String,
}

#[cfg(test)]
impl Fixture {
    #[allow(clippy::expect_used)]
    pub fn new() -> Self {
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

    pub fn cmd<I, S>(&self, args: I) -> Command
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let mut cmd = Command::new(get_cargo_bin("tash"));
        cmd.env("TASH_DATA_DIR", &self.tmp_dir_str);
        cmd.args(args);
        cmd
    }
}
