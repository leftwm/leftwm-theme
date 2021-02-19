use crate::errors;
use clap::ArgMatches;
use colored::*;
use log::{error, trace, warn};
use std::env;
use std::fs;
use std::os::unix;
use std::path::Path;
use std::process::Command;
use xdg::BaseDirectories;

/* This function sets a particular theme as the current theme in ~./config/leftwm/themes/
      Required args include "THEME", which defines the NAME of a theme as defined in a known.toml file or the themes.toml file in ~/.config/leftwm/
          TODO: THEME (with the -g/git or -f/folder flags) may also point to a git url (in the future) with a defined theme.toml file with enough global parameters defined to embed the theme in themes.toml  
      Possible optional args include debug, which prints all trace! commands, and no-reset, which prevents leftwm-theme from resetting the theme 
 */
pub fn apply(args: &ArgMatches) -> Result<(), errors::LeftError> {
    let themename = args.value_of("THEME")?;
    trace!("{:?}", &themename);
    use crate::models::{Config, Theme};
    let mut config = Config::load().unwrap_or_default();
    println!(
        "{}{}{}",
        "Setting ".blue().bold(),
        &themename.green().bold(),
        " as default theme.".blue().bold()
    );
    let mut dir = BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
    dir.push("themes");
    dir.push("current");
    trace!("{:?}", &dir);
    match Theme::find(&mut config.theme, args.value_of("THEME")?.to_string()) {
        Some(theme) => match theme.directory.as_ref() {
            Some(theme_dir) => {
                let path = Path::new(theme_dir);
                trace!("{:?}", &path);
                match fs::remove_dir_all(&dir) {
                    Ok(_) => {
                        warn!("Removed old current directory");
                    }
                    Err(_) => {
                        trace!("Nothing needed removed");
                    }
                }
                unix::fs::symlink(path, dir)?;
                println!(
                    "{}{}{}",
                    "Apply ".blue().bold(),
                    &themename.green().bold(),
                    " as default theme.".blue().bold()
                );
                if args.value_of("no-reset").is_none() {
                    println!("{}", "Reloading LeftWM".blue());
                    Command::new("pkill").arg("leftwm-worker").output()?;
                }
                Ok(())
            }
            None => {
                error!(
                    "\nTheme not installed. Try installing it with `leftwm-theme add {}`.",
                    &themename
                );
                Err(errors::LeftError::from("Theme not installed"))
            }
        },
        None => {
            error!("\n Theme not installed. Try checking your spelling?");
            Err(errors::LeftError::from("Theme not installed"))
        }
    }
}


