use crate::errors;
use crate::models::Config;
use clap::Parser;
use colored::Colorize;
use log::trace;

#[derive(Parser, Debug)]
pub struct List {
    /// Names only
    #[clap(short = 'n', long)]
    pub names: bool,
}

impl List {
    /// # Errors
    ///
    /// Should not error.
    pub fn exec(&self, config: &mut Config) -> Result<(), errors::LeftError> {
        if !self.names {
            println!("{}", "\nInstalled themes:".blue().bold());
        }
        let mut installed = 0;
        for repo in &config.repos {
            trace!("Printing themes from {}", &repo.name);
            for theme in &repo.themes {
                let current = match theme.current {
                    Some(true) => "Current: ".bright_yellow().bold(),
                    _ => "".white(),
                };
                if theme.directory.is_some() && !self.names {
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
                } else if theme.directory.is_some() && self.names {
                    println!("{}", theme.name);
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
