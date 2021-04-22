use clap::{clap_app, AppSettings};

fn main() {
    let args = clap_app! {
        ("Reveal.yaml Manager") =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (setting: AppSettings::ArgRequiredElseHelp)
        (@subcommand init =>
            (alias: "new")
            (about: "Create a new project")
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
    if let Some(cmd) = args.subcommand_matches("new") {
    } else if let Some(cmd) = args.subcommand_matches("serve") {
    } else if let Some(cmd) = args.subcommand_matches("help") {
    }
}
