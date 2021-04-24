use crate::loader::loader;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::{
    env::current_exe,
    fs::{canonicalize, create_dir, File},
    io::{Read, Result, Write},
    path::Path,
    path::PathBuf,
};
use temp_dir::TempDir;

const ROOT: &str = "reveal.yaml";
const WATERMARK_PATH: &str = "img/watermark.png";
const ICON_PATH: &str = "img/icon.png";
const WATERMARK: &[u8] = include_bytes!("../assets/img/watermark.png");
const ICON: &[u8] = include_bytes!("../assets/img/icon.png");
const BLANK_DOC: &[u8] = include_bytes!("../assets/blank.yaml");
const HELP_DOC: &str = include_str!("../assets/reveal.yaml");
const REVEAL: &str = "https://github.com/hakimel/reveal.js/archive/master.zip";
thread_local! {
    static RESOURCE: PathBuf = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("reveal.js-master.zip");
}

pub(crate) fn update() -> Result<()> {
    let b = reqwest::blocking::get(REVEAL).unwrap().bytes().unwrap();
    println!("Download archive: {}", REVEAL);
    RESOURCE.with(|path| -> Result<()> {
        let mut f = File::create(path)?;
        f.write(b.as_ref())?;
        Ok(())
    })?;
    println!("Done");
    Ok(())
}

pub(crate) async fn new_project(path: &str) -> Result<()> {
    let path = canonicalize(Path::new(path))?;
    let path_str = path.join("img");
    match create_dir(&path_str) {
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

#[get("/help")]
async fn help_page() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(loader(String::from(HELP_DOC))))
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    let mut buf = String::new();
    {
        let mut f = File::open(ROOT)?;
        f.read_to_string(&mut buf)?;
    }
    Ok(HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(loader(buf)))
}

pub(crate) async fn launch(port: u16, path: &str) -> Result<()> {
    let path = canonicalize(Path::new(path))?;
    let d = TempDir::new().unwrap();
    // Expand Reveal.js
    RESOURCE.with(|path| {
        if path.exists() {
            zip::read::ZipArchive::new(File::open(path).unwrap())
                .unwrap()
                .extract(d.path())
                .unwrap();
        } else {
            panic!("Archive not exist, please update first");
        }
    });
    // Start server
    let assets = d.path().join("reveal.js-master");
    let assets = assets.into_os_string().into_string().unwrap();
    let path = path.into_os_string().into_string().unwrap();
    println!("Serve at: http://localhost:{}/", port);
    println!("Local assets at: {}", path.as_str());
    println!("Global assets at: {}", assets.as_str());
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(help_page)
            .service(Files::new("/", path.as_str()))
            .service(Files::new("/", assets.as_str()))
    })
    .bind(("localhost", port))?
    .run()
    .await
}
