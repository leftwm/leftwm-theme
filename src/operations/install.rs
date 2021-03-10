use crate::errors;
use crate::errors::LeftError;
use crate::errors::Result;
use crate::models::{Config, Theme};
use clap::Clap;
use colored::*;
use git2::Repository;
use log::{error, trace};
use std::io;
use std::io::Write;
use xdg::BaseDirectories;

#[derive(Clap, Debug)]
pub struct Install {
    /// Read theme from git repository
    #[clap(short = 'g', long)]
    pub git: bool,

    /// Read theme from path
    #[clap(short = 'p', long)]
    pub path: bool,

    /// Location of theme
    pub name: String,
}

impl Install {
    pub fn exec(&self) -> Result<()> {
        println!("{}", "Looking for theme . . . ".bright_blue().bold());
        let mut config = Config::load().unwrap_or_default();
        trace!("{:?}", &mut config);
        //try to find the theme or exit
        match Theme::find_all(&mut config, self.name.clone()) {
            Some(themes) => {
                match choose_one(themes) {
                    Ok(theme) => {
                        trace!("{:?}", &theme);
                        let repo = match theme.repository.as_ref() {
                            Some(repo) => repo,
                            None => {
                                return Err(LeftError::from(
                                    "Repository information missing for theme",
                                ))
                            }
                        };
                        let mut dir =
                            BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
                        dir.push("themes");
                        let themename = theme.name.clone();
                        dir.push(&theme.name);

                        //Tell git to get the repo and save it to the proper XDG path
                        match Repository::clone(&repo, dir.clone()) {
                            Ok(_) => {
                                //add to config and save
                                match theme.source {
                                    Some(source) => {
                                        match Theme::find_mut(
                                            &mut config,
                                            self.name.clone(),
                                            source,
                                        ) {
                                            Some(target_theme) => {
                                                target_theme.directory(dir.to_str())
                                            }
                                            None => {
                                                return Err(LeftError::from(
                                                    "Theme not found in db",
                                                ))
                                            }
                                        }
                                    }
                                    None => return Err(LeftError::from("Theme not found in db")),
                                }
                                Config::save(&config)?;
                                println!(
                                    "{}{}{}{}{}{}",
                                    "Downloaded theme ".bright_blue().bold(),
                                    &themename.green(),
                                    ". \nTo set as default, use ".bright_blue().bold(),
                                    "leftwm-theme apply \"".bright_yellow().bold(),
                                    &themename.bright_yellow().bold(),
                                    "\"".bright_yellow().bold()
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
                    Err(_e) => {
                        error!("\n Theme not found");
                        Err(errors::LeftError::from("Theme not found"))
                    }
                }
            }
            None => {
                error!("\n Theme not found");
                Err(errors::LeftError::from("Theme not found"))
            }
        }
    }
}

pub fn choose_one(themes: Vec<Theme>) -> Result<Theme> {
    if themes.len() == 1 {
        Ok(themes[0].clone())
    } else if themes.is_empty() {
        error!("No themes have that name");
        Err(errors::LeftError::from("No themes with that name"))
    } else {
        let idx = ask(&themes)?;
        Ok(themes[idx].clone())
    }
}

pub fn ask(themes: &[Theme]) -> Result<usize> {
    #[allow(unused_assignments)]
    let mut return_index = Err(errors::LeftError::from("No themes available"));
    'outer: loop {
        println!(
            "{}",
            "Which theme would you like to install?"
                .bright_yellow()
                .bold()
        );
        for (id, theme) in themes.iter().enumerate() {
            if theme.directory.is_some() {
                error!("A theme with that name is already installed");
                return_index = Err(errors::LeftError::from("Theme already installed"));
                break 'outer;
            }
            let source_string = match &theme.source {
                Some(source) => source.clone(),
                None => String::from("UNKNOWN"),
            };
            println!(
                "    {}/{} [{}]",
                &source_string.bright_magenta().bold(),
                &theme.name.bright_green().bold(),
                &id.to_string().bright_yellow().bold()
            );
        }
        print!("{}", "=>".bright_yellow().bold());
        io::stdout().flush().unwrap();
        let val = read_num();
        if let Ok(index) = val {
            if index < themes.len() {
                return_index = Ok(index);
                break;
            }
        }
        println!("{}", "Error: Please select a number:".bright_red().bold())
    }
    return_index
}

pub fn read_num() -> Result<usize> {
    let mut words = String::new();
    io::stdin().read_line(&mut words).ok();
    let trimmed = words.trim();
    trace!("Trimmed receipt: {:?}", &trimmed);
    match trimmed.parse::<usize>() {
        Ok(size) => Ok(size),
        Err(err) => Err(errors::LeftError::from(err)),
    }
}
