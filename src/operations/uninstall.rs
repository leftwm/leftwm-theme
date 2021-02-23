use crate::errors;
use crate::models::{Config, Theme};
use clap::Clap;
use colored::*;
use log::error;
use std::fs;
use std::io;
use std::io::Write;
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
        let theme = Theme::find_installed(&mut config, self.name.clone())?;
        match theme.directory.as_ref() {
            Some(directory) => {
                let path = Path::new(directory);
                let mut state: String;
                loop {
                    println!(
                        "    Are you sure you want to uninstall this theme, located at {}?",
                        path.to_str()?
                    );
                    print!("{}", "yes/no =>".bright_yellow().bold());
                    io::stdout().flush().unwrap();
                    state = read_one().trim().to_uppercase();

                    if state == *"YES" || state == *"NO" {
                        break;
                    }

                    println!("Please write either yes or no.")
                }
                if state == *"YES" {
                    fs::remove_dir_all(path)?;
                    Theme::find_mut(&mut config, self.name.clone(), theme.source?)?.directory(None);
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

pub fn read_one() -> String {
    let mut words = String::new();
    io::stdin().read_line(&mut words).ok();
    words
}
