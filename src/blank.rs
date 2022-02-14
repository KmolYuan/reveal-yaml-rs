use crate::project::StringWrap;
use std::{
    fs::{create_dir, File},
    io::{stdin, stdout, Result, Write},
    path::Path,
};

/// The default root name.
pub const ROOT: &str = "reveal.yaml";
const BLANK_DOC: &str = include_str!("assets/blank.yaml");

/// Create new project.
pub fn blank<P>(path: P, new_dir: bool) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    if new_dir && !path.is_dir() {
        create_dir(path)?;
    }
    File::create(path.join(ROOT))?.write_all(
        BLANK_DOC
            .replace("{%title}", &ask("Title")?)
            .replace("{%author}", &ask("Author")?)
            .replace("{%description}", &ask("Description")?)
            .as_bytes(),
    )?;
    println!("Create project {:?}", path);
    Ok(())
}

fn ask(q: &'static str) -> Result<String> {
    print!("{}: ", q);
    let mut buf = String::new();
    stdout().flush()?;
    stdin().read_line(&mut buf)?;
    Ok(buf.trim_end().escape())
}
