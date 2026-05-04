/// Go back to last placed stack waypoint
use anyhow::Result;

use crate::cmd::{Back, Run};
use crate::utils;

impl Run for Back {
    fn run(&self) -> Result<()> {
        let dir = utils::popDirectory();
        println!("{}", dir.display());
        Ok(())
    }
}

impl Back {}
