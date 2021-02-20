use crate::errors;
use crate::models::LeftWm;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct Status {}

impl Status {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        dbg!(LeftWm::get()?);
        Ok(())
    }
}
