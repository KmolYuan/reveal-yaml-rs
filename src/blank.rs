use std::{
    fs::File,
    io::{Result, Write},
    path::Path,
};

pub const ROOT: &str = "reveal.yaml";
const BLANK_DOC: &[u8] = include_bytes!("assets/blank.yaml");

/// Create new project.
pub fn blank<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().join(ROOT);
    File::create(&path)?.write(BLANK_DOC)?;
    println!("Create {}", path.to_str().unwrap());
    Ok(())
}
