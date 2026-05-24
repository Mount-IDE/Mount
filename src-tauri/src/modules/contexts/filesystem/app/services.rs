use super::utils::{make_path, split_path};
use crate::modules::app::APP;
use crate::modules::contexts::filesystem::app::managers::SharedWatcherManager;
use crate::modules::contexts::filesystem::app::traits::{
    TFSReadService, TFSWriteService, TFWatchService,
};
use crate::modules::contexts::filesystem::domain::entities::{PDirectory, PFile};
use crate::modules::contexts::filesystem::domain::values::{
    FileType, FileWriteAccess, WatchInstance,
};
use crate::modules::shared::kernel::errors::FileSystemError;
use crate::modules::shared::kernel::values::Path;
use notify::event::{ModifyKind, RenameMode};
use notify::{
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Result as NResult, Watcher,
};
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::Read;
use std::path::{Path as std_path, PathBuf};
use std::sync::mpsc::{channel, RecvTimeoutError};
use std::time::Duration;
use std::{fs, thread};
use tauri::{Emitter, Manager};

pub struct FileSystemReadService();

impl TFSReadService for FileSystemReadService {
    ///
    ///
    ///
    fn read_file(&self, file_: &PFile) -> Result<String, FileSystemError> {
        let file = File::open(file_.path.get());
        if file.is_err() {
            return Err(FileSystemError::FileOpen {
                path: file_.path.clone(),
                err: file.unwrap_err(),
            });
        }
        let mut file = file.unwrap();
        let mut text = String::new();

        let _ = file.read_to_string(&mut text);

        Ok(text)
    }

    fn read_bytes(&self, file_: &PFile) -> Result<Vec<u8>, FileSystemError> {
        let bytes = fs::read(file_.path.get()).map_err(|e| FileSystemError::FileOpen {
            path: file_.path.clone(),
            err: e,
        })?;
        Ok(bytes)
    }

    ///
    ///
    ///
    fn read_dir(&self, dir_: &PDirectory) -> Result<PDirectory, FileSystemError> {
        let dir = fs::read_dir(dir_.path.get()).map_err(|e| FileSystemError::DirRead {
            path: dir_.path.clone(),
            err: e,
        })?;
        let mut files = Vec::<PFile>::new();
        let mut dirs = Vec::<PDirectory>::new();
        for i in dir {
            if i.is_ok() {
                let entry = i.unwrap();
                let name = entry.file_name().to_str().unwrap().to_string();
                let path = Path(entry.path().to_str().unwrap().to_string());

                if entry.file_type().unwrap().is_file() {
                    let typ = FileType::REGULAR;
                    let file = PFile {
                        name: name.clone(),
                        path: path.clone(),
                        typ,
                    };
                    files.push(file);
                } else {
                    let dir_ = PDirectory {
                        name: name.clone(),
                        path: path.clone(),
                        files: vec![],
                        directories: vec![],
                    };
                    let dir = self.read_dir(&dir_);
                    if dir.is_ok() {
                        dirs.push(dir?);
                    }
                }
            }
        }

        let splited = split_path(&dir_.path);

        let name = splited
            .get(splited.len() - 1)
            .ok_or(FileSystemError::PathParsing {
                path: dir_.path.clone(),
            })?;
        let directory = PDirectory {
            name: name.to_string(),
            path: dir_.path.clone(),
            files,
            directories: dirs,
        };
        Ok(directory)
    }

    ///
    ///
    ///
    fn exist_file(&self, file: &PFile) -> bool {
        let path = file.path.get();
        let ext = fs::exists(path);
        if ext.is_err() {
            return false;
        }
        let ext = ext.unwrap();
        ext
    }

    ///
    ///
    ///
    fn exist_dir(&self, file: &PDirectory) -> bool {
        let path = file.path.get();
        let ext = fs::exists(path);
        if ext.is_err() {
            return false;
        }
        let ext = ext.unwrap();
        ext
    }

    fn read_dir_recursive(&self, dir: &PDirectory) -> Result<PDirectory, FileSystemError> {
        let mut content = self.read_dir(&dir)?;
        for i in 0..content.directories.len() {
            let path_ = content.directories[i].path.clone();
            let dir_ = PDirectory::from_path(&path_);
            let content_ = self.read_dir_recursive(&dir_)?;
            content.directories[i].directories = content_.directories;
        }
        Ok(content)
    }

    fn exists(&self, path: Path) -> bool {
        let path_ = path.get();
        let ext = fs::exists(path_);
        if ext.is_err() {
            return false;
        }
        ext.unwrap()
    }
}

pub struct FileSystemWriteService();

impl TFSWriteService for FileSystemWriteService {
    ///
    ///
    ///
    fn create_file(&self, path: &Path) -> Result<PFile, FileSystemError> {
        File::create(path.get()).map_err(|e| FileSystemError::FileCreation {
            path: path.clone(),
            err: e,
        })?;
        println!("file {}", path.get());

        let splited = split_path(&path);
        let name = splited
            .get(splited.len() - 1)
            .ok_or(FileSystemError::PathParsing { path: path.clone() })?;
        let res = PFile {
            name: name.clone(),
            path: path.clone(),
            typ: FileType::REGULAR,
        };
        Ok(res)
    }

    ///
    ///
    ///
    fn create_dir(&self, path: &Path) -> Result<PDirectory, FileSystemError> {
        fs::create_dir_all(path.get()).map_err(|e| FileSystemError::DirCreation {
            path: path.clone(),
            err: e,
        })?;

        let splited = split_path(&path);
        let name = splited
            .get(splited.len() - 1)
            .ok_or(FileSystemError::PathParsing { path: path.clone() })?;
        Ok(PDirectory {
            name: name.clone(),
            path: path.clone(),
            files: Vec::new(),
            directories: vec![],
        })
    }

    ///
    ///
    ///
    fn remove_file(&self, file: &PFile) -> Result<(), FileSystemError> {
        let path = file.path.clone();
        fs::remove_file(path.get()).map_err(|e| FileSystemError::FileRemove {
            path: path.clone(),
            err: e,
        })?;
        Ok(())
    }

    ///
    ///
    ///
    fn remove_dir(&self, directory: &PDirectory) -> Result<(), FileSystemError> {
        let path = directory.path.clone();
        fs::remove_dir_all(path.get()).map_err(|e| FileSystemError::DirRemove {
            path: path.clone(),
            err: e,
        })?;
        Ok(())
    }

    ///
    ///
    ///
    fn write_file(
        &self,
        file: &PFile,
        text: String,
        _: FileWriteAccess,
    ) -> Result<(), FileSystemError> {
        let path = file.path.clone();
        fs::write(path.get(), text).map_err(|e| FileSystemError::FileWrite {
            path: path.clone(),
            err: e,
        })?;
        Ok(())
    }

    fn rename_file(&self, from: &PFile, to: &PFile) -> Result<(), FileSystemError> {
        fs::rename(from.path.clone().get(), to.path.clone().get()).map_err(|e|
            FileSystemError::Rename {
                from: from.path.clone(), 
                to: to.path.clone(),
                e
            }
        )?;
        Ok(())
    }

    fn rename_dir(&self, from: &PDirectory, to: &PDirectory) -> Result<(), FileSystemError> {
        fs::rename(from.path.clone().get(), to.path.clone().get()).map_err(|e|
            FileSystemError::Rename {
                from: from.path.clone(),
                to: to.path.clone(),
                e
            }
        )?;
        Ok(())    }
}

pub struct FileSystemWatchService();

#[derive(Clone, Serialize)]
#[serde(untagged)]
enum FsWatchNode {
    File(PFile),
    Directory(PDirectory),
}

#[derive(Clone, Serialize)]
struct FsWatchEvent {
    kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    old_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    new_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    node: Option<FsWatchNode>,
}

fn path_to_string(path: &PathBuf) -> String {
    path.to_string_lossy().to_string()
}

fn read_node(path: &str) -> Option<FsWatchNode> {
    let metadata = fs::metadata(path).ok()?;
    let path = Path(path.to_string());

    if metadata.is_dir() {
        let dir = PDirectory::from_path(&path);
        return FileSystemReadService()
            .read_dir_recursive(&dir)
            .ok()
            .map(FsWatchNode::Directory);
    }

    if metadata.is_file() {
        return Some(FsWatchNode::File(PFile::from_path_reg(path)));
    }

    None
}

fn emit_watch_event(window: &tauri::WebviewWindow, event: FsWatchEvent) {
    let _ = window.emit("fs-event", event);
}

fn emit_removed(window: &tauri::WebviewWindow, path: String) {
    emit_watch_event(
        window,
        FsWatchEvent {
            kind: "removed".to_string(),
            path: Some(path),
            old_path: None,
            new_path: None,
            node: None,
        },
    );
}

fn flush_pending_rename(window: &tauri::WebviewWindow, pending_rename_from: &mut Option<String>) {
    if let Some(path) = pending_rename_from.take() {
        emit_removed(window, path);
    }
}

fn emit_created(window: &tauri::WebviewWindow, path: String) {
    emit_watch_event(
        window,
        FsWatchEvent {
            kind: "created".to_string(),
            path: Some(path.clone()),
            old_path: None,
            new_path: None,
            node: read_node(&path),
        },
    );
}

fn emit_modified(window: &tauri::WebviewWindow, path: String) {
    emit_watch_event(
        window,
        FsWatchEvent {
            kind: "modified".to_string(),
            path: Some(path),
            old_path: None,
            new_path: None,
            node: None,
        },
    );
}

fn emit_renamed(window: &tauri::WebviewWindow, old_path: String, new_path: String) {
    emit_watch_event(
        window,
        FsWatchEvent {
            kind: "renamed".to_string(),
            path: None,
            old_path: Some(old_path),
            new_path: Some(new_path.clone()),
            node: read_node(&new_path),
        },
    );
}

fn handle_watch_event(
    window: &tauri::WebviewWindow,
    event: Event,
    pending_rename_from: &mut Option<String>,
) {
    match event.kind {
        EventKind::Create(_) => {
            flush_pending_rename(window, pending_rename_from);
            for path in event.paths {
                emit_created(window, path_to_string(&path));
            }
        }
        EventKind::Remove(_) => {
            flush_pending_rename(window, pending_rename_from);
            for path in event.paths {
                emit_removed(window, path_to_string(&path));
            }
        }
        EventKind::Modify(ModifyKind::Name(RenameMode::From)) => {
            flush_pending_rename(window, pending_rename_from);
            *pending_rename_from = event.paths.first().map(path_to_string);
        }
        EventKind::Modify(ModifyKind::Name(RenameMode::To)) => {
            if let Some(new_path) = event.paths.first().map(path_to_string) {
                if let Some(old_path) = pending_rename_from.take() {
                    emit_renamed(window, old_path, new_path);
                } else {
                    emit_created(window, new_path);
                }
            }
        }
        EventKind::Modify(ModifyKind::Name(RenameMode::Both)) => {
            flush_pending_rename(window, pending_rename_from);
            if event.paths.len() >= 2 {
                emit_renamed(
                    window,
                    path_to_string(&event.paths[0]),
                    path_to_string(&event.paths[1]),
                );
            }
        }
        EventKind::Modify(ModifyKind::Name(_)) => {
            flush_pending_rename(window, pending_rename_from);
            if event.paths.len() >= 2 {
                emit_renamed(
                    window,
                    path_to_string(&event.paths[0]),
                    path_to_string(&event.paths[1]),
                );
            }
        }
        EventKind::Modify(_) => {
            flush_pending_rename(window, pending_rename_from);
            for path in event.paths {
                emit_modified(window, path_to_string(&path));
            }
        }
        _ => {
            flush_pending_rename(window, pending_rename_from);
        }
    }
}

impl TFWatchService for FileSystemWatchService {
    fn watch(
        &self,
        cwd: Path,
        proj_path: Path,
        label: String,
        state: tauri::State<SharedWatcherManager>,
    ) -> Result<(), FileSystemError> {
        let app = APP
            .get()
            .ok_or(FileSystemError::Watch { path: cwd.clone() })?
            .clone();

        let window = app
            .get_webview_window(label.as_str())
            .ok_or(FileSystemError::Watch { path: cwd.clone() })?;

        let cwd_string = cwd.get();
        let project_string = proj_path.get();
        let new_path = if project_string.is_empty() {
            cwd
        } else if cwd_string.is_empty() {
            proj_path
        } else {
            make_path(vec![project_string.as_str(), cwd_string.as_str()])
        };
        let (tx, rx) = channel();

        let mut watcher = RecommendedWatcher::new(
            move |res: NResult<Event>| {
                tx.send(res).unwrap();
            },
            Config::default(),
        )
        .unwrap();

        watcher
            .watch(std_path::new(&new_path.get()), RecursiveMode::Recursive)
            .unwrap();

        let thread = thread::spawn(move || {
            let mut pending_rename_from: Option<String> = None;
            loop {
                match rx.recv_timeout(Duration::from_millis(150)) {
                    Ok(Ok(event)) => handle_watch_event(&window, event, &mut pending_rename_from),
                    Ok(Err(_)) => {}
                    Err(RecvTimeoutError::Timeout) => {
                        flush_pending_rename(&window, &mut pending_rename_from);
                    }
                    Err(RecvTimeoutError::Disconnected) => break,
                }
            }
        });

        let previous = state.lock().unwrap().watchers.remove(&label);
        if let Some(instance) = previous {
            drop(instance.watcher);
            let _ = instance.thread.join();
        }

        let instance = WatchInstance { watcher, thread };
        state.lock().unwrap().watchers.insert(label, instance);
        Ok(())
    }

    fn unwatch(
        &self,
        label: String,
        state: tauri::State<SharedWatcherManager>,
    ) -> Result<(), FileSystemError> {
        let instance = state.lock().unwrap().watchers.remove(&label);
        if let Some(instance) = instance {
            drop(instance.watcher);
            let _ = instance.thread.join();
        }
        Ok(())
    }
}
