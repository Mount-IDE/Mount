use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FsIcon {
    //#[serde(rename="type")]
    typ: String,
    ext: Vec<String>,
    #[serde(default)]
    icon: String,
}