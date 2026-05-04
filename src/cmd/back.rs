/// Go back to last placed stack waypoint

use anyhow::Result;

use crate::cmd::{Run, Back};
use crate::utils;

impl Run for Back {
    fn run(&self) -> Result<()> {
        let dir = utils::popDirectory();
        println!("{}", dir.display());
        Ok(())
    }
}

impl Back {

}

