use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::shared::kernel::entities::{Error, FileSystemError, LogLevel};
use crate::modules::shared::kernel::values::{FileSystemErrorType, Path};
use std::fs;
use std::io::Read;
use crate::modules::contexts::filesystem::domain::values::FileType;
use super::utils::split_path;
struct FileSystemReadService();

impl TFSReadService for FileSystemReadService {
    ///
    /// 
    /// 
    fn read_file(&self, path: &Path) -> Result<String, FileSystemError> {
        let file = fs::File::open(path.get());
        if file.is_err() {
            let err = FileSystemError::new(FileSystemErrorType::FileNotExists, LogLevel::ERROR);
            return Err(err);
        }
        let mut file = file.unwrap();
        let mut text = String::new();

        let _ = file.read_to_string(&mut text);

        Ok(text)
    }

    
    ///
    /// 
    /// 
    fn read_dir(&self, path: &Path) -> Result<PDirectory, FileSystemError> {
        let dir = fs::read_dir(path.get());
        if dir.is_err() {
            let err = FileSystemError::new(FileSystemErrorType::DirectoryNotExists, LogLevel::ERROR);
            return Err(err);
        }
        let mut dir = dir.unwrap();
        let mut files = Vec::<PFile>::new();
        for i in dir{
            if i.is_ok() {
                let entry = i.unwrap();
                let name = entry.file_name().to_str().unwrap().to_string();
                let path = Path(entry.path().to_str().unwrap().to_string());
                let typ= FileType::REGULAR;
                let file = PFile{name, path, typ};
                files.push(file);
            }
        }
        
        let splited = split_path(&path);
        
        let name = splited.get(splited.len()-1);
        if name.is_none() {
            return Err(FileSystemError::new(FileSystemErrorType::DirectoryPathParsing, LogLevel::ERROR));
        }
        let name = name.unwrap();
        
        let directory = PDirectory {
            name:name.to_string(),
            path:path.clone(),
            files
        };
        Ok(directory)
    }
}
