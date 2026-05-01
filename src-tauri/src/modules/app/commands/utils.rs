use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::shared::kernel::values::Path;

#[tauri::command]
pub fn make_path_command(components:Vec<String>)->Path{
    make_path(components.iter().map(|x| x.as_str()).collect())
}