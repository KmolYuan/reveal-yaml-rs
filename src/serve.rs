use crate::*;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::{
    env::set_current_dir,
    fs::{canonicalize, create_dir_all, read_to_string, File},
    io::{Result, Write},
    path::Path,
};
use temp_dir::TempDir;

pub(crate) const ROOT: &str = "reveal.yaml";
const WATERMARK_PATH: &str = "img/watermark.png";
const ICON_PATH: &str = "img/icon.png";
const WATERMARK: &[u8] = include_bytes!("assets/img/watermark.png");
const ICON: &[u8] = include_bytes!("assets/img/icon.png");
const BLANK_DOC: &[u8] = include_bytes!("assets/blank.yaml");
const HELP_DOC: &str = include_str!("assets/reveal.yaml");

/// Create new project.
pub fn new_project<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let path_str = path.join("img");
    match create_dir_all(&path_str) {
        Ok(_) => println!("Create directory: {}", path_str.to_str().unwrap()),
        Err(_) => println!("Directory exist: {}", path_str.to_str().unwrap()),
    }
    for (data_path, content) in &[
        (ROOT, BLANK_DOC),
        (WATERMARK_PATH, WATERMARK),
        (ICON_PATH, ICON),
    ] {
        let mut f = File::create(path.join(data_path))?;
        f.write(content)?;
    }
    Ok(())
}

#[get("/help/")]
async fn help_page() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(loader(HELP_DOC, "/static/")?))
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let yaml = match read_to_string(ROOT) {
        Ok(s) => s,
        Err(_) => return err!("can not found reveal.yaml file"),
    };
    Ok(HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(loader(&yaml, "/static/")?))
}

/// Launch function.
pub async fn launch<P>(port: u16, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    set_current_dir(path.as_ref())?;
    let temp = match TempDir::new() {
        Ok(v) => v,
        Err(s) => return err!(s),
    };
    // Expand Reveal.js
    extract(temp.path())?;
    // Start server
    let archive = temp.path().join(ARCHIVE);
    println!("Serve at: http://localhost:{}/", port);
    println!("Global archive at: {}", archive.to_str().unwrap());
    println!("Local assets at: {}", canonicalize(".")?.to_str().unwrap());
    let assets = listdir(".")?;
    HttpServer::new(move || {
        let mut app = App::new()
            .service(index)
            .service(help_page)
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
