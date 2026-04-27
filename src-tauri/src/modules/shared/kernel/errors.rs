use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("config error that can`t describe as project error")]
    ConfigError(#[from] ConfigError),
    #[error("filesystem error that can`t describe as project error")]
    FileSystemError(#[from] FileSystemError),
    
    #[error("project already exists")]
    AlreadyExists,
    #[error("project creation failed")]
    CreationFailed {
        #[source]
        source: FileSystemError,
    },

    #[error("failed to parse in project: {err}")]
    ParsingError{#[from]err: ParsingError},
    
    
    #[error("failed to get meta section (probably not set name or path)")]
    MetaNotFound,
    #[error("failed to get meta.-4 section (with name and path)")]
    MainMetaNotFound,
    #[error("failed to get name of project")]
    NameNotFound,
    #[error("failed to get project path")]
    PathNotFound,
    
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
}

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to get app_data_dir")]
    GetDataDir {
        #[source]
        err: tauri::Error,
    },
    #[error("global settings not found")]
    SettingsNotFound {
        #[source]
        err: FileSystemError,
    },
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
    HomeDir{
        #[source]
        err: tauri::Error
    }
    
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
