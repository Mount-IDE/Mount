use crate::modules::app::{CONFIG_SERVICE, FS_READ_SERVICE, LSP_SERVICE};
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::package::domain::SharedLspManager;
use crate::modules::contexts::package::traits::TLspService;
use crate::modules::contexts::project::domain::entities::{Package, PackageComponent};
use crate::modules::services::traits::TConfigService;
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::Path;
use serde_json::Value;
use tauri::{AppHandle, State, Window};

#[tauri::command]
pub async fn start_lsp(
    app: AppHandle,
    state: State<'_, SharedLspManager>,
    obj: PackageComponent,
    window: Window,
    package: String,
) -> Result<String, ErrorDto> {
    LSP_SERVICE
        .start_server(app, window.label().to_string(), state, package, obj)
        .await
        .map_err(|e| ErrorDto { message: e })
}

#[tauri::command]
pub async fn stop_lsp(state: State<'_, SharedLspManager>, id: String) -> Result<(), ErrorDto> {
    LSP_SERVICE
        .stop_server(id, state)
        .await
        .map_err(|e| ErrorDto { message: e })
}

#[tauri::command]
pub async fn send_to_lsp(
    state: State<'_, SharedLspManager>,
    id: String,
    message: Value,
) -> Result<(), ErrorDto> {
    LSP_SERVICE
        .send_to(state, id, message)
        .await
        .map_err(|e| ErrorDto { message: e })
}

#[tauri::command]
pub fn read_scm(pack: String, highlight: String) -> Result<String, ErrorDto> {
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let path = path_from![
        dir,
        "packages",
        pack,
        "highlights",
        highlight,
        "highlight.scm"
    ];
    let file = PFile::from_path_reg(path);
    let text = FS_READ_SERVICE.read_file(&file)?;
    Ok(text)
}

#[tauri::command]
pub fn read_scms(pack: Package) -> Result<Vec<String>, ErrorDto> {
    let dir = CONFIG_SERVICE.get_data_dir()?;

    let path = path_from![dir, "packages", pack.id.clone(), "highlights"];

    let mut list = Vec::<String>::new();
    for i in pack.highlight {
        let path_ = path_from![path, i.id, "highlight.scm"];
        let file = PFile::from_path_reg(path_);
        let text = FS_READ_SERVICE.read_file(&file)?;
        list.push(text)
    }
    Ok(list)
}
