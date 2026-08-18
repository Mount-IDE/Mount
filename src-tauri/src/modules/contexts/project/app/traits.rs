use crate::modules::contexts::project::domain::entities::{
    Package, PackageAction, Project, ProjectTemplate, Var, _Task,
};
use crate::modules::contexts::project::domain::values::{
    CreateProjectPackageResults, CreateProjectResult, CreateProjectTemplate, ResultsRecord,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{Dependency, Path, Val};

pub trait TProjectService {
    fn create_project(&self, proj: &Project) -> Result<(), ProjectError>;
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError>;
    fn delete_project(&self, project_path: &Path) -> Result<Project, ProjectError>;
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError>;
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectError>;
    fn save_project(&self, project: &Project) -> Result<(), ProjectError>;
    fn remove_from_recents(&self, proj: &Project) -> Result<(), ProjectError>;
    fn add_to_recents(&self, project: &Project) -> Result<(), ProjectError>;

    fn check_dependencies(&self, dependencies: Vec<Dependency>) -> Vec<Dependency>;
}

pub trait TActionProjectService {
    fn compile(
        &self,
        template: &ProjectTemplate,
        values: &CreateProjectResult,
        vars: &Vec<Var>,
        packages: &Vec<Package>,
        pack_results: &CreateProjectPackageResults,
    ) -> Option<(Vec<Var>, Vec<_Task>)>;
    fn precompile_condition(
        &self,
        sections: &Vec<CreateProjectTemplate>,
        vars: &Vec<Var>,
        action: &PackageAction,
        is_pack: bool,
        pack_params: &ResultsRecord,
    ) -> bool;

    fn format(
        &self,
        string: String,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        is_pack: bool,
        pack_params: &ResultsRecord,
    ) -> Option<String>;

    fn get_from_vars(&self, vars: &Vec<Var>, addr: String) -> Option<Val>;
    fn get_from_params(&self, params: &Vec<CreateProjectTemplate>, addr: String) -> Option<Val>;

    fn get_from_pack_params(&self, params: &ResultsRecord, addr: String) -> Option<Val>;

    fn compile_vars(
        &self,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
    ) -> Option<Vec<Var>>;

    fn make_task(
        &self,
        action: &PackageAction,
        actions: &Vec<PackageAction>,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        is_pack: bool,
        pack_params: &ResultsRecord,
    ) -> Option<_Task>;

    fn run_tasks(&self, project: &Project, tasks: &Vec<_Task>, window: String);

    fn run_task(&self, task: &_Task, path: &Path) -> i8;
}

pub trait TPackageService {
    fn read_packages(&self) -> Result<Vec<Package>, ProjectError>;

    fn add_package(&self, pack: Package) -> Result<(), ProjectError>;

    fn rem_package(&self, pack: Package) -> Result<(), ProjectError>;
}

pub trait TPackageCompileService {
    fn compile_package_actions(
        &self,
        pack: Package,
        results: CreateProjectResult,
    ) -> Result<(Vec<Var>, Vec<_Task>), ProjectError>;
}
