mod back;
mod base;
mod cli;
mod init;
mod list;

use anyhow::Result;

pub use crate::cmd::cli::*;

pub trait Run {
    fn run(&self) -> Result<()>;
}

impl Run for Commands {
    fn run(&self) -> Result<()> {
        match self {
            Commands::Init(cmd) => cmd.run(),
            Commands::Back(cmd) => cmd.run(),
            Commands::List(cmd) => cmd.run(),
        }
    }
}
