mod actions;
mod args;
mod common;
mod errors;
mod handle;

use args::Args;
use clap::Parser;
use handle::handle;

fn main() {
    let args = Args::parse();
    let result = handle(args);

    if let Err(error) = &result {
        eprintln!("Error: {}", error);
        if let Some(c) = error.code() {
            eprintln!(
                "
------

This error (code: {}) is unexpected.
Let @dhth know about this via https://github.com/dhth/tash/issues",
                c
            );
        }
        std::process::exit(1);
    }
}
