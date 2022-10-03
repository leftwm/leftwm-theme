use anyhow::Result;
use clap::Parser;
use leftwm_core::models::{Gutter, Margins};
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
pub struct Theme {
    pub border_width: i32,
    pub margin: CustomMargins,
    pub workspace_margin: Option<CustomMargins>,
    pub default_width: Option<i32>,
    pub default_height: Option<i32>,
    pub always_float: Option<bool>,
    pub gutter: Option<Vec<Gutter>>,
    pub default_border_color: String,
    pub floating_border_color: String,
    pub focused_border_color: String,
    #[serde(rename = "on_new_window")]
    pub on_new_window_cmd: Option<String>,
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
            Err(_) => return Err(LeftError::from("Failed to migrate theme.")),
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
        Err(_) => return Err(LeftError::from("Failed to write theme file.")),
    }
}
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum CustomMargins {
    Int(u32),
    // format: [top, right, bottom, left] as per HTML
    Vec(Vec<u32>),
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

impl std::convert::TryFrom<CustomMargins> for Margins {
    type Error = &'static str;

    fn try_from(c: CustomMargins) -> Result<Self, Self::Error> {
        match c {
            CustomMargins::Int(size) => Ok(Self::new(size)),
            CustomMargins::Vec(vec) => match vec.len() {
                1 => Ok(Self::new(vec[0])),
                2 => Ok(Self::new_from_pair(vec[0], vec[1])),
                3 => Ok(Self::new_from_triple(vec[0], vec[1], vec[2])),
                4 => Ok(Self {
                    top: vec[0],
                    right: vec[1],
                    bottom: vec[2],
                    left: vec[3],
                }),
                0 => Err("Empty margin or border array"),
                _ => Err("Too many entries in margin or border array"),
            },
        }
    }
}
