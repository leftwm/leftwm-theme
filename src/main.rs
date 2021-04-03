#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod models;
pub mod operations;
pub mod utils;

use colored::Colorize;
use errors::{LeftErrorKind, Result};

use crate::operations::{
    Apply, AutoFind, Install, List, New, Search, Status, Uninstall, Update, Upgrade,
};
use clap::Clap;
use log::error;
use std::env;

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Opt {
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    pub verbose: u8,
    /// Operation to be performed by the theme manager
    #[clap(subcommand)]
    pub operation: Operation,
}

#[derive(Clap, Debug)]
pub enum Operation {
    /// Finds themes not installed by LeftWM-theme
    AutoFind(AutoFind),
    /// Install a theme
    Install(Install),
    /// Uninstall a theme
    Uninstall(Uninstall),
    /// List installed theme(s)
    #[clap(name = "list")]
    List(List),
    /// Create new theme
    New(New),
    /// Update installed themes
    Upgrade(Upgrade),
    /// Update theme list
    Update(Update),
    /// Apply an already installed theme
    Apply(Apply),
    /// Print out current theme information
    Status(Status),
    /// Search for a theme by name
    Search(Search),
}

fn main() {
    let opt = Opt::parse();

    match opt.verbose {
        0 => env::set_var("RUST_LOG", "warn"),
        1 => env::set_var("RUST_LOG", "info"),
        2 => env::set_var("RUST_LOG", "debug"),
        _ => env::set_var("RUST_LOG", "trace"),
    }

    pretty_env_logger::init();
    let wrapper: Result<()> = match opt.operation {
        Operation::AutoFind(args) => AutoFind::exec(&args),
        Operation::Install(args) => Install::exec(&args),
        Operation::Uninstall(args) => Uninstall::exec(&args),
        Operation::List(args) => List::exec(&args),
        Operation::Apply(args) => Apply::exec(&args),
        Operation::Status(args) => Status::exec(&args),
        Operation::New(args) => New::exec(&args),
        Operation::Upgrade(args) => Upgrade::exec(&args),
        Operation::Update(args) => Update::exec(&args),
        Operation::Search(args) => Search::exec(&args),
    };

    if let Err(e) = wrapper {
        match e.inner {
            LeftErrorKind::UserFriendlyError(msg) => println!("{}", &msg.bright_red()),
            _ => error!("Operation did not complete successfully"),
        }
    }
}
