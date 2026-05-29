use crate::modules::app::TERMINAL_SERVICE;
use crate::modules::contexts::terminal::app::managers::SharedTerminalManager;
use crate::modules::contexts::terminal::app::traits::TTerminalService;
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;
use tauri::{State, WebviewWindow};

#[tauri::command]
pub async fn open_terminal(
    shell: String,
    cwd: String,
    rows: u16,
    cols: u16,
    window: WebviewWindow,
    state: State<'_, SharedTerminalManager>,
) -> Result<String, ErrorDto> {
    let id = TERMINAL_SERVICE
        .open_terminal(
            window.label().to_string(),
            shell,
            Path::new(&cwd),
            rows,
            cols,
            state,
        )
        .await?;

    Ok(id)
}

#[tauri::command]
pub async fn write_terminal(
    id: String,
    data: String,
    state: State<'_, SharedTerminalManager>,
) -> Result<(), ErrorDto> {
    TERMINAL_SERVICE.write_terminal(id, data, state).await?;

    Ok(())
}

#[tauri::command]
pub async fn resize_terminal(
    id: String,
    rows: u16,
    cols: u16,
    state: State<'_, SharedTerminalManager>,
) -> Result<(), ErrorDto> {
    TERMINAL_SERVICE
        .resize_terminal(id, rows, cols, state)
        .await?;

    Ok(())
}

#[tauri::command]
pub async fn close_terminal(
    id: String,
    state: State<'_, SharedTerminalManager>,
) -> Result<(), ErrorDto> {
    TERMINAL_SERVICE.close_terminal(id, state).await?;

    Ok(())
}

#[tauri::command]
pub async fn close_window_terminals(
    window: WebviewWindow,
    state: State<'_, SharedTerminalManager>,
) -> Result<(), ErrorDto> {
    TERMINAL_SERVICE
        .close_window_terminals(window.label().to_string(), state)
        .await?;

    Ok(())
}
