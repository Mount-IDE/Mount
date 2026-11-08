use serde::de::DeserializeOwned;
use serde::Serialize;
use mount_core::domain::Path;
use crate::traits::{ParsingError, TParsingService};

pub struct ParsingService;

impl TParsingService for ParsingService {
    fn to_string<T: Serialize>(&self, obj: T) -> Result<String, ParsingError> {
        let res: String = serde_json::to_string(&obj).map_err(|e| ParsingError::Serialize {
            path: Default::default(),
            err: e,
        })?;

        Ok(res)
    }

    fn _from_string<T: DeserializeOwned>(&self, obj: &String) -> Result<T, ParsingError> {
        let res = serde_json::from_str::<T>(&obj).map_err(|e| ParsingError::Deserialize {
            path: Path::new(""),
            json: obj.to_string(),
            err: e,
        });

        if let Err(v) = res {
            println!("___\nerror\n___|||{:?}", v);

            return Err(v);
        }
        Ok(res.unwrap())
    }
}