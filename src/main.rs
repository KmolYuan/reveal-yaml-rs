macro_rules! get_archive {
    () => {{
        use std::env::current_exe;
        let mut path = current_exe()?.with_file_name(ARCHIVE);
        path.set_extension("zip");
        path
    }};
}

pub use crate::blank::*;
pub use crate::fmt::*;
pub use crate::loader::*;
pub use crate::pack::*;
pub use crate::serve::*;
pub use crate::update::*;
use clap::{clap_app, AppSettings};

mod blank;
mod fmt;
mod loader;
mod pack;
mod serve;
mod update;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = clap_app! {
        ("Reveal.yaml Manager") =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (setting: AppSettings::ArgRequiredElseHelp)
        (@subcommand update =>
            (about: "Download the Reveal.js resources")
        )
        (@subcommand new =>
            (alias: "init")
            (about: "Create a new project")
            (@arg DIR: "Project dir")
        )
        (@subcommand serve =>
            (about: "Serve the current project")
            (@arg DIR: "Project dir")
            (@arg PORT: --port +takes_value "Set port")
            (@arg PROJECT: -n --name +takes_value "Set project name")
            (@arg use_cache: --cache "Parse once and cached project")
        )
        (@subcommand fmt =>
            (about: "Format the current project")
            (@arg DIR: "Project dir")
            (@arg PROJECT: -n --name +takes_value "Set project name")
            (@arg dry: --dry "Dry run")
        )
        (@subcommand pack =>
            (about: "Pack the current project")
            (@arg DIR: "Project dir")
            (@arg DIST: -o "Output dir")
            (@arg PROJECT: -n --name +takes_value "Set project name")
        )
    }
    .get_matches();
    if let Some(_) = args.subcommand_matches("update") {
        update()?;
    } else if let Some(cmd) = args.subcommand_matches("new") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        blank(path)?;
    } else if let Some(cmd) = args.subcommand_matches("serve") {
        let port = cmd.value_of("PORT").unwrap_or("8080");
        let path = cmd.value_of("DIR").unwrap_or(".");
        let project = cmd.value_of("PROJECT").unwrap_or(ROOT);
        let use_cache = cmd.is_present("use_cache");
        serve(port.parse().unwrap(), path, project, use_cache).await?;
    } else if let Some(cmd) = args.subcommand_matches("fmt") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        let project = cmd.value_of("PROJECT").unwrap_or(ROOT);
        let dry = cmd.is_present("dry");
        fmt(path, dry, project)?;
    } else if let Some(cmd) = args.subcommand_matches("pack") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        let dist = cmd.value_of("DIST").unwrap_or("./package");
        let project = cmd.value_of("PROJECT").unwrap_or(ROOT);
        pack(path, dist, project)?;
    }
    Ok(())
}