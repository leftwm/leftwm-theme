use crate::errors;
use crate::errors::Result;
use crate::models::theme::{TempThemes, Theme};
use log::{error, trace};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub repos: Vec<Repo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Repo {
    pub url: String,
    pub name: String,
    pub themes: Vec<Theme>,
}

impl Config {
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

    pub fn save(config: &Self) -> Result<&Config> {
        let path = BaseDirectories::with_prefix("leftwm")?;
        let config_filename = path.place_config_file("themes.toml")?;
        let toml = toml::to_string(&config).unwrap();
        let mut file = File::create(&config_filename)?;
        file.write_all(&toml.as_bytes())?;
        Ok(config)
    }

    pub fn update_or_append(config: &mut Self, theme: &Theme, repo: (&String, &String)) {
        match config
            .repos
            .iter_mut()
            .find(|ref p| repo.1.clone() == p.name)
        {
            Some(rrepo) => {
                match rrepo.themes.iter_mut().find(|ref o| theme.name == o.name) {
                    Some(rtheme) => {
                        // If there is one, update values
                        rtheme.repository = theme.repository.clone();
                        rtheme.description = theme.description.clone();
                        rtheme.commit = theme.commit.clone();
                        rtheme.version = theme.version.clone();
                        rtheme.leftwm_versions = theme.leftwm_versions.clone();
                        rtheme.dependencies = theme.dependencies.clone();
                    }
                    None => {
                        rrepo.themes.push(theme.clone());
                    }
                }
            }
            // o/w insert a new leaf at the end
            None => {
                config.repos.push(Repo {
                    url: repo.0.clone(),
                    name: repo.1.clone(),
                    themes: Vec::new(),
                });
                let lent = config.repos.len();
                config.repos[lent - 1].themes.push(theme.clone())
            }
        }
    }

    pub fn theme_dir(&self) -> Result<std::path::PathBuf> {
        let mut dir = BaseDirectories::with_prefix("leftwm")?.create_config_directory("")?;
        dir.push("themes");
        Ok(dir)
    }

    pub fn themes(&mut self, local: bool) -> Vec<Theme> {
        let mut themes: Vec<Theme> = Vec::new();
        for repo in &self.repos {
            if local && repo.name == *"LOCAL" {
                continue;
            }
            for theme in &repo.themes {
                themes.push(theme.clone().source(repo.name.clone()).to_owned());
            }
        }
        themes
    }

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
            let toml = toml::to_string(&config).unwrap();
            let mut file = File::create(&config_filename)?;
            file.write_all(&toml.as_bytes())?;
            Ok(config)
        }
    }
}

impl Repo {
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
            .find(|ref p| theme.name.clone() == p.name.clone())
        {
            Some(rtheme) => {
                // If there is one, update values
                rtheme.repository = theme.repository.clone();
                rtheme.description = theme.description.clone();
                rtheme.commit = theme.commit.clone();
                rtheme.version = theme.version.clone();
                rtheme.leftwm_versions = theme.leftwm_versions.clone();
                rtheme.dependencies = theme.dependencies.clone();
            }
            // o/w insert a new leaf at the end
            None => {
                repo.themes.push(theme.clone());
            }
        }
    }
}
