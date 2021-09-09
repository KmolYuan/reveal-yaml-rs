use crate::loader::wrap_string::WrapString;
use std::{
    fs::File,
    io::{stdin, stdout, Result, Write},
    path::Path,
};

/// The default root name.
pub const ROOT: &str = "reveal.yaml";
const BLANK_DOC: &str = include_str!("assets/blank.yaml");

/// Create new project.
pub fn blank<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().join(ROOT);
    File::create(&path)?.write_all(
        BLANK_DOC
            .replace("{%title}", &question("Title")?)
            .replace("{%author}", &question("Author")?)
            .replace("{%description}", &question("Description")?)
            .replace("{%hash}", &question_bool("Option - hash [y/N]")?)
            .as_bytes(),
    )?;
    println!("Create {:?}", path);
    Ok(())
}

fn question(q: &'static str) -> Result<String> {
    print!("{}: ", q);
    let mut buf = String::new();
    stdout().flush()?;
    stdin().read_line(&mut buf)?;
    Ok(buf.trim_end().escape())
}

fn question_bool(q: &'static str) -> Result<String> {
    Ok((question(q)? == "y").to_string())
}
