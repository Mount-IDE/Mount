use crate::modules::contexts::project::domain::entities::{
    Action, Project, ProjectTemplate, Var, _Task,
};
use crate::modules::contexts::project::domain::values::{
    CreateProjectResult, CreateProjectTemplate,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{Path, Val};

pub trait TProjectService {
    fn create_project(&self, proj: &Project) -> Result<(), ProjectError>;
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError>;
    fn delete_project(&self, project_path: &Path) -> Result<(), ProjectError>;
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError>;
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectError>;
    fn save_project(&self, project: &Project) -> Result<(), ProjectError>;
    fn remove_from_recents(&self, proj: &Project) -> Result<(), ProjectError>;
    fn add_to_recents(&self, project: &Project) -> Result<(), ProjectError>;
}

pub trait TActionProjectService {
    fn compile(
        &self,
        template: &ProjectTemplate,
        values: &CreateProjectResult,
        vars: &Vec<Var>,
    ) -> Option<(Vec<Var>, Vec<_Task>)>;
    fn precompile_condition(
        &self,
        sections: &Vec<CreateProjectTemplate>,
        vars: &Vec<Var>,
        action: &Action,
    ) -> bool;

    fn get_from_vars(&self, vars: &Vec<Var>, addr: String) -> Option<Val>;
    fn get_from_params(&self, params: &Vec<CreateProjectTemplate>, addr: String) -> Option<Val>;

    fn format_string(
        &self,
        string: String,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
    ) -> Option<String>;

    fn compile_vars(
        &self,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
    ) -> Option<Vec<Var>>;

    fn make_task(
        &self,
        action: &Action,
        actions: &Vec<Action>,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        os: &String,
    ) -> Option<_Task>;

    fn run_tasks(&self, project: &Project, tasks: &Vec<_Task>);

    fn run_task(&self, task: &_Task, path: &Path) -> i8;
}
