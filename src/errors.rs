use crate::actions::{DeleteError, EmptyError, GetError, ListError, PushError};
use std::io::Error as IOError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("couldn't get your DATA directory")]
    GetDataDir,
    #[error("TASH_DATA_DIR value is invalid: {0}")]
    DataDirEnvVarInvalid(std::env::VarError),
    #[error("couldn't create data directory: {0}")]
    CreateDataDir(IOError),
    #[error("couldn't empty stash: {0}")]
    EmptyStash(EmptyError),
    #[error("couldn't get content: {0}")]
    GetContent(GetError),
    #[error("couldn't list content: {0}")]
    ListContent(ListError),
    #[error("couldn't push content: {0}")]
    PushContent(PushError),
    #[error("couldn't delete content: {0}")]
    DeleteContent(DeleteError),
}

impl AppError {
    pub fn code(&self) -> Option<u16> {
        match self {
            AppError::GetDataDir => None,
            AppError::DataDirEnvVarInvalid(_) => None,
            AppError::CreateDataDir(_) => Some(101),
            AppError::EmptyStash(e) => match e {
                EmptyError::ReadFromDataDir(_) => Some(200),
                EmptyError::ReadFromStdin(_) => Some(201),
                EmptyError::DeleteDataDir(_) => Some(203),
            },
            AppError::GetContent(e) => match e {
                GetError::KeyDoesntExist => None,
                GetError::CouldntRemoveFile(_) => Some(300),
                GetError::CouldntOpenFile(_) => Some(301),
                GetError::CouldntReadFile(_) => Some(302),
                GetError::CouldntAccessSystemClipboard(_) => Some(303),
                GetError::CouldntWriteToSystemClipboard(_) => Some(304),
            },
            AppError::ListContent(e) => match e {
                ListError::ReadFilesInDataDir(_) => Some(400),
                ListError::GetFileFromDataDir(_) => Some(401),
                ListError::GetFileStem(_) => Some(402),
            },
            AppError::PushContent(e) => match e {
                PushError::IncorrectKeyProvided => None,
                PushError::KeyAlreadyExists => None,
                PushError::MultipleInputSourcesProvided => None,
                PushError::CouldntOpenFile(_) => Some(500),
                PushError::CouldntReadFile(_) => Some(501),
                PushError::CouldntReadFromStdin(_) => Some(502),
                PushError::CouldntAccessSystemClipboard(_) => Some(503),
                PushError::CouldntReadFromSystemClipboard(_) => Some(504),
                PushError::ContentTooLarge(_) => None,
                PushError::CouldntWriteToFile(_) => Some(505),
            },
            AppError::DeleteContent(e) => match e {
                DeleteError::KeysDontExist(_) => None,
                DeleteError::CouldntRemoveFiles(_, _) => Some(600),
            },
        }
    }
}
