use crate::errors;
use crate::errors::LeftError;
use crate::models::{Config, Theme};
use crate::utils::read::yes_or_no;
use clap::Parser;
use colored::Colorize;
use log::error;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
pub struct Uninstall {
    /// Name of theme to uninstall
    pub name: String,
    /// Whether to prompt for confirmation
    #[clap(long)]
    pub noconfirm: bool,
}

impl Uninstall {
    /// # Errors
    /// Will error if config cannot be saved.
    /// Will error if cannot remove directory.
    pub fn exec(&self, config: &mut Config) -> Result<(), errors::LeftError> {
        println!(
            "{}",
            "Looking for theme to uninstall . . . ".bright_blue().bold()
        );
        let Some(theme) = Theme::find_installed(config, &self.name) else {
            return Err(LeftError::from("Theme not found"));
        };
        if let Some(directory) = theme.directory {
            let path = Path::new(&directory);
            if self.noconfirm
                || yes_or_no(&format!(
                    "    Are you sure you want to uninstall this theme, located at {}?",
                    path.to_str().unwrap_or("Unknown location")
                ))
            {
                fs::remove_dir_all(path)?;
                match theme.source {
                    Some(source) => match Theme::find_mut(config, &self.name, &source) {
                        Some(target_theme) => {
                            target_theme.directory = None;
                            println!(
                                "{}",
                                format!("Theme {} uninstalled.", &self.name).green().bold()
                            );
                        }
                        None => return Err(LeftError::from("Could not find theme")),
                    },
                    None => return Err(LeftError::from("No source found")),
                }
                Config::save(config)?;
            } else {
                println!("{}", "No actions to take. Exiting . . . ".yellow().bold());
            }

            Ok(())
        } else {
            error!("Theme not installed");
            Err(errors::LeftError::from("Theme not installed"))
        }
    }
}
