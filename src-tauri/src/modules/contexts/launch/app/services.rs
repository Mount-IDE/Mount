use crate::modules::app::ACTION_PROJECT_SERVICE;
use crate::modules::contexts::launch::app::functions::{read_fields, read_from, FunctionResult};
use crate::modules::contexts::launch::app::traits::TLaunchCompileService;
use crate::modules::contexts::launch::domain::entities::{
    LaunchAction, LaunchFunction, LaunchFunctionArgument, LaunchObject, LaunchTask, LaunchTemplate,
    LaunchTemplateReference, LaunchTemplateResult,
};
use crate::modules::contexts::project::app::traits::TActionProjectService;
use crate::modules::contexts::project::domain::entities::{Action, ProjectTemplate, Var};
use crate::modules::shared::kernel::values::Val;
use regex::Regex;

#[allow(unused)]
pub struct LaunchCompileService();

#[allow(unused)]
const VAR_PREFIX: &str = "@";
#[allow(unused)]
const PARAM_PREFIX: &str = "#";

impl TLaunchCompileService for LaunchCompileService {
    fn compile_reference(
        &self,
        l_template: &LaunchTemplate,
        template: &ProjectTemplate,
        results: &LaunchTemplateResult,
    ) -> Option<LaunchTemplateReference> {
        let meta = results.get(&-1);
        if let None = meta {
            return None;
        }
        let meta = meta.unwrap();
        let name = meta.get("name");
        if let None = name {
            return None;
        }
        let name = name.unwrap();
        let obj = LaunchTemplateReference {
            id: 0,
            template: (template.id.clone(), l_template.id.clone()),
            scheme: Default::default(),
            name: name.clone(),
            results: results.clone(),
        };

        Some(obj)
    }

    fn compile_object(
        &self,
        reference: &LaunchTemplateReference,
        vars: &Vec<Var>,
        template: LaunchTemplate,
        results: LaunchTemplateResult,
    ) -> Option<LaunchObject> {
        let passed = self.compile_condition(&vars, &template, &results);
        if passed.len() == 0 {
            return None;
        }
        let mut obj = LaunchObject {
            id: 0,
            launch_reference: reference.id,
            scheme: Default::default(),
            tasks: vec![],
        };

        for act in passed.iter() {
            let res = self.compile_task(&vars, &passed, &act, &results);
            if let Some(val) = res {
                obj.tasks.push(val);
            }
        }
        Some(obj)
    }

    fn get_from_results(&self, results: &LaunchTemplateResult, addr: String) -> Option<Val> {
        let mut address = addr.clone();
        if addr.starts_with(PARAM_PREFIX) {
            address = addr
                .chars()
                .skip(PARAM_PREFIX.chars().count())
                .collect::<String>();
        }
        let address = Action::getaddr(address);
        if let Ok((section, parameter)) = address {
            let parameter = parameter.chars().skip(1).collect::<String>();

            // for sect in results {
            //
            // }
            let sect = results.get(&section);
            if let Some(sect) = sect {
                let param = sect.get(&parameter);
                if let Some(param) = param {
                    return Some(Val::STRING(param.clone()));
                }
            }
        }
        None
    }

    fn compile_condition(
        &self,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &LaunchTemplateResult,
    ) -> Vec<LaunchAction> {
        let mut actions = Vec::<LaunchAction>::new();
        for action in template.actions.iter() {
            let mut passed_ = true;
            'cond: for cond in action.if_.iter() {
                //and
                let mut passed = 0;
                'part: for obj in cond.iter() {
                    //or
                    // let from = obj.from;
                    let mut from = Val::STRING(obj.from.clone());
                    if obj.from.starts_with(VAR_PREFIX) {
                        let res = ACTION_PROJECT_SERVICE.get_from_vars(vars, obj.from.clone());
                        if let Some(val) = res {
                            from = val;
                        } else {
                            continue 'part;
                        }
                    } else if obj.from.starts_with(PARAM_PREFIX) {
                        let res = self.get_from_results(&results, obj.from.clone());
                        if let Some(res) = res {
                            from = res;
                        } else {
                            continue 'part;
                        }
                    }

                    let op = match obj.oper.clone().as_str() {
                        "==" => |a: Val, b: Val| -> bool { a == b },
                        ">" => |a: Val, b: Val| -> bool { a > b },
                        "<" => |a: Val, b: Val| -> bool { a < b },
                        ">=" => |a: Val, b: Val| -> bool { a >= b },
                        "<=" => |a: Val, b: Val| -> bool { a <= b },
                        "!=" => |a: Val, b: Val| -> bool { a != b },
                        "in" => |a: Val, b: Val| -> bool {
                            if let Val::ARRAY(val) = b {
                                if let Val::STRING(elem) = a {
                                    return val.contains(&elem);
                                }
                            }
                            false
                        },
                        "reg" => |a: Val, b: Val| -> bool {
                            if let Val::STRING(reg_) = b {
                                if let Val::STRING(a) = a {
                                    let reg = Regex::new(reg_.as_str());
                                    if let Err(_) = reg {
                                        return false;
                                    }
                                    return reg.unwrap().is_match(a.as_str());
                                }
                            }
                            false
                        },
                        "!reg" => |a: Val, b: Val| -> bool {
                            if let Val::STRING(reg_) = b {
                                if let Val::STRING(a) = a {
                                    let reg = Regex::new(reg_.as_str());
                                    if let Err(_) = reg {
                                        return false;
                                    }
                                    return !reg.unwrap().is_match(a.as_str());
                                }
                            }
                            false
                        },
                        "!in" => |a: Val, b: Val| -> bool {
                            if let Val::ARRAY(val) = b {
                                if let Val::STRING(elem) = a {
                                    return !val.contains(&elem);
                                }
                            }
                            false
                        },
                        "len" => |a: Val, b: Val| -> bool {
                            let val = match a {
                                Val::NUMBER(v) => Val::NUMBER(v),
                                Val::STRING(v) => Val::NUMBER(v.chars().count() as f64),
                                Val::BOOL(v) => Val::NUMBER(if v { 1f64 } else { 0f64 }),
                                Val::ARRAY(v) => Val::NUMBER(v.len() as f64),
                            };
                            return val == b;
                        },
                        "!empty" => |a: Val, _: Val| -> bool {
                            match a {
                                Val::STRING(v) => !v.is_empty(),
                                Val::ARRAY(v) => !v.is_empty(),
                                _ => false,
                            }
                        },
                        "empty" => |a: Val, _: Val| -> bool {
                            match a {
                                Val::STRING(v) => v.is_empty(),
                                Val::ARRAY(v) => v.is_empty(),
                                _ => false,
                            }
                        },
                        _ => |_: Val, _: Val| -> bool { false },
                    };

                    let mut val = obj.value.clone();
                    if let Val::STRING(val_) = val.clone() {
                        if val_.starts_with(PARAM_PREFIX) {
                            val = self.get_from_results(&results, val_.clone()).unwrap();
                        }
                        if val_.starts_with(VAR_PREFIX) {
                            val = ACTION_PROJECT_SERVICE
                                .get_from_vars(&vars, val_.clone())
                                .unwrap();
                        }
                    }
                    let res = op(from, val);
                    if res {
                        passed += 1;
                        break 'part;
                    }
                }
                if passed == 0 {
                    passed_ = false;
                    break 'cond;
                }
            }
            if passed_ {
                actions.push(action.clone());
            }
        }
        actions
    }

    fn compile_task(
        &self,
        vars: &Vec<Var>,
        actions: &Vec<LaunchAction>,
        action: &LaunchAction,
        results: &LaunchTemplateResult,
    ) -> Option<LaunchTask> {
        if let None = action.command.clone() {
            return None;
        }
        let act = action.command.clone();
        if let None = act.clone() {
            return None;
        }

        let act = act.unwrap();
        let mut command = act.command.clone();
        if let Some(args) = act.args {
            for arg in args.iter() {
                let res = self.format_string(arg.clone(), &vars, &results);
                if let None = res {
                    return None;
                }
                command = format!("{command} {}", res.unwrap());
            }
        }
        if let Some(val) = action.next {
            let act_ = actions.iter().find(|e| e.id == val);
            if let Some(act_) = act_ {
                let task = self.compile_task(&vars, &actions, act_, &results);
                if let Some(task) = task {
                    let obj = LaunchTask::GRAPH {
                        command,
                        next: Box::new(task),
                        env: act.env.clone(),
                        cwd: act.cwd.clone(),
                    };
                    return Some(obj);
                }
            }
        }
        let obj = LaunchTask::SINGLE {
            command,
            env: act.env.clone(),
            cwd: act.cwd.clone(),
        };
        Some(obj)
    }

    fn format_string(
        &self,
        input: String,
        vars: &Vec<Var>,
        params: &LaunchTemplateResult,
    ) -> Option<String> {
        #[derive(Debug, Clone, Copy)]
        enum State {
            Normal,
            Var,
            Param,
        }

        let mut result = String::new();
        let mut state = State::Normal;

        let mut buf = String::new();
        let mut brace_mode = false;

        let mut chars = input.chars().peekable();

        while let Some(ch) = chars.next() {
            match state {
                State::Normal => {
                    if ch == VAR_PREFIX.chars().next().unwrap() {
                        state = State::Var;
                        buf.clear();
                        brace_mode = false;
                    } else if ch == PARAM_PREFIX.chars().next().unwrap() {
                        state = State::Param;
                        buf.clear();
                        brace_mode = false;
                    } else {
                        result.push(ch);
                    }
                }

                State::Var | State::Param => {
                    // вход в {token}
                    if ch == '{' && buf.is_empty() {
                        brace_mode = true;
                        continue;
                    }

                    if brace_mode {
                        if ch == '}' {
                            let value = if let State::Var = state {
                                ACTION_PROJECT_SERVICE.get_from_vars(vars, format!("@{}", buf))?
                            } else {
                                self.get_from_results(params, format!("#{}", buf))?
                            };

                            result.push_str(&value.to_str());

                            state = State::Normal;
                            buf.clear();
                            brace_mode = false;
                            continue;
                        } else {
                            buf.push(ch);
                        }
                    } else {
                        // без {} — читаем до разделителя
                        if ch.is_whitespace() {
                            let value = if let State::Var = state {
                                ACTION_PROJECT_SERVICE.get_from_vars(vars, format!("@{}", buf))?
                            } else {
                                self.get_from_results(params, format!("#{}", buf))?
                            };

                            result.push_str(&value.to_str());
                            result.push(' ');

                            state = State::Normal;
                            buf.clear();
                        } else {
                            buf.push(ch);
                        }
                    }
                }
            }
        }

        // flush tail
        match state {
            State::Normal => {}
            State::Var => {
                if !buf.is_empty() {
                    let value = ACTION_PROJECT_SERVICE.get_from_vars(vars, format!("@{}", buf))?;
                    result.push_str(&value.to_str());
                }
            }
            State::Param => {
                if !buf.is_empty() {
                    let value = self.get_from_results(params, format!("#{}", buf))?;
                    result.push_str(&value.to_str());
                }
            }
        }

        Some(result)
    }

    fn run_function(
        &self,
        function: &LaunchFunction,
        template: &LaunchTemplate,
        results: &LaunchTemplateResult,
    ) -> Option<FunctionResult> {
        let mut first: LaunchFunctionArgument = LaunchFunctionArgument::STRING(Default::default());
        let mut seq = false;

        let mut result = FunctionResult::SCALAR(Default::default());
        for act in function.actions.iter() {
            let res: Option<FunctionResult> = match act.function.as_str() {
                "read_from" => {
                    if let Some(args) = &act.args {
                        if args.len() == 1 {
                            let arg = if seq { first.clone() } else { args[0].clone() };
                            read_from(arg.clone(), &results)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "read_fields" => {
                    if let Some(args) = &act.args {
                        if args.len() == 2 {
                            if let LaunchFunctionArgument::VEC(fields) = args[1].clone() {
                                let arg = if seq { first.clone() } else { args[0].clone() };
                                read_fields(arg.clone(), fields.clone(), &results)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                }
                "return" => {
                    if seq {
                        match first.clone() {
                            LaunchFunctionArgument::STRING(v) => result = FunctionResult::SCALAR(v),
                            LaunchFunctionArgument::VEC(v) => result = FunctionResult::VEC(v),
                        }
                        return Some(result);
                    } else {
                        None
                    }
                }
                _ => None,
            };
            if !seq {
                seq = true;
            }
            if let Some(val) = res {
                if let FunctionResult::SCALAR(s) = val.clone() {
                    first = LaunchFunctionArgument::STRING(s)
                }
                if let FunctionResult::VEC(v) = val {
                    first = LaunchFunctionArgument::VEC(v)
                }
            }
        }
        None
    }
}
