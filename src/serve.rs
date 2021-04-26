use crate::loader::loader;
use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::{
    env::{current_exe, set_current_dir},
    fs::{
        canonicalize, copy, create_dir, read_dir, read_to_string, remove_dir_all, remove_file,
        rename, write, File,
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

macro_rules! path_string {
    ($v:expr) => {
        $v.into_os_string().into_string().unwrap()
    };
}

fn extract<D>(d: D) -> Result<()>
where
    D: AsRef<Path>,
{
    RESOURCE.with(|path| -> Result<()> {
        if !path.exists() {
            update()?;
        }
        ZipArchive::new(File::open(path).unwrap())
            .unwrap()
            .extract(d.as_ref())
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
pub fn new_project<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
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
            println!("{:?} > {:?}", &path, dist);
            copy(&path, dist)?;
        }
    }
    Ok(())
}

fn listdir<P>(path: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut list = Vec::new();
    for entry in read_dir(path)? {
        list.push(entry?.path());
    }
    Ok(list)
}

pub fn pack<P, D>(path: P, dist: D) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    let path = path.as_ref();
    set_current_dir(path)?;
    let dist = dist.as_ref();
    if dist.is_dir() {
        println!("Remove {:?}", &dist);
        remove_dir_all(&dist)?;
    }
    let archive = format!("./{}", ARCHIVE);
    let archive = Path::new(&archive);
    extract(".")?;
    for e in read_dir(&archive)? {
        let path = e?.path();
        if path.is_file() {
            remove_file(path)?;
        } else if [".github", "examples", "test"]
            .contains(&path.file_name().unwrap().to_str().unwrap())
        {
            remove_dir_all(path)?;
        }
    }
    write(
        archive.join("index.html"),
        loader(&read_to_string(ROOT)?, "")?,
    )?;
    for assets in listdir(".")? {
        if assets == archive {
            continue;
        }
        if assets.is_dir() {
            let dist = archive.join(assets.file_name().unwrap().to_os_string());
            create_dir(&dist)?;
            copy_dir(&assets, dist)?;
        }
    }
    rename(archive, dist)?;
    println!("Done");
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
    Ok(HttpResponse::Ok()
        .content_type("text/html;charset=utf-8")
        .body(loader(&read_to_string(ROOT)?, "/static/")?))
}

/// Launch function.
pub async fn launch<P>(port: u16, path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    set_current_dir(path)?;
    let assets = listdir(path)?;
    let d = TempDir::new().unwrap();
    // Expand Reveal.js
    extract(d.path())?;
    // Start server
    let archive = d.path().join(ARCHIVE);
    let archive = path_string!(archive);
    println!("Serve at: http://localhost:{}/", port);
    println!("Global assets at: {}", archive.as_str());
    println!("Local assets at: {}", path_string!(canonicalize(path)?));
    HttpServer::new(move || {
        let mut app = App::new()
            .service(index)
            .service(help_page)
            .service(Files::new("/static", archive.as_str()));
        for asset in &assets {
            let name = format!(
                "/{}",
                asset
                    .file_name()
                    .unwrap()
                    .to_os_string()
                    .into_string()
                    .unwrap()
            );
            app = app.service(Files::new(
                name.as_str(),
                path_string!(asset.clone()).as_str(),
            ));
        }
        app
    })
    .bind(("localhost", port))?
    .run()
    .await
}
