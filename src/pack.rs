use crate::{
    get_archive,
    loader::loader,
    serve::{ICON, WATERMARK},
    update::{update, ARCHIVE},
};
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

pub(crate) fn extract<D>(d: D) -> Result<()>
where
    D: AsRef<Path>,
{
    let path = get_archive!();
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
pub(crate) fn pack<P, D>(path: P, dist: D, project: &str) -> Result<()>
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
    let archive = Path::new(ARCHIVE);
    extract(".")?;
    pack_inner(archive, project).map_err(|e| {
        remove_dir_all(archive).unwrap_or_default();
        e
    })?;
    rename(archive, dist)?;
    println!("Done");
    Ok(())
}

fn pack_inner(archive: &Path, project: &str) -> Result<()> {
    let contents = loader(&read_to_string(project)?, "", false)?;
    write(archive.join("index.html"), &contents)?;
    for assets in listdir(".")? {
        let name = assets.file_name().unwrap().to_str().unwrap();
        if name == ARCHIVE || name.starts_with('.') {
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
            if !folder.is_dir() {
                create_dir("help")?;
            }
            write(folder.join(path), data)?;
        }
    }
    Ok(())
}
