use crate::errors::Result;
use crate::models::theme::{TempThemes, Theme};
use log::trace;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xdg::BaseDirectories;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    list_repo: String,
    pub theme: Vec<Theme>,
}

impl Config {
    pub fn default() -> Self {
        let mut config = Config {
            list_repo: String::from("https://raw.githubusercontent.com/leftwm/leftwm-community-themes/master/known.toml"),
            theme: Vec::new(),
        };
        config.theme.push(Theme {
            name: "Orange Forest".to_string(),
            description: Some("The orange forest theme".to_string()),
            directory: None,
            repository: Some(String::from(
                "https://github.com/PVautour/leftwm-theme-orange-forest/",
            )),
            version: Some("0.0.1".to_string()),
            leftwm_versions: Some("*".to_string()),
            commit: Some("*".to_string()),
            dependencies: None,
            current: None,
        });
        config
    }

    pub fn source(&self) -> String {
        self.list_repo.to_string()
    }

    pub fn save(config: &Self) -> Result<&Config> {
        let path = BaseDirectories::with_prefix("leftwm")?;
        let config_filename = path.place_config_file("themes.toml")?;
        let toml = toml::to_string(&config).unwrap();
        let mut file = File::create(&config_filename)?;
        file.write_all(&toml.as_bytes())?;
        Ok(config)
    }

    pub fn update_or_append(themes: &mut Self, theme: &Theme) {
        match themes.theme.iter_mut().find(|ref p| theme.name == p.name) {
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
                themes.theme.push(theme.clone());
            }
        }
    }
    pub fn compare(&mut self, theme_wrap: TempThemes) -> Result<&Config> {
        let themes = theme_wrap.theme;
        trace!("Comparing themes");
        //iterate over all themes, and update/add if needed
        for tema in themes {
            Config::update_or_append(self, &tema);
        }
        Ok(self)
    }

    pub fn load() -> Result<Config> {
        let path = BaseDirectories::with_prefix("leftwm")?;
        let config_filename = path.place_config_file("themes.toml")?;
        if Path::new(&config_filename).exists() {
            let contents = fs::read_to_string(config_filename)?;
            Ok(toml::from_str(&contents)?)
        } else {
            let config = Config::default();
            let toml = toml::to_string(&config).unwrap();
            let mut file = File::create(&config_filename)?;
            file.write_all(&toml.as_bytes())?;
            Ok(config)
        }
    }
}
