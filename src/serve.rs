use crate::loader::loader;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::{
    env::{current_exe, set_current_dir},
    fs::{
        canonicalize, copy, create_dir, read_dir, read_to_string, remove_dir_all, remove_file,
        write, File,
    },
    io::{Result, Write},
    path::{Path, PathBuf},
};
use temp_dir::TempDir;
use zip::ZipArchive;

const ROOT: &str = "reveal.yaml";
const WATERMARK_PATH: &str = "img/watermark.png";
const ICON_PATH: &str = "img/icon.png";
const WATERMARK: &[u8] = include_bytes!("assets/img/watermark.png");
const ICON: &[u8] = include_bytes!("assets/img/icon.png");
const BLANK_DOC: &[u8] = include_bytes!("assets/blank.yaml");
const HELP_DOC: &str = include_str!("assets/reveal.yaml");
const REVEAL: &str = "https://github.com/hakimel/reveal.js/archive/master.zip";
const ARCHIVE: &str = "reveal.js-master";
thread_local! {
    static RESOURCE: PathBuf = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("{}.zip", ARCHIVE));
}

fn extract(d: &Path) -> Result<()> {
    RESOURCE.with(|path| -> Result<()> {
        if !path.exists() {
            update()?;
        }
        ZipArchive::new(File::open(path).unwrap())
            .unwrap()
            .extract(d)
            .unwrap();
        Ok(())
    })
}

/// Download the archive from Reveal.js repository.
pub fn update() -> Result<()> {
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

/// Create new project.
pub fn new_project(path: &str) -> Result<()> {
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

fn copy_dir<P, D>(path: P, dist: D) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    let path = path.as_ref();
    let dist = dist.as_ref();
    for entry in read_dir(path)? {
        let path = entry?.path();
        let file_name = path.file_name().unwrap();
        if path.is_dir() {
            copy_dir(&path, dist.join(file_name))?;
        } else if path.is_file() {
            let dist = dist.join(file_name);
            println!("{:?} > {:?}", &path, &dist);
            copy(&path, dist)?;
        }
    }
    Ok(())
}

pub fn pack(path: &str) -> Result<()> {
    let path = canonicalize(Path::new(path))?;
    let mut dist = path.join(ARCHIVE);
    if dist.is_dir() {
        println!("Remove {:?}", &dist);
        remove_dir_all(&dist)?;
    }
    extract(path.as_path())?;
    for e in read_dir(&dist)? {
        let path = e?.path();
        if path.is_file() {
            remove_file(path)?;
        }
    }
    write(
        &dist.join("index.html"),
        loader(read_to_string(path.join(ROOT))?)?,
    )?;
    dist.push("img");
    create_dir(&dist)?;
    copy_dir(path.join("img"), dist)?;
    println!("Done");
    Ok(())
}

#[get("/help/")]
async fn help_page() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(loader(String::from(HELP_DOC))?))
}

#[get("/")]
async fn index() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(loader(read_to_string(ROOT)?)?))
}

/// Launch function.
pub async fn launch(port: u16, path: &str) -> Result<()> {
    let mut path = canonicalize(Path::new(path))?;
    set_current_dir(&path)?;
    path.push("img");
    let d = TempDir::new().unwrap();
    // Expand Reveal.js
    extract(d.path())?;
    // Start server
    let assets = d.path().join(ARCHIVE);
    let assets = assets.into_os_string().into_string().unwrap();
    let path = path.into_os_string().into_string().unwrap();
    println!("Serve at: http://localhost:{}/", port);
    println!("Global assets at: {}", assets.as_str());
    println!("Local assets at: {}", path.as_str());
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(help_page)
            .service(Files::new("/static", assets.as_str()))
            .service(Files::new("/img", path.as_str()))
    })
    .bind(("localhost", port))?
    .run()
    .await
}
