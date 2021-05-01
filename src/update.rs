use reqwest::blocking::get;
use std::{
    env::current_exe,
    fs::File,
    io::{Cursor, Result},
    path::PathBuf,
};
use zip::{ZipArchive, ZipWriter};

const REVEAL: &str = "https://github.com/hakimel/reveal.js/archive/master.zip";
pub(crate) const ARCHIVE: &str = "reveal.js-master";
thread_local! {
    pub(crate) static RESOURCE: PathBuf = current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!("{}.zip", ARCHIVE));
}

/// Download the archive from Reveal.js repository.
pub fn update() -> Result<()> {
    let b = get(REVEAL).unwrap().bytes().unwrap();
    println!("Download archive: {}", REVEAL);
    let archive = RESOURCE.with(|path| path.to_path_buf());
    let mut r = ZipArchive::new(Cursor::new(b)).unwrap();
    let mut w = ZipWriter::new(File::create(&archive)?);
    let dist = format!("{}/dist/", ARCHIVE);
    let plugin = format!("{}/plugin/", ARCHIVE);
    for i in 0..r.len() {
        let file = r.by_index(i).unwrap();
        if file.is_dir() {
            continue;
        }
        let name = file.name();
        if name.starts_with(&dist) || name.starts_with(&plugin) {
            w.raw_copy_file(file).unwrap();
        }
    }
    w.finish().unwrap();
    println!("Done");
    Ok(())
}
