#![feature(try_trait)]
#[macro_use]
extern crate serde_derive;

pub mod errors;
pub mod models;
pub mod operations;

use crate::operations::{Apply, Install, List, New, Status, Uninstall, Update, Upgrade};
use clap::Clap;

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
}

fn main() -> Result<(), errors::LeftError> {
    pretty_env_logger::init();

    let opt = Opt::parse();

    match opt.verbose {
        0 => log::set_max_level(log::LevelFilter::Warn),
        1 => log::set_max_level(log::LevelFilter::Info),
        2 => log::set_max_level(log::LevelFilter::Debug),
        3 | _ => log::set_max_level(log::LevelFilter::Trace),
    }

    match opt.operation {
        Operation::Install(args) => Install::exec(&args),
        Operation::Uninstall(args) => Uninstall::exec(&args),
        Operation::List(args) => List::exec(&args),
        Operation::Apply(args) => Apply::exec(&args),
        Operation::Status(args) => Status::exec(&args),
        Operation::New(args) => New::exec(&args),
        Operation::Upgrade(args) => Upgrade::exec(&args),
        Operation::Update(args) => Update::exec(&args),
    }
}
