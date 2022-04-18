use std::io::{Cursor, Result};
use zip::{ZipArchive, ZipWriter};

macro_rules! archive {
    () => {
        "reveal.js-master"
    };
}

macro_rules! reveal_url {
    () => {
        "https://github.com/hakimel/reveal.js/archive/master.zip"
    };
}

pub(crate) use archive;

/// Download the archive from Reveal.js repository.
pub fn update() -> Result<()> {
    println!(concat!("Downloading archive from ", reveal_url!()));
    let b = actix_web::rt::System::new().block_on(async {
        reqwest::get(reveal_url!())
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap()
    });
    let archive = std::env::current_exe()?.with_file_name(concat!(archive!(), ".zip"));
    let mut r = ZipArchive::new(Cursor::new(b))?;
    let mut w = ZipWriter::new(std::fs::File::create(archive)?);
    for i in 0..r.len() {
        let file = r.by_index(i)?;
        if file.is_dir() {
            continue;
        }
        let name = file.name();
        if name.starts_with(concat!(archive!(), "/dist/"))
            || name.starts_with(concat!(archive!(), "/plugin/"))
        {
            w.raw_copy_file(file)?;
        }
    }
    w.finish()?;
    println!("Done");
    Ok(())
}
