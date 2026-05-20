use crate::modules::app::FS_READ_SERVICE;
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::domain::entities::PDirectory;
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;

#[tauri::command]
pub fn read_dir_rec(cwd: String)->Result<PDirectory, ErrorDto>{
    let dir_ = PDirectory::from_path(&Path(cwd.clone()));
    let dir = FS_READ_SERVICE.read_dir_recursive(&dir_)?;
    Ok(dir)
}




