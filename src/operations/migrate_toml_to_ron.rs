use anyhow::Result;
use clap::Parser;
use leftwm_core::models::Gutter;
use log::trace;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::errors::LeftError;

#[derive(Debug, Parser)]
pub struct Migrate {
    pub path: PathBuf,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Theme {
    border_width: i32,
    margin: CustomMargins,
    workspace_margin: Option<CustomMargins>,
    default_width: Option<i32>,
    default_height: Option<i32>,
    always_float: Option<bool>,
    gutter: Option<Vec<Gutter>>,
    default_border_color: String,
    floating_border_color: String,
    focused_border_color: String,
    #[serde(rename = "on_new_window")]
    on_new_window_cmd: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
enum CustomMargins {
    Int(u32),
    // format: [top, right, bottom, left] as per HTML
    Vec(Vec<u32>),
}

impl Migrate {
    /// # Errors
    ///
    /// Returns an error if config cannot be loaded / saved
    /// Returns an error if `BaseDirectory` not set.
    /// Returns an error if theme not found.
    pub fn exec(&self) -> Result<(), LeftError> {
        trace!("Migrating theme named {:?}", &self.path);
        match migrate(&self.path) {
            Ok(_) => Ok(()),
            Err(_) => Err(LeftError::from("Failed to migrate theme.")),
        }
    }
}

fn migrate(path: &PathBuf) -> Result<(), LeftError> {
    let theme = match load_theme_file(path) {
        Ok(theme) => theme,
        Err(_) => {
            return Err(LeftError::from("Theme not found"));
        }
    };
    let mut ron_path = path.clone();
    ron_path.set_extension("ron");
    match write_to_file(&ron_path, &theme) {
        Ok(_) => Ok(()),
        Err(_) => Err(LeftError::from("Failed to write theme file.")),
    }
}

fn write_to_file(ron_file: &PathBuf, theme: &Theme) -> Result<(), anyhow::Error> {
    let ron_pretty_conf = ron::ser::PrettyConfig::new()
        .depth_limit(2)
        .extensions(ron::extensions::Extensions::IMPLICIT_SOME);
    let ron_theme = ron::ser::to_string_pretty(&theme, ron_pretty_conf)?;
    let mut file = File::create(&ron_file)?;
    file.write_all(ron_theme.as_bytes())?;
    Ok(())
}

fn load_theme_file(path: impl AsRef<Path>) -> Result<Theme> {
    let contents = fs::read_to_string(&path)?;
    let from_file: Theme = toml::from_str(&contents)?;
    Ok(from_file)
}
