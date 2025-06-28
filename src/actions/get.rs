use arboard::{Clipboard, Error as ArboardError};
use std::fs::File;
use std::io::Error as IOError;
use std::io::Read;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum GetError {
    #[error("key doesn't exist in stash")]
    KeyDoesntExist,
    #[error("couldn't remove file from tash's data directory: {0}")]
    CouldntRemoveFile(IOError),
    #[error("couldn't open file in tash's data directory: {0}")]
    CouldntOpenFile(IOError),
    #[error("couldn't read file contents: {0}")]
    CouldntReadFile(IOError),
    #[error("couldn't access system clipboard: {0}")]
    CouldntAccessSystemClipboard(ArboardError),
    #[error("couldn't write to system clipboard: {0}")]
    CouldntWriteToSystemClipboard(ArboardError),
}

pub fn get_content(
    data_dir: &Path,
    key: &str,
    no_output: bool,
    copy_to_clipboard: bool,
    pop: bool,
) -> Result<(), GetError> {
    let stashed_file_path = data_dir.join(PathBuf::from(key));

    if !stashed_file_path.exists() {
        return Err(GetError::KeyDoesntExist);
    }

    if no_output && !copy_to_clipboard {
        if pop {
            std::fs::remove_file(&stashed_file_path).map_err(GetError::CouldntRemoveFile)?;
        }
        return Ok(());
    }

    let mut file = File::open(&stashed_file_path).map_err(GetError::CouldntOpenFile)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(GetError::CouldntReadFile)?;

    if !no_output {
        print!("{contents}");
    }

    if copy_to_clipboard {
        let mut clipboard = Clipboard::new().map_err(GetError::CouldntAccessSystemClipboard)?;

        clipboard
            .set_text(&contents)
            .map_err(GetError::CouldntWriteToSystemClipboard)?;
    }
    if pop {
        std::fs::remove_file(&stashed_file_path).map_err(GetError::CouldntRemoveFile)?;
    }

    Ok(())
}
