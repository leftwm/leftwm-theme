use crate::errors;
use crate::models::Config;
use clap::Clap;
use colored::Colorize;
use log::trace;

#[derive(Clap, Debug)]
pub struct Update {
    /// Don't list themes
    #[clap(short = 'f', long)]
    pub no_list: bool,
}

impl Update {
    /// Fetch themes from the themes repository.
    ///
    /// # Errors
    ///
    /// Will error if config cannot be saved
    /// Will error if upstream known.toml cannot be retrieved.
    /// Will error if TOML files themes.toml or known.toml cannot be parsed.
    pub fn exec(&self, config: &mut Config) -> Result<(), errors::LeftError> {
        println!("{}", "Fetching themes . . . ".bright_blue().bold());
        let config_dir = config.get_config_dir()?;
        // Attempt to fetch new themes and populate the config with remote
        // themes.
        trace!("{:?}", &config);
        for repo in &mut config.repos {
            if repo.name != "LOCAL" {
                println!("    Retrieving themes from {:?}", repo.name);
                let resp = reqwest::blocking::get(&repo.url)?.text_with_charset("utf-8")?;
                trace!("{:?}", &resp);

                // Compare to old themes
                repo.compare(toml::from_str(&resp)?, config_dir.clone())?;
            }
        }

        // Populate config based on the local themes.
        config.update_local_repo()?;

        Config::save(config)?;

        // Exit early if --no-list was passed
        if self.no_list {
            return Ok(());
        }

        // List themes
        println!("{}", "\nAvailable themes:".bright_blue().bold());

        for repo in &mut config.repos {
            for theme in &mut repo.themes {
                let current = match theme.current {
                    Some(true) => "Current: ".bright_green().bold(),
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
