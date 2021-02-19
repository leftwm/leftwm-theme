#![feature(try_trait)]
#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod functions;
pub mod models;

use crate::functions::{install, list, new, update, apply, uninstall, upgrade};
use clap::{App, Arg, ArgMatches, SubCommand};
use log::{error, info, trace};

fn main() {
    pretty_env_logger::init();
    // Initialize clap to determine arguments and call appropriate functions
    let version = env!("CARGO_PKG_VERSION");
    let matches = App::new("LeftWM Theme Manager")
        .author("Lex Childs <lex.childs@gmail.com>")
        .version(version)
        .about("Manages LeftWM themes")
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("print debug information verbosely"),
        )
        .arg(
            Arg::with_name("no-reset")
                .short("n")
                .help("prevents leftwm from restarting after applying new theme")
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("check")
                .about("checks whether a theme is installed correctly")
                .version(version),
        )
        .subcommand(
            SubCommand::with_name("install")
                .about("downloads a theme")
                .version(version)
                .arg(
                    Arg::with_name("Name")
                        .help("The name of the theme or the Git repository containing the theme")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("upgrade")
                .about("fetches updates to all themes")
                .version(version)
                .arg(
                    Arg::with_name("UTHEME")
                        .help("the name of the theme to update")
                        .required(false)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("apply")
                .about("applies a theme and then restarts LeftWM")
                .version(version)
                .arg(
                    Arg::with_name("THEME")
                        .help("the name of the theme to apply as your current theme")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("fetches LeftWM themes from provided sources and searches for new ones")
                .version(version)
                .arg(
                    Arg::with_name("cache")
                        .short("c")
                        .help("search in cache only, don't fetch a new list"),
                )                .arg(
                    Arg::with_name("TERM")
                        .help("retrieves only matching themes (still downloads list if not -c flagged)")
                        .required(false)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("list")
                .about("lists installed LeftWM themes")
                .version(version),
        )
        .subcommand(
            SubCommand::with_name("uninstall")
                .about("uninstalls an installed LeftWM theme")
                .version(version)
                .arg(
                    Arg::with_name("TNAME")
                        .help("The name of the theme or the Git repository containing the theme")
                        .required(true)
                        .index(1),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("creates a new theme")
                .version(version)
                .arg(
                    Arg::with_name("THEME_NAME")
                        .help("The name of the theme or the Git repository containing the theme")
                        .required(true)
                        .index(1),
                ),
        )
        .get_matches();

    if matches.value_of("debug").is_some() {
        log::set_max_level(log::LevelFilter::Trace);
    }

    match matches.subcommand() {
        ("check", Some(_sub_m)) => {
            dbg!("Not yet implemented");
        }
        ("install", Some(sub_m)) => {
            dofn(sub_m, &install);
        }
        ("upgrade", Some(sub_m)) => {
            dofn(sub_m, &upgrade);
        }
        ("apply", Some(sub_m)) => {
            dofn(sub_m, &apply);
        }
        ("new", Some(sub_m)) => {
            dofn(sub_m, &new);
        }
        ("update", Some(sub_m)) => {
            dofn(sub_m, &update);
        }
        ("list", Some(sub_m)) => {
            dofn(sub_m, &list);
        }
        ("uninstall", Some(sub_m)) => {
            dofn(sub_m, &uninstall);
        }
        _ => {
            error!("No operation specified (use -h for help)");
        }
    }
}

fn dofn(args: &ArgMatches, f: &dyn Fn(&ArgMatches) -> Result<(), errors::LeftError>) {
    trace!("{}", "Running function . . . ");
    match f(args) {
        Ok(_) => {
            info!("Completed successfully, exiting . . .");
        }
        Err(e) => {
            trace!("{:?}", e);
            error!("\nDid not complete successfully. Exiting.");
        }
    }
}
