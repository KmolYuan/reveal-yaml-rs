use crate::{
    loader::loader,
    pack::{extract, listdir},
    update::ARCHIVE,
};
use actix_files::Files;
use actix_web::{App, HttpServer};
use std::{
    env::set_current_dir,
    fs::{canonicalize, metadata, read_to_string},
    io::{Error, ErrorKind, Result},
    path::Path,
    sync::Mutex,
    time::SystemTime,
};
use temp_dir::TempDir;

const WATERMARK: &[u8] = include_bytes!("assets/help/watermark.png");
const ICON: &[u8] = include_bytes!("assets/help/icon.png");
const HELP_DOC: &str = include_str!("assets/reveal.yaml");

#[derive(Clone)]
struct Cache {
    project: String,
    doc: String,
    help_doc: String,
    reload: bool,
    last_modified: SystemTime,
}

/// Launch function.
pub(crate) async fn serve<P>(port: u16, path: P, project: &str, edit: bool) -> Result<()>
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
    let cache = Cache {
        project: project.to_string(),
        doc: if edit {
            String::new()
        } else {
            loader(&read_to_string(project)?, "/static/", edit)?
        },
        help_doc: loader(HELP_DOC, "/static/", false)?,
        reload: edit,
        last_modified: metadata(project)?.modified()?,
    };
    HttpServer::new(move || {
        let mut app = App::new()
            .data(Mutex::new(cache.clone()))
            .service(site::index)
            .service(site::help_page)
            .service(site::icon)
            .service(site::watermark)
            .service(site::check_project)
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

mod site {
    use super::*;
    use actix_web::{get, web::Data, HttpResponse};
    use std::collections::BTreeMap;

    #[get("/")]
    pub(super) async fn index(data: Data<Mutex<Cache>>) -> Result<HttpResponse> {
        let data = data.lock().unwrap();
        Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(if data.doc.is_empty() {
                loader(&read_to_string(&data.project)?, "/static/", data.reload)?
            } else {
                data.doc.clone()
            }))
    }

    #[get("/help/")]
    pub(super) async fn help_page(data: Data<Mutex<Cache>>) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(data.lock().unwrap().help_doc.clone()))
    }

    #[get("/help/icon.png")]
    pub(super) async fn icon() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().content_type("image/png").body(ICON))
    }

    #[get("/help/watermark.png")]
    pub(super) async fn watermark() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().content_type("image/png").body(WATERMARK))
    }

    #[get("/changed/")]
    pub(super) async fn check_project(data: Data<Mutex<Cache>>) -> Result<HttpResponse> {
        let mut data = data.lock().unwrap();
        let last = metadata(&data.project)?.modified()?;
        let modified = data.last_modified != last;
        if modified {
            data.last_modified = last;
        }
        let mut data = BTreeMap::new();
        data.insert("modified", modified);
        Ok(HttpResponse::Ok().json(data))
    }
}
