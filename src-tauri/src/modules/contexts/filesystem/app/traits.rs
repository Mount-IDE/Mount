use crate::modules::contexts::filesystem::app::managers::SharedWatcherManager;
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::FileWriteAccess;
use crate::modules::shared::kernel::errors::FileSystemError;
use crate::modules::shared::kernel::values::Path;

pub trait TFSReadService {
    fn read_file(&self, file: &PFile) -> Result<String, FileSystemError>;
    fn read_bytes(&self, file_: &PFile) -> Result<Vec<u8>, FileSystemError>;
    fn read_dir(&self, dir: &PDirectory) -> Result<PDirectory, FileSystemError>;
    fn exist_file(&self, file: &PFile) -> bool;
    fn exist_dir(&self, file: &PDirectory) -> bool;

    fn read_dir_recursive(&self, dir: &PDirectory) -> Result<PDirectory, FileSystemError>;

    fn exists(&self, path: Path) -> bool;
}

pub trait TFSWriteService {
    fn create_file(&self, path: &Path) -> Result<PFile, FileSystemError>;
    fn create_dir(&self, path: &Path) -> Result<PDirectory, FileSystemError>;
    fn remove_file(&self, file: &PFile) -> Result<(), FileSystemError>;
    fn remove_dir(&self, directory: &PDirectory) -> Result<(), FileSystemError>;
    fn write_file(
        &self,
        file: &PFile,
        text: String,
        access: FileWriteAccess,
    ) -> Result<(), FileSystemError>;

    fn rename_file(&self, from:&PFile, to:&PFile) -> Result<(), FileSystemError>;
    fn rename_dir(&self, from:&PDirectory, to:&PDirectory) -> Result<(), FileSystemError>;
}

pub trait TFSManageService {
    fn copy_file(&self, file: &PFile) -> PFile;
    fn copy_dir(&self, directory: &PDirectory) -> PDirectory;
    fn move_file(&self, file: &PFile, from: Path, to: Path) -> Result<PFile, FileSystemError>;
    fn move_dir(
        &self,
        directory: &PDirectory,
        from: Path,
        to: Path,
    ) -> Result<PDirectory, FileSystemError>;
}

pub trait TFWatchService {
    fn watch(
        &self,
        cwd: Path,
        proj_path: Path,
        label: String,
        state: tauri::State<SharedWatcherManager>,
    ) -> Result<(), FileSystemError>;
    fn unwatch(
        &self,
        label: String,
        state: tauri::State<SharedWatcherManager>,
    ) -> Result<(), FileSystemError>;
}
