use std::{
    fs::{read_to_string, write},
    io::{Error, ErrorKind, Result},
    path::Path,
};
use yaml_peg::{dump, parse, repr::RcRepr};

/// Reformat the project.
pub fn fmt<P>(path: P, dry: bool, project: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().join(project);
    let (yaml, _) = parse::<RcRepr>(&read_to_string(&path)?)
        .map_err(|s| Error::new(ErrorKind::InvalidData, s))?;
    let s = dump(&yaml);
    if dry {
        println!("{}", s);
    } else {
        write(path, s)?;
    }
    Ok(())
}
