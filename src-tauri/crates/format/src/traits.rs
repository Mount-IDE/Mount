use std::collections::HashMap;
use thiserror::Error;
use mount_core::domain::Val;
use mount_core::domain::Var;

pub trait TFormatService {
    fn format_string(
        &self,
        string: String,
        vars: &Vec<Var>,
        results: &HashMap<String, Val>,
    ) -> Result<String, FormatError>;
}


#[derive(Error, Debug)]
#[allow(unused)]
pub enum FormatError {
    #[error("Invalid declarator `{0}` of section")]
    InvalidDeclarator(char),
    #[error("Needed variable not found ({0})")]
    NeededVarNotFound(String),
    #[error("Needed result not found ({0})")]
    NeededResNotFound(String),
}