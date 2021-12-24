use self::edit_mode::ServerMonitor;
use crate::{
    loader::loader,
    pack::{extract, listdir},
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
    let temp = TempDir::new().map_err(|s| Error::new(ErrorKind::PermissionDenied, s))?;
    // Expand Reveal.js
    extract(temp.path()).await?;
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
            loader(&read_to_string(project)?, "/static/", edit)?
        },
        help_doc: loader(HELP_DOC, "/static/", false)?,
        reload: edit,
    });
    HttpServer::new(move || {
        let mut app = App::new()
            .app_data(cache.clone())
            .app_data(Data::new(ServerMonitor::new(cache.project.clone())))
            .service(site::index)
            .service(site::help_page)
            .service(site::icon)
            .service(site::watermark)
            .service(edit_mode::ws_index)
            .service(Files::new("/static", &archive));
        for asset in &assets {
            let name = format!("/{:?}", asset.file_name().unwrap());
            app = app.service(Files::new(&name, asset));
        }
        app
    })
    .bind(("localhost", port))?
    .run()
    .await
}
