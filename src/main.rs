use crate::serve::{launch, new_project, update};
use clap::{clap_app, AppSettings};

mod loader;
mod serve;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
            (@arg DIR: "Project dir")
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
    }
    .get_matches();
    if let Some(_) = args.subcommand_matches("update") {
        update()?;
    } else if let Some(cmd) = args.subcommand_matches("new") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        new_project(path).await?;
    } else if let Some(cmd) = args.subcommand_matches("serve") {
        let port = cmd.value_of("PORT").unwrap_or("8080");
        let path = cmd.value_of("DIR").unwrap_or(".");
        launch(port.parse().unwrap(), path).await?;
    }
    Ok(())
}
