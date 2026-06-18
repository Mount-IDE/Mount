use crate::modules::contexts::launch::app::functions::FunctionResult;
use crate::modules::contexts::launch::domain::entities::{
    LaunchAction, LaunchFunction, LaunchObject, LaunchTask, LaunchTemplate,
    LaunchTemplateReference, LaunchTemplateResult,
};
use crate::modules::contexts::project::domain::entities::{Project, ProjectTemplate, Var};
use crate::modules::contexts::project::domain::values::CreateProjectTemplate;
use crate::modules::shared::kernel::errors::LaunchError;
use crate::modules::shared::kernel::values::Val;

pub trait TLaunchCompileService {
    fn compile_reference(
        &self,
        l_template: &LaunchTemplate,
        template: &ProjectTemplate,
        results: &LaunchTemplateResult,
    ) -> Option<LaunchTemplateReference>;
    fn compile_object(
        &self,
        reference: &LaunchTemplateReference,
        vars: &Vec<Var>,
        template: LaunchTemplate,
        results: LaunchTemplateResult,
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
    fn launch_task(&self, project: &Project, object: LaunchObject) -> Result<(), LaunchError>;
}
