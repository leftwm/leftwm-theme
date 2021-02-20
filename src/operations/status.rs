use crate::errors;
use crate::models::LeftWM;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct Status {}

impl Status {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        dbg!(LeftWM::get()?);
        Ok(())
    }
}
