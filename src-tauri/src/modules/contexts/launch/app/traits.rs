use crate::modules::contexts::launch::app::functions::FunctionResult;
use crate::modules::contexts::launch::app::managers::SharedLaunchManager;
use crate::modules::contexts::launch::domain::entities::{
    LaunchAction, LaunchFlatTask, LaunchFunction, LaunchObject, LaunchTask, LaunchTemplate,
    LaunchTemplateReference, LaunchTemplateResult,
};
use crate::modules::contexts::project::domain::entities::{Project, ProjectTemplate, Var};
use crate::modules::shared::kernel::errors::LaunchError;
use crate::modules::shared::kernel::values::Val;
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

    fn get_from_results(&self, results: &LaunchTemplateResult, addr: String) -> Option<Val>;
    fn compile_condition(
        &self,
        vars: &Vec<Var>,
        template: &LaunchTemplate,
        results: &LaunchTemplateResult,
    ) -> Vec<LaunchAction>;

    fn compile_task(
        &self,
        vars: &Vec<Var>,
        actions: &Vec<LaunchAction>,
        action: &LaunchAction,
        results: &LaunchTemplateResult,
    ) -> Option<LaunchTask>;

    fn format_string(
        &self,
        input: String,
        vars: &Vec<Var>,
        params: &LaunchTemplateResult,
    ) -> Option<String>;

    fn run_function(
        &self,
        function: &LaunchFunction,
        template: &LaunchTemplate,
        results: &LaunchTemplateResult,
    ) -> Option<FunctionResult>;
}
pub trait TLaunchRunService {
    async fn launch_task(
        &self,
        task: LaunchFlatTask,
        window_id: String,
        project: Project,
        app: AppHandle,
        state: State<'_, SharedLaunchManager>,
    ) -> Result<String, LaunchError>;

    async fn write_to_launch(
        &self,
        id: String,
        text: String,
        state: State<'_, SharedLaunchManager>,
    );

    async fn close_task(&self, id: String, state: State<'_, SharedLaunchManager>);
}
