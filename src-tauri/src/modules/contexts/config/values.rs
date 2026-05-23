use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FsIcon {
    //#[serde(rename="type")]
    typ: String,
    ext: Vec<String>,
    #[serde(default)]
    icon: String,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FsType{
    file,
    dir
}


impl Default for FsType {
    fn default() -> Self {
        FsType::file
    }
}