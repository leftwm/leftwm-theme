use crate::errors;
use crate::models::Config;
use clap::Clap;
use colored::*;
use git2::{Oid, Repository};
use log::{error, trace};

#[derive(Clap, Debug)]
pub struct Upgrade {}

impl Upgrade {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        let mut config = Config::load().unwrap_or_default();
        //attempt to fetch new themes
        println!("    Retrieving themes from {:?}", &config.source());
        let resp = reqwest::blocking::get(&config.source())?.text_with_charset("utf-8")?;
        trace!("{:?}", &resp);

        //compare to old themes
        config.compare(toml::from_str(&resp)?)?;
        trace!("{:?}", &config);
        Config::save(&config)?;
        println!("{}", "\nUpdating themes:".blue().bold());
        let mut installed = 0;
        for x in 0..config.theme.len() {
            let current = match config.theme[x].current {
                Some(true) => "Current: ".magenta().bold(),
                _ => "".white(),
            };
            if config.theme[x].directory.is_some() {
                println!(
                    "    Updating . . . {}{}: {}",
                    current,
                    config.theme[x].name,
                    config.theme[x]
                        .description
                        .as_ref()
                        .unwrap_or(&"A LeftWM theme".to_string())
                );
                let repo = Repository::open(config.theme[x].directory.clone()?)?;
                match fetch_origin_main(&repo) {
                    Ok(_) => {
                        //if defined, attempt to checkout the specific index
                        if config.theme[x].commit.is_some()
                            && config.theme[x]
                                .commit
                                .clone()
                                .unwrap_or_else(|| "".to_string())
                                != *"*"
                        {
                            repo.set_head_detached(Oid::from_str(
                                &config.theme[x].commit.as_ref()?,
                            )?)?;
                            repo.checkout_head(None)?;
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
        if installed == 0 {
            println!("{}", "No themes installed.".red().bold());
        }
        Ok(())
    }
}

pub fn fetch_origin_main(repo: &git2::Repository) -> Result<(), git2::Error> {
    repo.find_remote("origin")?.fetch(&["main"], None, None)
}
