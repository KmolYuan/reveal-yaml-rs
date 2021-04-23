use clap::{clap_app, AppSettings};
use std::{
    fs::{canonicalize, create_dir},
    io::Write,
    path::Path,
};

const REVEAL: &str = "https://github.com/hakimel/reveal.js/archive/master.zip";
const RESOURCE: &str = "reveal.zip";

fn download(url: &str) {
    let b = reqwest::blocking::get(url).unwrap().bytes().unwrap();
    let mut path = std::env::current_dir().unwrap();
    path.push(RESOURCE);
    let mut f = std::fs::File::create(path).unwrap();
    f.write(b.to_vec().as_slice()).unwrap();
}

fn main() {
    let args = clap_app! {
        ("Reveal.yaml Manager") =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (setting: AppSettings::ArgRequiredElseHelp)
        (@subcommand update =>
            (alias: "upgrade")
            (about: "Download the Reveal.js resources")
        )
        (@subcommand new =>
            (alias: "init")
            (about: "Create a new project")
            (@arg DIR: +required "Project dir")
        )
        (@subcommand serve =>
            (about: "Serve the current project")
            (@arg PORT: --port +takes_value "Set port")
            (@arg DIR: -c +takes_value "Set current path")
        )
        (@subcommand pack =>
            (about: "Pack the current project")
            (@arg DIR: -c +takes_value "Set current path")
        )
        (@subcommand help =>
            (about: "Serve the documentation")
            (@arg PORT: --port +takes_value "Set port")
        )
    }
    .get_matches();
    if let Some(_) = args.subcommand_matches("update") {
        download(REVEAL);
    } else if let Some(cmd) = args.subcommand_matches("new") {
        let mut path = canonicalize(Path::new(cmd.value_of("DIR").unwrap())).unwrap();
        path.push("img");
        let path_str = path.to_str().unwrap();
        match create_dir(&path) {
            Ok(_) => println!("Create directory: {}", path_str),
            Err(_) => println!("Directory exist: {}", path_str),
        }
    } else if let Some(cmd) = args.subcommand_matches("serve") {
    } else if let Some(cmd) = args.subcommand_matches("help") {
    }
}
