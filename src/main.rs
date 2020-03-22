use clap::Clap;


#[derive(Clap, Debug)]
#[clap(author, about, version)]
struct Opt{
    /// Verbose mode (-v, -vv, -vvv, etc.)
    #[clap(short, long, parse(from_occurrences))]
    verbose: u8,
    /// Operation to be performed by the theme manager
    #[clap(subcommand)]
    operation: Operation,
}

#[derive(Clap, Debug)]
enum Operation {
    /// Install a theme
    Install(Install),
    /// Uninstall a theme
    Uninstall(Uninstall),
    /// List installed theme(s)
    #[clap(name="ls")]
    List(List),
    /// Apply an already installed theme
    Apply(Apply),
    /// Revert back to previous theme
    Undo(Undo),
    /// Print out current theme information
    Status(Status),
}

#[derive(Clap, Debug)]
struct Install {
    /// Read theme from git repository
    #[clap(short = "g", long)]
    git: bool,

    /// Location of theme
    name: String
}

#[derive(Clap, Debug)]
struct Uninstall {
    name: String
}

#[derive(Clap, Debug)]
struct Apply {
    name: String
}

#[derive(Clap, Debug)]
struct Undo {
}

#[derive(Clap, Debug)]
struct Status {
}

#[derive(Clap, Debug)]
struct List {
}


fn main() {
    let opt = Opt::parse();
    println!("{:?}", opt);
}
