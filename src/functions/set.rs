use crate::errors;
use clap::ArgMatches;
use colored::*;
use log::{error, trace, warn};
use std::fs;
use std::os::unix;
use std::path::Path;
use std::process::Command;
use xdg::BaseDirectories;

pub fn set(args: &ArgMatches) -> Result<(), errors::LeftError> {
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
                    "Set ".blue().bold(),
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
