mod cmd;
mod utils;

use clap::{Command, Parser, arg, value_parser};
use std::path::PathBuf;

use crate::cmd::Run;
use cmd::Base;

fn main() {
    let cli = Base::parse();

    let result = match &cli.command {
        Some(cmd) => cmd.run(),
        None => cli.run(),
    };

    // TODO: add handling for happy and sad paths
    match result {
        Ok(_) => {}
        Err(why) => {}
    }
}
