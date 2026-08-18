use crate::modules::app::commands::config::{get_file_templates, get_fs_ext_icons};
use crate::modules::app::{
    CONFIG_RECOVERY_SERVICE, CONFIG_SERVICE, FS_READ_SERVICE, PACKAGE_SERVICE, SETTINGS_SERVICE,
};
use crate::modules::contexts::config::values::Cache;
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::project::app::traits::TPackageService;
use crate::modules::contexts::project::domain::entities::{
    Package, ProjectTemplate, SharedPackages,
};
use crate::modules::contexts::settings::app::traits::TSettingsService;
use crate::modules::services::traits::{TConfigRecoveryService, TConfigService};
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;
use base64::engine::general_purpose;
use base64::Engine;
use tauri::State;
use which::which;

#[tauri::command]
pub fn read_packages(state: State<'_, SharedPackages>) -> Result<Vec<Package>, ErrorDto> {
    let packs = PACKAGE_SERVICE.read_packages()?;
    {
        let mut list = state.lock().unwrap();
        *list = Vec::from(packs.clone())
    }
    Ok(packs)
}

#[tauri::command]
pub fn read_templates() -> Result<Vec<ProjectTemplate>, ErrorDto> {
    let mut templates = CONFIG_SERVICE.read_templates()?;
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let path_ = path_from![dir, "icons"];
    if !FS_READ_SERVICE.exists(path_.clone()) {
        CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
    }

    for i in 0..templates.len() {
        let meta = templates[i].meta.clone();
        if meta.is_none() {
            continue;
        }
        let icon = meta.unwrap().icon;
        let path = path_from![path_, icon];
        let ext = FS_READ_SERVICE.exists(path.clone());
        if !ext {
            templates[i].meta.as_mut().unwrap().icon = String::new();
            continue;
        };
        let file = PFile::from_path_reg(path.clone());
        let bytes = FS_READ_SERVICE.read_bytes(&file)?;
        let res = general_purpose::STANDARD.encode(bytes);
        let typ = file.ext();
        if typ.is_none() {
            templates[i].meta.as_mut().unwrap().icon = String::new();
            continue;
        }
        let mut typ = typ.unwrap();
        if typ == "svg" {
            typ += "+xml";
        }
        let res = format!("data:image/{};base64,{}", typ, res);
        templates[i].meta.as_mut().unwrap().icon = res;
    }
    Ok(templates)
}

#[tauri::command]
pub fn get_data_dir() -> Result<Path, ErrorDto> {
    CONFIG_SERVICE.get_data_dir().map_err(|e| e.into())
}

#[tauri::command]
pub fn read_themes() -> Result<Vec<String>, ErrorDto> {
    SETTINGS_SERVICE.read_themes().map_err(|e| e.into())
}

#[tauri::command]
pub fn get_cache(state: State<'_, SharedPackages>) -> Result<Cache, ErrorDto> {
    let packages = read_packages(state).unwrap_or(Vec::new());
    let templates = read_templates().unwrap_or(vec![ProjectTemplate::new()]);
    let data_dir_path = get_data_dir()?;
    let settings = CONFIG_SERVICE.get_settings()?;

    let groups = settings.general.project_groups.clone();
    //println!("SETTINGS {settings:?}");
    // println!("GROUPS {groups:?}");
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };
    let mut projects_dir = settings.general.path_to_projects.clone();
    // println!("PROJECT DIR {projects_dir}");
    if projects_dir.get().len() == 0 {
        projects_dir = CONFIG_SERVICE.get_projects_dir()?;
    }
    let icons = get_fs_ext_icons()?;
    let f_templates = get_file_templates()?;

    let shells = settings.run.shells.clone();

    let exists = |e: String| {
        if cfg!(target_os = "windows") && e == "bash".to_string() {
            return false;
        }
        which(e).is_ok()
    };
    let shells = shells
        .iter()
        .filter(|e| exists(e.to_string().clone()))
        .map(|e| e.to_string())
        .collect::<Vec<String>>();

    let themes = read_themes()?;

    let res = Cache {
        settings,
        templates,
        packages,
        data_dir_path,
        groups,
        os: os.to_string(),
        projects_dir,
        file_icons: icons,
        file_templates: f_templates,
        shells,
        themes,
    };
    Ok(res)
}
