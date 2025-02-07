use std::io::Error as IOError;
use std::path::{Path, PathBuf};

#[derive(thiserror::Error, Debug)]
pub enum DeleteError {
    #[error("keys don't exist in stash: {0:?}")]
    KeysDontExist(Vec<String>),
    #[error("couldn't remove files for keys {0:?} from tash's data directory, errors: {1:?}")]
    CouldntRemoveFiles(Vec<String>, Vec<IOError>),
}

pub fn delete_content_items(data_dir: &Path, keys: &Vec<String>) -> Result<(), DeleteError> {
    let mut nonexistent_keys = Vec::new();
    for key in keys {
        let stashed_file_path = data_dir.join(PathBuf::from(key));
        if !stashed_file_path.exists() {
            nonexistent_keys.push(key);
        }
    }

    if !nonexistent_keys.is_empty() {
        return Err(DeleteError::KeysDontExist(
            nonexistent_keys.iter().map(|s| s.to_string()).collect(),
        ));
    }

    let mut delete_errors = Vec::new();
    let mut failed_keys = Vec::new();
    for key in keys {
        let stashed_file_path = data_dir.join(PathBuf::from(key));
        if let Err(e) = std::fs::remove_file(&stashed_file_path) {
            delete_errors.push(e);
            failed_keys.push(key.to_string());
        }
    }

    if !delete_errors.is_empty() {
        return Err(DeleteError::CouldntRemoveFiles(failed_keys, delete_errors));
    }

    Ok(())
}
