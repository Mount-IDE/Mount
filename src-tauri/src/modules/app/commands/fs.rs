use crate::modules::app::{FS_READ_SERVICE, FS_WATCH_SERVICE};
use crate::modules::contexts::filesystem::app::managers::SharedWatcherManager;
use crate::modules::contexts::filesystem::app::traits::{TFSReadService, TFWatchService};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
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
