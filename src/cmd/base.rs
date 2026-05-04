use crate::cmd::{Base, Run};
use crate::utils;
/// Add stack waypoint
use anyhow::Result;

impl Run for Base {
    fn run(&self) -> Result<()> {
        utils::pushDirectory(self.path.to_owned().unwrap()).unwrap();
        Ok(())
    }
}

impl Base {}
