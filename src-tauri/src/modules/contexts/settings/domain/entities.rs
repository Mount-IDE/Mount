use crate::modules::contexts::project::domain::values::ProjectMeta;
use crate::modules::shared::kernel::values::Path;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use ts_rs::TS;

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct Settings {
    #[serde(default)]
    pub doctype: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub general: GeneralSettings,
    #[serde(default)]
    pub appearance: Appearance,
    #[serde(default)]
    pub run: Run,
}

impl Settings {
    pub fn new() -> Settings {
        Self {
            doctype: String::from("opie/mount"),
            version: String::from("1.0.0"),
            general: GeneralSettings::new(),
            appearance: Appearance::new(),
            run: Run::default(),
        }
    }
}
///
///
///
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
pub struct Run {
    pub shells: Vec<String>,
}

impl Default for Run {
    fn default() -> Self {
        Self {
            shells: vec![
                "bash".to_string(),
                "sh".to_string(),
                "zsh".to_string(),
                "ksh".to_string(),
                "cmd".to_string(),
                "powershell".to_string(),
            ],
        }
    }
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct GeneralSettings {
    pub path_to_projects: Path,
    pub project_groups: Vec<String>,
}

impl Default for GeneralSettings {
    fn default() -> Self {
        Self {
            path_to_projects: Path("".to_string()),
            project_groups: vec![String::from("general")],
        }
    }
}

impl GeneralSettings {
    pub fn new() -> GeneralSettings {
        Self {
            path_to_projects: Path(String::new()),
            project_groups: Vec::new(),
        }
    }
}

///
///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
pub struct Appearance {
    pub theme: String,
    pub lang: String,
    pub font: String,
    pub font_size: u64,
}

impl Default for Appearance {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            lang: "en".to_string(),
            font: "Jetbrains Mono".to_string(),
            font_size: 16,
        }
    }
}

impl Appearance {
    pub fn new() -> Appearance {
        Self {
            theme: String::new(),
            lang: String::new(),
            font: String::new(),
            font_size: 16,
        }
    }
}

///
///
///
///
#[derive(Serialize, Deserialize, Clone, Debug, TS)]
#[ts(export)]
pub struct RecentProject {
    pub name: String,
    pub path: Path,
    pub last_opened: u64,
    pub meta: ProjectMeta,
    pub(crate) packages: Vec<String>,
}

///
///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Theme {
    #[serde(default)]
    id: String,
    #[serde(default)]
    name: String,
    meta: Option<ThemeMeta>,
    pages: Option<Vec<ThemePage>>,
    vars: Option<Vec<ThemeVar>>,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeMeta {
    version: Option<String>,
    authors: Option<Vec<String>>,
    description: Option<String>,
    source: Option<String>,
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemePage {
    #[serde(default)]
    id: String,
    #[serde(default)]
    elements: Vec<ThemeElement>,
}

pub type ThemeLightElement = HashMap<String, Value>;

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeElement {
    #[serde(default)]
    selector: String,

    color: Option<String>,              //
    background_color: Option<String>,   //
    background_image: Option<String>,   //
    background_opacity: Option<String>, //

    border: Option<String>,        //
    border_bottom: Option<String>, //
    border_left: Option<String>,   //
    border_right: Option<String>,  //
    border_top: Option<String>,    //
    border_x: Option<String>,      //
    border_y: Option<String>,      //
    border_size: Option<String>,   //
    border_color: Option<String>,  //
    border_style: Option<String>,  //

    border_radius: Option<String>, //

    margin: Option<String>,        //
    margin_left: Option<String>,   //
    margin_right: Option<String>,  //
    margin_top: Option<String>,    //
    margin_bottom: Option<String>, //
    margin_x: Option<String>,      //
    margin_y: Option<String>,      //

    padding: Option<String>,        //
    padding_left: Option<String>,   //
    padding_right: Option<String>,  //
    padding_top: Option<String>,    //
    padding_bottom: Option<String>, //
    padding_x: Option<String>,      //
    padding_y: Option<String>,      //

    width: Option<String>,  //
    height: Option<String>, //

    opacity: Option<String>, //

    font: Option<String>,        //
    font_family: Option<String>, //
    font_weight: Option<String>, //
}

///
///
///
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ThemeVar {
    #[serde(default)]
    name: String,
    #[serde(default)]
    value: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
            meta: None,
            pages: None,
            vars: None,
        }
    }
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            id: "opie:dark".to_string(),
            name: "Dark".to_string(),
            meta: Some(ThemeMeta {
                authors: Some(vec!["opie".to_string()]),
                description: None,
                version: None,
                source: None,
            }),
            pages: None,
            vars: None,
        }
    }

    pub fn light() -> Self {
        todo!()
    }
    pub fn islands_dark() -> Self {
        todo!()
    }
    pub fn islands_light() -> Self {
        todo!()
    }
}
