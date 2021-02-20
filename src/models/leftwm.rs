use crate::errors::*;
use std::process::Command;
use std::str;

#[derive(Debug)]
pub struct LeftWM {
    pub version: String,
}

impl LeftWM {
    pub fn get() -> Result<Self> {
        match str::from_utf8(&Command::new("leftwm-state").arg("-V").output()?.stdout) {
            Ok(output) => Ok(LeftWM {
                version: output.replace("LeftWM State ", "").replace("\n", ""),
            }),
            Err(_) => {
                log::error!("Could not get LeftWM version. Is LeftWM installed?");
                Err(LeftError::from("UTF Error"))
            }
        }
    }
}
