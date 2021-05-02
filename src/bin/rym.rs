use clap::{clap_app, AppSettings};
use reveal_yaml::*;

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
        )
        (@subcommand fmt =>
            (about: "Format the current project")
            (@arg DIR: "Project dir")
            (@arg dry: --dry "Dry run")
        )
        (@subcommand pack =>
            (about: "Pack the current project")
            (@arg DIR: "Project dir")
            (@arg DIST: -o "Output dir")
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
        launch(port.parse().unwrap(), path).await?;
    } else if let Some(cmd) = args.subcommand_matches("fmt") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        let dry = cmd.is_present("dry");
        fmt(path, dry)?;
    } else if let Some(cmd) = args.subcommand_matches("pack") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        let dist = cmd.value_of("DIST").unwrap_or("./package");
        pack(path, dist)?;
    }
    Ok(())
}
