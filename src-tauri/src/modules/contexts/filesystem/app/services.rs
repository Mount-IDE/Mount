use super::utils::split_path;
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::shared::kernel::errors::FileSystemError;
use crate::modules::shared::kernel::values::Path;
use std::fs;
use std::fs::File;
use std::io::Read;

pub struct FileSystemReadService();

impl TFSReadService for FileSystemReadService {
    ///
    ///
    ///
    fn read_file(&self, file_: &PFile) -> Result<String, FileSystemError> {
        let file = File::open(file_.path.get());
        if file.is_err() {
            return Err(FileSystemError::FileOpen {
                path: file_.path.clone(),
                err: file.unwrap_err(),
            });
        }
        let mut file = file.unwrap();
        let mut text = String::new();

        let _ = file.read_to_string(&mut text);

        Ok(text)
    }

    fn read_bytes(&self, file_: &PFile) -> Result<Vec<u8>, FileSystemError> {
        let bytes = fs::read(file_.path.get()).map_err(|e| FileSystemError::FileOpen {
            path: file_.path.clone(),
            err: e,
        })?;
        Ok(bytes)
    }

    ///
    ///
    ///
    fn read_dir(&self, dir_: &PDirectory) -> Result<PDirectory, FileSystemError> {
        let dir = fs::read_dir(dir_.path.get()).map_err(|e| FileSystemError::DirRead {
            path: dir_.path.clone(),
            err: e,
        })?;
        let mut files = Vec::<PFile>::new();
        let mut dirs = Vec::<PDirectory>::new();
        for i in dir {
            if i.is_ok() {
                let entry = i.unwrap();
                let name = entry.file_name().to_str().unwrap().to_string();
                let path = Path(entry.path().to_str().unwrap().to_string());

                if entry.file_type().unwrap().is_file() {
                    let typ = FileType::REGULAR;
                    let file = PFile {
                        name: name.clone(),
                        path: path.clone(),
                        typ,
                    };
                    files.push(file);
                } else {
                    let dir_ = PDirectory {
                        name: name.clone(),
                        path: path.clone(),
                        files: vec![],
                        directories: vec![],
                    };
                    let dir = self.read_dir(&dir_);
                    if dir.is_ok() {
                        dirs.push(dir?);
                    }
                }
            }
        }

        let splited = split_path(&dir_.path);

        let name = splited
            .get(splited.len() - 1)
            .ok_or(FileSystemError::PathParsing {
                path: dir_.path.clone(),
            })?;
        let directory = PDirectory {
            name: name.to_string(),
            path: dir_.path.clone(),
            files,
            directories: dirs,
        };
        Ok(directory)
    }

    ///
    ///
    ///
    fn exist_file(&self, file: &PFile) -> bool {
        let path = file.path.get();
        let ext = fs::exists(path);
        if ext.is_err() {
            return false;
        }
        let ext = ext.unwrap();
        ext
    }

    ///
    ///
    ///
    fn exist_dir(&self, file: &PDirectory) -> bool {
        let path = file.path.get();
        let ext = fs::exists(path);
        if ext.is_err() {
            return false;
        }
        let ext = ext.unwrap();
        ext
    }

    fn exists(&self, path: Path) -> bool {
        let path_ = path.get();
        let ext = fs::exists(path_);
        if ext.is_err() {
            return false;
        }
        ext.unwrap()
    }
}

pub struct FileSystemWriteService();

impl TFSWriteService for FileSystemWriteService {
    ///
    ///
    ///
    fn create_file(&self, path: &Path) -> Result<PFile, FileSystemError> {
        File::create(path.get()).map_err(|e| FileSystemError::FileCreation {
            path: path.clone(),
            err: e,
        })?;
        println!("file {}", path.get());

        let splited = split_path(&path);
        let name = splited
            .get(splited.len() - 1)
            .ok_or(FileSystemError::PathParsing { path: path.clone() })?;
        let res = PFile {
            name: name.clone(),
            path: path.clone(),
            typ: FileType::REGULAR,
        };
        Ok(res)
    }

    ///
    ///
    ///
    fn create_dir(&self, path: &Path) -> Result<PDirectory, FileSystemError> {
        fs::create_dir_all(path.get()).map_err(|e| FileSystemError::DirCreation {
            path: path.clone(),
            err: e,
        })?;

        let splited = split_path(&path);
        let name = splited
            .get(splited.len() - 1)
            .ok_or(FileSystemError::PathParsing { path: path.clone() })?;
        Ok(PDirectory {
            name: name.clone(),
            path: path.clone(),
            files: Vec::new(),
            directories: vec![],
        })
    }

    ///
    ///
    ///
    fn remove_file(&self, file: &PFile) -> Result<(), FileSystemError> {
        let path = file.path.clone();
        fs::remove_file(path.get()).map_err(|e| FileSystemError::FileRemove {
            path: path.clone(),
            err: e,
        })?;
        Ok(())
    }

    ///
    ///
    ///
    fn remove_dir(&self, directory: &PDirectory) -> Result<(), FileSystemError> {
        let path = directory.path.clone();
        fs::remove_dir_all(path.get()).map_err(|e| FileSystemError::DirRemove {
            path: path.clone(),
            err: e,
        })?;
        Ok(())
    }

    ///
    ///
    ///
    fn write_file(
        &self,
        file: &PFile,
        text: String,
        _: FileWriteAccess,
    ) -> Result<(), FileSystemError> {
        let path = file.path.clone();
        fs::write(path.get(), text).map_err(|e| FileSystemError::FileWrite {
            path: path.clone(),
            err: e,
        })?;
        Ok(())
    }
}
