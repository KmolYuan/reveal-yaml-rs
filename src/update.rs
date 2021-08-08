use reqwest::blocking::get;
use std::{
    fs::File,
    io::{Cursor, Result},
};
use zip::{ZipArchive, ZipWriter};

const REVEAL: &str = "https://github.com/hakimel/reveal.js/archive/master.zip";
pub(crate) const ARCHIVE: &str = "reveal.js-master";

/// Download the archive from Reveal.js repository.
pub(crate) fn update() -> Result<()> {
    let b = get(REVEAL).unwrap().bytes().unwrap();
    println!("Download archive: {}", REVEAL);
    let archive = get_archive!();
    let mut r = ZipArchive::new(Cursor::new(b))?;
    let mut w = ZipWriter::new(File::create(archive)?);
    let dist = format!("{}/dist/", ARCHIVE);
    let plugin = format!("{}/plugin/", ARCHIVE);
    for i in 0..r.len() {
        let file = r.by_index(i)?;
        if file.is_dir() {
            continue;
        }
        let name = file.name();
        if name.starts_with(&dist) || name.starts_with(&plugin) {
            w.raw_copy_file(file)?;
        }
    }
    w.finish()?;
    println!("Done");
    Ok(())
}
