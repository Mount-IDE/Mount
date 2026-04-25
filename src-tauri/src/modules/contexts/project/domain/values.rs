use std::fmt::Debug;
use serde::{Deserialize, Serialize, Serializer};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct ProjectMeta{
    authors: Vec<String>,
    description: String,
    license: Option<String>,
    group: String,
    tags: Vec<String>,
    
}

impl ProjectMeta{
    pub fn new()->Self{
        Self {
            authors: Vec::new(),
            description: String::new(),
            license: None,
            group: String::new(),
            tags: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct PackageMeta {
    authors: Vec<String>,
    description: String,
}

impl PackageMeta{
    pub fn new()->Self{
        Self {
            authors: Vec::new(),
            description: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct TemplateMeta {
    pub authors: Vec<String>,
    pub description: String,
    pub icon: String
}

#[derive(Deserialize, Clone, Debug, TS)]
pub enum ParameterLabel{
    STR(String),
    COUPLE((String, String)),
}

impl Serialize for ParameterLabel{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            ParameterLabel::STR(s) => serializer.serialize_str(s),
            ParameterLabel::COUPLE((c1,c2)) =>serializer.collect_seq(vec![c1, c2]),
        }
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct ActionCommand{
    shell: String,
    env: Option<Vec<(String, String)>>,
    command: ActionCommandIn
}

impl ActionCommand{
    pub fn new()->ActionCommand{
        Self{
            shell: String::new(),
            env: None,
            command: ActionCommandIn::Single(String::new())
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub enum ActionCommandIn{
    Single(String),
    WithArgs(String, Vec<String>)
}



#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub enum ButtonPos{
    LeftTop,
    LeftBottom,
    RightTop,
}


