use crate::commands::config::{get_file_templates, get_fs_ext_icons};
use crate::commands::project::get_recent_projects;
use crate::domain::ErrorDto;
use base64::Engine;
use base64::engine::general_purpose;
use cache::{Cache, PackageInner};
use config::traits::{TConfigService, TSettingsService};
use config::{CONFIG_SERVICE, SETTINGS_SERVICE};
use filesystem::FS_READ_SERVICE;
use filesystem::domain::PFile;
use filesystem::traits::TFSReadService;
use mount_core::domain::Path;
use mount_core::make_path;
use project::project::domain::{Package, ProjectTemplate, SharedPackages};
use project::project::traits::{TPackageService, TProjectService};
use project::{PACKAGE_SERVICE, PROJECT_SERVICE};
use recovery::CONFIG_RECOVERY_SERVICE;
use recovery::traits::TConfigRecoveryService;
use std::collections::HashMap;
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
    let mut templates = PROJECT_SERVICE.read_templates()?;
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let path_ = make_path![dir, "icons"];
    if !FS_READ_SERVICE.exists(path_.clone()) {
        CONFIG_RECOVERY_SERVICE.repair_data_dir()?;
    }

    for i in 0..templates.len() {
        let meta = templates[i].meta.clone();
        if meta.is_none() {
            continue;
        }
        let icon = meta.unwrap().icon;
        let path = make_path![path_, icon];
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

    let shells = shells.iter();
    let shells = shells.filter(|e| exists(e.to_string()));
    let shells = shells.map(|e| e.clone()).collect::<Vec<String>>();

    let themes = read_themes()?;

    let recent = get_recent_projects().unwrap_or(vec![]);

    let packages = read_packages(state).unwrap_or(Vec::new());
    let res = Cache {
        recent_projects: recent,
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

#[tauri::command]
pub fn get_meta_of_selected_packages(
    packs: Vec<Package>,
) -> Result<HashMap<String, PackageInner>, ErrorDto> {
    let mut res = HashMap::<String, PackageInner>::new();
    for i in packs {
        let config = PACKAGE_SERVICE.read_config(i.id.clone());
        let inner = PackageInner {
            main: i.clone(),
            config: config.unwrap_or("".to_string()),
        };
        res.insert(i.id, inner);
    }
    Ok(res)
}
