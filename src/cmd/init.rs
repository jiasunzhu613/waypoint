/// Used to initialize CLI setup in different shells

use anyhow::Result;
use crate::cmd::{Run, Init};

impl Run for Init {
    fn run(&self) -> Result<()> {
        println!("Ran init"); // TODO
        Ok(())
    }
}

impl Init {

}



