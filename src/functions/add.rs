use crate::errors;
use crate::models::{Config, Theme};
use clap::ArgMatches;
use colored::*;
use git2::Repository;
use log::{error, trace};
use xdg::BaseDirectories;
pub fn add(args: &ArgMatches) -> Result<(), errors::LeftError> {
    println!("{}", "Looking for theme . . . ".blue().bold());
    let mut config = Config::load().unwrap_or_default();
    trace!("{:?}", &config);
    //try to find the theme or exit
    match Theme::find(&mut config.theme, args.value_of("Name")?.to_string()) {
        Some(theme) => {
            trace!("{:?}", &theme);
            let repo = theme.repository.as_ref()?.clone();
            let mut dir = BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
            dir.push("themes");
            let themename = theme.name.clone();
            dir.push(&theme.name.clone());

            //Tell git to get the repo and save it to the proper XDG path
            match Repository::clone(&repo, dir.clone()) {
                Ok(_) => {
                    //add to config and save
                    Theme::find(&mut config.theme, args.value_of("Name")?.to_string())?
                        .directory(dir.to_str());
                    Config::save(&config)?;
                    println!(
                        "{}{}{}{}{}{}",
                        "Downloaded theme ".green().bold(),
                        &themename.cyan(),
                        ". \nTo set as default, use ".green().bold(),
                        "leftwm-theme set \"".cyan(),
                        &themename.cyan(),
                        "\"".cyan()
                    );
                    Ok(())
                }
                Err(e) => {
                    error!(
                        "\n{} could not be installed because {:?}",
                        &themename,
                        e.message()
                    );
                    Err(errors::LeftError::from("Theme not installed"))
                }
            }
        }
        None => {
            error!("\n Theme not found");
            Err(errors::LeftError::from("Theme not found"))
        }
    }
}
