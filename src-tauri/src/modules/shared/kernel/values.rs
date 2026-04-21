use serde::Deserialize;

#[derive(Clone)]
pub struct Path(String);
impl Path{
    pub fn new(name: &str) -> Self{
        Self {
            0: name.to_string(),
        }
    }
    
    
    pub fn get(&self) -> String{
        self.0
    }
}

pub struct Schema(u8);

#[derive(Clone)]
pub enum Val {
    NUMBER(f64),
    STRING(String),
    BOOL(bool),
    ARRAY(Vec<Val>),
}

#[derive(Clone)]

pub struct IfStatement{
    or: Vec<IfStatementPart>,
    all: Vec<IfStatementPart>
}

pub struct IfStatementPart{
    from: String,
    oper: String,
    value: Val
}

pub enum FileSystemErrorType {
    FileNotExists,
    DirectoryNotExists,
    DirectoryAlreadyExists,
    FileAlreadyExists,
    FilePathParsing,
    DirectoryPathParsing
}


#[derive(Clone)]

pub struct Dependency{
    program: String,
    level: DependencyLevel
}

#[derive(Clone)]

pub enum DependencyLevel{
    CRITICAL,
    CONFLICTS,
    OPTIONAL
}