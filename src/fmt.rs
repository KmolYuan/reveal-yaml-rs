use std::{
    fs::{read_to_string, write},
    io::{Error, ErrorKind, Result},
    path::Path,
};
use yaml_peg::{dump, parser::Loader, repr::RcRepr};

/// Reformat the project.
pub fn fmt<P>(path: P, dry: bool, project: &str) -> Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref().join(project);
    let doc = read_to_string(&path)?;
    let mut loader = Loader::<RcRepr>::new(doc.as_bytes()).keep_anchors(true);
    let yaml = loader
        .parse()
        .map_err(|e| Error::new(ErrorKind::InvalidData, e.to_string()))?;
    let anchor = loader.get_anchors();
    let s = dump(&yaml, &anchor);
    if dry {
        println!("{}", s);
    } else {
        write(path, s)?;
    }
    Ok(())
}
