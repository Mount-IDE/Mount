use crate::modules::app::{APP, LAUNCH_COMPILE_SERVICE, LAUNCH_RUN_SERVICE, PROJECT_SERVICE};
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::launch::app::functions::FunctionResult;
use crate::modules::contexts::launch::app::managers::SharedLaunchManager;
use crate::modules::contexts::launch::app::traits::{TLaunchCompileService, TLaunchRunService};
use crate::modules::contexts::launch::domain::entities::{
    LaunchFlatTask, LaunchFunction, LaunchObject, LaunchReferenceBefore, LaunchTemplate,
    LaunchTemplateReference, LaunchTemplateResult,
};
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::{
    ActionTask, Project, ProjectTemplate, Var,
};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::errors::LaunchError;
use crate::modules::shared::kernel::values::Val;
use macroses::log;
use std::collections::HashMap;
use tauri::{State, Window};

#[tauri::command]
pub fn create_ref(
    #[allow(unused)] results: LaunchTemplateResult,
    l_template: LaunchTemplate,
    template: ProjectTemplate,
    mut project: Project,
) -> Result<(Project, LaunchTemplateReference), ErrorDto> {
    let res = LAUNCH_COMPILE_SERVICE.compile_reference(&l_template, &template);

    if let Some(res) = res {
        project.workspace.launch_references.push(res.clone());
        PROJECT_SERVICE.save_project(&project)?;
        return Ok((project, res));
    }
    Err(LaunchError::RefFailed(l_template).into())
}

#[tauri::command]
pub fn create_object(
    reference: LaunchTemplateReference,
    template: LaunchTemplate,
    vars: Vec<Var>,
    mut project: Project,
) -> Result<(Project, LaunchObject), ErrorDto> {
    let res =
        LAUNCH_COMPILE_SERVICE.compile_object(&reference, &vars, &template, &reference.results);
    if let Some(res) = res {
        project.workspace.launch_objects.push(res.clone());
        PROJECT_SERVICE.save_project(&project)?;
        return Ok((project, res));
    }
    Err(LaunchError::ObjFailed(template).into())
}

#[tauri::command]
pub fn call_function(
    func: LaunchFunction,
    temp: LaunchTemplate,
    results: LaunchTemplateResult,
) -> Result<FunctionResult, ErrorDto> {
    let mut flat_results = HashMap::<String, Val>::new();
    for i in results.iter() {
        for j in i.1.iter() {
            flat_results.insert(format!("{}.{}", i.0, j.0.clone()), j.1.clone());
        }
    }
    let res = LAUNCH_COMPILE_SERVICE.run_function(&func, &temp, &flat_results);
    if let None = res {
        return Err(LaunchError::RunFn(func.clone()).into());
    }
    Ok(res.unwrap())
}

#[tauri::command]
pub fn create_objects(mut project: Project) -> Result<(Project, Vec<LaunchObject>), ErrorDto> {
    let mut res = Vec::<LaunchObject>::new();

    log!("PROJECT:\n{:?}", project.workspace.launch_references);

    let templates = project.workspace.launch_templates.clone();
    let references = project.workspace.launch_references.clone();
    let vars = project.vars.clone();
    let mut tasks = Vec::<ActionTask>::new();
    for ref_ in references.iter() {
        let Some(needed_template) = templates.iter().find(|e| e.id == ref_.template.1) else {
            continue;
        };
        log!("CURENT REF :\n{ref_:?}");
        if let Some(before) = ref_.before.clone() {
            log!("BEFORE ::\n{before:?}");
            let r = before
                .iter()
                .filter_map(|e| {
                    if let LaunchReferenceBefore::OTHER { id } = e {
                        let Some(ref__) = references.iter().find(|e2| e2.id.eq(id)) else {
                            return None;
                        };
                        let Some(tmp) = templates.iter().find(|e2| e2.id == ref__.template.1)
                        else {
                            return None;
                        };
                        return LAUNCH_COMPILE_SERVICE.compile_object(
                            &ref__,
                            &vars,
                            &tmp,
                            &ref__.results,
                        );
                    }
                    return None;
                })
                .map(|e| e.tasks)
                .flatten()
                .collect::<Vec<ActionTask>>();
            log!("AFTER:\n {r:?}");
            r.iter().for_each(|e| tasks.push(e.clone()))
        }

        let obj =
            LAUNCH_COMPILE_SERVICE.compile_object(&ref_, &vars, &needed_template, &ref_.results);
        if let Some(mut obj) = obj {
            if tasks.len() > 0 {
                for j in 0..tasks.len() {
                    obj.tasks.insert(j, tasks[j].clone());
                }
            }
            res.push(obj);
        } else {
            return Err(LaunchError::NotAllObjects(ref_.clone()).into());
        }
    }
    if res.len() == references.len() {
        project.workspace.launch_objects = res.clone();
        // println!("{project:?}");
        PROJECT_SERVICE.save_project(&project)?;
    }
    log!("OBJECT CREATED \n::{res:?}\n\n::{references:?}");
    Ok((project, res))
}

#[tauri::command]
pub fn create_references(
    l_templates: Vec<LaunchTemplate>,
    template: ProjectTemplate,
    mut project: Project,
) -> Result<(Project, Vec<LaunchTemplateReference>), ErrorDto> {
    let mut res = Vec::<LaunchTemplateReference>::new();
    for i in l_templates.iter() {
        let obj = LAUNCH_COMPILE_SERVICE.compile_reference(&i, &template);
        if let Some(obj) = obj {
            res.push(obj);
        }
    }
    project.workspace.launch_references = res.clone();
    PROJECT_SERVICE.save_project(&project)?;
    Ok((project, res))
}

#[tauri::command]
pub async fn launch_tasks(
    tasks: Vec<ActionTask>,
    project: Project,
    window: Window,
    state: State<'_, SharedLaunchManager>,
) -> Result<(), ErrorDto> {
    println!("start tasks");
    let id = window.label();
    let app = APP.get();
    if let Some(app) = app {
        log!("TASKS1 {tasks:?}");

        let tasks = tasks
            .into_iter()
            .map(|e| {
                let cwd = e.cwd;
                return e.commands.into_iter().map(move |e2| {
                    return LaunchFlatTask {
                        command: e2.command,
                        env: e2.env,
                        cwd: Some(cwd.clone()),
                    };
                });
            })
            .flatten()
            .collect::<Vec<LaunchFlatTask>>();

        log!("TASKS {tasks:?}");
        for i in tasks.iter() {
            let _ = LAUNCH_RUN_SERVICE
                .launch_task(i, id, &project, &app, &state)
                .await;
        }
        Ok(())
    } else {
        Err(ErrorDto {
            message: "app not found".to_string(),
        })
    }
}

#[tauri::command]
pub async fn launch_task(
    task: LaunchFlatTask,
    project: Project,
    window: Window,
    state: State<'_, SharedLaunchManager>,
) -> Result<String, ErrorDto> {
    println!("start task");
    let id = window.label();
    let app = APP.get();
    if let Some(app) = app {
        LAUNCH_RUN_SERVICE
            .launch_task(&task, id, &project, &app, &state)
            .await
            .map_err(|e| e.into())
    } else {
        Err(ErrorDto {
            message: "".to_string(),
        })
    }
}

#[tauri::command]
pub async fn write_launch(
    id: String,
    text: String,
    state: State<'_, SharedLaunchManager>,
) -> Result<(), ErrorDto> {
    LAUNCH_RUN_SERVICE.write_to_launch(id, text, state).await;
    Ok(())
}
#[tauri::command]
pub async fn close_launch(
    id: String,
    state: State<'_, SharedLaunchManager>,
    window: Window,
) -> Result<(), ErrorDto> {
    println!("close task");
    let res = LAUNCH_RUN_SERVICE
        .close_task(id, state, window.label())
        .await;
    if let Err(_) = res {
        return Err(ErrorDto {
            message: "".to_string(),
        });
    }
    Ok(())
}

#[tauri::command]
pub async fn close_window_launches(
    state: State<'_, SharedLaunchManager>,
    window: Window,
) -> Result<(), ErrorDto> {
    LAUNCH_RUN_SERVICE
        .close_window_tasks(state, window.label())
        .await
        .map_err(|_| ErrorDto {
            message: "".__get(),
        })?;
    Ok(())
}
