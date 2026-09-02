use crate::modules::app::{CONFIG_SERVICE, FS_READ_SERVICE};
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::path_from;
use crate::modules::contexts::filesystem::app::utils::PathPart;
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::services::traits::TConfigService;
use crate::modules::shared::kernel::entities::ErrorDto;
use crate::modules::shared::kernel::values::Path;
use base64::engine::general_purpose;
use base64::Engine;

#[tauri::command]
pub fn make_path_command(components: Vec<String>) -> Path {
    path_from![components]
    //make_path(components.iter().map(|x| x.as_str()).collect())
}

#[tauri::command]
pub fn make_path_from_icon(
    components: String,
    path: String,
    code: bool,
) -> Result<String, ErrorDto> {
    let dir = CONFIG_SERVICE.get_data_dir()?;
    let path = path_from![dir, path, components];
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
