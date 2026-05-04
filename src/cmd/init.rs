use crate::cmd::{Init, Run};
/// Used to initialize CLI setup in different shells
use anyhow::Result;

impl Run for Init {
    fn run(&self) -> Result<()> {
        println!("Ran init"); // TODO
        Ok(())
    }
}

impl Init {}
