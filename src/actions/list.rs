use std::io::Error as IOError;
use std::path::PathBuf;

#[derive(thiserror::Error, Debug)]
pub enum ListError {
    #[error("couldn't read files in tash's data directory: {0}")]
    ReadFilesInDataDir(IOError),
    #[error("couldn't get file from tash's data directory: {0}")]
    GetFileFromDataDir(IOError),
    #[error("couldn't get the name of a file in tash's data directory; path: {0}")]
    GetFileStem(String),
}

pub fn list_content(data_dir: &PathBuf) -> Result<(), ListError> {
    let mut stashed_files = Vec::new();
    for entry in std::fs::read_dir(data_dir).map_err(ListError::ReadFilesInDataDir)? {
        let entry = entry.map_err(ListError::GetFileFromDataDir)?;
        let path = entry.path();
        if path.is_file() {
            let f = path
                .file_stem()
                .ok_or(ListError::GetFileStem(
                    path.to_string_lossy().to_ascii_lowercase(),
                ))?
                .to_owned();
            stashed_files.push(f);
        }
    }

    stashed_files.sort();

    let output = stashed_files
        .iter()
        .map(|os_str| os_str.to_string_lossy())
        .collect::<Vec<_>>()
        .join("\n");
    println!("{}", output);

    Ok(())
}
