use crate::errors;
use crate::models::Config;
use clap::Clap;
use colored::Colorize;
use log::trace;

#[derive(Clap, Debug)]
pub struct List {}

impl List {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        let config = Config::load().unwrap_or_default();
        println!("{}", "\nInstalled themes:".blue().bold());
        let mut installed = 0;
        for repo in config.repos {
            trace!("Printing themes from {}", &repo.name);
            for theme in repo.themes {
                let current = match theme.current {
                    Some(true) => "Current: ".bright_yellow().bold(),
                    _ => "".white(),
                };
                if theme.directory.is_some() {
                    println!(
                        "    {}{}/{}: {}",
                        current,
                        repo.name.bright_magenta().bold(),
                        theme.name.bright_green().bold(),
                        theme
                            .description
                            .as_ref()
                            .unwrap_or(&"A LeftWM theme".to_string())
                    );
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
