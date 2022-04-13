use self::edit_mode::Monitor;
use crate::{
    pack::{extract, listdir},
    project::load,
    update::archive,
};
use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};
use std::{
    env::set_current_dir,
    fs::{canonicalize, read_to_string},
    io::{Error, ErrorKind, Result},
    path::Path,
};
use temp_dir::TempDir;

mod edit_mode;
mod site;

const HELP_DOC: &str = include_str!("../assets/reveal.yaml");

#[derive(Clone)]
struct Cache {
    project: String,
    doc: String,
    help_doc: String,
    reload: bool,
}

/// Launch function.
pub fn serve<P>(port: u16, path: P, project: &str, edit: bool, open: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    set_current_dir(path.as_ref())?;
    let temp = TempDir::new().map_err(|s| Error::new(ErrorKind::PermissionDenied, s))?;
    // Expand Reveal.js
    extract(temp.path())?;
    // Start server
    let archive = temp.path().join(archive!());
    println!("Serve at: http://localhost:{}/", port);
    println!("Global archive at: {:?}", archive);
    println!("Local assets at: {:?}", canonicalize(".")?);
    println!("Edit mode: {}", edit);
    println!("Press Ctrl+C to close the server...");
    let assets = listdir(".")?;
    let cache = Data::new(Cache {
        project: project.to_string(),
        doc: if edit {
            String::new()
        } else {
            load(&read_to_string(project)?, "/static/", edit)?
        },
        help_doc: load(HELP_DOC, "/static/", false)?,
        reload: edit,
    });
    if open {
        webbrowser::open(&format!("http://localhost:{}/", port))?;
    }
    let server = HttpServer::new(move || {
        let app = App::new()
            .app_data(cache.clone())
            .app_data(Data::new(Monitor::new(cache.project.clone())))
            .service(site::index)
            .service(site::help_page)
            .service(edit_mode::ws_index)
            .service(Files::new("/static", &archive));
        assets.iter().fold(app, |app, asset| {
            let name = asset.strip_prefix(".").unwrap_or(asset).to_str().unwrap();
            app.service(Files::new(name, asset))
        })
    })
    .bind(("localhost", port))?
    .run();
    actix_web::rt::System::new().block_on(server)
}
