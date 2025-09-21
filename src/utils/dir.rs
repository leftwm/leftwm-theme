use crate::errors::Result;
use xdg::BaseDirectories;
/// # Errors
///
/// Will error if `BaseDirectory` not set
/// Will error if unable to create theme leftwm directory
pub fn theme() -> Result<std::path::PathBuf> {
    let mut dir = BaseDirectories::with_prefix("leftwm").create_config_directory("")?;
    dir.push("themes");
    Ok(dir)
}
