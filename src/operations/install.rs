use crate::errors;
use crate::errors::friendly_message;
use crate::errors::Result;
use crate::models::{Config, Theme};
use clap::Clap;
use colored::Colorize;
use git2::Repository;
use log::{error, trace};
use std::io;
use std::io::Write;

#[derive(Clap, Debug)]
pub struct Install {
    /// Read theme from git repository
    #[clap(short = 'g', long)]
    pub git: bool,

    /// Read theme from path
    #[clap(short = 'p', long)]
    pub path: bool,

    /// Location of theme
    pub name: String,
}

impl Install {
    pub fn exec(&self) -> Result<()> {
        println!("{}", "Looking for theme . . . ".bright_blue().bold());
        let mut config = Config::load().unwrap_or_default();
        trace!("{:?}", &mut config);

        let mut found = Theme::find_all(&mut config, &self.name)
            .ok_or_else(|| friendly_message("Could not found find theme"))?;

        //ask the user to pick a matching theme
        let mut selected = choose_one(&mut found)?;

        //install the selected theme
        self.install_selected_theme(&mut selected, config)?;

        Ok(())
    }

    fn install_selected_theme(&self, theme: &mut Theme, config: Config) -> Result<()> {
        trace!("{:?}", &theme);
        //get the repo
        let repo = theme
            .repository
            .as_ref()
            .ok_or_else(|| friendly_message("Repository information missing for theme"))?;
        //build the path
        let mut dir = config.theme_dir()?;
        dir.push(&theme.name);
        //clone the repo
        Repository::clone(&repo, dir.clone()).map_err(|err| {
            let msg = format!(
                "\n{} could not be installed because {:?} \n\n Theme not installed",
                &theme.name,
                err.message()
            );
            friendly_message(&msg)
        })?;
        //
        self.add_to_config_and_save(theme, config, dir)
    }

    fn add_to_config_and_save(
        &self,
        theme: &mut Theme,
        mut config: Config,
        dir: std::path::PathBuf,
    ) -> Result<()> {
        let not_in_db = || friendly_message("Theme not found in db");

        // update the directory info of theme entry in the config
        let source = theme.source.as_ref().ok_or_else(not_in_db)?;
        let target_theme =
            Theme::find_mut(&mut config, &self.name, source).ok_or_else(not_in_db)?;
        target_theme.directory = Some(dir);
        Config::save(&config)?;

        print_theme_install_info(theme);

        Ok(())
    }
}

fn print_theme_install_info(theme: &Theme) {
    //print the friendly info about the installed theme
    println!(
        "{}{}{}{}{}{}",
        "Downloaded theme ".bright_blue().bold(),
        &theme.name.green(),
        ". \nTo set as default, use ".bright_blue().bold(),
        "leftwm-theme apply \"".bright_yellow().bold(),
        &theme.name.bright_yellow().bold(),
        "\"".bright_yellow().bold()
    );
}

fn choose_one(themes: &mut [Theme]) -> Result<&mut Theme> {
    if themes.len() == 1 {
        Ok(&mut themes[0])
    } else if themes.is_empty() {
        Err(friendly_message("No themes with that name were found"))
    } else {
        let idx = ask(&themes)?;
        Ok(&mut themes[idx])
    }
}

fn ask(themes: &[Theme]) -> Result<usize> {
    #[allow(unused_assignments)]
    let mut return_index = Err(errors::LeftError::from("No themes available"));
    'outer: loop {
        println!(
            "{}",
            "Which theme would you like to install?"
                .bright_yellow()
                .bold()
        );
        for (id, theme) in themes.iter().enumerate() {
            if theme.directory.is_some() {
                error!("A theme with that name is already installed");
                return_index = Err(errors::LeftError::from("Theme already installed"));
                break 'outer;
            }
            let source_string = match &theme.source {
                Some(source) => source.clone(),
                None => String::from("UNKNOWN"),
            };
            println!(
                "    {}/{} [{}]",
                &source_string.bright_magenta().bold(),
                &theme.name.bright_green().bold(),
                &id.to_string().bright_yellow().bold()
            );
        }
        print!("{}", "=>".bright_yellow().bold());
        io::stdout().flush().unwrap();
        let val = read_num();
        if let Ok(index) = val {
            if index < themes.len() {
                return_index = Ok(index);
                break;
            }
        }
        println!("{}", "Error: Please select a number:".bright_red().bold())
    }
    return_index
}

fn read_num() -> Result<usize> {
    let mut words = String::new();
    io::stdin().read_line(&mut words).ok();
    let trimmed = words.trim();
    trace!("Trimmed receipt: {:?}", &trimmed);
    match trimmed.parse::<usize>() {
        Ok(size) => Ok(size),
        Err(err) => Err(errors::LeftError::from(err)),
    }
}
