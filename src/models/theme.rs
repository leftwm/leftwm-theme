use crate::errors;
use crate::models::{Config, THEMES_DIR};
use std::fs;
use std::path::{Path, PathBuf};

/// Contains information about a theme contained within themes.toml (or known.toml upstream).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Theme {
    /// Name of the theme, must follow arch convention [az-_]
    pub name: String,
    /// A helpful description of the theme
    pub description: Option<String>,
    /// (Local) (Managed by leftwm-theme), the local directory where the theme is stored
    pub directory: Option<PathBuf>,
    /// The git repository where the theme may be downloaded from
    pub repository: Option<String>,
    /// The commit to use for the theme; can use * for HEAD
    pub commit: Option<String>,
    /// The version for the theme, incrementing will force updates
    pub version: Option<String>,
    /// Compatible leftwm versions
    pub leftwm_versions: Option<String>,
    /// (Local) Whether the theme is the current theme
    pub current: Option<bool>,
    /// A list of dependencies
    pub dependencies: Option<Vec<DependencyL>>,
    /// Path to the directory containing up, down, and theme.toml w.r.t. root
    pub relative_directory: Option<String>,
    #[serde(skip)]
    pub source: Option<String>,
}

/// Contains a vector of themes used for processing.
#[derive(Debug, Deserialize)]
pub struct TempThemes {
    pub theme: Vec<Theme>,
}

/// Contains information pertaining to a program dependency (name, required/optional, package).
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct DependencyL {
    pub program: String,
    pub optional: Option<bool>,
    pub package: Option<String>,
}

impl Default for DependencyL {
    fn default() -> DependencyL {
        DependencyL {
            program: String::from("leftwm"),
            optional: None,
            package: None,
        }
    }
}

impl Theme {
    #[must_use]
    pub fn new(name: &str, description: Option<String>, directory: Option<PathBuf>) -> Self {
        Theme {
            name: name.to_string(),
            description,
            directory,
            repository: None,
            commit: None,
            version: Some("0.0.0".to_string()),
            leftwm_versions: Some("*".to_string()),
            dependencies: None,
            current: Some(false),
            relative_directory: None,
            source: None,
        }
    }

    #[must_use]
    pub fn is_installed(&self) -> bool {
        match &self.directory {
            Some(_dir) => true,
            None => false,
        }
    }

    pub fn find(config: &mut Config, name: &str) -> Option<Theme> {
        config
            .themes(false)
            .iter()
            .find(|p| name == p.name)
            .cloned()
    }

    pub fn find_installed(config: &mut Config, name: &str) -> Option<Theme> {
        config
            .themes(false)
            .iter()
            .find(|p| name == p.name && p.directory.is_some())
            .cloned()
    }

    pub fn find_all(config: &mut Config, name: &str) -> Option<Vec<Theme>> {
        let (themes, _) = config
            .themes(false)
            .iter()
            .cloned()
            .partition::<Vec<Theme>, _>(|p| name == p.name);
        Some(themes)
    }

    pub fn find_mut<'a>(
        config: &'a mut Config,
        name: &str,
        repo_name: &str,
    ) -> Option<&'a mut Theme> {
        match config.repos.iter_mut().find(|p| repo_name == p.name) {
            Some(reposit) => reposit.themes.iter_mut().find(|o| name == o.name),
            None => None,
        }
    }

    pub fn source(&mut self, name: String) -> &mut Theme {
        self.source = Some(name);
        self
    }

    /// Sets relative directory; abstracting because behavior might change
    pub fn set_relative_directory(&mut self, rel_dir: Option<String>) {
        self.relative_directory = rel_dir;
    }

    /// Gets relative directory; abstracting because behavior might change; <3 JKN MGK
    pub fn relative_directory(&self) -> Option<String> {
        self.relative_directory.clone()
    }

    pub fn current(&mut self, currency: bool) {
        self.current = if currency { Some(true) } else { None }
    }

    /// Gets the name of the theme after applying any theme changes.
    pub fn get_name(&self) -> String {
        // TODO: When theme rename change is implemented, return the newly
        // obtained name after applying the rename change if any.
        self.name.clone()
    }

    /// Applies changes to the repo if changes are defined for it.
    /// # Errors
    ///
    /// Errors if the directory rename change fails.
    pub fn apply_changes(&self, config_dir: &Path) -> Result<(), errors::LeftError> {
        let effectual_name = self.get_name();
        if self.name != effectual_name {
            Theme::apply_change_rename_dir(&self.name, &effectual_name, config_dir)?;
        }
        Ok(())
    }

    // Applies the theme rename change. Given an old name and a new name of the
    // theme, it renames the theme's old directory to the new name if found.
    fn apply_change_rename_dir(
        old_name: &str,
        new_name: &str,
        config_dir: &Path,
    ) -> Result<(), errors::LeftError> {
        let old_theme_dir = config_dir.join(THEMES_DIR).join(&old_name);
        if old_theme_dir.exists() {
            println!("Moving theme {} to {}", old_name, new_name);
            let new_theme_dir = config_dir.join(THEMES_DIR).join(&new_name);
            fs::rename(old_theme_dir, new_theme_dir)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply_change_rename_dir_existing_theme() {
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);
        let old_theme_dir = themes_dir.join("theme-x");
        let new_theme_dir = themes_dir.join("theme-y");

        // Create old theme dir.
        assert!(fs::create_dir_all(&old_theme_dir).is_ok());

        assert!(Theme::apply_change_rename_dir("theme-x", "theme-y", &tmpdir.into_path()).is_ok());

        // Check if the new dir exists.
        assert!(new_theme_dir.exists());
    }

    #[test]
    fn test_apply_change_rename_dir_no_existing_theme() {
        let tmpdir = tempfile::tempdir().unwrap();
        let themes_dir = tmpdir.path().join(THEMES_DIR);
        let new_theme_dir = themes_dir.join("theme-y");

        // No theme directory exists.

        assert!(Theme::apply_change_rename_dir("theme-x", "theme-y", &tmpdir.into_path()).is_ok());
        assert!(!new_theme_dir.exists());
    }
}
