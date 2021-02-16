use crate::errors;
use crate::models::Config;
use clap::ArgMatches;
use colored::*;
pub fn list(_args: &ArgMatches) -> Result<(), errors::LeftError> {
    let config = Config::load().unwrap_or_default();
    println!("{}", "\nAvailable themes:".blue().bold());
    let mut installed = 0;
    for x in 0..config.theme.len() {
        let current = match config.theme[x].current {
            Some(true) => "Current: ".magenta().bold(),
            _ => "".white(),
        };
        match config.theme[x].directory {
            Some(_) => {
                println!(
                    "    {}{}: {}",
                    current,
                    config.theme[x].name,
                    config.theme[x]
                        .description
                        .as_ref()
                        .unwrap_or(&"A LeftWM theme".to_string())
                );
                installed += 1;
            }
            None => {}
        };
    }
    if installed == 0 {
        println!("{}", "No themes installed.".red().bold());
    }
    Ok(())
}
