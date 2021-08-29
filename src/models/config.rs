use crate::errors;
use crate::errors::Result;
use crate::models::theme::{TempThemes, Theme};
use log::{error, trace};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xdg::BaseDirectories;

/// Contains a vector of all global repositories.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub repos: Vec<Repo>,
}

/// Contains global repository information. Akin to known.toml or themes.toml
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Repo {
    pub url: String,
    pub name: String,
    pub themes: Vec<Theme>,
}

impl Config {
    #[must_use]
    pub fn default() -> Self {
        Config {
            repos: vec![
                Repo {
                    url: String::from("https://raw.githubusercontent.com/leftwm/leftwm-community-themes/master/known.toml"),
                    name: String::from("community"),
                    themes: Vec::new()
                },
                Repo {
                    url: String::from("localhost"),
                    name: String::from("LOCAL"),
                    themes: Vec::new(),
                },
            ],
        }
    }

    /// # Errors
    ///
    /// Errors if toml cannot be obtained for the themes.toml file
    /// Errors if the `BaseDirectory` is not set (no systemd)
    /// Errors if no file can be saved
    pub fn save(config: &Self) -> Result<&Config> {
        let path = BaseDirectories::with_prefix("leftwm")?;
        let config_filename = path.place_config_file("themes.toml")?;
        let toml = toml::to_string(&config)?;
        let mut file = File::create(&config_filename)?;
        file.write_all(toml.as_bytes())?;
        Ok(config)
    }

    pub fn update_or_append(config: &mut Self, theme: &Theme, repo: (&String, &String)) {
        #![allow(clippy::option_if_let_else)]
        if let Some(target_repo) = config.repos.iter_mut().find(|p| repo.1.clone() == p.name) {
            match target_repo.themes.iter_mut().find(|o| theme.name == o.name) {
                Some(target_theme) => {
                    // If there is one, update values
                    target_theme.repository = theme.repository.clone();
                    target_theme.description = theme.description.clone();
                    target_theme.commit = theme.commit.clone();
                    target_theme.version = theme.version.clone();
                    target_theme.leftwm_versions = theme.leftwm_versions.clone();
                    target_theme.dependencies = theme.dependencies.clone();
                }
                None => {
                    target_repo.themes.push(theme.clone());
                }
            }
        }
        // o/w insert a new leaf at the end
        else {
            config.repos.push(Repo {
                url: repo.0.clone(),
                name: repo.1.clone(),
                themes: Vec::new(),
            });
            let lent = config.repos.len();
            config.repos[lent - 1].themes.push(theme.clone())
        }
    }

    pub fn themes(&mut self, local: bool) -> Vec<Theme> {
        let mut themes: Vec<Theme> = Vec::new();
        for repo in &self.repos {
            if local && repo.name == *"LOCAL" {
                continue;
            }
            for theme in &repo.themes {
                themes.push(theme.clone().source(repo.name.clone()).clone());
            }
        }
        themes
    }

    /// # Errors
    ///
    /// Will error if `BaseDirectory` not set (no systemd)
    /// Will error if themes.toml doesn't exist
    /// Will error if themes.toml has invalid content.
    /// Will error if themes.toml cannot be written to.
    pub fn load() -> Result<Config> {
        let path = BaseDirectories::with_prefix("leftwm")?;
        let config_filename = path.place_config_file("themes.toml")?;
        if Path::new(&config_filename).exists() {
            let contents = fs::read_to_string(config_filename)?;
            trace!("{:?}", &contents);
            match toml::from_str::<Config>(&contents) {
                Ok(config) => Ok(config),
                Err(err) => {
                    error!("TOML error: {:?}", err);
                    Err(errors::LeftError::from("TOML Invalid"))
                }
            }
        } else {
            let config = Config::default();
            let toml = toml::to_string(&config)?;
            let mut file = File::create(&config_filename)?;
            file.write_all(toml.as_bytes())?;
            Ok(config)
        }
    }
}

impl Repo {
    /// # Errors
    ///
    /// No errors should occur.
    pub fn compare(&mut self, theme_wrap: TempThemes) -> Result<&Repo> {
        let themes = theme_wrap.theme;
        trace!("Comparing themes");
        //iterate over all themes, and update/add if needed
        for tema in themes {
            Repo::update_or_append(self, &tema);
        }
        Ok(self)
    }

    pub fn update_or_append(repo: &mut Self, theme: &Theme) {
        let name = repo.name.clone();
        trace!("{:?} in {:?}", &theme, &name);
        match repo
            .themes
            .iter_mut()
            .find(|p| theme.name.clone() == p.name.clone())
        {
            Some(target_theme) => {
                // If there is one, update values
                target_theme.repository = theme.repository.clone();
                target_theme.description = theme.description.clone();
                target_theme.commit = theme.commit.clone();
                target_theme.version = theme.version.clone();
                target_theme.leftwm_versions = theme.leftwm_versions.clone();
                target_theme.dependencies = theme.dependencies.clone();
            }
            // o/w insert a new leaf at the end
            None => {
                repo.themes.push(theme.clone());
            }
        }
    }
}
