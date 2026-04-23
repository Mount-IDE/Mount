use super::utils::split_path;
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFSWriteService};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{FileType, FileWriteAccess};
use crate::modules::shared::kernel::entities::{FileSystemError, LogLevel};
use crate::modules::shared::kernel::values::{FileSystemErrorType, Path};
use std::fs;
use std::io::{Read, Write};

pub struct FileSystemReadService();

impl TFSReadService for FileSystemReadService {
    ///
    ///
    ///
    fn read_file(&self, file: &PFile) -> Result<String, FileSystemError> {
        let file = fs::File::open(file.path.get());
        if file.is_err() {
           return Err(FileSystemError::new(FileSystemErrorType::FileNotExists, LogLevel::ERROR));
        }
        let mut file = file.unwrap();
        let mut text = String::new();

        let _ = file.read_to_string(&mut text);

        Ok(text)
    }

    ///
    ///
    ///
    fn read_dir(&self, dir_: &PDirectory) -> Result<PDirectory, FileSystemError> {
        let dir = fs::read_dir(dir_.path.get());
        if dir.is_err() {
            return Err(FileSystemError::new(
                FileSystemErrorType::DirectoryNotExists, LogLevel::ERROR));
        }
        let dir = dir.unwrap();
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
                    let dir_ = PDirectory{name:name.clone(), path:path.clone(), files: vec![], directories: vec![]};
                    let dir = self.read_dir(&dir_);
                    if dir.is_ok() {
                        dirs.push(dir?);
                    }

                }
            }
        }

        let splited = split_path(&dir_.path);

        let name = splited.get(splited.len() - 1);
        if name.is_none() {
            return Err(FileSystemError::new(
                FileSystemErrorType::DirectoryPathParsing,
                LogLevel::ERROR,
            ));
        }
        let name = name.unwrap();

        let directory = PDirectory {
            name: name.to_string(),
            path: dir_.path.clone(),
            files,
            directories:dirs
        };
        Ok(directory)
    }

    fn exist_file(&self, file: &PFile) -> bool {
        let path = file.path.get();
        let ext = fs::exists(path);
        if ext.is_err(){
            return false;
        }
        let ext = ext.unwrap();
        ext
    }

    fn exist_dir(&self, file: &PDirectory) -> bool {
        let path = file.path.get();
        let ext = fs::exists(path);
        if ext.is_err(){
            return false;
        }
        let ext = ext.unwrap();
        ext
    }
}


pub struct FileSystemWriteService();

impl TFSWriteService for FileSystemWriteService {
    fn create_file(&self, path: &Path) -> Result<PFile, FileSystemError> {
        let file = fs::File::create(path.get());
        println!("file {}", path.get());
        if file.is_err() {
            println!("error file create");
           return Err(
               FileSystemError::new(FileSystemErrorType::FileAlreadyExists, LogLevel::ERROR));
        }
        let _ = file.unwrap();
        let splited = split_path(&path);
        let name = splited.get(splited.len() - 1);
        if name.is_none() {
            println!("error split name");
            return Err(FileSystemError::new(
                FileSystemErrorType::FilePathParsing,
                LogLevel::ERROR,
            ));
        }
        let name = name.unwrap();
        let res = PFile {
            name: name.clone(),
            path: path.clone(),
            typ: FileType::REGULAR,
        };
        Ok(res)
    }

    fn create_dir(&self, path: &Path) -> Result<PDirectory, FileSystemError> {
        let dir = fs::create_dir_all(path.get());
        if dir.is_err() {
            return Err(FileSystemError::new(
                FileSystemErrorType::DirectoryAlreadyExists,
                LogLevel::ERROR,
            ));
        }
        let splited = split_path(&path);
        let name = splited.get(splited.len() - 1);
        if name.is_none() {
            return Err(FileSystemError::new(
                FileSystemErrorType::DirectoryPathParsing,
                LogLevel::ERROR,
            ));
        }
        let name = name.unwrap();
        Ok(PDirectory {
            name: name.clone(),
            path: path.clone(),
            files: Vec::new(),
            directories: vec![],
        })
    }

    fn remove_file(&self, file: &PFile) -> Result<(), FileSystemError> {
        let path = file.path.clone();
        let res = fs::remove_file(path.get());
        if res.is_err() {
            return Err(FileSystemError::new(
                FileSystemErrorType::FileRemovingError,
                LogLevel::ERROR,
            ));
        }
        Ok(())
    }

    fn remove_dir(&self, directory: &PDirectory) -> Result<(), FileSystemError> {
        let path = directory.path.clone();
        let res = fs::remove_dir_all(path.get());
        if res.is_err() {
            return Err(FileSystemError::new(
                FileSystemErrorType::DirectoryRemovingError,
                LogLevel::ERROR,
            ));
        }
        Ok(())
    }

    fn write_file(
        &self,
        file: &PFile,
        text: String,
        access: FileWriteAccess,
    ) -> Result<(), FileSystemError> {
        let path = file.path.clone();
        let res = fs::write(path.get(), text);
        if res.is_err() {
            return Err(FileSystemError::new(
                FileSystemErrorType::FileWritingError,
                LogLevel::ERROR,
            ));
        }
        Ok(())
    }
}
