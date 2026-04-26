use std::fmt::Display;
use serde::{Deserialize, Serialize, Deserializer, Serializer, de};
use ts_rs::TS;

#[derive(Clone, Serialize, Debug, Deserialize, TS)]
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


impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get())
    }
}



#[derive(Deserialize, Serialize, Clone, Debug, TS)]
pub struct Schema(pub u8);


#[derive(Clone,Serialize, Deserialize, Debug, TS)]
#[serde(untagged)]
pub enum Val {
    NUMBER(f64),
    STRING(String),
    BOOL(bool),
    ARRAY(Vec<String>),
}

impl Default for Val {
    fn default() -> Self {
        Val::NUMBER(0.0)
    }
}


#[derive(Clone, Debug, TS)]
pub enum ParameterTyp {
    INPUT(String),
    CHECK,
    LIST(Vec<String>),
    FILE(Vec<String>)
}

impl Default for ParameterTyp{
    fn default() -> Self {
        Self::CHECK
    }
}

impl<'de> Deserialize<'de> for ParameterTyp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>
    {
        let v: Vec<&str> = Vec::deserialize(deserializer)?;

        match v.as_slice() {
            ["check"] => Ok(ParameterTyp::CHECK),
            ["input", val] => Ok(ParameterTyp::INPUT(val.to_string())),
            ["list", rest @ ..] => Ok(ParameterTyp::LIST(rest.iter().map(|s| s.to_string()).collect())),
            ["file", rest @ ..] => Ok(ParameterTyp::FILE(rest.iter().map(|s| s.to_string()).collect())),
            _ => Err(de::Error::custom("invalid ParameterTyp"))
        }
    }
}


impl Serialize for ParameterTyp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
    {
        match self {
            ParameterTyp::INPUT(val) =>
                vec!["input", val].serialize(serializer),

            ParameterTyp::CHECK =>
                vec!["check"].serialize(serializer),

            ParameterTyp::LIST(val) =>
                vec!["list"].into_iter().chain(val.iter().map(|s| s.as_str())).collect::<Vec<_>>().serialize(serializer),

            ParameterTyp::FILE(val) =>
                vec!["file"].into_iter().chain(val.iter().map(|s| s.as_str())).collect::<Vec<_>>().serialize(serializer),
        }
    }
}


#[derive(Clone, Serialize, Deserialize, Debug, TS)]
pub struct IfStatement{
    or: Option<Vec<IfStatementPart>>,
    all: Option<Vec<IfStatementPart>>
}

#[derive(Clone, Serialize, Deserialize, Debug, TS)]
pub struct IfStatementPart{
    from: String,
    oper: String,
    value: Val
}



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Dependency{
    program: String,
    level: DependencyLevel
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum DependencyLevel{
    CRITICAL,
    CONFLICTS,
    OPTIONAL
}




