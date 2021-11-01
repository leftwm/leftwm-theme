//! Updates each individual theme.
// Currently, there is no way of knowing whether a theme needs updated. In a later version it would
// be nice to skip themes that do not need updates.
use crate::errors;
use crate::models::Config;
use clap::Parser;
use colored::Colorize;
use git2::{Oid, Repository};
use log::{error, trace};

#[derive(Parser, Debug)]
pub struct Upgrade {
    /// Don't update db
    #[clap(short = 'i', long)]
    pub skipdbupdate: bool,
}

impl Upgrade {
    /// # Errors
    ///
    /// This function will return an error if the known.toml file fails to load correctly, or if
    /// comparisons fail (i.e. the TOML files do not parse), or if the config file cannot be saved.
    /// It will also throw an error if a config file does not have a proper directory
    /// It will not throw an error to the program that a particular theme repository failed to
    /// load, instead passing that information to the user.
    ///
    ///# Panics
    ///
    /// Panics are not expected. `theme.commit.as_ref().unwrap()` is within an if `is_some()`
    // Todo: allow passage of failed themes in either () or errors::LeftError
    pub fn exec(&self, config: &mut Config) -> Result<(), errors::LeftError> {
        //attempt to fetch new themes
        if !self.skipdbupdate {
            println!("{}", "Fetching known themes:".bright_blue().bold());
            let config_dir = config.get_config_dir()?;
            for repo in &mut config.repos {
                if repo.name == "LOCAL" {
                    continue;
                }
                println!(
                    "    Retrieving themes from {}",
                    &repo.name.bright_magenta().bold()
                );
                // We probably ought to add a better warning here if this fails to load
                let resp = reqwest::blocking::get(&repo.url)?.text_with_charset("utf-8")?;
                trace!("{:?}", &resp);

                //compare to old themes
                repo.compare(toml::from_str(&resp)?, &config_dir)?;
            }
            Config::save(config)?;
        }
        // Update themes
        println!("{}", "\nUpdating themes:".bright_blue().bold());
        let mut installed = 0;
        for repo in &config.repos {
            trace!("Upgrading themes in repo {:?}", &repo.name);
            if repo.name == "LOCAL" {
                continue;
            }
            for theme in &repo.themes {
                let current = match theme.current {
                    Some(true) => "Current: ".bright_green().bold(),
                    _ => "".white(),
                };
                if let Some(theme_directory) = &theme.directory {
                    println!(
                        "    Updating {}{}/{}: {}",
                        current,
                        repo.name.bright_magenta().bold(),
                        theme.name.bright_yellow().bold(),
                        theme
                            .description
                            .as_ref()
                            .unwrap_or(&"A LeftWM theme".to_string())
                    );
                    let git_repo = Repository::open(theme_directory)?;
                    match fetch_origin_main(&git_repo) {
                        Ok(_) => {
                            //if defined, attempt to checkout the specific index
                            if theme.commit.is_some()
                                && theme.commit.clone().unwrap_or_default() != *"*"
                            {
                                git_repo.set_head_detached(Oid::from_str(
                                    theme.commit.as_ref().unwrap(),
                                )?)?;
                                git_repo.checkout_head(None)?;
                            }
                        }
                        Err(e) => {
                            trace!("Error: {:?}", e);
                            error!("Could not fetch repo.");
                        }
                    }

                    installed += 1;
                }
            }
        }
        if installed == 0 {
            println!("{}", "No themes installed.".red().bold());
        }
        Ok(())
    }
}

pub(crate) fn fetch_origin_main(repo: &git2::Repository) -> Result<(), git2::Error> {
    return repo.find_remote("origin")?.fetch(&["main"], None, None);
}
