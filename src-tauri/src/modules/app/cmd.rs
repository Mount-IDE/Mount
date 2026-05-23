use crate::modules::app::PROJECT_SERVICE;
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;

#[tauri::command]
pub fn show_win(window: tauri::Window) {
    let res = window.show();
    match res {
        Err(e) => println!("Error: {:?}", e),
        Ok(e) => println!("Ok: {:?}", e),
    }
}

#[tauri::command]
pub fn get_projects(cwd: String) -> Result<Vec<Project>, ErrorDto> {
    PROJECT_SERVICE
        .get_projects(&Path(cwd))
        .map_err(|e| e.into())
}
