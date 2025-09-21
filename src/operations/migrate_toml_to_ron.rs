use clap::Parser;
use leftwm_core::models::Gutter;
use log::trace;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::errors::LeftError;

/* Thes function converts a `theme.toml` provided by the `path` arg
   into a `theme.ron` at the same directory as the input file.
    Required argument is the path to the file that should be converted.
*/

#[derive(Debug, Parser)]
pub struct Migrate {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Theme {
    pub border_width: Option<i32>,
    pub margin: Option<CustomMargins>,
    pub workspace_margin: Option<CustomMargins>,
    pub default_width: Option<i32>,
    pub default_height: Option<i32>,
    pub always_float: Option<bool>,
    pub gutter: Option<Vec<Gutter>>,
    pub default_border_color: Option<String>,
    pub floating_border_color: Option<String>,
    pub focused_border_color: Option<String>,
    pub background_color: Option<String>,
    #[serde(rename = "on_new_window")]
    pub on_new_window_cmd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum CustomMargins {
    Int(u32),
    // format: [top, right, bottom, left] as per HTML
    Vec(Vec<u32>),
}

impl Migrate {
    /// # Errors
    ///
    /// Returns an error if theme file cannot be loaded / saved
    /// Returns an error if theme not found.
    pub fn exec(&self) -> Result<(), LeftError> {
        trace!("Migrating theme named {}", &self.path.display());
        match migrate(&self.path) {
            Ok(()) => Ok(()),
            Err(_) => Err(LeftError::from("Failed to migrate theme.")),
        }
    }
}

fn migrate(path: &PathBuf) -> Result<(), LeftError> {
    let Ok(theme) = load_theme_file(path) else {
        return Err(LeftError::from("Theme not found"));
    };
    let mut ron_path = path.clone();
    ron_path.set_extension("ron");
    match write_to_file(&ron_path, &theme) {
        Ok(()) => Ok(()),
        Err(_) => Err(LeftError::from("Failed to write theme file.")),
    }
}

fn write_to_file(ron_file: &PathBuf, theme: &Theme) -> Result<(), LeftError> {
    let ron_pretty_conf = ron::ser::PrettyConfig::new()
        .depth_limit(2)
        .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
    let ron_theme = ron::ser::to_string_pretty(&theme, ron_pretty_conf)?;
    let mut file = File::create(ron_file)?;
    file.write_all(ron_theme.as_bytes())?;
    Ok(())
}

fn load_theme_file(path: impl AsRef<Path>) -> Result<Theme, LeftError> {
    let contents = fs::read_to_string(&path)?;
    let from_file: Theme = toml::from_str(&contents)?;
    Ok(from_file)
}
