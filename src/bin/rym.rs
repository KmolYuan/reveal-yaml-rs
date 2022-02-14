use clap::{AppSettings, Parser};
use reveal_yaml::*;
use std::io::Error;

#[derive(Parser)]
#[clap(
    name = "Reveal.yaml Manager",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    setting = AppSettings::ArgRequiredElseHelp
)]
struct Entry {
    #[clap(subcommand)]
    subcommand: Subcommand,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Download the Reveal.js resources
    Update,
    /// Create a new project and its directory
    New {
        /// Project dir
        dir: String,
    },
    /// Create a new project from exist directory
    Init {
        /// Project dir
        #[clap(default_value = ".")]
        dir: String,
    },
    /// Serve the current project
    Serve {
        /// Project dir
        #[clap(default_value = ".")]
        dir: String,
        /// Port number
        #[clap(long, default_value = "8080")]
        port: u16,
        /// Project filename
        #[clap(short, long, default_value = ROOT)]
        name: String,
        /// Edit mode, watch the state of the project
        #[clap(short, long)]
        edit: bool,
        /// Open the browser.
        #[clap(long)]
        open: bool,
    },
    /// Format the current project
    Fmt {
        /// Project dir
        #[clap(default_value = ".")]
        dir: String,
        /// Project filename
        #[clap(short, long, default_value = "ROOT")]
        name: String,
        /// Dry run
        #[clap(short, long)]
        dry_run: bool,
    },
    /// Pack the current project
    Pack {
        /// Project dir
        dir: String,
        /// Project filename
        #[clap(short, long, default_value = ROOT)]
        name: String,
        /// Output dir
        #[clap(short, long, default_value = "./package")]
        out: String,
    },
}

fn main() -> Result<(), Error> {
    let args = Entry::parse();
    match args.subcommand {
        Subcommand::Update => update(),
        Subcommand::New { dir } => blank(dir, true),
        Subcommand::Init { dir } => blank(dir, false),
        Subcommand::Serve {
            dir,
            port,
            name,
            edit,
            open,
        } => serve(port, dir, &name, edit, open),
        Subcommand::Fmt { dir, name, dry_run } => fmt(dir, dry_run, &name),
        Subcommand::Pack { dir, name, out } => pack(dir, out, &name),
    }
}
