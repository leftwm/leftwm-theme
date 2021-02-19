use crate::errors;
use clap::Clap;
use colored::*;
use log::trace;

#[derive(Clap, Debug)]
pub struct Update {}

impl Update {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        println!("{}", "Fetchning themes . . . ".blue().bold());
        use crate::models::Config;
        let mut config = Config::load().unwrap_or_default();
        //attempt to fetch new themes
        println!("    Retrieving themes from {:?}", &config.source());
        let resp = reqwest::blocking::get(&config.source())?.text_with_charset("utf-8")?;
        trace!("{:?}", &resp);

        //compare to old themes
        config.compare(toml::from_str(&resp)?)?;
        trace!("{:?}", &config);
        Config::save(&config)?;

        //List themes
        println!("{}", "\nAvailable themes:".blue().bold());
        for x in 0..config.theme.len() {
            let current = match config.theme[x].current {
                Some(true) => "Current: ".magenta().bold(),
                _ => "".white(),
            };
            let installed = match config.theme[x].directory {
                Some(_) => "-Installed".red().bold(),
                None => "".white(),
            };
            println!(
                "    {}{}: {}{}",
                current,
                config.theme[x].name,
                config.theme[x]
                    .description
                    .as_ref()
                    .unwrap_or(&"A LeftWM theme".to_string()),
                installed
            );
        }

        Ok(())
    }
}
