use serde::{Deserialize, Serialize, Serializer};

#[derive(Clone, Deserialize)]
pub struct Path(pub String);
impl Path{
    pub fn new(name: &str) -> Self{
        Self {
            0: name.to_string(),
        }
    }


    pub fn get(&self) -> String{
        self.0.clone()
    }
}


impl Serialize for Path {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.collect_str(&self.0)
    }
}

#[derive(Deserialize, Clone)]
pub struct Schema(pub u8);
impl Serialize for Schema {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        serializer.serialize_u8(self.0)
    }
}

#[derive(Clone, Deserialize)]
pub enum Val {
    NUMBER(f64),
    STRING(String),
    BOOL(bool),
    ARRAY(Vec<Val>),
}

impl Serialize for Val {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self{
            Val::NUMBER(val)=>serializer.serialize_f64(*val),
            Val::STRING(val)=>serializer.serialize_str(val),
            Val::BOOL(val)=>serializer.serialize_bool(*val),
            Val::ARRAY(val)=>serializer.collect_seq(val)
        }
    }
}


#[derive(Clone, Serialize, Deserialize)]
pub struct IfStatement{
    or: Vec<IfStatementPart>,
    all: Vec<IfStatementPart>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IfStatementPart{
    from: String,
    oper: String,
    value: Val
}

#[derive(Debug, Clone)]
pub enum FileSystemErrorType {
    FileCreationError,
    DirectoryCreationError,
    FileRemovingError,
    DirectoryRemovingError,
    FileWritingError,
    FileNotExists,
    DirectoryNotExists,
    DirectoryAlreadyExists,
    FileAlreadyExists,
    FilePathParsing,
    DirectoryPathParsing,
    DEFAULT
}

impl Default for FileSystemErrorType{
    fn default() -> Self {
        Self::DEFAULT
    }
}



#[derive(Clone, Serialize, Deserialize)]
pub struct Dependency{
    program: String,
    level: DependencyLevel
}

#[derive(Clone, Serialize, Deserialize)]
pub enum DependencyLevel{
    CRITICAL,
    CONFLICTS,
    OPTIONAL
}