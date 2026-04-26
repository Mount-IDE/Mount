use crate::modules::contexts::project::domain::entities::TemplateStartup;
use crate::modules::shared::kernel::values::Schema;

pub fn t_id() ->String{"opie".to_string()}
pub fn t_name()->String{"Empty Project".to_string()}
pub fn t_schema()->Schema{Schema(0)}
pub fn t_meta_icon()->String{"empty.svg".to_string()}
pub fn t_startup()->TemplateStartup{
    TemplateStartup::new()
}
