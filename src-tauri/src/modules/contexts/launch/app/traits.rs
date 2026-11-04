use crate::modules::contexts::launch::app::functions::FunctionResult;
use crate::modules::contexts::launch::app::managers::SharedLaunchManager;
use crate::modules::contexts::launch::domain::entities::{
    LaunchFlatTask, LaunchFunction, LaunchObject, LaunchTemplate, LaunchTemplateReference,
    LaunchTemplateResult,
};
use crate::modules::contexts::project::domain::entities::{
    PackageAction, Project, ProjectTemplate, Var,
};
use crate::modules::shared::kernel::errors::LaunchError;
use crate::modules::shared::kernel::values::Val;
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

    fn run_function(
        &self,
        function: &LaunchFunction,
        template: &LaunchTemplate,
        results: &HashMap<String, Val>,
    ) -> Option<FunctionResult>;
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
