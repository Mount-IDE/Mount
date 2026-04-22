use crate::modules::shared::kernel::values::Path;
use crate::modules::contexts::filesystem::domain::values::FileType;
#[derive(Clone)]
pub struct PFile {
    pub name: String,
    pub path: Path,
    pub typ: FileType,
}


#[derive(Clone)]

pub struct PDirectory {
    pub name: String,
    pub path: Path,
    pub files: Vec<PFile>,
    pub directories: Vec<PDirectory>,
}




