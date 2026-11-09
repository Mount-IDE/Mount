use crate::domain::ErrorDto;
use crate::get_data_dir;
use config::CONFIG_SERVICE;
use config::domain::config::FsConfigIcons;
use config::domain::settings::Settings;
use config::traits::TConfigService;
use filesystem::FS_READ_SERVICE;
use filesystem::domain::{ConfigFsTemplate, PFile};
use filesystem::traits::TFSReadService;
use mount_core::domain::Path;
use mount_core::make_path;
use parsing::traits::ParsingError;
use recovery::CONFIG_RECOVERY_SERVICE;
use recovery::traits::TConfigRecoveryService;

#[tauri::command]
pub fn get_home_dir() -> Result<Path, ErrorDto> {
    let res = CONFIG_SERVICE.get_home_dir()?;
    Ok(res)
}

#[tauri::command]
pub fn get_projects_dir() -> Result<Path, ErrorDto> {
    let res = CONFIG_SERVICE.get_projects_dir()?;
    Ok(res)
}

#[tauri::command]
pub fn get_groups() -> Result<Vec<String>, ErrorDto> {
    let settings = CONFIG_SERVICE.get_settings()?;
    let mut groups = settings.general.project_groups;
    if !groups.contains(&"general".to_string()) {
        groups.insert(0, "general".to_string())
    }
    Ok(groups.clone())
}

#[tauri::command]
pub fn get_fs_ext_icons() -> Result<Vec<FsConfigIcons>, ErrorDto> {
    let path = get_data_dir!();
    let path = make_path![path, "file_ext_icons.json"];
    if !FS_READ_SERVICE.exists(path.clone()) {
        CONFIG_RECOVERY_SERVICE.check_data_dir()?;
    }
    let file = PFile::from_path_reg(path.clone());
    let content = FS_READ_SERVICE.read_file(&file)?;
    let str = serde_json::from_str::<Vec<FsConfigIcons>>(content.as_str()).map_err(|e| {
        ParsingError::Deserialize {
            path,
            json: content,
            err: e,
        }
    })?;

    Ok(str)
}

#[tauri::command]
pub fn get_file_templates() -> Result<Vec<ConfigFsTemplate>, ErrorDto> {
    CONFIG_SERVICE.get_file_templates().map_err(|e| e.into())
}

#[tauri::command]
pub fn get_os() -> String {
    if cfg!(target_os = "windows") {
        "windows".to_string()
    } else if cfg!(target_os = "macos") {
        "macos".to_string()
    } else {
        "linux".to_string()
    }
}

#[tauri::command]
pub fn save_settings(settings: Settings) -> Result<(), ErrorDto> {
    CONFIG_SERVICE.save_settings(&settings)?;
    Ok(())
}
