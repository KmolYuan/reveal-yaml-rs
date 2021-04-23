use actix_files::Files;
use actix_web::{get, App, HttpResponse, HttpServer};
use std::{
    env::current_exe,
    fs::{canonicalize, create_dir, File},
    io::Write,
    path::Path,
    path::PathBuf,
};
use temp_dir::TempDir;

const WATERMARK: &[u8] = include_bytes!("../assets/img/watermark.png");
const ICON: &[u8] = include_bytes!("../assets/img/icon.png");
const HELP_DOC: &str = include_str!("../assets/reveal.yaml");
const REVEAL: &str = "https://github.com/hakimel/reveal.js/archive/master.zip";
thread_local! {
    static RESOURCE: PathBuf = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("reveal.js-master.zip");
}

#[get("/help")]
async fn help_page() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body("help!")
}

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(r##"
    <!doctype html>
    <html>
        <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">

            <title>reveal.js</title>

            <link rel="stylesheet" href="dist/reset.css">
            <link rel="stylesheet" href="dist/reveal.css">
            <link rel="stylesheet" href="dist/theme/black.css" id="theme">

            <!-- Theme used for syntax highlighted code -->
            <link rel="stylesheet" href="plugin/highlight/monokai.css" id="highlight-theme">
        </head>
        <body>
            <div class="reveal">
                <div class="slides">
                    <section>Slide 1</section>
                    <section>Slide 2</section>
                </div>
            </div>

            <script src="dist/reveal.js"></script>
            <script src="plugin/notes/notes.js"></script>
            <script src="plugin/markdown/markdown.js"></script>
            <script src="plugin/highlight/highlight.js"></script>
            <script>
                // More info about initialization & config:
                // - https://revealjs.com/initialization/
                // - https://revealjs.com/config/
                Reveal.initialize({
                    hash: true,

                    // Learn about plugins: https://revealjs.com/plugins/
                    plugins: [ RevealMarkdown, RevealHighlight, RevealNotes ]
                });
            </script>
        </body>
    </html>"##)
}

pub(crate) async fn launch(port: u16) -> std::io::Result<()> {
    let d = TempDir::new().unwrap();
    RESOURCE.with(|path| {
        if path.exists() {
            zip::read::ZipArchive::new(File::open(path).unwrap())
                .unwrap()
                .extract(d.path())
                .unwrap()
        } else {
            panic!("Archive not exist, please update first");
        }
    });
    let path = d
        .path()
        .join("reveal.js-master")
        .into_os_string()
        .into_string()
        .unwrap();
    println!("Serve at: http://localhost:{}/", port);
    println!("Local assets at: {}", path.as_str());
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(help_page)
            .service(Files::new("/", path.as_str()).show_files_listing())
    })
    .bind(("localhost", port))?
    .run()
    .await
}

pub(crate) async fn new_project(path: &str) -> std::io::Result<()> {
    let mut path = canonicalize(Path::new(path))?;
    path.push("img");
    let path_str = path.to_str().unwrap();
    match create_dir(&path) {
        Ok(_) => println!("Create directory: {}", path_str),
        Err(_) => println!("Directory exist: {}", path_str),
    }
    Ok(())
}

pub(crate) fn update() -> std::io::Result<()> {
    let b = reqwest::blocking::get(REVEAL).unwrap().bytes().unwrap();
    println!("Download archive: {}", REVEAL);
    RESOURCE.with(|path| {
        let mut f = std::fs::File::create(path).unwrap();
        f.write(b.as_ref()).unwrap();
    });
    println!("Done");
    Ok(())
}
