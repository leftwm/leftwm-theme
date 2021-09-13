use crate::models::{Config, Theme};
use crate::{errors, utils};
use clap::Clap;
use colored::Colorize;
use errors::LeftError;
use log::{error, trace, warn};
use std::os::unix;
use std::path::Path;
use std::process::Command;
use std::{env, fs};
use xdg::BaseDirectories;

/* This function sets a particular theme as the current theme in ~./config/leftwm/themes/
     Required args include "THEME", which defines the NAME of a theme as defined in a known.toml file or the themes.toml file in ~/.config/leftwm/
         TODO: THEME (with the -g/git or -f/folder flags) may also point to a git url (in the future) with a defined theme.toml file with enough global parameters defined to embed the theme in themes.toml
     Possible optional args include debug, which prints all trace! commands, and no-reset, which prevents leftwm-theme from resetting the theme
*/

#[derive(Clap, Debug)]
pub struct Apply {
    pub name: String,

    /// Don't restart leftwm-worker
    #[clap(short = 'n', long)]
    pub no_reset: bool,

    /// Ignore checks
    #[clap(short = 'o', long)]
    pub override_checks: bool,
}

impl Apply {
    /// # Errors
    ///
    /// Returns an error if config cannot be loaded / saved
    /// Returns an error if `BaseDirectory` not set.
    /// Returns an error if symlink cannot be made.
    /// Returns an error if theme not found.
    /// Returns an error if leftwm-worker cannot be killed.
    pub fn exec(&self, mut config: &mut Config) -> Result<(), errors::LeftError> {
        trace!("Applying theme named {:?}", &self.name);
        println!(
            "{}{}{}",
            "Setting ".bright_blue().bold(),
            &self.name.bright_green().bold(),
            " as default theme.".bright_blue().bold()
        );
        let mut dir = BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
        dir.push("themes");
        dir.push("current");
        trace!("{:?}", &dir);
        if let Some(theme) = Theme::find(&mut config, &self.name) {
            if let Some(theme_dir) = theme.directory.as_ref() {
                //Do all necessary checks
                if !checks(&theme) && !self.override_checks {
                    error!("Not all prerequirements passed");
                    return Err(errors::LeftError::from("PreReqs"));
                }
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
                    "Applying ".bright_blue().bold(),
                    &self.name.bright_green().bold(),
                    " as default theme.".bright_blue().bold()
                );
                trace!("{:?}", "Altering config");
                for repo in &mut config.repos {
                    for theme in &mut repo.themes {
                        theme.current = Some(false);
                    }
                }
                if let Some(source) = theme.source {
                    if let Some(target_theme) = Theme::find_mut(&mut config, &theme.name, &source) {
                        target_theme.current(true)
                    } else {
                        error!("Theme not found");
                        return Err(LeftError::from("Theme not found"));
                    }
                } else {
                    error!("Theme does not have a source");
                }

                Config::save(config)?;
                if !self.no_reset {
                    println!("{}", "Reloading LeftWM.".bright_blue().bold());
                    Command::new("pkill").arg("leftwm-worker").output()?;
                }
                Ok(())
            } else {
                error!(
                    "\nTheme not installed. Try installing it with `leftwm-theme install {}`.",
                    &self.name
                );
                Err(errors::LeftError::from("Theme not installed"))
            }
        } else {
            error!("\n Theme not installed. Try checking your spelling?");
            Err(errors::LeftError::from("Theme not installed"))
        }
    }
}

pub(crate) fn checks(theme: &Theme) -> bool {
    trace!("Checking dependencies.");
    match theme.dependencies.clone() {
        None => {
            trace!("No dependencies detected");
        }
        Some(theme_dependencies) => {
            for dependency in theme_dependencies {
                if !is_program_in_path(&dependency.program) {
                    return false;
                }
            }
        }
    }
    trace!("Checking LeftWM version.");
    if let Ok(true) = utils::versions::check(
        &theme
            .leftwm_versions
            .clone()
            .unwrap_or_else(|| "*".to_string()),
    ) {
        true
    } else {
        error!("This theme is incompatible with the installed version of LeftWM.");
        false
    }
}

fn is_program_in_path(program: &str) -> bool {
    trace!("Checking dependency {}", program);
    if let Ok(path) = env::var("PATH") {
        for p in path.split(':') {
            let p_str = format!("{}/{}", p, program);
            if fs::metadata(p_str).is_ok() {
                return true;
            }
        }
    }
    false
}
