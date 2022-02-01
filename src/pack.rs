use crate::{
    loader::loader,
    serve::{ICON, WATERMARK},
    update::{archive, update},
};
use std::{
    env::{current_exe, set_current_dir},
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
        if !dist.is_dir() {
            create_dir(dist)?;
        }
        let file_name = path.file_name().unwrap();
        let dist = dist.join(file_name);
        println!("{:?} > {:?}", &path, &dist);
        if path.is_dir() {
            copy_dir(&path, dist)?;
        } else if path.is_file() {
            copy(path, dist)?;
        }
    }
    Ok(())
}

pub(crate) async fn extract<D>(d: D) -> Result<()>
where
    D: AsRef<Path>,
{
    let path = current_exe()?.with_file_name(concat!(archive!(), ".zip"));
    if !path.exists() {
        update().await?;
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
        let path = entry?.path();
        if !path.file_name().unwrap().to_str().unwrap().starts_with('.') {
            list.push(path);
        }
    }
    Ok(list)
}

/// Pack project to an archive.
pub async fn pack<P, D>(path: P, dist: D, project: &str) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    set_current_dir(path.as_ref())?;
    let dist = dist.as_ref();
    if dist.is_dir() {
        println!("Remove {:?}", dist);
        remove_dir_all(dist)?;
    }
    extract(".").await?;
    pack_inner(project).map_err(|e| {
        remove_dir_all(archive!()).unwrap_or_default();
        e
    })?;
    rename(archive!(), dist)?;
    println!("Done");
    Ok(())
}

fn pack_inner(project: &str) -> Result<()> {
    let archive = Path::new(archive!());
    let contents = loader(&read_to_string(project)?, "", false)?;
    write(archive.join("index.html"), &contents)?;
    for assets in listdir(".")? {
        let name = assets.file_name().unwrap().to_str().unwrap();
        if name == archive!() || name.starts_with('.') {
            continue;
        }
        if assets.is_dir() {
            let dist = archive.join(name);
            if !dist.is_dir() {
                create_dir(&dist)?;
            }
            println!("{:?} > {:?}", &assets, &dist);
            copy_dir(assets, dist)?;
        }
    }
    // Use help resources
    for (pat, path, data) in [
        ("help/icon.png", "icon.png", ICON),
        ("help/watermark.png", "watermark.png", WATERMARK),
    ] {
        if contents.contains(pat) {
            let folder = archive.join("help");
            let dist = folder.join(path);
            if !folder.is_dir() {
                create_dir(folder)?;
            }
            println!("{} > {:?}", pat, &dist);
            write(dist, data)?;
        }
    }
    Ok(())
}
