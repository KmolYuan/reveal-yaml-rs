use self::edit_mode::Monitor;
use crate::{
    pack::{extract, listdir},
    project::{error_page, load, Slides},
    update::archive,
};
use actix_files::Files;
use actix_web::{web, App, HttpServer};
use std::{
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
pub fn serve<P>(port: u16, path: P, project: String, edit: bool, open: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    std::env::set_current_dir(path.as_ref())?;
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
    let cache = web::Data::new(Cache {
        doc: if edit {
            String::new()
        } else {
            load(&read_to_string(&project)?, "/static/", edit).unwrap_or_else(error_page)
        },
        project,
        help_doc: load(HELP_DOC, "/static/", false)?,
        reload: edit,
    });
    let server = HttpServer::new(move || {
        let app = App::new()
            .app_data(cache.clone())
            .app_data(web::Data::new(Monitor::new(cache.project.clone())))
            .service(site::index)
            .service(site::help_page)
            .default_service(web::route().to(site::not_found))
            .service(edit_mode::ws_index)
            .service(Files::new("/static", &archive));
        assets.iter().fold(app, |app, asset| {
            let name = asset.strip_prefix(".").unwrap_or(asset).to_str().unwrap();
            app.service(Files::new(name, asset))
        })
    })
    .bind(("localhost", port))?
    .run();
    if open {
        webbrowser::open(&format!("http://localhost:{}/", port))?;
    }
    actix_web::rt::System::new().block_on(server)
}
