use serde::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Package {}

#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorDto {
    pub(crate) message: String,
}
