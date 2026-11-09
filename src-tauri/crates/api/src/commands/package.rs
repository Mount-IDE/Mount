use crate::domain::ErrorDto;
use crate::get_data_dir;
use filesystem::FS_READ_SERVICE;
use filesystem::domain::PFile;
use filesystem::traits::TFSReadService;
use mount_core::make_path;
use project::project::domain::Package;
/*#[tauri::command]
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
}*/

#[tauri::command]
pub fn read_scm(pack: String, highlight: String) -> Result<String, ErrorDto> {
    let dir = get_data_dir!();
    let path = make_path![
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
    let dir = get_data_dir!();

    let path = make_path![dir, "packages", pack.id.clone(), "highlights"];

    let mut list = Vec::<String>::new();
    for i in pack.highlight {
        let path_ = make_path![path, i.id, "highlight.scm"];
        let file = PFile::from_path_reg(path_);
        let text = FS_READ_SERVICE.read_file(&file)?;
        list.push(text)
    }
    Ok(list)
}
