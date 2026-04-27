use crate::modules::app::{CONFIG_SERVICE};
use crate::modules::services::traits::TConfigService;
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;

#[tauri::command]
pub fn get_home_dir()->Result<Path, ErrorDto>{
    let res = CONFIG_SERVICE.get_home_dir()?;
    Ok(res)
}

#[tauri::command]
pub fn get_projects_dir()->Result<Path, ErrorDto>{
    let res = CONFIG_SERVICE.get_projects_dir()?;
    Ok(res)
}


#[tauri::command]
pub fn get_groups()->Result<Vec<String>, ErrorDto>{
    let settings = CONFIG_SERVICE.get_settings()?;
    let mut groups = settings.general.project_groups;
    if !groups.contains(&"general".to_string()){
        groups.insert(0, "general".to_string())
    }
    Ok(groups.clone())
}