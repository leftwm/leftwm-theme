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
        println!("{}", "Fetching known themes:".bright_blue().bold());
        let mut config = Config::load().unwrap_or_default();
        //attempt to fetch new themes
        for repo in &mut config.repos {
            if repo.name == "LOCAL" {
                continue;
            }
            println!(
                "    Retrieving themes from {}",
                &repo.name.bright_magenta().bold()
            );
            let resp = reqwest::blocking::get(&repo.url)?.text_with_charset("utf-8")?;
            trace!("{:?}", &resp);

            //compare to old themes
            repo.compare(toml::from_str(&resp)?)?;
        }
        Config::save(&config)?;
        // Update themes
        println!("{}", "\nUpdating themes:".bright_blue().bold());
        let mut installed = 0;
        for repo in config.repos {
            trace!("Upgrading themes in repo {:?}", &repo.name);
            if repo.name == "LOCAL" {
                continue;
            }
            for theme in repo.themes {
                let current = match theme.current {
                    Some(true) => "Current: ".magenta().bold(),
                    _ => "".white(),
                };
                if theme.directory.is_some() {
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
                    let git_repo = Repository::open(theme.directory.clone()?)?;
                    match fetch_origin_main(&git_repo) {
                        Ok(_) => {
                            //if defined, attempt to checkout the specific index
                            if theme.commit.is_some()
                                && theme.commit.clone().unwrap_or_else(|| "".to_string()) != *"*"
                            {
                                git_repo
                                    .set_head_detached(Oid::from_str(theme.commit.as_ref()?)?)?;
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

pub fn fetch_origin_main(repo: &git2::Repository) -> Result<(), git2::Error> {
    repo.find_remote("origin")?.fetch(&["main"], None, None)
}
