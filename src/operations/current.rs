use crate::errors;
use crate::errors::friendly_message;
use crate::models::Config;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use toml::Value;

#[derive(Parser, Debug)]
pub struct Current {
    pub field: String,
}

impl Current {
    /// # Errors
    ///
    /// Will error if the requested field in not found in theme.toml
    ///
    /// # Panics
    ///
    /// Will panic if unable to retrieve value but file does exist.
    /// Will panic if a current theme is set but directory is not set.
    pub fn exec(&self, config: &mut Config) -> Result<(), errors::LeftError> {
        // define directory so it can be used outside the scope of the loop
        let mut directory: PathBuf = PathBuf::new();

        // get the path to the current theme.toml
        for theme in config.themes(false) {
            if theme.current == Some(true) {
                directory = theme.directory.unwrap();
                directory.push("theme");
                directory.set_extension("toml");
            }
        }

        // read the current theme.toml
        let file_data: String = fs::read_to_string(directory).unwrap();
        let cfg_data: Value = toml::from_str(&file_data).expect("no data");

        // check if the field exists if it doesn't return an error
        if let Some(field_value) = cfg_data.get(self.field.clone()) {
            //return requested field
            println!("{}", format!("{}", field_value).replace("\"", ""));
            Ok(())
        } else {
            // returning an error
            Err(friendly_message("That field was not found"))
        }
    }
}
