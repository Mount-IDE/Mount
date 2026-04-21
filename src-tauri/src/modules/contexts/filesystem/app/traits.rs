use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::FileWriteAccess;
use crate::modules::shared::kernel::entities::{Error, FileSystemError};
use crate::modules::shared::kernel::values::{FileSystemErrorType, Path};

pub trait TFSReadService{
    fn read_file(&self, file: &PFile) -> Result<String, FileSystemError>;
    fn read_dir(&self, dir: &PDirectory) -> Result<PDirectory, FileSystemError>;
}

pub trait TFSWriteService {
    fn create_file(&self, path: &Path) -> Result<PFile, FileSystemError>;
    fn create_dir(&self, path: &Path) -> Result<PDirectory, FileSystemError>;
    fn remove_file(&self, file: &PFile) -> Result<PFile, FileSystemError>;
    fn remove_dir(&self, directory: &PDirectory) -> Result<PFile, FileSystemError>;
    fn write_file(&self, file: &PFile, text: String, access: FileWriteAccess) -> Result<PFile, FileSystemError>;
}

pub trait TFSManageService {
    fn copy_file(&self, file: &PFile) -> PFile;
    fn copy_dir(&self, directory: &PDirectory) -> PDirectory;
    fn move_file(&self, file: &PFile, from: Path, to: Path) -> Result<PFile,FileSystemError>;
    fn move_dir(&self, directory: &PDirectory, from: Path, to: Path) -> Result<PDirectory, FileSystemError>;


}
