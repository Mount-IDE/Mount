use crate::modules::shared::kernel::values::Path;

pub fn split_path(path: &Path) -> Vec<String>{
    if cfg!(target_os = "windows") {
        let splited = path.get().split('\\').collect::<Vec<String>>();
        splited
    }
    else {
        let splited = path.get().split('/').collect::<Vec<String>>();
        splited
    }
}