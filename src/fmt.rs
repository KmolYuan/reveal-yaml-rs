use crate::*;
use std::{
    fs::{read_to_string, write},
    io::Result,
    path::Path,
};
use yaml_peg::{dump, parse};

pub fn fmt<P: AsRef<Path>>(path: P, dry: bool) -> Result<()> {
    let path = path.as_ref().join(ROOT);
    if !path.is_file() {
        return err!("can not found project file");
    }
    let yaml = match parse(&read_to_string(&path)?) {
        Ok(v) => v,
        Err(e) => return err!(e),
    };
    let s = dump(yaml);
    if dry {
        println!("{}", s);
    } else {
        write(path, s)?;
    }
    Ok(())
}
