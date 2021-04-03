use crate::errors;
use crate::models::{Config, Theme};
use crate::utils::read::one;
use clap::Clap;
use colored::Colorize;
use git2::Repository;
use log::{error, trace};
use std::io;
use std::io::Write;
use std::path::Path;
use xdg::BaseDirectories;

#[derive(Clap, Debug)]
pub struct New {
    pub name: String,
}

// TODO: Allow themes with the same name in different namespaces
impl New {
    /// # Errors
    ///
    /// Will send an error if theme has a `/`.
    /// Will error if a theme with same name already exists.
    /// Will error if config cannot be loaded or saved properly.
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        if self.name.contains('/') {
            error!(
                "\n{} could not be created because a theme name should not contain '/'",
                &self.name,
            );
            return Err(errors::LeftError::from("Theme name not valid."));
        }
        let mut config = Config::load().unwrap_or_default();
        if let Some(_theme) = Theme::find(&mut config, &self.name) {
            error!(
                "\n{} could not be created because a theme with that name already exists",
                &self.name,
            );
            Err(errors::LeftError::from("Theme not installed"))
        } else {
            //Create the new git in the leftwm directory
            let mut dir = BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
            dir.push("themes");
            dir.push(&self.name);
            match Repository::init(&dir) {
                Ok(_repo) => {
                    Config::update_or_append(
                        &mut config,
                        &Theme::new(&self.name, None, Some(dir.clone())),
                        (&String::from("localhost"), &String::from("LOCAL")),
                    );
                    Config::save(&config)?;
                    println!(
                        "{} {} {} {}",
                        "Theme".green().bold(),
                        &self.name.red().bold(),
                        "created successfully in".green().bold(),
                        dir.to_str().unwrap_or("Unknown directory").red().bold()
                    );
                    println!(
                        "{}Which theme would you like to prefill?",
                        "::".bright_yellow().bold()
                    );
                    print!("  [0] basic_lemonbar\n  [1] basic_polybar\n  [2] basic_xmobar\n  [3] None\n");
                    let state = loop {
                        print!("{}", "0-3 =>".bright_yellow().bold());
                        io::stdout().flush().unwrap_or_default();
                        let state = one().trim().to_uppercase();

                        if state == *"0" || state == *"1" || state == *"2" || state == *"3" {
                            break state;
                        }

                        println!("Please write a number 0-3.")
                    };
                    match state.as_str() {
                        "0" => copy_files("/usr/share/leftwm/themes/basic_lemonbar/", &dir),
                        "1" => copy_files("/usr/share/leftwm/themes/basic_polybar/", &dir),
                        "2" => copy_files("/usr/share/leftwm/themes/basic_xmobar/", &dir),
                        _ => {
                            trace!("Doing nothing");
                            Ok(())
                        }
                    }
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

fn copy_files(dir: &str, left_path: &Path) -> Result<(), errors::LeftError> {
    trace!("{:?}", &dir);
    let directory = Path::new(dir);
    trace!("{:?}", &directory);
    if directory.is_dir() {
        trace!("Directory Exists");
        for entry in std::fs::read_dir(directory)? {
            trace!("{:?}", &entry);
            let entry = entry?;
            let path = entry.path();
            trace!(
                "{:?}",
                std::fs::copy(&path, left_path.join(entry.file_name()))
            );
        }
    } else {
        error!("Basic themes directory /usr/share/leftwm/ not found. Was it installed by LeftWM?");
        return Err(errors::LeftError::from("Theme not prefilled"));
    }
    Ok(())
}
