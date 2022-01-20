use crate::errors;
use crate::errors::Result;
use crate::models::theme::{TempThemes, Theme};
use colored::Colorize;
use log::{error, trace};
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use xdg::BaseDirectories;

pub const THEMES_DIR: &str = "themes";

const BASE_DIR_PREFIX: &str = "leftwm";
const CURRENT_DIR: &str = "current";
const LOCAL_REPO_NAME: &str = "LOCAL";
const COMMUNITY_REPO_NAME: &str = "community";
const THEMES_CONFIG_FILENAME: &str = "themes.toml";
pub const CURRENT_DEFINITIONS_VERSION: i16 = 1;

/// Contains a vector of all global repositories.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Config {
    pub repos: Vec<Repo>,
    pub config_dir: Option<PathBuf>,
}

/// Contains global repository information. Akin to known.toml or themes.toml
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Repo {
    pub url: String,
    pub name: String,
    #[serde(default)]
    pub definitions_version: i16,
    pub themes: Vec<Theme>,
}

impl Config {
    #[must_use]
    // Create a new Config at the given path.
    pub fn new(config_path: Option<PathBuf>) -> Self {
        Config {
            repos: vec![],
            config_dir: config_path,
        }
    }

    // Populates the Config with defaults and returns it.
    pub fn default(&mut self) -> Self {
        let community_repo = Repo {
                    url: String::from("https://raw.githubusercontent.com/leftwm/leftwm-community-themes/master/known.toml"),
                    name: String::from(COMMUNITY_REPO_NAME),
                    definitions_version: 1,
                    themes: Vec::new()
                };
        let local_repo = Repo {
            url: String::from("localhost"),
            name: String::from(LOCAL_REPO_NAME),
            definitions_version: CURRENT_DEFINITIONS_VERSION,
            themes: Vec::new(),
        };
        self.repos.push(community_repo);
        self.repos.push(local_repo);
        self.clone()
    }

    // Returns the config dir path. If config path is None, it constructs and
    // returns a default config path (~/.config/leftwm).
    /// # Errors
    ///
    /// Will error if base directory cannot be obtained AND path does not exist on passthru.
    /// Will error if unable to create configuration directory.
    pub fn get_config_dir(&self) -> Result<PathBuf> {
        match &self.config_dir {
            Some(path) => Ok(path.clone()),
            None => {
                let path = BaseDirectories::with_prefix(BASE_DIR_PREFIX)?;
                // Create the directory if it doesn't exist
                fs::create_dir_all(&path.get_config_home())?;
                Ok(path.get_config_home())
            }
        }
    }

    /// # Errors
    ///
    /// Errors if toml cannot be obtained for the themes.toml file
    /// Errors if the `BaseDirectory` is not set (no systemd)
    /// Errors if no file can be saved
    pub fn save(config: &Self) -> Result<&Config> {
        let config_filename = config.get_config_dir()?.join(THEMES_CONFIG_FILENAME);
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
                    target_theme.support_url = theme.support_url.clone();
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
                definitions_version: CURRENT_DEFINITIONS_VERSION,
            });
            let lent = config.repos.len();
            config.repos[lent - 1].themes.push(theme.clone());
        }
    }

    pub fn themes(&mut self, local: bool) -> Vec<Theme> {
        let mut themes: Vec<Theme> = Vec::new();
        for repo in &self.repos {
            if local && repo.name == *LOCAL_REPO_NAME {
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
    pub fn load(&self) -> Result<Config> {
        let config_filename = self.get_config_dir()?.join(THEMES_CONFIG_FILENAME);
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
            let config = Config::new(None).default();
            let toml = toml::to_string(&config)?;
            let mut file = File::create(&config_filename)?;
            file.write_all(toml.as_bytes())?;
            Ok(config)
        }
    }

    // Updates the Config with local themes. This depends on the config to
    // already have remote theme repos populated. It assumes that the themes
    // that aren't in any of the remote repos are local themes.
    //
    /// # Errors
    ///
    /// Will return an error if the installed themes listing cannot be obtained.
    pub fn update_local_repo(&mut self) -> Result<()> {
        // Get a list of all the themes in the themes directory.
        let existing_themes = Repo::installed_themes(&self.get_config_dir()?)?;

        let mut local_themes: Vec<String> = Vec::new();

        // Iterate through the existing themes and check if they are from the
        // remote repos. If not, consider the theme to be a local theme.
        for tt in existing_themes {
            let mut found: bool = false;
            for repo in &self.repos {
                if repo.name != LOCAL_REPO_NAME {
                    for theme in &repo.themes {
                        if tt.eq(&theme.name) {
                            found = true;
                            break;
                        }
                    }
                }
                // Break out of the loop, since we already found the theme,
                // so as to process the next existing theme.
                if found {
                    break;
                }
            }
            if !found {
                local_themes.push(tt);
            }
        }

        // Create TempThemes from the local themes.
        let mut local_temp_themes = TempThemes {
            theme: vec![],
            definitions_version: CURRENT_DEFINITIONS_VERSION,
        };
        let config_dir = self.get_config_dir()?;
        for lt in local_themes {
            let path = config_dir.clone().join(THEMES_DIR).join(&lt);
            let t = Theme::new(&lt, None, Some(path));
            local_temp_themes.theme.push(t);
        }

        // Update the local themes in the Config.
        for repo in &mut self.repos {
            if repo.name == LOCAL_REPO_NAME {
                repo.compare(local_temp_themes, &config_dir)?;
                break;
            }
        }

        Ok(())
    }
}

impl Repo {
    /// # Errors
    ///
    /// Returns an error if the definitions file is OOD.
    pub fn compare(&mut self, theme_wrap: TempThemes, config_dir: &Path) -> Result<&Repo> {
        if self.definitions_version > CURRENT_DEFINITIONS_VERSION
            || theme_wrap.definitions_version > CURRENT_DEFINITIONS_VERSION
        {
            println!("{}", "========== ERROR ==========".bold().red());
            println!("REPOSITORY DEFINITION HAS INCREASED.");
            println!("USUALLY THIS MEANS YOUR LEFTWM-THEME IS OUT OF DATE.");
            println!("{}", "========== ERROR ==========".bold().red());
            return Err(errors::LeftError::from("Definitions file out of date."));
        }
        let themes = theme_wrap.theme;
        trace!("Comparing themes");

        // Get a list of existing themes.
        let existing_themes = Repo::installed_themes(&config_dir.to_path_buf())?;
        let current_theme = Repo::current_theme(&config_dir.to_path_buf())?.unwrap_or_default();
        let themes_dir = config_dir.join(THEMES_DIR);

        // Iterate over all the themes, and update/add if needed.
        for mut tema in themes {
            // Apply any theme changes before updating or adding it.
            tema.apply_changes(config_dir)?;

            // Check if the theme is already installed and update the theme
            // directory attribute.
            if existing_themes.contains(&tema.name.clone()) {
                tema.directory = Some(themes_dir.join(tema.name.clone()));
            }

            // Check if this is the current theme.
            if current_theme.eq(&tema.name.clone()) {
                tema.current = Some(true);
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
                target_theme.support_url = theme.support_url.clone();
                target_theme.set_relative_directory(theme.relative_directory.clone());
                target_theme.dependencies = theme.dependencies.clone();
                target_theme.directory = theme.directory.clone();
            }
            // o/w insert a new leaf at the end
            None => {
                repo.themes.push(theme.clone());
            }
        }
    }

    // Looks for the current theme in the themes directory and returns the name
    // of the current theme.
    fn current_theme(config_path: &Path) -> Result<Option<String>> {
        let theme_path = config_path.join(THEMES_DIR);

        // Return None if themes directory doesn't exist.
        if !theme_path.exists() {
            return Ok(None);
        }

        // Read the themes directory, find the "current" theme and get the
        // current theme name.
        let mut result = String::new();
        let current_dir = OsStr::new(CURRENT_DIR);
        let paths = fs::read_dir(theme_path)?;
        for path in paths {
            let p = &path?.path();
            // Get the file with name "current" and check if it's a symlink.
            // Follow the symlink to find the target theme.
            let target_file_name = p
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            if target_file_name.eq(current_dir) {
                let metadata = fs::symlink_metadata(p)?;
                let file_type = metadata.file_type();
                if file_type.is_symlink() {
                    result = String::from(
                        fs::read_link(p)?
                            .file_name()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default(),
                    );
                }
                break;
            }
        }

        if result.is_empty() {
            return Ok(None);
        }

        Ok(Some(result))
    }

    // Returns a list of all the installed theme names under a given config
    // path.
    fn installed_themes(config_path: &Path) -> Result<Vec<String>> {
        let mut result: Vec<String> = Vec::new();

        let theme_path = config_path.join(THEMES_DIR);

        // Return empty result if the themes directory is not present.
        if !theme_path.exists() {
            return Ok(result);
        }

        // Read the themes directory, iterate through the entries, determine
        // which of them are theme directories and add them into the result.
        let paths = fs::read_dir(theme_path)?;
        for path in paths {
            let p = path?;
            // NOTE: For symlinks, metadata() traverses any symlinks and queries
            // the metadata information from the destination.
            let metadata = fs::metadata(p.path())?;
            let file_type = metadata.file_type();

            // Only process directories.
            if !file_type.is_dir() {
                continue;
            }

            // Ignore the "current" directory for installed theme list.
            let current_dir = String::from(CURRENT_DIR);
            let target_path = p.path();
            if target_path
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .eq(&current_dir)
            {
                continue;
            }

            // Extract only the theme name for the result.
            let theme_name = target_path.file_name().unwrap_or_default();
            result.push(String::from(theme_name.to_str().unwrap_or_default()));
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
        let current = themes_dir.join(CURRENT_DIR);
        let src = theme2.to_str().unwrap();
        let dst = current.to_str().unwrap();
        assert!(unix_fs::symlink(src, dst).is_ok());

        let config_dir = tmpdir.path().to_path_buf();
        let result = Repo::installed_themes(&config_dir);
        assert!(result.is_ok());
        let mut result_vec = result.unwrap();
        result_vec.sort();
        assert_eq!(
            result_vec,
            vec!["test-theme1".to_string(), "test-theme2".to_string(),],
        );
    }

    #[test]
    fn test_installed_themes_no_themes_dir() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config_dir = tmpdir.path().to_path_buf();
        assert!(Repo::installed_themes(&config_dir).is_ok());
    }

    #[test]
    fn test_current_theme() {
        // Create a temporary directory as the config path and create the
        // directory layout within it for themes.
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);
        let theme1 = themes_dir.join("test-theme1");
        let theme2 = themes_dir.join("test-theme2");
        assert!(fs::create_dir_all(&theme1).is_ok());
        assert!(fs::create_dir_all(&theme2).is_ok());

        // Create current theme as a symlink to an existing theme.
        let current = themes_dir.join(CURRENT_DIR);
        let src = theme2.to_str().unwrap();
        let dst = current.to_str().unwrap();
        assert!(unix_fs::symlink(src, dst).is_ok());

        let result = Repo::current_theme(&tmpdir.path().to_path_buf());
        assert_eq!(result.unwrap().unwrap(), "test-theme2");
    }

    #[test]
    fn test_current_theme_unmanaged() {
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);

        // Custom theme, not a symlink, not managed by leftwm-theme.
        let current = themes_dir.join(CURRENT_DIR);
        assert!(fs::create_dir_all(&current).is_ok());

        let result = Repo::current_theme(&tmpdir.path().to_path_buf());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_current_theme_no_themes_dir() {
        let tmpdir = tempfile::tempdir().unwrap();
        assert!(Repo::current_theme(&tmpdir.path().to_path_buf())
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_current_theme_no_current() {
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);
        let theme1 = themes_dir.join("test-theme1");
        let theme2 = themes_dir.join("test-theme2");
        assert!(fs::create_dir_all(&theme1).is_ok());
        assert!(fs::create_dir_all(&theme2).is_ok());
        assert!(Repo::current_theme(&tmpdir.path().to_path_buf())
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_current_theme_current_file() {
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);
        assert!(fs::create_dir_all(&themes_dir).is_ok());

        // Create a file "current", instead of a directory.
        let current_file = themes_dir.join(CURRENT_DIR);
        assert!(File::create(current_file).is_ok());

        assert!(Repo::current_theme(&tmpdir.path().to_path_buf())
            .unwrap()
            .is_none());
    }

    #[test]
    fn test_config_new() {
        let config1 = Config::new(None);
        assert!(config1.config_dir.is_none());
        assert!(config1
            .get_config_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .ends_with("/.config/leftwm/"));

        let config2 = Config::new(Some(PathBuf::from("/tmp/foo")));
        assert!(config2.config_dir.is_some());
        assert!(config2
            .get_config_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .eq("/tmp/foo"));
    }

    #[test]
    fn test_config_default() {
        let config = Config::new(None).default();
        assert_eq!(config.repos.len(), 2);
        assert!(config.repos.iter().any(|x| x.name == COMMUNITY_REPO_NAME));
        assert!(config.repos.iter().any(|x| x.name == LOCAL_REPO_NAME));
    }

    #[test]
    fn test_config_update_local_repo() {
        // Create test config directory layout with community and local themes.
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);
        let comm_theme1 = themes_dir.join("community-theme1");
        let comm_theme2 = themes_dir.join("community-theme2");
        let local_theme1 = themes_dir.join("local-theme1");
        let local_theme2 = themes_dir.join("local-theme2");

        assert!(fs::create_dir_all(&comm_theme1).is_ok());
        assert!(fs::create_dir_all(&comm_theme2).is_ok());
        assert!(fs::create_dir_all(&local_theme1).is_ok());
        assert!(fs::create_dir_all(&local_theme2).is_ok());

        // Set current theme symlink to a local theme.
        let current = themes_dir.join(CURRENT_DIR);
        let src = local_theme1.to_str().unwrap();
        let dst = current.to_str().unwrap();
        assert!(unix_fs::symlink(src, dst).is_ok());

        // Construct themes to be added to the community repo.
        let t1 = Theme::new("community-theme1", None, Some(comm_theme1));
        let t2 = Theme::new("community-theme2", None, Some(comm_theme2));
        // Extra uninstalled theme.
        let t3 = Theme::new("community-theme3", None, None);

        let mut config = Config::new(Some(tmpdir.path().to_path_buf())).default();

        // Append the themes created above to the community repo.
        for repo in &mut config.repos {
            if repo.name == COMMUNITY_REPO_NAME {
                repo.themes.push(t1);
                repo.themes.push(t2);
                repo.themes.push(t3);
                break;
            }
        }

        assert!(config.update_local_repo().is_ok());

        let comm_repo = config
            .repos
            .clone()
            .into_iter()
            .find(|x| x.name == COMMUNITY_REPO_NAME)
            .unwrap();
        assert_eq!(comm_repo.themes.len(), 3);

        let local_repo = config
            .repos
            .into_iter()
            .find(|x| x.name == LOCAL_REPO_NAME)
            .unwrap();
        assert_eq!(local_repo.themes.len(), 2);

        // Check if local theme is the current theme.
        let local_theme1 = local_repo
            .themes
            .into_iter()
            .find(|x| x.name == "local-theme1")
            .unwrap();
        assert!(local_theme1.current.unwrap());
    }
}
