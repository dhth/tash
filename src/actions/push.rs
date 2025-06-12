use arboard::{Clipboard, Error as ArboardError};
use regex::Regex;
use std::fs::File;
use std::io::Error as IOError;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

const KEY_REGEX_STR: &str = r"^[a-z0-9_-]{1,30}$";
const CONTENT_MAX_BYTES: usize = 50 * 1024 * 1024;

#[derive(thiserror::Error, Debug)]
pub enum PushError {
    #[error("incorrect key provided (valid regex: {KEY_REGEX_STR})")]
    IncorrectKeyProvided,
    #[error("key already exists in the stash")]
    KeyAlreadyExists,
    #[error("multiple input sources provided, only one can be used at a time")]
    MultipleInputSourcesProvided,
    #[error("couldn't open file in tash's data directory: {0}")]
    CouldntOpenFile(IOError),
    #[error("couldn't read file contents: {0}")]
    CouldntReadFile(IOError),
    #[error("couldn't read from stdin: {0}")]
    CouldntReadFromStdin(IOError),
    #[error("couldn't access system clipboard: {0}")]
    CouldntAccessSystemClipboard(ArboardError),
    #[error("couldn't read from system clipboard: {0}")]
    CouldntReadFromSystemClipboard(ArboardError),
    #[error("content is too large (actual: {0} bytes, threshold: {CONTENT_MAX_BYTES} bytes)")]
    ContentTooLarge(usize),
    #[error("couldn't write to file in tash's data directory: {0}")]
    CouldntWriteToFile(IOError),
    #[error("couldn't echo content back to stdout: {0}")]
    CouldntEchoContent(std::string::FromUtf8Error),
}

#[allow(clippy::too_many_arguments)]
pub fn push_content(
    data_dir: &Path,
    key: &str,
    data: Option<&str>,
    file_path: Option<&str>,
    get_content_from_clipboard: bool,
    prevent_overwrite: bool,
    echo: bool,
    verbose: bool,
) -> Result<(), PushError> {
    #[allow(clippy::expect_used)]
    let re = Regex::new(KEY_REGEX_STR).expect("regex is invalid");
    if !re.is_match(key) {
        return Err(PushError::IncorrectKeyProvided);
    }

    let stash_file_path = data_dir.join(PathBuf::from(key));

    if prevent_overwrite && stash_file_path.exists() {
        return Err(PushError::KeyAlreadyExists);
    }

    let content = match (data, file_path, get_content_from_clipboard) {
        (Some(_), Some(_), true) | (Some(_), Some(_), false) | (Some(_), None, true) => {
            return Err(PushError::MultipleInputSourcesProvided)
        }
        (None, Some(p), false) => {
            let mut file = File::open(p).map_err(PushError::CouldntOpenFile)?;
            let mut contents = Vec::new();
            file.read_to_end(&mut contents)
                .map_err(PushError::CouldntReadFile)?;

            contents
        }
        (None, None, true) => {
            let mut clipboard =
                Clipboard::new().map_err(PushError::CouldntAccessSystemClipboard)?;
            let clipboard_text = clipboard
                .get_text()
                .map_err(PushError::CouldntReadFromSystemClipboard)?;

            clipboard_text.into_bytes()
        }
        (None, Some(_), true) => todo!(),
        (None, None, false) => {
            let mut buffer = Vec::new();
            std::io::stdin()
                .read_to_end(&mut buffer)
                .map_err(PushError::CouldntReadFromStdin)?;
            buffer
        }
        (Some(c), None, false) => c.as_bytes().to_vec(),
    };

    if content.len() > CONTENT_MAX_BYTES {
        return Err(PushError::ContentTooLarge(content.len()));
    }

    let mut stash_file = File::create(&stash_file_path).map_err(PushError::CouldntOpenFile)?;

    stash_file
        .write_all(&content)
        .map_err(PushError::CouldntWriteToFile)?;

    if verbose {
        println!("stashed {} bytes", content.len());
    }

    if echo {
        let content_str = String::from_utf8(content).map_err(PushError::CouldntEchoContent)?;
        print!("{}", content_str);
    }

    Ok(())
}
