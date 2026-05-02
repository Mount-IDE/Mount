use base64::Engine;
use base64::engine::general_purpose;
use crate::modules::app::{CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE};
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::{make_path, make_path_string};
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::project::domain::entities::{ProjectPackage, ProjectTemplate};
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;

#[tauri::command]
pub fn read_packages()->Result<Vec<ProjectPackage>, ErrorDto>{
    let packs = CONFIG_SERVICE.read_packages()?;
    Ok(packs)
}



#[tauri::command]
pub fn read_templates() -> Result<Vec<ProjectTemplate>, ErrorDto> {
    let mut templates = CONFIG_SERVICE.read_templates()?;
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let path_ = make_path_string(vec![dir.get().clone().as_str(), "icons"]);
    if !FS_READ_SERVICE.exists(Path(path_.clone())) {
        CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
    }

    for i in 0..templates.len() {
        let meta = templates[i].meta.clone();
        if meta.is_none(){continue;}
        let icon= meta.unwrap().icon;
        let path = make_path(vec![
            path_.clone().as_str(),
            icon.clone().as_str()
        ]);
        let ext = FS_READ_SERVICE.exists(path.clone());
        if !ext {
            templates[i].meta.as_mut().unwrap().icon = String::new();
            continue;
        };
        let file = PFile::from_path_reg(path.clone());
        let bytes = FS_READ_SERVICE.read_bytes(&file)?;
        let res= general_purpose::STANDARD.encode(bytes);
        let typ = file.ext();
        if typ.is_none(){
            templates[i].meta.as_mut().unwrap().icon = String::new();
            continue;
        }
        let mut typ = typ.unwrap();
        if typ=="svg"{
            typ+="+xml";
        }
        let res = format!("data:image/{};base64,{}", typ, res);
        templates[i].meta.as_mut().unwrap().icon = res;
    }
    Ok(templates)
}



#[tauri::command]
pub fn get_data_dir() -> Result<Path, ErrorDto> {
    CONFIG_SERVICE.get_data_dir().map_err(|e|e.into())
}