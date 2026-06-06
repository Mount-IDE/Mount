use crate::modules::contexts::terminal::app::managers::SharedTerminalManager;
use crate::modules::shared::kernel::errors::TerminalError;
use crate::modules::shared::kernel::values::Path;
use async_trait::async_trait;
use tauri::State;

#[async_trait]
pub trait TTerminalService {
    async fn open_terminal(
        &self,
        window_id: String,
        shell: String,
        cwd: Path,
        rows: u16,
        cols: u16,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<String, TerminalError>;

    async fn write_terminal(
        &self,
        id: String,
        data: String,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError>;

    async fn resize_terminal(
        &self,
        id: String,
        rows: u16,
        cols: u16,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError>;

    async fn close_terminal(
        &self,
        id: String,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError>;

    async fn close_window_terminals(
        &self,
        window_id: String,
        state: State<'_, SharedTerminalManager>,
    ) -> Result<(), TerminalError>;
}
