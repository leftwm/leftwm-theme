use crate::errors;
use crate::models::Config;
use clap::Clap;
use colored::Colorize;
use edit_distance::edit_distance;
use errors::LeftError;
use log::trace;

/* This function searches for themes, but does not update them by default
 *                     */

#[derive(Clap, Debug)]
pub struct Search {
    /// Name of theme to find
    pub name: String,
}

impl Search {
    /// # Errors
    ///
    /// No errors expected.
    pub fn exec(&self) -> Result<(), LeftError> {
        // Load the configuration
        println!(
            "{}",
            "Searching for themes with similar names . . . "
                .bright_blue()
                .bold()
        );
        let mut config = Config::load().unwrap_or_default();
        // Iterate over the different themes, if the distance
        for theme in config.themes(false) {
            trace!(
                "Theme: {}, Distance:{}",
                &theme.name,
                edit_distance(&theme.name, &self.name)
            );
            if edit_distance(&theme.name, &self.name) <= 3 {
                let current = match theme.current {
                    Some(true) => "Current: ".bright_yellow().bold(),
                    _ => "".white(),
                };
                let installed = match theme.directory {
                    Some(_) => "-Installed".red().bold(),
                    None => "".white(),
                };
                println!(
                    "   {}{}/{}: {}{}",
                    current,
                    theme.source.unwrap_or_default().bright_magenta().bold(),
                    theme.name.bright_green().bold(),
                    theme
                        .description
                        .as_ref()
                        .unwrap_or(&"A LeftWM theme".to_string()),
                    installed
                );
            }
        }

        Ok(())
    }
}
