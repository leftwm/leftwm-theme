pub mod config;
mod leftwm;
mod theme;

pub use config::{Config, Repo, THEMES_DIR};
pub use leftwm::LeftWm;
pub use theme::{DependencyL, Theme};
