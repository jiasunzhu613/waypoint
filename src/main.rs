mod utils;

use std::path::PathBuf;
use clap::{Command, arg, value_parser, Parser};
use std::env;


fn main() {
    // TODO: change this to derive way of expressing clap parser?
    let matches = Command::new("waypoint")
        .version("1.0")
        .about("Set waypoints to your directories like its a game!")
        .args_conflicts_with_subcommands(true)
        .arg(
            arg!([path] "Path to set as a temporary waypoint")
                .required(true)
                .value_parser(value_parser!(PathBuf))
        )
        .subcommand(
            Command::new("back")
            .about("Go back to directory of last saved waypoint")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("back", sub_matches)) => {
            // println!("Go back to last saved waypoint");
            let dir = utils::popDirectory();
            println!("{dir}");
            
            // env::set_current_dir(&dir).unwrap();
            // unsafe {
            //     env::set_var("PWD", "~");
            // }
        },
        _ => {
            if let Some(path) = matches.get_one::<PathBuf>("path") {
                // println!("Path to be saved: {}", path.display());
                utils::pushDirectory(path.to_string_lossy().into_owned()).unwrap();

            }
        }
    }
}
