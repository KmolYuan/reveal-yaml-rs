use self::edit_mode::ServerMonitor;
use crate::{
    loader::loader,
    pack::{extract, listdir},
    update::ARCHIVE,
};
use actix_files::Files;
use actix_web::{App, HttpServer};
use std::{
    env::set_current_dir,
    fs::{canonicalize, read_to_string},
    io::{Error, ErrorKind, Result},
    path::Path,
    sync::Arc,
};
use temp_dir::TempDir;

mod edit_mode;
mod site;

pub(crate) const WATERMARK: &[u8] = include_bytes!("../assets/help/watermark.png");
pub(crate) const ICON: &[u8] = include_bytes!("../assets/help/icon.png");
const HELP_DOC: &str = include_str!("../assets/reveal.yaml");

#[derive(Clone)]
struct Cache {
    project: String,
    doc: String,
    help_doc: String,
    reload: bool,
}

/// Launch function.
pub async fn serve<P>(port: u16, path: P, project: &str, edit: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    set_current_dir(path.as_ref())?;
    let temp = match TempDir::new() {
        Ok(v) => v,
        Err(s) => return Err(Error::new(ErrorKind::InvalidData, s)),
    };
    // Expand Reveal.js
    extract(temp.path())?;
    // Start server
    let archive = temp.path().join(ARCHIVE);
    println!("Serve at: http://localhost:{}/", port);
    println!("Global archive at: {:?}", archive);
    println!("Local assets at: {:?}", canonicalize(".")?);
    println!("Edit mode: {}", edit);
    println!("Press Ctrl+C to close the server ...");
    let assets = listdir(".")?;
    let cache = Arc::new(Cache {
        project: project.to_string(),
        doc: if edit {
            String::new()
        } else {
            loader(&read_to_string(project)?, "/static/", edit)?
        },
        help_doc: loader(HELP_DOC, "/static/", false)?,
        reload: edit,
    });
    HttpServer::new(move || {
        let mut app = App::new()
            .data(cache.clone())
            .data(ServerMonitor::new(cache.project.clone()))
            .service(site::index)
            .service(site::help_page)
            .service(site::icon)
            .service(site::watermark)
            .service(edit_mode::ws_index)
            .service(Files::new("/static", &archive));
        for asset in &assets {
            let name = format!("/{}", asset.file_name().unwrap().to_str().unwrap());
            app = app.service(Files::new(&name, asset));
        }
        app
    })
    .bind(("localhost", port))?
    .run()
    .await
}
