use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorDto {
    pub message: String,
}

impl<T: Debug> From<T> for ErrorDto {
    fn from(value: T) -> Self {
        Self {
            message: format!("{:?}", value),
        }
    }
}

/*

impl From<FileSystemError> for ErrorDto {
    fn from(value: FileSystemError) -> Self {
        Self{message: format!("{:?}", value)}
    }
}


impl From<ProjectError> for ErrorDto {
    fn from(value: ProjectError) -> Self {
        Self{message: format!("{:?}", value)}
    }
}

impl From<LaunchError> for ErrorDto{
    fn from(value: LaunchError) -> Self {
        Self{message: format!("{:?}", value)}
    }
}*/
