use crate::modules::contexts::launch::domain::entities::{
    LaunchFunction, LaunchObject, LaunchTemplate, LaunchTemplateReference,
};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("config error that can`t describe as project error")]
    ConfigError(#[from] ConfigError),
    #[error("filesystem error that can`t describe as project error")]
    FileSystemError(
        #[from]
        #[source]
        FileSystemError,
    ),

    #[error("project already exists")]
    AlreadyExists,
    #[error("project creation failed")]
    CreationFailed {
        #[source]
        source: FileSystemError,
    },

    #[error("failed to parse in project: {err}")]
    ParsingError {
        #[from]
        err: ParsingError,
    },

    #[error("failed to get meta section (probably not set name or path)")]
    MetaNotFound,
    #[error("failed to get meta.-4 section (with name and path)")]
    MainMetaNotFound,
    #[error("failed to get name of project")]
    NameNotFound,
    #[error("failed to get project path")]
    PathNotFound,
    #[error("failed to parse address in action {address}")]
    IncorrectAddress { address: String },
}

#[derive(Error, Debug)]
pub enum FileSystemError {
    #[error("failed to create directory: {path}")]
    DirCreation {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to create file: {path}")]
    FileCreation {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to open file: {path}")]
    FileOpen {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to read file: {path}")]
    FileRead {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to read directory: {path}")]
    DirRead {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to write file: {path}")]
    FileWrite {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to remove directory: {path}")]
    DirRemove {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to remove file: {path}")]
    FileRemove {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("file already exists: {path}")]
    FileAlreadyExists {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("directory already exists: {path}")]
    DirAlreadyExists {
        path: Path,
        #[source]
        err: std::io::Error,
    },
    #[error("failed to parse path: {path}")]
    PathParsing { path: Path },
    #[error("failed to watch directory {path}")]
    Watch { path: Path },
    #[error("failed to rename from \"{from}\" to \"{to}\"")]
    Rename {
        from: Path,
        to: Path,
        e: std::io::Error,
    },
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to get app_data_dir")]
    GetDataDir {
        #[source]
        err: tauri::Error,
    },
    #[error("failed to save settings")]
    SavingSettings,
    #[error("Cannot get app")]
    App,
    #[error("global settings not found")]
    SettingsNotFound {
        #[source]
        err: FileSystemError,
    },
    #[error("cannot read settings")]
    ReadSettingsError,
    #[error("settings not in global scope")]
    SettingsIsEmpty,
    #[error("failed to parse in config: {err}")]
    ParsingError {
        #[from]
        err: ParsingError,
    },
    #[error("failed to create app directory")]
    MakeDataDir {
        #[source]
        err: FileSystemError,
    },
    #[error("filesystem error that can`t describe as config error")]
    FileSystem(#[from] FileSystemError),
    #[error("failed to get home dir")]
    HomeDir {
        #[source]
        err: tauri::Error,
    },
}

#[derive(Debug, Error)]
pub enum ParsingError {
    #[error("failed to parse json to {path}")]
    Serialize {
        path: Path,
        #[source]
        err: serde_json::Error,
    },
    #[error("failed to parse json from {path}: {json}")]
    Deserialize {
        path: Path,
        json: String,
        #[source]
        err: serde_json::Error,
    },
}

#[derive(Debug, Error)]
pub enum TerminalError {
    #[error("error of config context tha can`t described by terminal err")]
    Config { err: ConfigError },
    #[error("cannot spawn terminal process {shell}")]
    Spawn { shell: String },
    #[error("Terminal width {id} not found")]
    NotFound { id: String },
    #[error("Cannot write to terminal")]
    Write { err: std::io::Error, id: String },
    #[error("cannot resize the terminal")]
    Resize { id: String },
    #[error("cannot close terminal")]
    Close { err: std::io::Error, id: String },
}

#[derive(Debug, Error)]
pub enum LaunchError {
    #[error("Invalid object of launch configuration {0:?}")]
    InvalidObject(LaunchObject),
    #[error("Error while launch {0:?}")]
    ErrorWhileLaunch(LaunchObject),
    #[error("error while compile template {0:?}")]
    InvalidTemplate(LaunchTemplate),
    #[error("error while compile reference {0:?}")]
    RefFailed(LaunchTemplate),
    #[error("error while compile object {0:?}")]
    ObjFailed(LaunchTemplate),
    #[error("error while running function {0:?}")]
    RunFn(LaunchFunction),
    #[error("not all objects compiled {0:?}")]
    NotAllObjects(LaunchTemplateReference),
    #[error("error while spawn command")]
    Spawn,
}

#[derive(Debug, Error)]
pub enum SettingsError {
    #[error("Themes config not found")]
    ThemesFileNotFound,
    #[error("Config Error")]
    Config(
        #[source]
        #[from]
        ConfigError,
    ),
    #[error("FS Error")]
    FS(
        #[source]
        #[from]
        FileSystemError,
    ),
    #[error("Parsing Error")]
    Parsing(
        #[source]
        #[from]
        ParsingError,
    ),
}

impl From<LaunchError> for ErrorDto {
    fn from(value: LaunchError) -> Self {
        Self {
            message: format!("{:?}", value),
        }
    }
}

impl From<SettingsError> for ErrorDto {
    fn from(value: SettingsError) -> Self {
        Self {
            message: format!("{:?}", value),
        }
    }
}
impl From<TerminalError> for ErrorDto {
    fn from(value: TerminalError) -> Self {
        Self {
            message: format!("{:?}", value),
        }
    }
}

impl From<ProjectError> for ErrorDto {
    fn from(value: ProjectError) -> Self {
        Self {
            message: format!("{:#}", value),
        }
    }
}
impl From<FileSystemError> for ErrorDto {
    fn from(value: FileSystemError) -> Self {
        Self {
            message: format!("{:#}", value),
        }
    }
}
impl From<ConfigError> for ErrorDto {
    fn from(value: ConfigError) -> Self {
        Self {
            message: format!("{:#}", value),
        }
    }
}
impl From<ParsingError> for ErrorDto {
    fn from(value: ParsingError) -> Self {
        Self {
            message: format!("{:#}", value),
        }
    }
}
