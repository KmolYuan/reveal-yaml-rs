use std::{
    env::current_exe,
    fs::File,
    io::{Result, Write},
    path::PathBuf,
};

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
