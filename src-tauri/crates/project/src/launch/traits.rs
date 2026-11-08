use crate::launch::domain::{
    LaunchError, LaunchFlatTask, LaunchObject, LaunchTemplate, LaunchTemplateReference,
    LaunchTemplateResult,
};
use crate::launch::managers::SharedLaunchManager;
use crate::project::domain::{Project, ProjectTemplate};
use declarative::domain::PackageAction;
use mount_core::domain::{Val, Var};
use std::collections::HashMap;
use tauri::{AppHandle, State};

pub trait TLaunchCompileService {
    fn compile_reference(
        &self,
        l_template: &LaunchTemplate,
        template: &ProjectTemplate,
    ) -> Option<LaunchTemplateReference>;
    fn compile_object(
        &self,
        reference: &LaunchTemplateReference,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &LaunchTemplateResult,
    ) -> Option<LaunchObject>;

    fn compile_condition(
        &self,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &HashMap<String, Val>,
    ) -> Vec<PackageAction>;

    /* fn run_function(
        &self,
        function: &LaunchFunction,
        template: &LaunchTemplate,
        results: &HashMap<String, Val>,
    ) -> Option<FunctionResult>;*/
}
pub trait TLaunchRunService {
    async fn launch_task(
        &self,
        task: &LaunchFlatTask,
        window_id: &str,
        project: &Project,
        app: &AppHandle,
        state: &State<'_, SharedLaunchManager>,
    ) -> Result<String, LaunchError>;

    async fn launch_object(
        &self,
        object: &LaunchObject,
        objects: &Vec<LaunchObject>,
        current: &LaunchTemplateReference,
        project: &Project,
        window: &String,
        app: &AppHandle,
        state: &State<'_, SharedLaunchManager>,
    ) -> Result<(), LaunchError>;

    async fn write_to_launch(
        &self,
        id: String,
        text: String,
        state: State<'_, SharedLaunchManager>,
    );

    async fn close_task(
        &self,
        id: String,
        state: State<'_, SharedLaunchManager>,
        label: &str,
    ) -> Result<(), ()>;

    async fn close_window_tasks(
        &self,
        state: State<'_, SharedLaunchManager>,
        label: &str,
    ) -> Result<(), ()>;
}
