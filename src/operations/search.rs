use crate::errors;
use crate::models::Config;
use clap::Clap;
use colored::Colorize;
use errors::LeftError;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

/* This function searches for themes, but does not update them by default
 *                     */

#[derive(Clap, Debug)]
pub struct Search {
    /// Name of theme to find
    pub name: String,
}

impl Search {
    /// # Errors
    ///
    /// No errors expected.
    pub fn exec(&self, config: &mut Config) -> Result<(), LeftError> {
        // Load the configuration
        println!(
            "{}",
            "Searching for themes with similar names . . . "
                .bright_blue()
                .bold()
        );
        // Iterate over the different themes, if the distance
        for theme in &config.themes(false) {
            if Search::fuzzy_matcher_match(&theme.name, &self.name) {
                let current = match theme.current {
                    Some(true) => "Current: ".bright_yellow().bold(),
                    _ => "".white(),
                };
                let installed = match theme.directory {
                    Some(_) => "-Installed".red().bold(),
                    None => "".white(),
                };
                println!(
                    "   {}{}/{}: {}{}",
                    current,
                    theme
                        .source
                        .clone()
                        .unwrap_or_default()
                        .bright_magenta()
                        .bold(),
                    theme.name.clone().bright_green().bold(),
                    theme
                        .description
                        .as_ref()
                        .unwrap_or(&"A LeftWM theme".to_string()),
                    installed
                );
            }
        }

        Ok(())
    }

    // Performs a using fuzzy matcher.
    fn fuzzy_matcher_match(a: &str, b: &str) -> bool {
        let matcher = SkimMatcherV2::default();
        matcher.fuzzy_match(a, b).is_some()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_match() {
        assert!(Search::fuzzy_matcher_match("apple pie", "apple"));
        assert!(Search::fuzzy_matcher_match("apple pie", "pie"));
        assert!(Search::fuzzy_matcher_match("Windows XP", "xp"));
        assert!(Search::fuzzy_matcher_match("Windows XP", "windows"));
        assert!(!Search::fuzzy_matcher_match("Windows XP", "zinbows"));
        assert!(Search::fuzzy_matcher_match("Soothe", "soo"));
        assert!(Search::fuzzy_matcher_match("Soothe", "soohe"));
    }
}
