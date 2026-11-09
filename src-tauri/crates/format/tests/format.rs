use format::FORMAT_SERVICE;
use format::traits::TFormatService;
use macroses::log;
use mount_core::domain::{Val, Var};
use mount_core::traits::PathPart;
use std::collections::HashMap;

#[test]
pub fn format_empty() {
    let str = "dddl";

    let res = FORMAT_SERVICE.format_string(str.to_string(), &vec![], &HashMap::new());
    assert_eq!(res.unwrap(), str);
}

#[test]
pub fn format_vars() {
    let str = "dddl  @{var}";
    let str_res = "dddl  none";
    let vars: Vec<Var> = vec![Var::new("var", Val::NONE)];
    let res = FORMAT_SERVICE.format_string(str.to_string(), &vars, &HashMap::new());
    assert_eq!(res.unwrap(), str_res);
}

#[test]
pub fn format_vars_results() {
    let str = "dddl @{var} #dd @{var}";
    let str_res = "dddl none 2 none";
    let vars = vec![Var::new("var", Val::NONE)];
    let results = HashMap::from([("dd".__get(), Val::NUMBER(2.))]);

    let res = FORMAT_SERVICE.format_string(str.to_string(), &vars, &results);
    log!("FORMAT: {res:?}");
    assert_eq!(res.unwrap(), str_res);
}
