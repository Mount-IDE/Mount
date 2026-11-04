use crate::modules::contexts::project::domain::entities::{
    Button, Package, PackageAction, Project, ProjectTag, ProjectTemplate, SharedPackages, Var,
    _Task,
};
use crate::modules::contexts::project::domain::values::{
    CreateProjectPackageResults, CreateProjectResult, CreateProjectTemplate, ProjectMeta,
    ResultsRecord,
};
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::shared::kernel::errors::ProjectError;
use crate::modules::shared::kernel::values::{Dependency, Path, Val};
use std::collections::HashMap;
use tauri::State;

pub trait TCreateProjectService {
    fn compile_vars(&self, vars: &mut Vec<Var>, results: &HashMap<String, Val>);

    #[allow(unused)]
    fn create_project(
        &self,
        template: ProjectTemplate,
        results: CreateProjectResult,
        packages: Vec<String>,
        tags: Vec<ProjectTag>,
        pack_results: CreateProjectPackageResults,
        window: String,
        pack_state: State<'_, SharedPackages>,
    ) -> Result<Project, ProjectError>;

    fn add_license(
        &self,
        proj: &Project,
        license: String,
        results: &CreateProjectResult,
    ) -> Result<(), ProjectError>;

    fn check_dependencies(
        &self,
        window: String,
        template: &ProjectTemplate,
    ) -> Result<(), ProjectError>;

    fn check_dependency(&self, dependency: &Dependency) -> bool;

    fn add_vars(
        &self,
        vars: &mut Vec<Var>,
        additions: &ProjectMeta,
        name: String,
        path: String,
        template: &ProjectTemplate,
    );

    fn modify_template(&self, template: &mut ProjectTemplate);

    fn make_meta(
        &self,
        additions: Option<&HashMap<String, Val>>,
        tags: &Vec<ProjectTag>,
    ) -> ProjectMeta;
    fn make_buttons(&self) -> Vec<Button>;

    fn exec_tasks(
        &self,
        template: &ProjectTemplate,
        vars: &Vec<Var>,
        results: &HashMap<String, Val>,
        path: Path,
    );
}

pub trait TProjectService {
    fn open_project(&self, project_path: &Path) -> Result<Project, ProjectError>;
    fn delete_project(&self, project_path: &Path) -> Result<Project, ProjectError>;
    fn get_projects(&self, dir: &Path) -> Result<Vec<Project>, ProjectError>;
    fn get_recent_projects(&self) -> Result<Vec<RecentProject>, ProjectError>;
    fn save_project(&self, project: &Project) -> Result<(), ProjectError>;

    fn save_packages(&self, packs: Vec<Package>, path: Path) -> Result<(), ProjectError>;

    fn remove_from_recents(&self, proj: &Project) -> Result<(), ProjectError>;
    fn add_to_recents(&self, project: &Project) -> Result<(), ProjectError>;
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

    #[deprecated]
    fn format(
        &self,
        string: String,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        is_pack: bool,
        pack_params: &ResultsRecord,
    ) -> Option<String>;

    #[deprecated]
    fn get_from_vars(&self, vars: &Vec<Var>, addr: String) -> Option<Val>;

    #[deprecated]
    fn get_from_params(&self, params: &Vec<CreateProjectTemplate>, addr: String) -> Option<Val>;

    #[deprecated]
    fn get_from_pack_params(&self, params: &ResultsRecord, addr: String) -> Option<Val>;

    fn compile_vars(
        &self,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
    ) -> Option<Vec<Var>>;

    #[deprecated]
    fn make_task(
        &self,
        action: &PackageAction,
        actions: &Vec<PackageAction>,
        vars: &Vec<Var>,
        params: &Vec<CreateProjectTemplate>,
        is_pack: bool,
        pack_params: &ResultsRecord,
    ) -> Option<_Task>;

    #[deprecated]
    fn run_tasks(&self, project: &Project, tasks: &Vec<_Task>, window: String);

    #[deprecated]
    fn run_task(&self, task: &_Task, path: &Path) -> i8;
}

pub trait TPackageService {
    fn read_packages(&self) -> Result<Vec<Package>, ProjectError>;

    #[allow(unused)]
    fn add_package(&self, pack: Package) -> Result<(), ProjectError>;

    #[allow(unused)]
    fn rem_package(&self, pack: Package) -> Result<(), ProjectError>;

    fn read_config(&self, id: String) -> Result<String, ProjectError>;
}
