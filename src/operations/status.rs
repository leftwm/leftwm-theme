use crate::errors;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct Status {}

impl Status {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        println!("Status");
        Ok(())
    }
}
