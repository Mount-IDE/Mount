
use crate::modules::app::PROJECT_SERVICE;
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::contexts::project::domain::values::ProjectToFS;
use crate::modules::shared::kernel::entities::ProjectError;
use crate::modules::shared::kernel::values::Path;

#[tauri::command]
pub fn show_win(window: tauri::Window) {
    let res = window.show();
    match res {
        Err(e)=>println!("Error: {:?}", e),
        Ok(e)=>println!("Ok: {:?}", e),
    }
}

#[tauri::command]
pub fn get_projects(cwd: String) -> Result<Vec<Project>, ProjectToFS> {
    let res = PROJECT_SERVICE.get_projects(Path(cwd));
    res
}