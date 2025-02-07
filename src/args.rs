use clap::{Parser, Subcommand};

/// tash "stashes" content that you can access later
#[derive(Parser, Debug)]
#[command(about)]
pub struct Args {
    /// action to take
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Delete one or more content items
    D {
        /// keys to delete
        #[arg(value_name = "KEY")]
        keys: Vec<String>,
    },
    /// Empty entire stash
    E {
        /// skip confirmation
        #[arg(short = 'y', long = "yes")]
        skip_confirmation: bool,
    },
    /// List stashed content keys
    LS,
    /// Get content from stash
    G {
        /// key used when stashing content
        #[arg(value_name = "KEY")]
        key: String,
        /// don't output content to stdout
        #[arg(short = 'n', long = "no-output")]
        no_output: bool,
        /// whether to copy content to system clipboard
        #[arg(short = 'c', long = "clipboard")]
        copy_to_clipboard: bool,
        /// whether to remove content from stash
        #[arg(short = 'p', long = "pop")]
        pop: bool,
    },
    /// Push content to stash
    P {
        /// a short string to remember the content by (needs to conform to the regex ^[a-z0-9_-]{1,30}$)
        #[arg(value_name = "KEY")]
        key: String,
        /// content to stash
        #[arg(short = 'd', long = "data", value_name = "STRING")]
        data: Option<String>,
        /// path of the file whose contents to stash
        #[arg(short = 'f', long = "file-path", value_name = "STRING")]
        file_path: Option<String>,
        /// whether to get content from system clipboard
        #[arg(short = 'c', long = "clipboard")]
        get_content_from_clipboard: bool,
        /// fail if key already exists in the stash
        #[arg(short = 'p', long = "prevent-overwrite")]
        prevent_overwrite: bool,
        /// echo contents back to stdout
        #[arg(short = 'e', long = "echo")]
        echo: bool,
        /// whether to output information about the stashed file
        #[arg(short = 'v', long = "verbose")]
        verbose: bool,
    },
}
