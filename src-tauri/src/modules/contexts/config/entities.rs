use super::values::FsIcon;
use crate::modules::shared::kernel::values::Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FsConfigIcons {
    theme: String,
    icons: Vec<FsIcon>,
    scheme: Schema,
}
