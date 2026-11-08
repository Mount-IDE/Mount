use config::errors::ConfigError;
use filesystem::domain::FileSystemError;
use mount_core::domain::Dependency;
use parsing::traits::ParsingError;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(unused)]
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
    #[error("not all dependencies suplied")]
    NotAllDependenciesSuplied(Vec<Dependency>),
}
