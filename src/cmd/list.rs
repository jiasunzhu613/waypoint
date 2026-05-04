/// Lists all existing waypoints

use anyhow::Result;

use crate::cmd::{Run, List};
use crate::utils;

impl Run for List {
    fn run(&self) -> Result<()> {
        let json = utils::read_waypoint_info().unwrap();

        if json.stack.len() == 0 {
            println!("Empty directory stack.");
            return Ok(());
        }

        println!("Current directory stack:");
        for path in json.stack.iter().rev() {
            println!("{}", path.display());
        }

        Ok(())
    }
}

impl List {

}


