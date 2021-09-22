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
            (@arg PROJECT: -n --name +takes_value "Set project name")
            (@arg edit: -e --edit "Edit mode, watch the state of the project")
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
    if args.subcommand_matches("update").is_some() {
        update()
    } else if let Some(cmd) = args.subcommand_matches("new") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        blank(path)
    } else if let Some(cmd) = args.subcommand_matches("serve") {
        let port = cmd.value_of("PORT").unwrap_or("8080");
        let path = cmd.value_of("DIR").unwrap_or(".");
        let project = cmd.value_of("PROJECT").unwrap_or(ROOT);
        let edit = cmd.is_present("edit");
        serve(port.parse().unwrap(), path, project, edit).await
    } else if let Some(cmd) = args.subcommand_matches("fmt") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        let project = cmd.value_of("PROJECT").unwrap_or(ROOT);
        let dry = cmd.is_present("dry");
        fmt(path, dry, project)
    } else if let Some(cmd) = args.subcommand_matches("pack") {
        let path = cmd.value_of("DIR").unwrap_or(".");
        let dist = cmd.value_of("DIST").unwrap_or("./package");
        let project = cmd.value_of("PROJECT").unwrap_or(ROOT);
        pack(path, dist, project)
    } else {
        panic!("invalid input")
    }
}
