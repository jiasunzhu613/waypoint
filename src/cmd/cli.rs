use clap::{Parser, Subcommand, value_parser};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None,
    args_conflicts_with_subcommands = true
)]
pub struct Base {
    #[arg(value_parser = value_parser!(PathBuf), required = true)]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Init(Init),
    Back(Back),
    List(List),
}

#[derive(Parser)]
pub struct Init {} // TODO

#[derive(Parser)]
pub struct Back {} // TODO

#[derive(Parser)]
pub struct List {} // TODO
