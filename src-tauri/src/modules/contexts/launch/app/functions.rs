use crate::modules::app::FS_READ_SERVICE;
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::launch::domain::entities::{
    LaunchFunctionArgument, LaunchTemplateResult, LaunchVecType,
};
use crate::modules::shared::kernel::values::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;

fn get_from(addr: Vec<LaunchVecType>, results: &LaunchTemplateResult) -> Option<String> {
    if let LaunchVecType::NUMBER(num) = addr[0] {
        if let LaunchVecType::STRING(str) = addr[1].clone() {
            let sec = results.get(&(num as i8));
            if let Some(sec) = sec {
                let opt = sec.get(&str);
                if let Some(opt) = opt {
                    return Some(opt.clone());
                }
            }
        }
    }
    None
}

pub fn read_from(
    option: LaunchFunctionArgument,
    results: &LaunchTemplateResult,
) -> Option<FunctionResult> {
    if let LaunchFunctionArgument::VEC(val) = option {
        if val.len() != 2 {
            return None;
        }
        let val = get_from(val, &results);
        if let None = val {
            return None;
        }
        let val = val.unwrap();
        let file = PFile::from_path_reg(Path(val));
        let text = FS_READ_SERVICE.read_file(&file);
        if let Err(_) = text {
            return None;
        }
        let text = text.unwrap();
        return Some(FunctionResult::SCALAR(text));
    }
    None
}

#[allow(unused)]
pub fn read_field(
    option: LaunchFunctionArgument,
    fields: Vec<LaunchVecType>,
    results: &LaunchTemplateResult,
) -> Option<FunctionResult> {
    let val = read_from(option, results);
    if let Some(val) = val {
        if let FunctionResult::SCALAR(val) = val {
            let val: Result<Value, _> = serde_json::from_str(&val.as_str());
            if let Err(_) = val {
                return None;
            }
            let mut val = val.unwrap();
            for field in fields.iter() {
                if let LaunchVecType::STRING(str) = field {
                    let val_ = val.get(str);
                    if let Some(val_) = val_ {
                        val = val_.clone()
                    } else {
                        return None;
                    }
                }
            }
            if val.is_string() {
                return Some(FunctionResult::SCALAR(val.to_string()));
            }
        }
    }
    None
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum FunctionResult {
    SCALAR(String),
    VEC(Vec<LaunchVecType>),
}

pub fn read_fields(
    option: LaunchFunctionArgument,
    fields: Vec<LaunchVecType>,
    results: &LaunchTemplateResult,
) -> Option<FunctionResult> {
    let val = read_from(option, results);
    if let Some(val) = val {
        if let FunctionResult::SCALAR(val) = val {
            let val: Result<Value, _> = serde_json::from_str(&val.as_str());
            if let Err(_) = val {
                return None;
            }
            let mut val = val.unwrap();
            for field in fields.iter() {
                if let LaunchVecType::STRING(str) = field {
                    let val_ = val.get(str);
                    if let Some(val_) = val_ {
                        val = val_.clone()
                    } else {
                        return None;
                    }
                }
            }
            if val.is_array() {
                let mut res = Vec::<LaunchVecType>::new();
                let arr = val.as_array().unwrap();
                for i in arr {
                    if i.is_string() {
                        res.push(LaunchVecType::STRING(i.to_string()))
                    } else if i.is_i64() {
                        res.push(LaunchVecType::NUMBER(i.as_i64().unwrap() as i32));
                    }
                }
                return Some(FunctionResult::VEC(res));
            }
        }
    }
    None
}
