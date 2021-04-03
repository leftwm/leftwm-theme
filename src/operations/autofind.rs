use crate::errors;
use crate::models::Config;
use clap::Clap;
use colored::Colorize;
use std::path::Path;
use xdg::BaseDirectories;

#[derive(Clap, Debug)]
pub struct AutoFind {
    /// Optional directory to search for themes defined by a theme.toml / git pair
    pub dir: Option<String>,
}

impl AutoFind {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        unimplemented!("Not yet implemented");
        Ok(())
    }
}
