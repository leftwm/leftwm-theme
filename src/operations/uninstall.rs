use crate::errors;
use crate::models::{Config, Theme};
use clap::Clap;
use colored::*;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Clap, Debug)]
pub struct Uninstall {
    pub name: String,
}

impl Uninstall {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        println!("{}", "Looking for theme to uninstall . . . ".blue().bold());
        let mut config = Config::load().unwrap_or_default();
        let theme = Theme::find(&mut config.theme, self.name.clone())?;
        let path = Path::new(theme.directory.as_ref()?);
        let mut state: String;
        loop {
            println!(
                "Are you sure you want to uninstall this theme, located at {} ? yes/no:",
                path.to_str()?
            );
            state = read_one().trim().to_uppercase();

            if state == *"YES" || state == *"NO" {
                break;
            }

            println!("Please write either yes or no.")
        }
        if state == *"YES" {
            fs::remove_dir_all(path)?;
            Theme::find(&mut config.theme, self.name.clone())?.directory(None);
            Config::save(&config)?;
        } else {
            println!("{}", "No actions to take. Exiting . . . ".yellow().bold());
        }

        Ok(())
    }
}

pub fn read_one() -> String {
    let mut words = String::new();
    io::stdin().read_line(&mut words).ok();
    words
}
