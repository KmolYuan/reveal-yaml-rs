use std::{
    fs::{read_to_string, write},
    io::{Error, ErrorKind, Result},
    path::Path,
};
use yaml_peg::{dump, parse_cyclic, repr::RcRepr};

/// Reformat the project.
pub fn fmt<P>(path: P, dry: bool, project: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().join(project);
    let (yaml, anchor) = parse_cyclic::<RcRepr>(&read_to_string(&path)?)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
    let s = dump(&yaml, &anchor);
    if dry {
        println!("{s}");
        Ok(())
    } else {
        write(path, s)
    }
}
