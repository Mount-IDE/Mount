use serde::{Serialize, Deserialize};
use crate::modules::shared::kernel::values::Schema;
use super::values::FsIcon;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsConfigIcons {
    theme: String,
    icons: Vec<FsIcon>,
    scheme: Schema
}