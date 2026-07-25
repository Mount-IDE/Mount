use crate::modules::contexts::settings::domain::entities::Theme;
use crate::modules::shared::kernel::errors::SettingsError;

pub trait TSettingsService {
    fn read_themes(&self) -> Result<Vec<Theme>, SettingsError>;
}
