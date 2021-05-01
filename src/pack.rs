use crate::*;
use std::{
    env::set_current_dir,
    fs::{copy, create_dir, read_dir, read_to_string, remove_dir_all, rename, write, File},
    io::Result,
    path::{Path, PathBuf},
};
use zip::ZipArchive;

fn copy_dir<P, D>(path: P, dist: D) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    let path = path.as_ref();
    let dist = dist.as_ref();
    for entry in read_dir(path)? {
        let path = entry?.path();
        let file_name = path.file_name().unwrap();
        if path.is_dir() {
            copy_dir(&path, dist.join(file_name))?;
        } else if path.is_file() {
            let dist = dist.join(file_name);
            println!("{:?} > {:?}", &path, dist);
            copy(&path, dist)?;
        }
    }
    Ok(())
}

pub(crate) fn extract<D>(d: D) -> Result<()>
where
    D: AsRef<Path>,
{
    let path = RESOURCE.with(|path| path.to_path_buf());
    if !path.exists() {
        update()?;
    }
    ZipArchive::new(File::open(path)?)
        .unwrap()
        .extract(d.as_ref())
        .unwrap();
    Ok(())
}

pub(crate) fn listdir<P>(path: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    let mut list = Vec::new();
    for entry in read_dir(path)? {
        list.push(entry?.path());
    }
    Ok(list)
}

/// Pack project to an archive.
pub fn pack<P, D>(path: P, dist: D) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    set_current_dir(path.as_ref())?;
    let dist = dist.as_ref();
    if dist.is_dir() {
        println!("Remove {:?}", &dist);
        remove_dir_all(&dist)?;
    }
    let archive = Path::new(ARCHIVE);
    extract(".")?;
    write(
        archive.join("index.html"),
        loader(&read_to_string(ROOT)?, "")?,
    )?;
    for assets in listdir(".")? {
        if assets.file_name().unwrap() == archive {
            continue;
        }
        if assets.is_dir() {
            let dist = archive.join(assets.file_name().unwrap().to_os_string());
            create_dir(&dist)?;
            copy_dir(&assets, dist)?;
        }
    }
    rename(archive, dist)?;
    println!("Done");
    Ok(())
}
