use crate::modules::shared::kernel::traits::TError;
use crate::modules::shared::kernel::values::FileSystemErrorType;

pub struct Settings {
    doctype: String,
    version: String,

}


pub struct Package{

}


pub enum LogLevel{
    INFO,
    WARN,
    ERROR,
    CRITICAL
}


pub struct Error<T> {
    message: T,
    level: LogLevel,
    
}

impl<T> Error<T>{
    pub fn new(message: T, level: LogLevel) -> Error<T> {
        Self {
            message,
            level,
        }
    }
}

impl<T> TError<T> for Error<T> {
    fn message(&self) -> T {
        self.message
    }

    fn level(&self) -> LogLevel {
        self.level
    }
}


pub type FileSystemError = Error<FileSystemErrorType>;

