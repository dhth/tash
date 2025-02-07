use crate::errors::AppError::{self, *};

use crate::actions::{delete_content_items, empty_stash, get_content, list_content, push_content};
use crate::args::{Action, Args};
use dirs::data_dir;
use std::path::PathBuf;

const DATA_DIR: &str = "tash";
const DATA_DIR_ENV_VAR: &str = "TASH_DATA_DIR";

pub fn handle(args: Args) -> Result<(), AppError> {
    let data_dir = match std::env::var(DATA_DIR_ENV_VAR) {
        Ok(data_dir_from_env_var) => PathBuf::from(data_dir_from_env_var),
        Err(e) => match e {
            std::env::VarError::NotPresent => {
                let user_data_dir = data_dir().ok_or(GetDataDir)?;
                user_data_dir.join(PathBuf::from(DATA_DIR))
            }
            std::env::VarError::NotUnicode(_) => return Err(DataDirEnvVarInvalid(e)),
        },
    };

    if !data_dir.exists() {
        std::fs::create_dir_all(&data_dir).map_err(CreateDataDir)?;
    }

    match args.action {
        Action::D { keys } => {
            delete_content_items(&data_dir, &keys).map_err(DeleteContent)?;
        }
        Action::E { skip_confirmation } => {
            empty_stash(&data_dir, skip_confirmation).map_err(EmptyStash)?;
        }
        Action::G {
            key,
            no_output,
            copy_to_clipboard,
            pop,
        } => {
            get_content(&data_dir, &key, no_output, copy_to_clipboard, pop).map_err(GetContent)?;
        }
        Action::LS => {
            list_content(&data_dir).map_err(ListContent)?;
        }
        Action::P {
            key,
            data,
            file_path,
            get_content_from_clipboard,
            prevent_overwrite,
            verbose,
        } => {
            push_content(
                &data_dir,
                &key,
                data.as_deref(),
                file_path.as_deref(),
                get_content_from_clipboard,
                prevent_overwrite,
                verbose,
            )
            .map_err(PushContent)?;
        }
    }

    Ok(())
}
