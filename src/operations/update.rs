use crate::errors;
use clap::Clap;
use colored::*;
use log::trace;

#[derive(Clap, Debug)]
pub struct Update {}

impl Update {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        println!("{}", "Fetching themes . . . ".bright_blue().bold());
        use crate::models::Config;
        let mut config = Config::load().unwrap_or_default();
        //attempt to fetch new themes
        trace!("{:?}", &config);
        for repo in &mut config.repos {
            if repo.name != "LOCAL" {
                println!("    Retrieving themes from {:?}", repo.name);
                let resp = reqwest::blocking::get(&repo.url)?.text_with_charset("utf-8")?;
                trace!("{:?}", &resp);

                //compare to old themes
                repo.compare(toml::from_str(&resp)?)?;
            }
        }
        Config::save(&config)?;
        //List themes
        println!("{}", "\nAvailable themes:".bright_blue().bold());

        for repo in &mut config.repos {
            for theme in &mut repo.themes {
                let current = match theme.current {
                    Some(true) => "Current: ".bright_yellow().bold(),
                    _ => "".white(),
                };
                let installed = match theme.directory {
                    Some(_) => "-Installed".red().bold(),
                    None => "".white(),
                };
                println!(
                    "   {}{}/{}: {}{}",
                    current,
                    repo.name.bright_magenta().bold(),
                    theme.name.bright_green().bold(),
                    theme
                        .description
                        .as_ref()
                        .unwrap_or(&"A LeftWM theme".to_string()),
                    installed
                );
            }
        }

        Ok(())
    }
}
