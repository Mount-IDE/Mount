use crate::modules::contexts::package::domain::SharedLspManager;
use crate::modules::contexts::project::domain::entities::PackageComponent;
use serde_json::Value;
use tauri::{AppHandle, State};

pub trait TLspService {
    async fn start_server(
        &self,
        app: AppHandle,
        window: String,
        state: State<'_, SharedLspManager>,
        package: String,
        obj: PackageComponent,
    ) -> Result<String, String>;

    async fn stop_server(
        &self,
        id: String,
        state: State<'_, SharedLspManager>,
    ) -> Result<(), String>;

    async fn send_to(
        &self,
        state: State<'_, SharedLspManager>,
        id: String,
        message: Value,
    ) -> Result<(), String>;
}
