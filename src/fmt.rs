use std::{
    fs::{read_to_string, write},
    io::{Error, ErrorKind, Result},
    path::Path,
};
use yaml_peg::{dump, parse, repr::RcRepr};

/// Reformat the project.
pub fn fmt<P: AsRef<Path>>(path: P, dry: bool, project: &str) -> Result<()> {
    let path = path.as_ref().join(project);
    let (yaml, _) = match parse::<RcRepr>(&read_to_string(&path)?) {
        Ok(v) => v,
        Err(s) => return Err(Error::new(ErrorKind::InvalidData, s)),
    };
    let s = dump(&yaml);
    if dry {
        println!("{}", s);
    } else {
        write(path, s)?;
    }
    Ok(())
}
