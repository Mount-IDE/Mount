use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Package {}

/*#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    CRITICAL,
}

#[derive(Debug, Clone)]
pub struct Error<T: Default + Debug + Clone> {
    message: T,
    level: LogLevel,
}

impl<T> From<Error<T>> for InvokeError
where
    T: Default + Debug + Clone + Serialize,
{
    fn from(value: Error<T>) -> Self {
        InvokeError::from((value.message, value.level))
    }
}

impl<T: Default + Debug + Clone> Error<T> {
    pub fn new(message: T, level: LogLevel) -> Error<T> {
        Self { message, level }
    }

    pub fn error(message: T) -> Error<T> {
        Self::new(message, LogLevel::ERROR)
    }

    pub fn empty() -> Error<T> {
        Self::new(T::default(), LogLevel::CRITICAL)
    }
}

impl<T: Default + Debug + Clone> TError<T> for Error<T> {
    fn message(&self) -> T {
        self.message.clone()
    }

    fn level(&self) -> LogLevel {
        self.level.clone()
    }
}
pub type ProjectError = Error<ProjectErrorType>;
pub type ConfigError = Error<i32>;
pub type FileSystemError = Error<FileSystemErrorType>;
*/

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorDto{
    pub(crate) message: String
}



