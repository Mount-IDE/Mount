use crate::modules::contexts::settings::app::traits::TSettingsService;
use crate::modules::contexts::settings::domain::entities::ITheme;
use crate::modules::shared::kernel::errors::SettingsError;

pub struct SettingsService();

impl TSettingsService for SettingsService {
    fn read_themes(&self) -> Result<Vec<ITheme>, SettingsError> {
        Ok(vec![])
    }
}
