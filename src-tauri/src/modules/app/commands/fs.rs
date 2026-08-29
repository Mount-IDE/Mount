use crate::modules::app::{FS_READ_SERVICE, FS_WATCH_SERVICE, FS_WRITE_SERVICE};
use crate::modules::contexts::filesystem::app::managers::SharedWatcherManager;
use crate::modules::contexts::filesystem::app::traits::{
    TFSReadService, TFSWriteService, TFWatchService,
};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::FileWriteAccess;
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;
use tauri::{State, WebviewWindow};

#[tauri::command]
pub fn read_dir_rec(cwd: String) -> Result<PDirectory, ErrorDto> {
    let dir_ = PDirectory::from_path(&Path(cwd.clone()));
    let dir = FS_READ_SERVICE.read_dir_recursive(&dir_)?;
    Ok(dir)
}

#[tauri::command]
pub fn read_file(path: String) -> Result<String, ErrorDto> {
    let path = Path::new(&path);
    let file = PFile::from_path_reg(path);
    let content = FS_READ_SERVICE.read_file(&file)?;
    Ok(content)
}

#[tauri::command]
pub fn watch_project(
    project_path: String,
    window: WebviewWindow,
    state: State<SharedWatcherManager>,
) -> Result<(), ErrorDto> {
    FS_WATCH_SERVICE.watch(
        Path::new(&project_path),
        Path::new(""),
        window.label().to_string(),
        state,
    )?;
    Ok(())
}

#[tauri::command]
pub fn unwatch_project(
    window: WebviewWindow,
    state: State<SharedWatcherManager>,
) -> Result<(), ErrorDto> {
    FS_WATCH_SERVICE.unwatch(window.label().to_string(), state)?;
    Ok(())
}

#[tauri::command]
pub fn create_file(path: String, content: Option<String>) -> Result<(), ErrorDto> {
    let path = Path::new(&path);
    FS_WRITE_SERVICE.create_file(&path)?;
    if let Some(content) = content {
        let file = PFile::from_path_reg(path);
        FS_WRITE_SERVICE.write_file(&file, content, FileWriteAccess::WRITE)?;
    }
    Ok(())
}

#[tauri::command]
pub fn create_dir(path: String) -> Result<(), ErrorDto> {
    let path = Path::new(&path);
    FS_WRITE_SERVICE.create_dir(&path)?;

    Ok(())
}

#[tauri::command]
pub fn write_file(path: String, content: String) -> Result<(), ErrorDto> {
    let file = PFile::from_path_reg(Path(path));
    FS_WRITE_SERVICE.write_file(&file, content, FileWriteAccess::WRITE)?;
    Ok(())
}

#[tauri::command]
pub fn remove_file(path: String) -> Result<(), ErrorDto> {
    let path = Path::new(&path);
    let file = PFile::from_path_reg(path);
    FS_WRITE_SERVICE.remove_file(&file)?;
    Ok(())
}

#[tauri::command]
pub fn remove_dir(path: String) -> Result<(), ErrorDto> {
    let path = Path::new(&path);
    let dir = PDirectory::from_path(&path);
    FS_WRITE_SERVICE.remove_dir(&dir)?;
    Ok(())
}

#[tauri::command]
pub fn rename_file(from: String, to: String) -> Result<(), ErrorDto> {
    let file1 = PFile::from_path_reg(Path(from));
    let file2 = PFile::from_path_reg(Path(to));
    FS_WRITE_SERVICE.rename_file(&file1, &file2)?;
    Ok(())
}

#[tauri::command]
pub fn read_binary(path: Path) -> Result<Vec<u8>, ErrorDto> {
    let file = PFile::from_path_reg(path);
    let arr = FS_READ_SERVICE.read_bytes(&file)?;
    Ok(arr)
}
