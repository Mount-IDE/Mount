use crate::modules::contexts::settings::domain::entities::ITheme;
use crate::modules::shared::kernel::errors::SettingsError;

pub trait TSettingsService {
    fn read_themes(&self) -> Result<Vec<ITheme>, SettingsError>;
}
