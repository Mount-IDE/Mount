use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;
use mount_core::domain::Path;

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

pub trait TParsingService {
    fn to_string<T: Serialize>(&self, obj: T) -> Result<String, ParsingError>;

    fn _from_string<T: DeserializeOwned>(&self, obj: &String) -> Result<T, ParsingError>;
}