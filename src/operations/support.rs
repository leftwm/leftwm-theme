use crate::errors;
use crate::models::Config;
use clap::Parser;
use log::trace;
use std::process::Command;

#[derive(Parser, Debug)]
pub struct Support {
    /// Names
    pub name: String,
}

impl Support {
    /// # Errors
    ///
    /// Should not error.
    pub fn exec(&self, config: &mut Config) -> Result<(), errors::LeftError> {
        'outer: for repo in &config.repos {
            trace!("Searching themes from {}", &repo.name);
            for theme in &repo.themes {
                if theme.name == self.name{
                    if let Some(s_url) = &theme.support_url{
                        Command::new("xdg-open").arg(s_url).spawn().expect("Could not xdg-open");
                    } else {
                        println!("Theme does not have associated help page.");
                    }
                    break 'outer;
                }
            }
        }
        Ok(())
    }
}
