use crate::errors;
use crate::models::Config;
use clap::Clap;
use colored::Colorize;
use edit_distance::edit_distance;
use errors::LeftError;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use log::trace;

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
            trace!(
                "Theme: {}, Distance:{}",
                &theme.name,
                edit_distance(&theme.name, &self.name)
            );
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

    #[allow(dead_code)]
    // Performs a match using edit_distance.
    fn edit_distance_match(a: &str, b: &str) -> bool {
        if edit_distance(a, b) <= 3 {
            return true;
        }
        false
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
        // Temporary, to demonstrate mismatch. Will be removed later.
        assert!(!Search::edit_distance_match("apple pie", "apple"));
        assert!(!Search::edit_distance_match("apple pie", "pie"));
        assert!(!Search::edit_distance_match("Windows XP", "windows"));
        assert!(!Search::edit_distance_match("Windows XP", "xp"));
        assert!(!Search::edit_distance_match("Windows XP", "zinbows"));
        assert!(!Search::edit_distance_match("Soothe", "soo"));
        assert!(Search::edit_distance_match("Soothe", "soohe"));

        assert!(Search::fuzzy_matcher_match("apple pie", "apple"));
        assert!(Search::fuzzy_matcher_match("apple pie", "pie"));
        assert!(Search::fuzzy_matcher_match("Windows XP", "xp"));
        assert!(Search::fuzzy_matcher_match("Windows XP", "windows"));
        assert!(!Search::fuzzy_matcher_match("Windows XP", "zinbows"));
        assert!(Search::fuzzy_matcher_match("Soothe", "soo"));
        assert!(Search::fuzzy_matcher_match("Soothe", "soohe"));
    }
}
