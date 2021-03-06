use clap::Parser;
use reveal_yaml::*;
use std::{io::Error, path::PathBuf};

#[derive(Parser)]
#[clap(
    name = "Reveal.yaml Manager",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
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
        dir: PathBuf,
    },
    /// Create a new project from an existing directory
    Init {
        /// Project dir
        #[clap(default_value = ".")]
        dir: PathBuf,
    },
    /// Serve the current project
    Serve {
        /// Project dir
        #[clap(default_value = ".")]
        dir: PathBuf,
        /// Port number
        #[clap(long, default_value = "8080")]
        port: u16,
        /// Project filename
        #[clap(short, long, default_value = ROOT)]
        name: String,
        /// Edit mode, watch the state of the project
        #[clap(short, long)]
        edit: bool,
        /// Do not open the browser.
        #[clap(long)]
        no_open: bool,
    },
    /// Format the current project
    Fmt {
        /// Project dir
        #[clap(default_value = ".")]
        dir: PathBuf,
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
        dir: PathBuf,
        /// Project filename
        #[clap(short, long, default_value = ROOT)]
        name: String,
        /// Output dir
        #[clap(short, long, default_value = "./package")]
        out: String,
    },
}

fn main() -> Result<(), Error> {
    match Entry::parse().subcommand {
        Subcommand::Update => update(),
        Subcommand::New { dir } => blank(dir, true),
        Subcommand::Init { dir } => blank(dir, false),
        Subcommand::Serve { dir, port, name, edit, no_open } => {
            serve(port, dir, name, edit, !no_open)
        }
        Subcommand::Fmt { dir, name, dry_run } => fmt(dir, dry_run, &name),
        Subcommand::Pack { dir, name, out } => pack(dir, out, &name),
    }
}
