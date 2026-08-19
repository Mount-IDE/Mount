use crate::modules::app::LSP_SERVICE;
use crate::modules::contexts::package::domain::SharedLspManager;
use crate::modules::contexts::package::traits::TLspService;
use crate::modules::contexts::project::domain::entities::PackageComponent;
use crate::modules::shared::kernel::entities::ErrorDto;
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
