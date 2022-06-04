use crate::models::Config;
use crate::{errors, utils};
use clap::Parser;
use colored::Colorize;
use log::trace;
use std::fs;
use url::Url;

#[derive(Parser, Debug)]
pub struct Update {
    /// Don't list themes
    #[clap(short = 'f', long)]
    pub no_list: bool,
    /// List incompatible themes
    #[clap(short = 'i', long)]
    pub incompatible: bool,
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
        Update::update_repos(config)?;
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
                // Only list installable themes
                if let Ok(true) = utils::versions::check(
                    &theme
                        .leftwm_versions
                        .clone()
                        .unwrap_or_else(|| "*".to_string()),
                ) {
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
                } else {
                    // Show incompatible themes if requested
                    if self.incompatible {
                        println!(
                            "   {}{}/{}: {}{} (not compatible with your version of leftwm)",
                            current,
                            repo.name.bright_magenta().bold(),
                            theme.name.bright_red().bold(),
                            theme
                                .description
                                .as_ref()
                                .unwrap_or(&"A LeftWM theme".to_string()),
                            installed
                        );
                    }
                }
            }
        }

        Ok(())
    }

    // Iterates through the repos in the config, fetches themes from the repos
    // and updates the config with the themes. The downloaded themes are
    // compared with any existing themes and updated.
    fn update_repos(config: &mut Config) -> Result<(), errors::LeftError> {
        println!("{}", "Fetching themes . . . ".bright_blue().bold());
        let config_dir = config.get_config_dir()?;
        // Attempt to fetch new themes and populate the config with remote
        // themes.
        trace!("{:?}", &config);
        for repo in &mut config.repos {
            if repo.name == "LOCAL" {
                // Update local repos separately after processing the remote
                // repos.
                continue;
            }

            let content: String;
            // Check the url scheme to determine how to fetch the themes.
            let repo_url = Url::parse(repo.url.clone().as_str())?;
            if repo_url.scheme() == "file" {
                content = fs::read_to_string(repo_url.path())?;
            } else {
                content = reqwest::blocking::get(repo_url.as_str())?.text_with_charset("utf-8")?;
                trace!("{:?}", &content);
            }
            if !content.is_empty() {
                repo.compare(toml::from_str(&content)?, &config_dir)?;
            }
        }

        // Populate config based on the local themes.
        config.update_local_repo()?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::models::Repo;
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn test_update_repos() {
        // Create a local repo file with test themes.
        let tmpdir = tempfile::tempdir().unwrap();
        let repo_file_path = tmpdir.path().join("repo.toml");
        let mut repo_file = File::create(&repo_file_path).unwrap();
        write!(
            repo_file,
            r#"
[[theme]]
name = "test-theme1"
repository = "https://github.com/leftwm/testtheme1/"
commit = "*"
version = "0.0.5"
leftwm_versions = "*"

[[theme]]
name = "test-theme2"
repository = "https://github.com/leftwm/testtheme2/"
commit = "*"
version = "0.0.3"
leftwm_versions = "*"
"#
        )
        .unwrap();
        let url_base = Url::parse("file://").unwrap();
        let local_file_url = url_base.join(repo_file_path.to_str().unwrap()).unwrap();

        let mut config: Config = Config {
            repos: vec![
                Repo {
                     definitions_version: crate::models::config::CURRENT_DEFINITIONS_VERSION,
                url: String::from(local_file_url.as_str()),
                name: String::from("test-repo"),
                themes: Vec::new(),
                },
                Repo{
                    definitions_version: crate::models::config::CURRENT_DEFINITIONS_VERSION,
                    url: String::from("https://raw.githubusercontent.com/leftwm/leftwm-community-themes/master/known.toml"),
                    name: String::from("community"),
                    themes: Vec::new(),
                },
            ],
            config_dir: Some(tmpdir.path().to_path_buf()),
        };

        assert!(Update::update_repos(&mut config).is_ok());
        let local_repo = config
            .repos
            .into_iter()
            .find(|x| x.name == "test-repo")
            .unwrap();
        assert_eq!(local_repo.themes.len(), 2);
        assert!(local_repo
            .clone()
            .themes
            .into_iter()
            .any(|x| x.name == "test-theme1"));
        assert!(local_repo
            .themes
            .into_iter()
            .any(|x| x.name == "test-theme2"));
    }
}
