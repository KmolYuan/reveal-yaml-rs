use crate::*;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::{
    env::set_current_dir,
    fs::{canonicalize, read_to_string},
    io::{Error, ErrorKind, Result},
    path::Path,
};
use temp_dir::TempDir;

pub(crate) const ROOT: &str = "reveal.yaml";
const WATERMARK: &[u8] = include_bytes!("assets/help/watermark.png");
const ICON: &[u8] = include_bytes!("assets/help/icon.png");
const HELP_DOC: &str = include_str!("assets/reveal.yaml");

/// Launch function.
pub async fn launch<P>(port: u16, path: P) -> Result<()>
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
    println!("Global archive at: {}", archive.to_str().unwrap());
    println!("Local assets at: {}", canonicalize(".")?.to_str().unwrap());
    println!("Press Ctrl+C to close the server ...");
    let assets = listdir(".")?;
    HttpServer::new(move || {
        let mut app = App::new()
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
    pub(super) async fn index() -> Result<HttpResponse> {
        let yaml = read_to_string(ROOT)?;
        Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(loader(&yaml, "/static/")?))
    }

    #[get("/help/")]
    pub(super) async fn help_page() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok()
            .content_type("text/html;charset=utf-8")
            .body(loader(HELP_DOC, "/static/")?))
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
