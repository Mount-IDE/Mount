use crate::modules::app::{APP, LAUNCH_COMPILE_SERVICE, LAUNCH_RUN_SERVICE, PROJECT_SERVICE};
use crate::modules::contexts::launch::app::functions::FunctionResult;
use crate::modules::contexts::launch::app::managers::SharedLaunchManager;
use crate::modules::contexts::launch::app::traits::{TLaunchCompileService, TLaunchRunService};
use crate::modules::contexts::launch::domain::entities::{
    LaunchFlatTask, LaunchFunction, LaunchObject, LaunchTemplate, LaunchTemplateReference,
    LaunchTemplateResult,
};
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::{Project, ProjectTemplate, Var};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::errors::LaunchError;
use tauri::{State, Window};

#[tauri::command]
pub fn create_ref(
    results: LaunchTemplateResult,
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
    let res = LAUNCH_COMPILE_SERVICE.run_function(&func, &temp, &results);
    if let None = res {
        return Err(LaunchError::RunFn(func.clone()).into());
    }
    Ok(res.unwrap())
}

#[tauri::command]
pub fn create_objects(
    references: Vec<LaunchTemplateReference>,
    template: LaunchTemplate,
    vars: Vec<Var>,
    mut project: Project,
) -> Result<(Project, Vec<LaunchObject>), ErrorDto> {
    let mut res = Vec::<LaunchObject>::new();
    for i in references.iter() {
        let obj = LAUNCH_COMPILE_SERVICE.compile_object(&i, &vars, &template, &i.results);
        if let Some(obj) = obj {
            res.push(obj);
        } else {
            return Err(LaunchError::NotAllObjects(i.clone()).into());
        }
    }
    if res.len() == references.len() {
        project.workspace.launch_objects = res.clone();
        // println!("{project:?}");
        PROJECT_SERVICE.save_project(&project)?;
    }
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
pub async fn launch_task(
    task: LaunchFlatTask,
    project: Project,
    window: Window,
    state: State<'_, SharedLaunchManager>,
) -> Result<String, ErrorDto> {
    let id = window.label();
    let app = APP.get();
    println!("APP {}", app.is_some());
    if let Some(app) = app {
        LAUNCH_RUN_SERVICE
            .launch_task(task, id.to_string(), project, app.clone(), state)
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
) -> Result<(), ErrorDto> {
    LAUNCH_RUN_SERVICE.close_task(id, state).await;
    Ok(())
}
