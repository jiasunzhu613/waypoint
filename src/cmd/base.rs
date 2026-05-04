/// Add stack waypoint

use anyhow::Result;
use crate::cmd::{Run, Base};
use crate::utils;

impl Run for Base {
    fn run(&self) -> Result<()> {
        utils::pushDirectory(self.path.to_owned().unwrap()).unwrap();
        Ok(())
    }
}

impl Base {

}

