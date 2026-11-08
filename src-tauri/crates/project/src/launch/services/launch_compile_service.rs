use crate::launch::domain::{
    LaunchObject, LaunchTemplate, LaunchTemplateReference, LaunchTemplateResult,
};
use crate::launch::traits::TLaunchCompileService;
use crate::project::domain::ProjectTemplate;
use declarative::domain::{ActionTask, PackageAction};
use macroses::log;
use mount_core::domain::{Val, Var};
use rand::RngExt;
use std::collections::HashMap;

#[allow(unused)]
pub struct LaunchCompileService;

impl TLaunchCompileService for LaunchCompileService {
    fn compile_reference(
        &self,
        l_template: &LaunchTemplate,
        template: &ProjectTemplate,
    ) -> Option<LaunchTemplateReference> {
        let obj = LaunchTemplateReference {
            id: 0,
            template: (template.id.clone(), l_template.id.clone()),
            scheme: Default::default(),
            icon: l_template.icon.clone(),
            name: l_template.title.clone(),
            results: HashMap::new(),
            before: None,
        };

        Some(obj)
    }

    fn compile_object(
        &self,
        reference: &LaunchTemplateReference,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &LaunchTemplateResult,
    ) -> Option<LaunchObject> {
        let mut flat_results = HashMap::<String, Val>::new();
        for i in results.iter() {
            for j in i.1.iter() {
                flat_results.insert(format!("{}.{}", i.0, j.0.clone()), j.1.clone());
            }
        }

        log!("RESULTS {results:?} {template:?}");
        let Some(needed) = results.get(&template.id) else {
            log!("NONE1");
            return None;
        };
        let passed = self.compile_condition(&vars, &template, &flat_results);
        log!("{passed:?}");
        if passed.len() == 0 {
            log!("NONE2");
            return None;
        }
        let mut rng = rand::rng();
        let num: i32 = rng.random();

        let actions = passed
            .iter()
            .filter_map(|e| {
                e.compile(&passed, &vars, &flat_results, &mut vec![], false)
                    .ok()
            })
            .flatten()
            .collect::<Vec<ActionTask>>();
        let before = if let Some(_) = reference.before.clone() {
            Some(Vec::<i32>::new())
        } else {
            None
        };
        let obj = LaunchObject {
            id: num,
            launch_reference: reference.id,
            scheme: Default::default(),
            tasks: actions,
            before,
        };
        log!("COMPILED :: {reference:?}\n :: {obj:?}");
        Some(obj)
    }

    fn compile_condition(
        &self,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &HashMap<String, Val>,
    ) -> Vec<PackageAction> {
        println!("\n\n\t{results:?} {template:?}");

        let actions = &template.actions;
        actions
            .iter()
            .filter(|e| {
                let a = e.check_condition(&results, &vars) && e.check_platform();
                println!("\t {a}");
                return a;
            })
            .cloned()
            .collect()
    }

    /*  fn run_function(
        &self,
        _function: &LaunchFunction,
        _template: &LaunchTemplate,
        _results: &HashMap<String, Val>,
    ) -> Option<FunctionResult> {
        unimplemented!()
        /*let mut first: LaunchFunctionArgument = LaunchFunctionArgument::STRING(Default::default());
        let mut seq = false;

        let result: FunctionResult;
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
        None*/
    }*/
}
