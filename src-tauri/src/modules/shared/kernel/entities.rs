use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone)]
pub struct Package {}



#[derive(Serialize, Deserialize, Clone)]
pub struct ErrorDto{
    pub(crate) message: String
}



