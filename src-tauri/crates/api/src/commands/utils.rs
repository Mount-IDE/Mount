use crate::domain::ErrorDto;
use base64::Engine;
use base64::engine::general_purpose;
use config::CONFIG_SERVICE;
use config::traits::TConfigService;
use filesystem::FS_READ_SERVICE;
use filesystem::domain::PFile;
use filesystem::traits::TFSReadService;
use macroses::join;
use mount_core::domain::Path;
use mount_core::make_path;
use project::PROJECT_SERVICE;
use project::project::domain::Project;
use project::project::traits::TProjectService;

#[tauri::command]
pub fn make_path_command(components: Vec<String>) -> Path {
    make_path![components]
    //make_path(components.iter().map(|x| x.as_str()).collect())
}

#[tauri::command]
pub fn make_path_from_icon(
    components: String,
    path: String,
    code: bool,
    builtin: bool,
) -> Result<String, ErrorDto> {
    if builtin {
        let path = join!["/", "/builtin/aside-icons/aside", components];
        return Ok(path);
    }
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let path = make_path![dir, path, components];
    if code {
        return make_base64(path.get());
    }
    Ok(path.get())
}

#[tauri::command]
pub fn make_base64(src: String) -> Result<String, ErrorDto> {
    let file = PFile::from_path_reg(Path(src));
    let bytes = FS_READ_SERVICE.read_bytes(&file)?;
    let res = general_purpose::STANDARD.encode(bytes);
    let typ = file.ext();
    if typ.is_none() {
        return Ok(String::new());
    }
    let mut typ = typ.unwrap();
    if typ == "svg" {
        typ += "+xml";
    }
    let res = format!("data:image/{};base64,{}", typ, res);
    Ok(res)
}

#[tauri::command]
pub fn show_win(window: tauri::Window) {
    let _ = window.show();
}

#[tauri::command]
pub fn get_projects(cwd: String) -> Result<Vec<Project>, ErrorDto> {
    PROJECT_SERVICE
        .get_projects(&Path(cwd))
        .map_err(|e| e.into())
}
