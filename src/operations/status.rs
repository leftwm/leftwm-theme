use crate::errors;
use crate::models::{Config, LeftWm};
use clap::Clap;
use colored::Colorize;

#[derive(Clap, Debug)]
pub struct Status {}

impl Status {
    pub fn exec(&self) -> Result<(), errors::LeftError> {
        println!(
            "{} {}",
            "Your LeftWM version is".bright_blue().bold(),
            LeftWm::get()?.version.bright_green().bold()
        );
        let mut config = Config::load()?;
        let mut current = 0;
        let mut installed = 0;
        for theme in config.themes(false) {
            if theme.current == Some(true) {
                current += 1;
                println!(
                    "{} {}, {} {} {}",
                    "Your current theme is".bright_blue().bold(),
                    theme.name.bright_green().bold(),
                    "located in the".bright_blue().bold(),
                    theme
                        .source
                        .unwrap_or_else(|| "unknown".to_string())
                        .bright_magenta()
                        .bold(),
                    "repo".bright_blue().bold()
                );
            }
            if theme.directory.is_some() {
                installed += 1;
            }
        }
        println!(
            "{} {} {}",
            "There are".bright_blue().bold(),
            installed.to_string().bright_green().bold(),
            "themes installed in your ~/.config/leftwm/themes/ directory known to LeftWM."
                .bright_blue()
                .bold()
        );
        if current == 0 {
            println!(
                "{} \n    {}",
                "WARNING! NO KNOWN THEME IS CURRENTLY SET."
                    .bright_red()
                    .bold(),
                "A theme may be set, but LeftWM theme doesn't know about it.\n    If it is a local theme, try leftwm-theme new themename.\n    If it is a repo theme, try leftwm-theme install themename"
                    .bright_yellow()
                    .bold()
            )
        }
        Ok(())
    }
}
