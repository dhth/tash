use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum EmptyError {
    #[error("couldn't read from tash's data directory: {0}")]
    ReadFromDataDir(IOError),
    #[error("couldn't read from stdin: {0}")]
    ReadFromStdin(IOError),
    #[error("couldn't delete tash's data directory: {0}")]
    DeleteDataDir(IOError),
}

pub fn empty_stash(data_dir: &PathBuf, skip_confirmation: bool) -> Result<(), EmptyError> {
    let entries = std::fs::read_dir(data_dir).map_err(EmptyError::ReadFromDataDir)?;

    let file_count = entries
        .filter_map(Result::ok)
        .filter(|entry| entry.path().is_file())
        .count();

    if file_count == 0 {
        println!("nothing to delete");
        return Ok(());
    }

    let entry_str = if file_count == 1 { "entry" } else { "entries" };

    if !skip_confirmation {
        println!(
            "This will permanently delete {file_count} {entry_str} from the stash. Enter \"yes\" to continue."
        );

        let mut input = String::new();

        std::io::stdin()
            .read_line(&mut input)
            .map_err(EmptyError::ReadFromStdin)?;

        let input = input.trim();
        if input != "yes" {
            println!("deletion cancelled");
            return Ok(());
        }
    }

    std::fs::remove_dir_all(data_dir).map_err(EmptyError::DeleteDataDir)?;
    println!("Deleted {file_count} {entry_str}");

    Ok(())
}
