use std::{
    env::home_dir,
    fs::{self, create_dir},
    io,
    path::PathBuf,
};

use tempfile::tempdir;

use crate::engine::{FileInfo, FileInfoVecExt};

const FOLDER_NAME: &str = ".TraceHell";

pub struct MotherDir;
impl MotherDir {
    pub fn new() -> io::Result<PathBuf> {
        let path = MotherDir::create_if_not_present()?;
        Ok(path)
    }
    fn create_if_not_present() -> io::Result<PathBuf> {
        let dir = MotherDir::create_dir();
        if !dir.exists() {
            fs::create_dir_all(&dir).unwrap();
        }
        Ok(dir)
    }
    fn create_dir() -> PathBuf {
        let home_dir = home_dir().unwrap();

        home_dir.join(FOLDER_NAME)
    }
}

pub fn create() -> io::Result<String> {
    let dir_path = MotherDir::new()?;

    let (file, folder_name) = FileInfo::new(&dir_path)?;
    file.copy_scanfold(&dir_path);

    Ok(folder_name)
}
