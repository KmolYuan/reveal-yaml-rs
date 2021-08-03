use crate::*;
use actix_files::Files;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer};
use std::{
    env::set_current_dir,
    fs::{canonicalize, read_to_string},
    io::{Error, ErrorKind, Result},
    path::Path,
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
}

/// Launch function.
pub async fn serve<P>(port: u16, path: P, project: &str, use_cache: bool) -> Result<()>
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
    println!("Press Ctrl+C to close the server ...");
    let assets = listdir(".")?;
    let cache = Cache {
        project: project.to_string(),
        doc: if use_cache {
            loader(&read_to_string(project)?, "/static/")?
        } else {
            String::new()
        },
        help_doc: loader(HELP_DOC, "/static/")?,
    };
    HttpServer::new(move || {
        let mut app = App::new()
            .data(cache.clone())
            .service(site::index)
            .service(site::help_page)
            .service(site::icon)
            .service(site::watermark)
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

    #[get("/")]
    pub(super) async fn index(data: Data<Cache>) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(if data.doc.is_empty() {
                loader(&read_to_string(&data.project)?, "/static/")?
            } else {
                data.doc.clone()
            }))
    }

    #[get("/help/")]
    pub(super) async fn help_page(data: Data<Cache>) -> Result<HttpResponse> {
        Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(data.help_doc.clone()))
    }

    #[get("/help/icon.png")]
    pub(super) async fn icon() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().content_type("image/png").body(ICON))
    }

    #[get("/help/watermark.png")]
    pub(super) async fn watermark() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().content_type("image/png").body(WATERMARK))
    }
}
