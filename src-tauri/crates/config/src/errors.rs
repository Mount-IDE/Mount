use filesystem::domain::FileSystemError;
use parsing::traits::ParsingError;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(unused)]
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
#[allow(unused)]
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
