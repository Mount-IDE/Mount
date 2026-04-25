use crate::modules::contexts::filesystem::app::utils::split_path;
use crate::modules::contexts::filesystem::domain::values::FileType;
use crate::modules::shared::kernel::values::Path;
#[derive(Clone)]
pub struct PFile {
    pub name: String,
    pub path: Path,
    pub typ: FileType,
}

impl PFile {
    pub fn new() -> PFile {
        Self {
            name: String::new(),
            path: Path(String::new()),
            typ: FileType::REGULAR,
        }
    }

    pub fn regular(name: String, path: Path) -> PFile {
        Self {
            name,
            path,
            typ: FileType::REGULAR,
        }
    }
    pub fn binary(name: String, path: Path) -> PFile {
        Self {
            name,
            path,
            typ: FileType::REGULAR,
        }
    }

    pub fn ext(&self)->Option<String> {
        let found = self.name.rfind(".");
        if found.is_some() {
            let found = found.unwrap();
            let slice = &self.name.clone()[found+1..];
            return Some(slice.to_string());
        }
        None
    }
    
    pub fn from_path_reg(path: Path) -> PFile {
        let path_ = split_path(&path);
        if path_.len() == 0 {
            return Self {
                name: "".to_string(),
                path: Path(String::new()),
                typ: FileType::REGULAR,
            };
        }
        let name = path_.get(path_.len() - 1);
        if name.is_none() {
            return Self {
                name: "".to_string(),
                path: Path(String::new()),
                typ: FileType::REGULAR,
            };
        }
        let name = name.unwrap();
        Self {
            name: name.to_string(),
            path,
            typ: FileType::REGULAR,
        }
    }
}

#[derive(Clone)]

pub struct PDirectory {
    pub name: String,
    pub path: Path,
    pub files: Vec<PFile>,
    pub directories: Vec<PDirectory>,
}
impl PDirectory {
    pub fn new() -> PDirectory {
        Self {
            name: String::new(),
            path: Path(String::new()),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
    
    pub fn from_path(path: &Path) -> PDirectory {
        let path_ = split_path(&path);
        if path_.len() == 0 {
            return PDirectory::new();
        }
        let name = path_.get(path_.len() - 1);
        if name.is_none() {
            return PDirectory::new();
        }
        let name = name.unwrap();
        Self {
            name: name.to_string(),
            path: path.clone(),
            files: Vec::new(),
            directories: Vec::new(),
        }
    }
}
