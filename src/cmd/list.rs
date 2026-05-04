/// Lists all existing waypoints

use anyhow::Result;
use crate::cmd::{Run, List};

impl Run for List {
    fn run(&self) -> Result<()> {
        println!("Ran list"); // TODO
        Ok(())
    }
}

impl List {

}


