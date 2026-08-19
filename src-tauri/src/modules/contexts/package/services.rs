use crate::modules::contexts::package::domain::{LspEntity, SharedLspManager};
use crate::modules::contexts::package::traits::TLspService;
use crate::modules::contexts::project::domain::entities::PackageComponent;
use serde_json::Value;
use std::process::Stdio;
use tauri::{AppHandle, Emitter, State};
use tokio::io::BufReader;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;
use tokio::sync::mpsc;
pub struct LspService();

impl TLspService for LspService {
    async fn start_server(
        &self,
        app: AppHandle,
        window: String,
        state: State<'_, SharedLspManager>,
        package: String,
        obj: PackageComponent,
    ) -> Result<String, String> {
        let program = obj.get_command(package);
        let mut child = Command::new(&program)
            .args(&obj.arguments.unwrap_or(vec![]))
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|_| String::new())?;
        let mut stdin = child.stdin.take().unwrap();
        let stdout = child.stdout.take().unwrap();
        let stderr = child.stderr.take().unwrap();

        let (tx, mut rx) = mpsc::unbounded_channel::<Value>();
        tokio::spawn(async move {
            while let Some(v) = rx.recv().await {
                let body = serde_json::to_string(&v).map_err(|_| String::new());
                if let Ok(body) = body {
                    let header = format!("Content-Length: {}\r\n\r\n", body.len());
                    if stdin.write_all(header.as_bytes()).await.is_err() {
                        break;
                    }
                    if stdin.write_all(body.as_bytes()).await.is_err() {
                        break;
                    }
                }
            }
        });

        let mut app_clone = app.clone();
        let id_clone = obj.id.clone();
        let window_clone = window.clone();
        tokio::spawn(async move {
            let cloned = window_clone.clone();
            let mut reader = BufReader::new(stdout);
            loop {
                let mut content_length = Option::<usize>::None;
                loop {
                    let mut line = String::new();
                    if reader.read_line(&mut line).await.unwrap_or(0) == 0 {
                        return;
                    }
                    let line = line.trim_end();
                    if line.is_empty() {
                        break;
                    }
                    if let Some(v) = line.strip_prefix("Content-Length: ") {
                        content_length = v.trim().parse().ok();
                    }
                }
                let Some(len) = content_length else { continue };
                let mut buf = vec![0u8; len];
                if reader.read_exact(&mut buf).await.is_err() {
                    return;
                }
                if let Ok(json) = serde_json::from_slice::<Value>(&buf) {
                    let _ = app_clone.emit_to(
                        cloned.clone(),
                        "lsp:message",
                        serde_json::json!({
                            "id": id_clone,
                            "payload": json
                        }),
                    );
                }
            }
        });

        let id_err = obj.id.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr);
            let mut line = String::new();
            while reader.read_line(&mut line).await.unwrap_or(0) > 0 {
                eprintln!("LSP ERR({}): {line}", id_err.clone());
                line.clear();
            }
        });

        state.lock().await.servers.insert(
            obj.id.clone(),
            LspEntity {
                window_id: window.clone(),
                child,
                writter: tx,
            },
        );
        Ok(obj.id.clone())
    }

    async fn stop_server(
        &self,
        id: String,
        state: State<'_, SharedLspManager>,
    ) -> Result<(), String> {
        state.lock().await.servers.remove(&id);
        Ok(())
    }

    async fn send_to(
        &self,
        state: State<'_, SharedLspManager>,
        id: String,
        message: Value,
    ) -> Result<(), String> {
        let servers = &state.lock().await.servers;
        let handle = servers.get(&id).ok_or_else(|| String::new())?;
        handle.writter.send(message).map_err(|e| e.to_string())?;
        Ok(())
    }
}
