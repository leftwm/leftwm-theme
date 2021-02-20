use crate::errors;
use crate::models::{Config, Theme};
use clap::Clap;
use colored::*;
use git2::Repository;
use log::error;
use xdg::BaseDirectories;

#[derive(Clap, Debug)]
pub struct New {
    pub name: String,
}

impl New {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        let mut config = Config::load().unwrap_or_default();
        match Theme::find(&mut config, self.name.clone()) {
            Some(_theme) => {
                error!(
                    "\n{} could not be created because a theme with that name already exists",
                    &self.name,
                );
                Err(errors::LeftError::from("Theme not installed"))
            }
            None => {
                //Create the new git in the leftwm directory
                let mut dir =
                    BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
                dir.push("themes");
                dir.push(&self.name);
                match Repository::init(&dir) {
                    Ok(_repo) => {
                        Config::update_or_append(
                            &mut config,
                            &Theme::new(self.name.clone(), None, Some(dir.to_str()?.to_string())),
                            (&String::from("localhost"), &String::from("LOCAL")),
                        );
                        Config::save(&config)?;
                        println!(
                            "{} {} {} {}",
                            "Theme".green().bold(),
                            &self.name.red().bold(),
                            "created successfully in".green().bold(),
                            dir.to_str()?.red().bold()
                        );
                        Ok(())
                    }
                    Err(e) => {
                        error!(
                            "\n{} could not be created because {:?}",
                            &self.name,
                            e.message()
                        );
                        Err(errors::LeftError::from("Theme not created"))
                    }
                }
            }
        }
    }
}
