use crate::errors;
use crate::models::{Config, Theme};
use clap::ArgMatches;
use colored::*;
use git2::Repository;
use log::error;
use xdg::BaseDirectories;

pub fn new(args: &ArgMatches) -> Result<(), errors::LeftError> {
    let name = args.value_of("THEME_NAME")?;
    let mut config = Config::load().unwrap_or_default();
    match Theme::find(&mut config.theme, args.value_of("THEME_NAME")?.to_string()) {
        Some(_theme) => {
            error!(
                "\n{} could not be created because a theme with that name already exists",
                args.value_of("THEME_NAME")?,
            );
            Err(errors::LeftError::from("Theme not installed"))
        }
        None => {
            //Create the new git in the leftwm directory
            let mut dir = BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
            dir.push("themes");
            dir.push(name);
            match Repository::init(&dir) {
                Ok(_repo) => {
                    config.theme.push(Theme::new(
                        name.to_string(),
                        None,
                        Some(dir.to_str()?.to_string()),
                    ));
                    Config::save(&config)?;
                    println!(
                        "{} {} {} {}",
                        "Theme".green().bold(),
                        name.red().bold(),
                        "created successfully in".green().bold(),
                        dir.to_str()?.red().bold()
                    );
                    Ok(())
                }
                Err(e) => {
                    error!(
                        "\n{} could not be created because {:?}",
                        args.value_of("THEME_NAME")?,
                        e.message()
                    );
                    Err(errors::LeftError::from("Theme not created"))
                }
            }
        }
    }
}
