use crate::modules::app::{CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, PROJECT_SERVICE};
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::filesystem::domain::values::FileType;
use crate::modules::contexts::project::app::traits::TProjectService;
use crate::modules::contexts::project::domain::entities::Project;
use crate::modules::contexts::settings::domain::entities::RecentProject;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;

#[tauri::command]
pub fn get_recent_projects() -> Result<Vec<RecentProject>, ErrorDto> {
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let file = PFile{name: "recent-projects.json".to_string(), path:dir, typ: FileType::REGULAR};
    let ext = FS_READ_SERVICE.exist_file(&file);
    if !ext {
        let _ = CONFIG_RECOVERY_SERVICE.check_data_dir()?;
        println!("get second");
    }
    let recent = PROJECT_SERVICE.get_recent_projects()?;
    println!("GET_RECENT_OK");
    Ok(recent)
}


#[tauri::command]
pub fn read_recent_projects(recent: Vec<RecentProject>) -> Result<Vec<Project>, ErrorDto> {
    let vec_path = recent.iter().map(|e|e.path.clone()).collect::<Vec<Path>>();
    let mut res: Vec<Project>=vec![];

    for path in vec_path {
        let get = path.get().clone();

        let path_ = make_path(vec![get.as_str(), ".mount", "project.json"]);
        let file = PFile{name: "project.json".to_string(), path:Path(path_.clone()), typ: FileType::REGULAR};
        if FS_READ_SERVICE.exist_file(&file) {
            let config = FS_READ_SERVICE.read_file(&file);
            if config.is_err(){
                continue;
            }
            let config = config.unwrap();
            let json = serde_json::from_str::<Project>(config.as_str());
            if json.is_err(){
                continue;
            }
            let json = json.unwrap();
            res.push(json);
        }
    }
    println!("READ_RECENT_OK");
    Ok(res)
}