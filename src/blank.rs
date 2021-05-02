use crate::*;
use std::{
    fs::File,
    io::{Result, Write},
    path::Path,
};

const BLANK_DOC: &[u8] = include_bytes!("assets/blank.yaml");

/// Create new project.
pub fn blank<P>(path: P) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let mut f = File::create(path.join(ROOT))?;
    f.write(BLANK_DOC)?;
    Ok(())
}
