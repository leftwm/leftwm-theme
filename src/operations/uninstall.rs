use crate::errors;
use crate::errors::LeftError;
use crate::models::{Config, Theme};
use crate::utils::read::yes_or_no;
use clap::Clap;
use colored::*;
use log::error;
use std::fs;
use std::path::Path;

#[derive(Clap, Debug)]
pub struct Uninstall {
    pub name: String,
}

impl Uninstall {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        println!(
            "{}",
            "Looking for theme to uninstall . . . ".bright_blue().bold()
        );
        let mut config = Config::load().unwrap_or_default();
        let theme = match Theme::find_installed(&mut config, self.name.clone()) {
            Some(target_theme) => target_theme,
            None => return Err(LeftError::from("Theme not found")),
        };
        match theme.directory.as_ref() {
            Some(directory) => {
                let path = Path::new(directory);
                if yes_or_no(format!(
                    "    Are you sure you want to uninstall this theme, located at {}?",
                    path.to_str().unwrap_or("Unknown location")
                )) {
                    fs::remove_dir_all(path)?;
                    match theme.source {
                        Some(source) => match Theme::find_mut(&mut config, &self.name, &source) {
                            Some(target_theme) => target_theme.directory = None,
                            None => return Err(LeftError::from("Could not find theme")),
                        },
                        None => return Err(LeftError::from("No source found")),
                    }
                    Config::save(&config)?;
                } else {
                    println!("{}", "No actions to take. Exiting . . . ".yellow().bold());
                }

                Ok(())
            }
            None => {
                error!("Theme not installed");
                Err(errors::LeftError::from("Theme not installed"))
            }
        }
    }
}
