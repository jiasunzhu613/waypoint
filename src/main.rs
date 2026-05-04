mod utils;
mod cmd;

use std::path::PathBuf;
use clap::{Command, arg, value_parser, Parser};

use cmd::Base;
use crate::cmd::Run;

fn main() {
    let cli = Base::parse();

    let result = match &cli.command {
        Some(cmd) => cmd.run(),
        None => cli.run(),
    };

    // TODO: add handling for happy and sad paths
    match result {
        Ok(_) => {},
        Err(why) => {}
    }
}
