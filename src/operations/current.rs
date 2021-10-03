use std::path::PathBuf;
use crate::models::Config;
use crate::errors;
use clap::Clap;
use toml::Value;
use std::fs;



#[derive(Clap, Debug)]
pub struct Current {
    pub field: String,
}


impl Current {
    /// # Errors
    ///
    /// Will error if the requested field in not found in theme.toml
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

        //read the current theme.toml and define a value for the the requested field isnt found.
        let file_data: String = fs::read_to_string(directory).unwrap();
        let cfg_data: Value = toml::from_str(&file_data).expect("no data");
        let error_value = Value::String("Not Found".to_string());

        //return the requested field.
        println!("{}", cfg_data.get(self.field.clone()).unwrap_or(&error_value));

        Ok(())
    }

}
