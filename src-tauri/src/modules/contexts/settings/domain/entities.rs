use crate::modules::contexts::project::domain::values::ProjectMeta;
use crate::modules::shared::kernel::values::{Path, Schema};
use serde::{Deserialize, Serialize};
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
            theme: "opie.dark".to_string(),
            lang: "en".to_string(),
            font: "Jetbrains Mono".to_string(),
            font_size: 16,
        }
    }
}

impl Appearance {
    pub fn new() -> Appearance {
        Self {
            theme: String::from("opie.dark"),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ITheme {
    pub id: String,
    pub name: String,
    pub schema: Schema,
    pub meta: Option<IThemeMeta>,
    pub colors: Option<Vec<ThemeColor>>,
    pub elements: Option<ThemeElements>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeColor {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct IThemeMeta {
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThemeInner {
    SINGLE(String),
    STRUCT(_ThemeInner),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct _ThemeInner {
    pub this: Option<String>,
    pub top: Option<String>,
    pub bottom: Option<String>,
    pub left: Option<String>,
    pub right: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeTitleBar {
    pub this: Option<ThemeTitleBarThis>,
    pub button: Option<ThemeTitleBarButton>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeTitleBarThis {
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeTitleBarButton {
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
    pub hover: Option<ThemeTitleBarButtonHover>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ThemeTitleBarButtonHover {
    SINGLE(_ThemeTitleBarButtonHoverSingle),
    ARRAY(_ThemeTitleBarButtonHoverArray),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct _ThemeTitleBarButtonHoverSingle {
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct _ThemeTitleBarButtonHoverArray {
    pub id: _ThemeTitleBarVariant,
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum _ThemeTitleBarVariant {
    Wrap,
    Resize,
    Close,
    Other,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectInner {
    pub color: Option<String>,
    pub underscore: Option<String>,
    pub hover: Option<ThemeProjectInnerHover>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectInnerHover {
    pub color: Option<String>,
    pub underscore: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemePage {
    pub background: Option<String>,
    pub rounded: Option<String>,
    pub border: Option<ThemeInner>,
    pub padding: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeElements {
    pub mainpage: Option<ThemeMainPage>,
    pub project_space: Option<ThemeProjectSpace>,
    pub launch: Option<ThemeLaunch>,
    pub settings: Option<ThemeSettings>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeMainPage {
    pub left: Option<ThemePage>,
    pub right: Option<ThemePage>,
    pub filters: Option<ThemeFilters>,
    pub project: Option<ThemeProject>,
    pub title_bar: Option<ThemeTitleBar>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProject {
    pub this: Option<ThemeProjectThis>,
    pub icon: Option<ThemeProjectIcon>,
    pub name: Option<ThemeProjectInner>,
    pub path: Option<ThemeProjectInner>,
    pub packages: Option<ThemeProjectInner>,
    pub tags: Option<ThemeProjectInner>,
    pub more: Option<ThemeProjectMore>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectMore {
    pub img: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectIcon {
    pub rounded: Option<String>,
    pub border: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectThis {
    pub background: Option<String>,
    pub padding: Option<ThemeInner>,
    pub rounded: Option<String>,
    pub hover: Option<ThemeProjectThisHover>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectThisHover {
    pub background: Option<String>,
    pub rounded: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeFilters {
    pub border: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectSpace {
    pub this: Option<ThemeProjectSpaceThis>,
    pub mini_aside: Option<ThemeMiniAside>,
    pub aside: Option<ThemeAside>,
    pub center: Option<ThemeCenter>,
    pub bottom: Option<ThemePage>,
    pub footer: Option<ThemePage>,
    pub title_bar: Option<ThemeTitleBar>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeAside {
    pub left: Option<ThemePage>,
    pub right: Option<ThemePage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeCenter {
    pub this: Option<ThemePage>,
    pub file_list: Option<ThemeFileList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeFileList {
    pub this: Option<ThemeFileListThis>,
    pub element: Option<ThemeFileListElement>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeFileListThis {
    pub border: Option<ThemeInner>,
    pub background: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeFileListElement {
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
    pub rounded: Option<String>,
    pub hover: Option<ThemeHF>,
    pub focus: Option<ThemeHF>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeHF {
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeMiniAside {
    pub right: Option<ThemeMiniAsideIn>,
    pub left: Option<ThemeMiniAsideIn>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeMiniAsideIn {
    pub border: Option<ThemeInner>,
    pub background: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeProjectSpaceThis {
    pub background: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeLaunch {
    pub this: Option<ThemePage>,
    pub left: Option<ThemeLaunchLeft>,
    pub right: Option<ThemeInner>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeLaunchLeft {
    pub this: Option<ThemePage>,
    pub list: Option<ThemeLaunchList>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeLaunchList {
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
    pub padding: Option<ThemeInner>,
    pub rounded: Option<String>,
    pub hover: Option<ThemeHF>,
    pub focus: Option<ThemeHF>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeSettings {
    pub this: Option<ThemePage>,
    pub left: Option<ThemeSettingsLeft>,
    pub right: Option<ThemePage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeSettingsLeft {
    pub this: Option<ThemePage>,
    pub list: Option<ThemeSettingsLeftList>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeSettingsLeftList {
    pub background: Option<String>,
    pub border: Option<ThemeInner>,
    pub rounded: Option<String>,
    pub hover: Option<ThemeHF>,
    pub focus: Option<ThemeHF>,
}

impl Default for ITheme {
    fn default() -> Self {
        Self {
            id: String::from("opie.dark"),
            name: "Dark".to_string(),
            meta: None,
            colors: None,
            schema: Schema(1),
            elements: None,
        }
    }
}

impl ITheme {
    pub fn light() -> Self {
        Self {
            id: "opie.light".to_string(),
            name: "Light".to_string(),
            meta: None,
            colors: Some(vec![
                ThemeColor {
                    name: "bg".to_string(),
                    value: "#fff".to_string(),
                },
                ThemeColor {
                    name: "title".to_string(),
                    value: "#000".to_string(),
                },
                ThemeColor {
                    name: "subtitle".to_string(),
                    value: "#d9d9d9".to_string(),
                },
            ]),
            elements: None,
            schema: Schema(1),
        }
    }
}
