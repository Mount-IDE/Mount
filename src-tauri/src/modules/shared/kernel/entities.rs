use std::fmt::Debug;
use serde::{Deserialize, Serialize};
use crate::modules::shared::kernel::traits::TError;
use crate::modules::shared::kernel::values::FileSystemErrorType;

pub struct Settings {
    doctype: String,
    version: String,

}

#[derive(Serialize, Deserialize, Clone)]
pub struct Package{

}

#[derive(Debug, Clone)]
pub enum LogLevel{
    INFO,
    WARN,
    ERROR,
    CRITICAL
}

#[derive(Debug)]
pub struct Error<T: Default + Debug+Clone> {
    message: T,
    level: LogLevel,
    
}

impl<T: Default+Debug+Clone> Error<T>{
    pub fn new(message: T, level: LogLevel) -> Error<T> {
        Self {
            message,
            level,
        }
    }
    
    pub fn empty() -> Error<T>{
        Self::new(T::default(), LogLevel::CRITICAL)
    }
}

impl<T: Default+Debug+Clone> TError<T> for Error<T> {
    fn message(&self) -> T {
        self.message.clone()
    }

    fn level(&self) -> LogLevel {
        self.level.clone()
    }
}





pub type ProjectError = Error<i32>;
pub type FileSystemError = Error<FileSystemErrorType>;

