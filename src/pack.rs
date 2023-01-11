use crate::{
    project::load,
    update::{archive, update},
};
use binstall_zip::ZipArchive;
use std::{
    fs,
    io::Result,
    path::{Path, PathBuf},
};

fn copy_dir<P, D>(path: P, dist: D) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    let path = path.as_ref();
    let dist = dist.as_ref();
    for entry in fs::read_dir(path)? {
        let path = entry?.path();
        if !dist.is_dir() {
            fs::create_dir(dist)?;
        }
        let file_name = path.file_name().unwrap();
        let dist = dist.join(file_name);
        println!("{path:?} > {dist:?}");
        if path.is_dir() {
            copy_dir(&path, dist)?;
        } else if path.is_file() {
            fs::copy(path, dist)?;
        }
    }
    Ok(())
}

pub(crate) fn extract<D>(d: D) -> Result<()>
where
    D: AsRef<Path>,
{
    let path = std::env::current_exe()?.with_file_name(concat!(archive!(), ".zip"));
    if !path.exists() {
        update()?;
    }
    ZipArchive::new(fs::File::open(path)?)
        .unwrap()
        .extract(d.as_ref())
        .unwrap();
    Ok(())
}

pub(crate) fn listdir<P>(path: P) -> Result<Vec<PathBuf>>
where
    P: AsRef<Path>,
{
    fs::read_dir(path)?.map(|e| e.map(|f| f.path())).collect()
}

/// Pack project to an archive.
pub fn pack<P, D>(path: P, dist: D, project: &str) -> Result<()>
where
    P: AsRef<Path>,
    D: AsRef<Path>,
{
    std::env::set_current_dir(path.as_ref())?;
    let dist = dist.as_ref();
    if dist.is_dir() {
        println!("Remove {dist:?}");
        fs::remove_dir_all(dist)?;
    }
    extract(".")?;
    pack_inner(project).map_err(|e| {
        fs::remove_dir_all(archive!()).unwrap_or_default();
        e
    })?;
    fs::rename(archive!(), dist)?;
    println!("Done");
    Ok(())
}

fn pack_inner(project: &str) -> Result<()> {
    let archive = Path::new(archive!());
    let contents = load(&fs::read_to_string(project)?, "", false)?;
    fs::write(archive.join("index.html"), contents)?;
    for assets in listdir(".")? {
        let name = assets.file_name().unwrap().to_str().unwrap();
        if name == archive!() || name.starts_with('.') {
            continue;
        }
        if assets.is_dir() {
            let dist = archive.join(name);
            if !dist.is_dir() {
                fs::create_dir(&dist)?;
            }
            println!("{assets:?} > {dist:?}");
            copy_dir(assets, dist)?;
        }
    }
    Ok(())
}
