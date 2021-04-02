use crate::errors::{LeftError, Result};
use std::process::Command;
use std::str;

#[derive(Debug)]
pub struct LeftWm {
    pub version: String,
}

impl LeftWm {
    pub fn get() -> Result<Self> {
        let version_raw = &Command::new("leftwm-state").arg("-V").output()?.stdout;
        let version_utf8 = str::from_utf8(version_raw).map_err(|_| LeftError::from("UTF Error"))?;
        let version = version_utf8.replace("LeftWM State ", "").replace("\n", "");
        Ok(LeftWm { version })
    }
}
