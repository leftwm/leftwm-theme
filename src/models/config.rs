use crate::errors;
use crate::errors::Result;
use crate::models::theme::{TempThemes, Theme};
use log::{error, trace};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use xdg::BaseDirectories;

const THEMES_DIR: &str = "themes";

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

        // Get a list of existing themes.
        let path = BaseDirectories::with_prefix("leftwm")?;
        let base_config_path = String::from(path.get_config_home().to_str().unwrap());
        let existing_themes = Repo::installed_themes(base_config_path).unwrap();
        let themes_dir = path.get_config_home().join(THEMES_DIR);

        // Iterate over all the themes, and update/add if needed.
        for mut tema in themes {
            // Check if the theme is already installed and update the theme
            // directory attribute.
            if existing_themes.contains(&tema.name.clone()) {
                tema.directory = Some(themes_dir.join(tema.name.clone()));
            }
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
                target_theme.directory = theme.directory.clone();
            }
            // o/w insert a new leaf at the end
            None => {
                repo.themes.push(theme.clone());
            }
        }
    }

    // Returns a list of all the installed theme names under a given config
    // path.
    fn installed_themes(config_path: String) -> Result<Vec<String>> {
        let mut result: Vec<String> = Vec::new();

        let theme_path = Path::new(&config_path).join(THEMES_DIR);

        // Return empty result if the themes directory is not present.
        if !theme_path.exists() {
            return Ok(result);
        }

        // Read the themes directory, iterate through the entries, determine
        // which of them are theme directories and add them into the result.
        let paths = fs::read_dir(theme_path).unwrap();
        for path in paths {
            let p = path.unwrap();
            // NOTE: For symlinks, metadata() traverses any symlinks and queries
            // the metadata information from the destination.
            let metadata = fs::metadata(p.path())?;
            let file_type = metadata.file_type();

            // Only process directories.
            if !file_type.is_dir() {
                continue;
            }

            // Ignore the "current" directory for installed theme list.
            let current_dir = String::from("current");
            let target_path = p.path();
            if target_path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .eq(&current_dir)
            {
                continue;
            }

            // Extract only the theme name for the result.
            let theme_name = target_path.file_name().unwrap();
            result.push(String::from(theme_name.to_str().unwrap()));
        }

        Ok(result)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::os::unix::fs as unix_fs;

    #[test]
    fn test_installed_themes() {
        // Create a temporary directory as the config path and create the
        // directory layout within it for themes.
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);
        let theme1 = themes_dir.join("test-theme1");
        let theme2 = themes_dir.join("test-theme2");
        let unrelated_file = themes_dir.join("some-file");
        assert!(fs::create_dir_all(&theme1).is_ok());
        assert!(fs::create_dir_all(&theme2).is_ok());
        assert!(File::create(unrelated_file).is_ok());

        // Create current theme as a symlink to an existing theme.
        let current = themes_dir.join("current");
        let src = theme2.to_str().unwrap();
        let dst = current.to_str().unwrap();
        assert!(unix_fs::symlink(src, dst).is_ok());

        let config_dir = tmpdir.path().to_str().unwrap();
        let result = Repo::installed_themes(String::from(config_dir));
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec!["test-theme2".to_string(), "test-theme1".to_string(),],
        )
    }

    #[test]
    fn test_installed_themes_no_themes_dir() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config_dir = tmpdir.path().to_str().unwrap();
        assert!(Repo::installed_themes(String::from(config_dir)).is_ok());
    }
}
