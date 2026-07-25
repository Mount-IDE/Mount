use crate::modules::app::{CONFIG_SERVICE, FS_READ_SERVICE, PARSING_SERVICE};
use crate::modules::contexts::filesystem::app::traits::TFSReadService;
use crate::modules::contexts::filesystem::app::utils::make_path;
use crate::modules::contexts::filesystem::domain::entities::PFile;
use crate::modules::contexts::settings::app::traits::TSettingsService;
use crate::modules::contexts::settings::domain::entities::Theme;
use crate::modules::services::traits::{TConfigService, TParsingService};
use crate::modules::shared::kernel::errors::SettingsError;

pub struct SettingsService();

impl TSettingsService for SettingsService {
    fn read_themes(&self) -> Result<Vec<Theme>, SettingsError> {
        let dir = CONFIG_SERVICE.get_data_dir()?;
        let themes_path = make_path(vec![dir.get().as_str(), "themes.json"]);
        let file = PFile::from_path_reg(themes_path);
        let text = FS_READ_SERVICE.read_file(&file)?;
        let json = PARSING_SERVICE._from_string::<Vec<Theme>>(text)?;
        Ok(json)
    }
}
