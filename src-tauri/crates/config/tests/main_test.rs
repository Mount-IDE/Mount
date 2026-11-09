use config::CONFIG_SERVICE;
use config::traits::TConfigService;

/*#[test]
pub fn read_themes() {
    let theme = SETTINGS_SERVICE.read_themes();
    log!("THEME {theme:?}");
    assert!(theme.is_ok());
}

#[test]
pub fn read_settings() {
    let settings = CONFIG_SERVICE.read_settings();
    log!("SETTINGS {settings:?}");
    assert!(settings.is_ok());
}


#[test]
pub fn get_data_dir() {
    let dir = CONFIG_SERVICE.get_data_dir();
    log!("DATA DIR {dir:?}");
    assert!(dir.is_ok());
}


#[test]
pub fn get_home_dir() {
    let home = CONFIG_SERVICE.get_home_dir();
    log!("DATA DIR {home:?}");
    assert!(home.is_ok());
}

#[test]
pub fn get_projects_dir() {
    let dir = CONFIG_SERVICE.get_projects_dir();
    log!("PROJECTS DIR {dir:?}");
    assert!(dir.is_ok());
}


#[test]
pub fn get_settings() {
    let res = CONFIG_SERVICE.get_settings();
    log!("SETTINGS GET {res:?}");
    assert!(res.is_ok());
}


#[test]
pub fn get_file_templates() {
    let res = CONFIG_SERVICE.get_file_templates();
    log!("FILE TEMPLATES {res:?}");
    assert!(res.is_ok());
}

#[test]
pub fn get_licenses() {
    let res = CONFIG_SERVICE.get_licenses();
    log!("LICENSES {res:?}");
    assert!(res.is_ok());
}
*/
#[test]
pub fn get_shell() {
    let res = CONFIG_SERVICE.get_shell();
    assert_eq!(res, "powershell");
}
